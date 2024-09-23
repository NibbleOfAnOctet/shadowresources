# A tool for extracting graphics and tilemaps from SNES games.
The tool can handle 2BPP and 4BPP tile data. It can also use the data to reconstruct a tilemap from nametable data. The crate is under construction and stuff will break between versions.

## Example
`
let palette = PaletteRGB24::load(&palette_data);
let tileset = DefaultTileset::load(&tileset_data, format);
let mut tilemap = Tilemap::load(&tilemap_data, &tileset, &palette);

tilemap.generate_image(32)
    .save("tilemap.png")
    .unwrap();

let all_tiles = tileset.get_tile_images(&palette, palette_index);

DefaultTileset::merge_tiles(&all_tiles, 16)
        .save(format!("{}tileset.png", &base_dir))
        .unwrap();
`