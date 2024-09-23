#![allow(dead_code)]

mod compression;
mod tiled_map;

use serde::Deserialize;
use snes_gfx::{
    palette::{PaletteRGB24, Format},
    tilemap::Tilemap,
    tileset::{DefaultTileset, Tileset},
};
use std::{
    fs::{self, File},
    io::Seek,
};

use byteorder::ReadBytesExt;

fn read_bytes(rom: &mut File, offset: u32, num_bytes: u32) -> Vec<u8> {
    rom.seek(std::io::SeekFrom::Start(offset.into())).unwrap();
    let mut colors = Vec::<u8>::new();
    for _ in 0..num_bytes {
        colors.push(rom.read_u8().unwrap());
    }

    colors
}

fn extract_splash(
    prefix: &str, rom: &mut File, palette_offset: u32, palette_length: u32, palette_index: u8, map_offset: u32,
    tiles_offset: u32, format: Format,
) {
    let palette_data = read_bytes(rom, palette_offset, palette_length);
    let tileset_data = compression::decompress(rom, tiles_offset);
    let tilemap_data = compression::decompress(rom, map_offset);

    let palette = PaletteRGB24::load(&palette_data);
    let tileset = DefaultTileset::load(&tileset_data, format);
    let mut tilemap = Tilemap::load(&tilemap_data, &tileset, &palette);

    let base_dir = format!("decompressed/{prefix}");
    fs::create_dir(&base_dir).unwrap_or_default();

    tilemap
        .generate_image(32)
        .save(format!("{}tilemap.png", &base_dir))
        .unwrap();

    let all_tiles = tileset.get_tile_images(&palette, palette_index);
    DefaultTileset::merge_tiles(&all_tiles, 16)
        .save(format!("{}tileset.png", &base_dir))
        .unwrap();
}

#[derive(Deserialize)]
enum TileFormat {
    BPP2,
    BPP4,
}

#[derive(Deserialize)]
struct Layer {
    map_offset: u32,
    tile_data: u32,
    bpp: TileFormat,
    palette_index: u8,
}
#[derive(Deserialize)]
struct SplashData {
    layers: Vec<Layer>,
    palette_offset: u32,
    num_colors: u32,
}

fn main() {
    let mut rom = File::open("shadowrun.sfc").expect("Could not open ROM-file!");
    let splash_data: SplashData =
        serde_json::from_str(fs::read_to_string("decompressed/splash1.json").unwrap().as_str()).unwrap();
    for (index, layer) in splash_data.layers.iter().enumerate() {
        extract_splash(
            &format!("splash1/layer{}/", index),
            &mut rom,
            splash_data.palette_offset,
            splash_data.num_colors * 2,
            layer.palette_index,
            layer.map_offset,
            layer.tile_data,
            match layer.bpp {
                TileFormat::BPP2 => Format::BPP2,
                TileFormat::BPP4 => Format::BPP4,
            },
        );
    }
}
