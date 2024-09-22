use image::{GenericImage, ImageBuffer, Rgba, RgbaImage};

use crate::palette::{Format, Palette};
use std::io::{BufReader, Cursor, Read};

pub struct Tileset {
    pub tiles: Vec<[u8; 64]>,
    pub format: Format,
}

impl Tileset {
    pub fn load(tileset_data: &Vec<u8>, format: Format) -> Self {
        
        Self {
            tiles: match format {
                Format::BPP2 => Tileset::convert_tiles::<16>(tileset_data),
                Format::BPP4 => Tileset::convert_tiles::<32>(tileset_data),
            },
            format,
        }
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
    fn convert_tiles<const BYTES_PER_TILE: usize>(tile_data: &Vec<u8>) -> Vec<[u8; 64]> {
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

    /// Converts the tile pixel data to images using provided palette and palette index.
    pub fn convert_to_tile_images(&self, palette: &Palette, palette_index:u8) -> Vec<ImageBuffer<Rgba<u8>, Vec<u8>>> {
        let mut tileset: Vec<ImageBuffer<Rgba<u8>, Vec<u8>>> = Vec::new();
        for tile in &self.tiles{
            let tile_image = RgbaImage::from_fn(8, 8, |x, y| {
                let color_index = tile[(y * 8 + x) as usize];
                let color = palette.get_rgb_color(palette_index, color_index);
                let alpha = if color_index == 0 { 0 } else { 255 };

                Rgba([color[0], color[1], color[2], alpha])
            });
            tileset.push(tile_image);
        }
        tileset
    }

    /// Merges a vector of images into a single tileset image
    pub fn merge_tiles(tiles:&Vec<ImageBuffer<Rgba<u8>, Vec<u8>>>, cols:u32) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
        let image_width = cols*8;
        let image_height = 8*(tiles.len() as u32/cols)+8;
        let mut tileset_image = RgbaImage::new(image_width, image_height);
        for (index, image) in tiles.iter().enumerate() {
            let offset_x = 8 * (index as u32 % cols) as u32;
            let offset_y = 8 * (index as u32 / cols) as u32;
            tileset_image.copy_from(image, offset_x, offset_y).unwrap();
        }
        tileset_image
    }
}
