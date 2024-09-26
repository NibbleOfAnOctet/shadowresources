#![allow(dead_code)]

mod compression;
mod tiled_map;

use bitstream_io::{ByteRead, ByteReader, LittleEndian};

use serde::Deserialize;
use snes_gfx::{
    palette::{Format, Palette},
    tilemap::Tilemap,
    tileset::Tileset,
};
use std::{
    fs::{self, File},
    io::{Read, Seek},
    path::Path,
};

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
    directory:String,
    layers: Vec<Layer>,
    palette_offset: u32,
    num_colors: u32,
}

fn extract(rom_file: &str, metadata: &SplashData) {
    let mut rom = File::open(rom_file).expect("Could not open rom.");
    for (layer_index, layer) in metadata.layers.iter().enumerate() {
        rom.seek(std::io::SeekFrom::Start(layer.tile_data as u64)).unwrap();
        let mut compressed_tileset = Vec::<u8>::new();
        rom.read_to_end(&mut compressed_tileset).unwrap();
        let tileset_data = compression::decompress(&compressed_tileset);

        rom.seek(std::io::SeekFrom::Start(layer.map_offset as u64)).unwrap();
        let mut compressed_tilemap = Vec::<u8>::new();
        rom.read_to_end(&mut compressed_tilemap).unwrap();
        let tilemap_data = compression::decompress(&compressed_tilemap);

        rom.seek(std::io::SeekFrom::Start(metadata.palette_offset as u64)).unwrap();

        let mut reader = ByteReader::endian(&rom, LittleEndian);
        let palette_data = reader.read_to_vec(metadata.num_colors as usize * 2).unwrap();
        let format = match layer.bpp {
            TileFormat::BPP2 => Format::BPP2,
            TileFormat::BPP4 => Format::BPP4,
        };

        // Load necessary data from little endian byte data.
        let palette = Palette::new(&palette_data);
        let tileset = Tileset::new(&tileset_data, format);
        let mut tilemap = Tilemap::new(&tilemap_data);
        let basepath = Path::new("./extracted/").join(metadata.directory.as_str());
        fs::create_dir_all(&basepath).unwrap_or_default();

        // Generate tilemap
        tilemap
            .generate_image(32, &tileset, &palette)
            .save(basepath.join(format!("layer{}_tilemap.png",layer_index)))
            .expect("Could not save tilemap image!");

        // Create an iterator over tileset images
        let images = tileset.image_iter(layer.palette_index, &palette);

        // Merge into tileset 16 tiles wide
        Tileset::merge_tiles(&images.collect(), 16)
            .save(basepath.join(format!("layer{}_tileset.png",layer_index)))
            .expect("Could not save tileset image!");
    }
}

fn main() {
    let splash1_metadata =
        serde_json::from_str::<SplashData>(fs::read_to_string("extracted/splash1.json").unwrap().as_str())
            .expect("Could not read splash 1 metadata!");
    let splash2_metadata =
        serde_json::from_str::<SplashData>(fs::read_to_string("extracted/splash2.json").unwrap().as_str())
            .expect("Could not read splash 1 metadata!");

    extract("shadowrun.sfc", &splash1_metadata);
    extract("shadowrun.sfc", &splash2_metadata);
}
