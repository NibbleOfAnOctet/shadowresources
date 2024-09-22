#![allow(dead_code)]

mod compression;

mod tiled_map;

use std::{fs::File, io::Seek};
use snes_gfx::{palette::{Format, Palette}, tilemap::Tilemap, tileset::Tileset};

use byteorder::ReadBytesExt;



fn read_bytes(rom: &mut File, offset:u32, num_bytes:u32) -> Vec<u8> {
    rom.seek(std::io::SeekFrom::Start(offset.into())).unwrap();
    let mut colors = Vec::<u8>::new();
    for _ in 0..num_bytes{
        colors.push(rom.read_u8().unwrap());
    }

    colors
}

fn main() {
    let mut rom = File::open("shadowrun.sfc").expect("Could not open ROM-file!");

    let palette_data = read_bytes(&mut rom, 327729,64);
    let tileset_data = compression::decompress(&mut rom, 327985);
    let tilemap_data = compression::decompress(&mut rom, 329628);

    let palette = Palette::load(&palette_data);
    let tileset1 = Tileset::load(&tileset_data, Format::BPP4);
    let mut tilemap1 = Tilemap::load(&tilemap_data, &tileset1, &palette, Format::BPP4);

    tilemap1.generate_image().save("decompressed/tilemap1.png").unwrap();

    let all_tiles = tileset1.convert_to_tile_images(&palette, 1);
    snes_gfx::tileset::Tileset::merge_tiles(&all_tiles,12).save("decompressed/tileset1.png").unwrap();
    
}
