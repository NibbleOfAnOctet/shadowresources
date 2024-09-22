use image::{
    imageops::{flip_horizontal, flip_vertical},
    GenericImage, ImageBuffer, Rgba, RgbaImage,
};
use std::hash::{Hash, Hasher};

use crate::{
    palette::{Format, Palette},
    tileset::Tileset,
};

pub struct Tile {
    flip_h: bool,
    flip_v: bool,
    priority: bool,
    palette_index: u8,
    tile_index: u16,
}

impl Tile {
    pub fn from_nametable_entry(entry: u16) -> Self {
        Self {
            tile_index: (entry & 0x3ff),
            palette_index: ((entry & 0x1c00) >> 10) as u8,
            priority: (entry & 0x2000) >> 13 == 1,
            flip_h: (entry & 0x4000) >> 14 == 1,
            flip_v: (entry & 0x8000) >> 15 == 1,
        }
    }
}

impl Hash for Tile {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.flip_h.hash(state);
        self.flip_v.hash(state);
        self.palette_index.hash(state);
        self.tile_index.hash(state);
    }
}

impl PartialEq for Tile {
    fn eq(&self, other: &Self) -> bool {
        self.flip_h == other.flip_h
            && self.flip_v == other.flip_v
            && self.palette_index == other.palette_index
            && self.tile_index == other.tile_index
    }
}

impl Eq for Tile {}

pub struct Tilemap<'a> {
    nametable: &'a Vec<u8>,
    palette: &'a Palette,
    tileset: &'a Tileset,
    tiles: Vec<Tile>,
}

impl<'a> Tilemap<'a> {
    pub fn load(nametable_data:&'a Vec<u8>, tileset: &'a Tileset, palette: &'a Palette) -> Self {
        Self {
            nametable: nametable_data,
            palette: palette,
            tileset: tileset,
            tiles: Vec::new(),
        }
    }
    pub fn generate_tileset(&self)->ImageBuffer<Rgba<u8>,Vec<u8>> {
        let mut tileset: Vec<ImageBuffer<Rgba<u8>, Vec<u8>>> = Vec::new();

        for nametable_index in 0..&self.nametable.len() / 2 {
            let tileword =
                (self.nametable[2 * nametable_index + 1] as u16) << 8 | (self.nametable[2 * nametable_index] as u16);

            let tile = Tile::from_nametable_entry(tileword);
            let chr = self.tileset.tiles[tile.tile_index as usize];

            let mut tile_image = RgbaImage::from_fn(8, 8, |x, y| {
                let color_index = chr[(y * 8 + x) as usize];
                let color = self.palette.get_rgb_color(tile.palette_index, color_index);
                let alpha = if color_index == 0 { 0 } else { 255 };

                Rgba([color[0], color[1], color[2], alpha])
            });
            if tile.flip_h {
                tile_image = flip_horizontal(&tile_image);
            }
            if tile.flip_v {
                tile_image = flip_vertical(&tile_image);
            }
            if !tileset.iter().any(|entry| entry.eq(&tile_image)) {
                tileset.push(tile_image);
            }
        }
        let mut tileset_image = RgbaImage::new(128, (tileset.len() as u32 / 2) + 8);
        for (index, image) in tileset.iter().enumerate() {
            let offset_x = 8 * (index % 16) as u32;
            let offset_y = 8 * (index / 16) as u32;
            tileset_image.copy_from(image, offset_x, offset_y).unwrap();
        }
        tileset_image
    }

    pub fn generate_image(&mut self) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
        image::RgbaImage::from_fn(256, 256, |x, y| {
            let tilex = x / 8;
            let tiley = y / 8;
            let nametable_index = 2 * (tiley * 32 + tilex) as usize;

            if nametable_index >= (self.nametable.len() - 1) {
                return Rgba([0, 0, 0, 0]);
            }

            let tileword = (self.nametable[nametable_index + 1] as u16) << 8 | (self.nametable[nametable_index] as u16);

            let tile = Tile {
                tile_index: (tileword & 0x3ff),
                palette_index: ((tileword & 0x1c00) >> 10) as u8,
                priority: (tileword & 0x2000) >> 13 == 1,
                flip_h: (tileword & 0x4000) >> 14 == 1,
                flip_v: (tileword & 0x8000) >> 15 == 1,
            };

            let pixel_index = (y % 8) * 8 + (x % 8);

            let color_index = self.tileset.tiles[tile.tile_index as usize][pixel_index as usize] as u16;

            let color = self
                .palette
                .get_rgb_color(tile.palette_index, color_index as u8);

            let alpha = if color_index == 0 { 0 } else { 255 };
            self.tiles.push(tile);

            Rgba([color[0], color[1], color[2], alpha])
        })
    }
}