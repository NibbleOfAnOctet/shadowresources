use std::io::Cursor;

use byteorder::{LittleEndian, ReadBytesExt};
use image::{
    imageops::{flip_horizontal, flip_vertical},
    GenericImage, ImageBuffer, Rgba,
};

use crate::{palette::Palette, tileset::Tileset};

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

pub struct Tilemap<'a> {
    palette: &'a dyn Palette,
    tileset: &'a dyn Tileset,
    tiles: Vec<Tile>,
}

impl<'a> Tilemap<'a> {
    /// Loads and parses a SNES-nametable and associates it with a tileset and palette. 
    /// 
    /// nametable_data should be a LittleEndian nametable, ready for the PPU.
    pub fn load(nametable_data: &'a Vec<u8>, tileset: &'a dyn Tileset, palette: &'a dyn Palette) -> Self {
        Self {
            palette: palette,
            tileset: tileset,
            tiles: Tilemap::parse_nametable(nametable_data),
        }
    }

    fn parse_nametable(nametable_data: &'a Vec<u8>) -> Vec<Tile> {
        let mut tiles = Vec::<Tile>::new();
        let mut cursor = Cursor::new(nametable_data);
        loop {
            match cursor.read_u16::<LittleEndian>() {
                Ok(entry) => tiles.push(Tile::from_nametable_entry(entry)),
                Err(_) => break,
            }
        }
        tiles
    }

    /// Generates an image from the tilemap given a width in tiles.
    pub fn generate_image(&mut self, tiles_wide: u32) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
        let image_width = 8 * tiles_wide;
        let image_height = 8 * (self.tiles.len() as u32 / tiles_wide) + 8;
        let mut target_image = image::RgbaImage::new(image_width, image_height);

        for (index, tile) in self.tiles.iter().enumerate() {
            let mut tile_image = self
                .tileset
                .get_tile_image(tile.tile_index, tile.palette_index, self.palette);

            if tile.flip_h {
                tile_image = flip_horizontal(&tile_image);
            }
            if tile.flip_v {
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
}

#[cfg(test)]
pub mod tests {
    use crate::{palette::tests::MockPalette, tileset::tests::MockTileset};

    use super::*;
    
    #[test]
    fn tilemap_returns_correct_tile_index() {
        let tileset = MockTileset::new();
        let palette = MockPalette::new();
        let nametable_data:Vec<u8> = vec!(0x01,0x00,0x02,0x00,0x03,0x00);
        let tilemap = Tilemap::load(&nametable_data, &tileset, &palette);
        assert_eq!(tilemap.tiles[0].tile_index,0x01);
        assert_eq!(tilemap.tiles[1].tile_index,0x02);
        assert_eq!(tilemap.tiles[2].tile_index,0x03);
    }

    #[test]
    fn tilemap_returns_correct_tile_palette_index() {
        let tileset = MockTileset::new();
        let palette = MockPalette::new();
        let nametable_data:Vec<u8> = vec!(0x00,0x1C);
        let tilemap = Tilemap::load(&nametable_data, &tileset, &palette);
        assert_eq!(tilemap.tiles[0].palette_index,7);
    }

    #[test]
    fn tilemap_returns_correct_tile_priority() {
        let tileset = MockTileset::new();
        let palette = MockPalette::new();
        let nametable_data:Vec<u8> = vec!(0x00,0x20);
        let tilemap = Tilemap::load(&nametable_data, &tileset, &palette);
        assert_eq!(tilemap.tiles[0].priority,true);
    }

    #[test]
    fn tilemap_returns_correct_tile_flip_v() {
        let tileset = MockTileset::new();
        let palette = MockPalette::new();
        let nametable_data:Vec<u8> = vec!(0x00,0x80);
        let tilemap = Tilemap::load(&nametable_data, &tileset, &palette);
        assert_eq!(tilemap.tiles[0].flip_v,true);
    }

    #[test]
    fn tilemap_returns_correct_tile_flip_h() {
        let tileset = MockTileset::new();
        let palette = MockPalette::new();
        let nametable_data:Vec<u8> = vec!(0x00,0x40);
        let tilemap = Tilemap::load(&nametable_data, &tileset, &palette);
        assert_eq!(tilemap.tiles[0].flip_h,true);
    }
}
