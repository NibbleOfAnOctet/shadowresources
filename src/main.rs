#![allow(dead_code)]

mod compression;
mod palette;
mod tilemap;
mod tileset;

use std::fs::File;

use palette::{Format, Palette};
use tilemap::Tilemap;
use tileset::Tileset;

fn main() {
    let mut rom = File::open("shadowrun.sfc").expect("Could not open ROM-file!");

    let palette = Palette::load(&mut rom, 327729, 32);

    let tileset = Tileset::load(&mut rom, 327985, Format::BPP4);
    let mut tilemap = Tilemap::load(&mut rom, 329628, &tileset, &palette, palette::Format::BPP4);

    tilemap.generate_image().save("decompressed/tilemap1.png").unwrap();

    let tileset = Tileset::load(&mut rom, 63531, Format::BPP2);
    let mut tilemap = Tilemap::load(&mut rom, 64113, &tileset, &palette, palette::Format::BPP2);

    tilemap.generate_image().save("decompressed/tilemap2.png").unwrap();
}
