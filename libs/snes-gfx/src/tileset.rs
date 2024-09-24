use crate::palette::{Format, Palette, PaletteTrait};
use image::{GenericImage, ImageBuffer, Rgba, RgbaImage};
use std::{
    io::{BufReader, Cursor, Read},
    ops::Index,
};

#[cfg_attr(test, mockall::automock)]
pub trait TilesetTrait {
    fn get_pixel_data(&self) -> &Vec<[u8; 64]>;
    fn get_tile_image(&self, index: u16, palette_index: u8, palette: &dyn PaletteTrait) -> RgbaImage;
}
pub trait TilesetIterators {
    fn image_iter(&self, palette_index: u8, palette: &Palette) -> impl Iterator<Item = RgbaImage>;
    fn pixeldata_iter(&self) -> impl Iterator<Item = &[u8; 64]>;
}

pub struct Tileset {
    tiles: Vec<[u8; 64]>,
    format: Format,
}

impl Index<usize> for Tileset {
    type Output = [u8; 64];

    /// Gets the pixel data of the tile at provided index.
    fn index(&self, index: usize) -> &Self::Output {
        &self.tiles[index]
    }
}

impl TilesetTrait for Tileset {
    /// Gets pixel values for all tiles.
    fn get_pixel_data(&self) -> &Vec<[u8; 64]> {
        &self.tiles
    }

    /// Generates a tile image from the tileset given a palette and palette index.
    fn get_tile_image(&self, index: u16, palette_index: u8, palette: &dyn PaletteTrait) -> RgbaImage {
        image::RgbaImage::from_fn(8, 8, |x, y| {
            let pixel_index = ((y * 8) + x % 8) as usize;
            let color_index = self.tiles[index as usize][pixel_index];
            let color = palette.get_rgb_color(palette_index, color_index);
            let alpha = if color_index == 0 { 0 } else { 255 };
            Rgba([color[0], color[1], color[2], alpha])
        })
    }
}

impl TilesetIterators for Tileset {
    fn image_iter(&self, palette_index: u8, palette: &Palette) -> impl Iterator<Item = RgbaImage> {
        self.tiles.iter().map(move |tile| {
            image::RgbaImage::from_fn(8, 8, |x, y| {
                let pixel_index = ((y * 8) + x % 8) as usize;
                let color_index = tile[pixel_index];
                let color = palette.get_rgb_color(palette_index, color_index);
                let alpha = if color_index == 0 { 0 } else { 255 };
                Rgba([color[0], color[1], color[2], alpha])
            })
        })
    }

    fn pixeldata_iter(&self) -> impl Iterator<Item = &[u8; 64]> {
        self.tiles.iter()
    }
}

impl Tileset {
    pub fn load(tileset_data: &[u8], format: Format) -> Self {
        Self {
            tiles: match format {
                Format::BPP2 => Tileset::convert_tiles::<16>(tileset_data),
                Format::BPP4 => Tileset::convert_tiles::<32>(tileset_data),
            },
            format,
        }
    }
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

    /// Converts the SNES 8x8 planar format to an array of u8 values containing the pixel data. (Indexed color)
    fn bitplanes_to_tile<const BYTES_PER_TILE: usize>(tile_data: &[u8]) -> [u8; 64] {
        // Final 8x8 pixel values
        let mut pixel_indices = [0u8; 64];

        // Fetch the bitplanes for the current row of pixels
        for row in 0..8 {
            //Get bitplane bytes. Starting with BP1 and BP2 interleaved, followed by BP3 and BP4 if its a 4BPP tile.
            let b0 = tile_data[row * 2];
            let b1 = tile_data[row * 2 + 1];

            let (b2, b3) = match BYTES_PER_TILE {
                32 => (tile_data[16 + row * 2], tile_data[16 + row * 2 + 1]),
                16 => (0, 0),
                _ => panic!("Invalid number of bytes per tile."),
            };

            for col in 0..8 {
                let shift = 7 - col; // Shift to align the bits
                let bit0 = (b0 >> shift) & 1;
                let bit1 = (b1 >> shift) & 1;
                let bit2 = (b2 >> shift) & 1;
                let bit3 = (b3 >> shift) & 1;

                let pixel_value = (bit3 << 3) | (bit2 << 2) | (bit1 << 1) | bit0;

                pixel_indices[row * 8 + col] = pixel_value;
            }
        }
        return pixel_indices;
    }

    /// Converts the bitplane tile data to 8x8 pixel values.
    /// BYTES_PER_TILE: 32 (4BPP) or 16 (2BPP)
    fn convert_tiles<const BYTES_PER_TILE: usize>(tile_data: &[u8]) -> Vec<[u8; 64]> {
        let cursor = Cursor::new(tile_data);
        let mut reader = BufReader::new(cursor);
        let mut tiles: Vec<[u8; 64]> = Vec::new();
        loop {
            let mut buf: [u8; BYTES_PER_TILE] = [0; BYTES_PER_TILE];
            match reader.read(&mut buf) {
                Ok(0) => break,
                Ok(_) => {
                    let tile = Tileset::bitplanes_to_tile::<BYTES_PER_TILE>(&buf);
                    tiles.push(tile);
                }
                Err(e) => panic!("{}", e),
            }
        }
        tiles
    }
}

#[cfg(test)]
pub mod tests {
    use crate::palette::MockPaletteTrait;

    use super::*;

    #[test]
    fn tileset_generates_correct_pixel_data_for_bpp2() {
        let tileset_data: Vec<u8> = vec![
            0x00, 0x00, 0x24, 0x00, 0x24, 0x00, 0x00, 0x00, 0x42, 0x00, 0x3C, 0x00, 0x00, 0x00, 0x00, 0x00,
        ];

        let expected = vec![[
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00,
            0x01, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00,
            0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x01, 0x01, 0x01, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        ] as [u8; 64]];

        let tileset = Tileset::load(&tileset_data, Format::BPP2);
        assert_eq!(tileset.get_pixel_data(), &expected);
    }

    #[test]
    fn tileset_generates_correct_image_from_tile() {
        let tileset_data: Vec<u8> = vec![
            0x00, 0x00, 0x24, 0x00, 0x24, 0x00, 0x00, 0x00, 0x42, 0x00, 0x3C, 0x00, 0x00, 0x00, 0x00, 0x00,
        ];
        let mut palette = MockPaletteTrait::new();
        let mut i = 0;
        palette.expect_get_rgb_color().returning(move |_a, _b| {
            let color = [i, i, i];
            i += 1;
            color
        });

        let tileset = Tileset::load(&tileset_data, Format::BPP2);
        let tile_image = tileset.get_tile_image(0, 0, &palette);

        for (index, pixel) in tile_image.pixels().enumerate() {
            assert_eq!(pixel.0[0], index as u8);
        }
    }

    #[test]
    fn tileset_generates_correct_pixel_data_for_bpp4() {
        let tileset_data: Vec<u8> = vec![
            0x00, 0x00, 0x24, 0x00, 0x24, 0x00, 0x00, 0x00, 0x42, 0x00, 0x3C, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        ];

        let expected = vec![[
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00,
            0x01, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00,
            0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x01, 0x01, 0x01, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        ] as [u8; 64]];

        let tileset = Tileset::load(&tileset_data, Format::BPP4);
        assert_eq!(tileset.get_pixel_data(), &expected);
    }
}
