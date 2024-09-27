# SNES-GFX

## A tool for converting tiles and tilemaps from SNES games into images.
The tool uses data in the original LittleEndian SNES formats. The tool can extract 2BPP and 4BPP tiles and tilemaps into RGB images. The crate is under construction and stuff will definitely break between versions. 

The library is coded as a learning experience, and will probably not follow best-practices.

## Usage example
```rust ignore
use image::imageops::{flip_horizontal, flip_vertical};
use snes_gfx::{
    palette::{Format, Palette},
    tilemap::Tilemap,
    tileset::{Tileset, TilesetIterators, TilesetTrait},
};

[...]

// Load necessary data from little endian byte data.
let palette = Palette::new(&palette_data);
let tileset = Tileset::new(&tileset_data, format);
let tilemap = Tilemap::new(&tilemap_data);
        
let basepath = Path::new("./extracted/").join(metadata.directory.as_str());
fs::create_dir_all(&basepath).unwrap_or_default();
// Generate tilemap
tilemap
    .generate_image(32, &tileset, &palette)
    .save(basepath.join(format!("layer{}_tilemap.png", layer_index)))
    .expect("Could not save tilemap image!");
// Create an iterator over tileset images
    let images = tileset.image_iter(layer.palette_index, &palette);

    // Merge into tileset 16 tiles wide
    Tileset::merge_tiles(&images.collect(), 16)
        .save(basepath.join(format!("layer{}_tileset.png", layer_index)))
        .expect("Could not save tileset image!");
```