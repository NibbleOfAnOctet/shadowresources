pub mod tile;
use crate::palette::{Format, Palette};
use image::{GenericImage, ImageBuffer, Rgba, RgbaImage};
use std::io::{BufReader, Cursor};
use tile::Tile;

pub struct Tileset {
    tiles: Vec<Tile>,
    format: Format,
}

impl Tileset {
    pub fn new(tileset_data: &[u8], format: Format) -> Self {
        let mut cursor = Cursor::new(tileset_data);
        let mut reader = BufReader::new(&mut cursor);
        let mut tiles = Vec::new();
        loop {
            match Tile::load(&mut reader, format) {
                Some(tile) => tiles.push(tile),
                None => break,
            }
        }
        Self { tiles, format }
    }

    pub fn image_iter<'a>(&'a self, palette_index: u8, palette: &'a Palette) -> impl Iterator<Item = RgbaImage>+'a {
        self.tiles
            .iter()
            .map(move |tile| tile.get_image(palette_index, palette))
    }

    pub fn tile_iter(&self) -> impl Iterator<Item = &Tile> {
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
