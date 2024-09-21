use std::fs::File;

use image::{ImageBuffer, Rgba};

use crate::{
    compression,
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

pub struct Tilemap {
    nametable: Vec<u8>,
    palette: Palette,
    tileset: Box<Tileset>,
    tiles: Vec<Tile>,
    format: Format,
}

impl Tilemap {
    pub fn load(
        rom: &mut File,
        offset: u32,
        tileset: Tileset,
        palette: Palette,
        format: Format,
    ) -> Self {
        Self {
            nametable: compression::decompress(rom, offset),
            palette,
            tileset: Box::new(tileset),
            tiles: Vec::new(),
            format,
        }
    }

    pub fn generate_image(&mut self) -> ImageBuffer<Rgba<u8>, Vec<u8>> {

        image::RgbaImage::from_fn(256, 256, |x, y| {
            let tilex = x / 8;
            let tiley = y / 8;
            let nametable_index = 2 * (tiley * 32 + tilex) as usize;

            if nametable_index>=(self.nametable.len()-1){
                return Rgba([0,0,0,0]);
            }

            let tileword = (self.nametable[nametable_index + 1] as u16) << 8
                | (self.nametable[nametable_index] as u16);

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
                .get_color(tile.palette_index, color_index as u8, self.format);

            let alpha = if color_index == 0 { 0 } else { 255 };
            self.tiles.push(tile);

            Rgba([color[0], color[1], color[2], alpha])
        })
    }
}
