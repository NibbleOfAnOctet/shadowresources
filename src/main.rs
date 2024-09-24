#![allow(dead_code)]

mod compression;
mod tiled_map;

use bitstream_io::{ByteRead, ByteReader, LittleEndian};

use serde::Deserialize;
use image::imageops::{flip_horizontal, flip_vertical};
use snes_gfx::{
    palette::{Format, Palette},
    tilemap::Tilemap,
    tileset::{Tileset, TilesetIterators, TilesetTrait},
};
use std::{fs::File, io::Seek};

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
    let tileset_data = compression::decompress(&mut rom, 327985);
    let tilemap_data = compression::decompress(&mut rom, 329628);
    let palette_data_offset = 327729;
    let num_colors = 32;

    rom.seek(std::io::SeekFrom::Start(palette_data_offset as u64)).unwrap();
    let mut reader = ByteReader::endian(rom, LittleEndian);
    let palette_data = reader.read_to_vec(num_colors * 2 as usize).unwrap();

    // Load necessary data from little endian byte data.
    let palette = Palette::load(&palette_data);
    let tileset = Tileset::load(&tileset_data, Format::BPP4);
    let mut tilemap = Tilemap::load(&tilemap_data);

    // Generate tilemap
    tilemap
        .generate_image(32, &tileset, &palette)
        .save("tilemap.png")
        .expect("Could not save tilemap image!");

    // Iterate over nametable and generate tile images for each entry.
    for tile_data in tilemap.tile_iter() {
        let mut tile_image = tileset.get_tile_image(tile_data.tile_index, tile_data.palette_index, &palette);
        if tile_data.flip_h{
            flip_horizontal(&tile_image);
        }
        if tile_data.flip_v{
            flip_vertical(&tile_image);
        }
    }

    // Create an iterator over tileset images
    let images = tileset.image_iter(1, &palette);

    // Merge into tileset 16 tiles wide
    Tileset::merge_tiles(&images.collect(), 16)
        .save("tileset.png")
        .expect("Could not save tileset image!");
}
