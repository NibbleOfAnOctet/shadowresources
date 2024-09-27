use std::{io::Cursor, ops::Index};

use byteorder::{LittleEndian, ReadBytesExt};
use image::{
    imageops::{flip_horizontal, flip_vertical},
    GenericImage, ImageBuffer, Rgba,
};

use crate::{palette::Palette, tileset::Tileset};

pub struct Tilemap {
    tiles: Vec<NametableEntry>,
}

pub struct NametableEntry {
    pub flip_h: bool,
    pub flip_v: bool,
    pub priority: bool,
    pub palette_index: u8,
    pub tile_index: u16,
}

impl NametableEntry {
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

impl Index<usize> for Tilemap {
    type Output = NametableEntry;

    /// Gets the tile data from the tile at the specified index.
    fn index(&self, index: usize) -> &Self::Output {
        &self.tiles[index]
    }
}


impl Tilemap {
    /// Loads and parses a SNES-nametable from little-endian byte data. It also associates it with a tileset and palette.
    pub fn new(nametable_data: &[u8]) -> Self {
        Self {
            tiles: Tilemap::parse_nametable(nametable_data),
        }
    }

    pub fn tile_iter(&self) -> impl Iterator<Item = &NametableEntry>{
        self.tiles.iter()
    }

    /// Generates an image from the tilemap given a width in tiles.
    pub fn generate_image(
        &self, tiles_wide: u32, tileset: &Tileset, palette: &Palette,
    ) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
        let image_width = 8 * tiles_wide;
        let image_height = 8 * (self.tiles.len() as u32 / tiles_wide) + 8;
        let mut target_image = image::RgbaImage::new(image_width, image_height);

        for (index, nametable_entry) in self.tiles.iter().enumerate() {
            let tile = tileset.tile_iter().nth(nametable_entry.tile_index.into()).unwrap();
            let mut tile_image = tile.get_image(nametable_entry.palette_index,palette);
            
            if nametable_entry.flip_h {
                tile_image = flip_horizontal(&tile_image);
            }
            if nametable_entry.flip_v {
                tile_image = flip_vertical(&tile_image);
            }

            let x_offset = 8 * (index as u32 % tiles_wide);
            let y_offset = 8 * (index as u32 / tiles_wide);

            target_image
                .copy_from(&tile_image, x_offset, y_offset)
                .expect("Could not copy tile to target image.");
        }

        target_image
    }

    fn parse_nametable(nametable_data: &[u8]) -> Vec<NametableEntry> {
        let mut tiles = Vec::<NametableEntry>::new();
        let mut cursor = Cursor::new(nametable_data);
        loop {
            match cursor.read_u16::<LittleEndian>() {
                Ok(entry) => tiles.push(NametableEntry::from_nametable_entry(entry)),
                Err(_) => break,
            }
        }
        tiles
    }
}