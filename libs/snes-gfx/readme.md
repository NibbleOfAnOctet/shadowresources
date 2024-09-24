# SNES-GFX

## A tool for extracting graphics and tilemaps from SNES games.
The tool uses data in the original LittleEndian SNES formats. The tool can extract 2BPP and 4BPP tiles and tilemaps into RGB images. The crate is under construction and stuff will definitely break between versions. 

The library is coded as a learning experience, and will probably not follow best-practices.

## Usage example
```rust
// Load necessary data from little endian byte data.
let palette = Palette::load(&palette_data);
let tileset = Tileset::load(&tileset_data, Format::BPP4);
let mut tilemap = Tilemap::load(&tilemap_data);

// Generate tilemap
tilemap
    .generate_image(32, &tileset, &palette)
    .save("tilemap.png")
    .expect("Could not save tilemap image!");

// Create an iterator over tileset images
let images = tileset.image_iter(1, &palette);

// Merge into tileset 16 tiles wide
Tileset::merge_tiles(&images.collect(), 16)
    .save("tileset.png")
    .expect("Could not save tileset image!");
```