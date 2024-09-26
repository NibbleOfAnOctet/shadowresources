pub mod tile;
use crate::palette::{Format, PaletteTrait};
use image::{GenericImage, ImageBuffer, Rgba, RgbaImage};
use std::io::{BufReader, Cursor};
use tile::{Tile, TileEnum};

pub trait TilesetTrait {
    fn load(tileset_data: &[u8], format: Format) -> Self;
    fn image_iter(&self, palette_index: u8, palette: &dyn PaletteTrait) -> impl Iterator<Item = RgbaImage>;
    fn tile_iter(&self) -> impl Iterator<Item = &TileEnum>;
}

pub struct Tileset {
    tiles: Vec<TileEnum>,
    format: Format,
}

impl TilesetTrait for Tileset {
    fn load(tileset_data: &[u8], format: Format) -> Self {
        let mut cursor = Cursor::new(tileset_data);
        let mut reader = BufReader::new(&mut cursor);
        let mut tiles = Vec::new();
        loop {
            match TileEnum::load(&mut reader, format) {
                tile::TileLoadResult::Ok(tile) => tiles.push(tile),
                tile::TileLoadResult::Done => break,
            }
        }
        Self { tiles, format }
    }

    fn image_iter(&self, palette_index: u8, palette: &dyn PaletteTrait) -> impl Iterator<Item = RgbaImage> {
        self.tiles
            .iter()
            .map(move |tile| tile.get_image(palette_index, palette))
    }

    fn tile_iter(&self) -> impl Iterator<Item = &TileEnum> {
        self.tiles.iter()
    }
}

impl Tileset {
    /// Merges a vector of images into a single tileset image
    pub fn merge_tiles(tiles: &Vec<ImageBuffer<Rgba<u8>, Vec<u8>>>, tiles_wide: u32) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
        let image_width = tiles_wide * 8;
        let image_height = 8 * (tiles.len() as u32 / tiles_wide) + 8;
        let mut tileset_image = RgbaImage::new(image_width, image_height);
        for (index, image) in tiles.iter().enumerate() {
            let offset_x = 8 * (index as u32 % tiles_wide) as u32;
            let offset_y = 8 * (index as u32 / tiles_wide) as u32;
            tileset_image.copy_from(image, offset_x, offset_y).unwrap();
        }
        tileset_image
    }
}
