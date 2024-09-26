use std::io::{ErrorKind, Read};

use image::{Rgba, RgbaImage};

use crate::palette::{Format, Palette};

pub struct Tile {
    pixel_data: [u8; 64],
}

impl Tile {
    pub fn load(reader: &mut impl Read, format: Format) -> Option<Self> {
        let mut tile_data: Vec<u8> = match format {
            Format::BPP2 => vec![0u8; 16],
            Format::BPP4 => vec![0u8; 32],
            Format::BPP8 => vec![0u8; 64],
        };

        match reader.read_exact(&mut tile_data) {
            Err(e) if e.kind() == ErrorKind::UnexpectedEof => return None,
            Err(e)=>panic!("Error reading tile data!: {}",e),
            Ok(_) => (),
        }

        let mut pixel_data = [0u8; 64];

        let num_planes = match format {
            Format::BPP2 => 2,
            Format::BPP4 => 4,
            Format::BPP8 => 8,
        };

        for row in 0..8 {
            let mut row_planes: Vec<u8> = Vec::new();
            
            for plane in 0..num_planes/2 {
                let plane_offset = 16 * plane;
                row_planes.push(tile_data[plane_offset + row * 2]);
                row_planes.push(tile_data[plane_offset + row * 2 + 1]);
            }

            for col in 0..8 {
                let shift = 7-col;
                let mut pixel_value = 0;

                for plane in 0..num_planes {
                    let bit = (row_planes[plane] >> shift) & 1;
                    pixel_value |= bit << plane;
                }

                pixel_data[row * 8 + col] = pixel_value as u8;
            }
        }
        Some(Tile { pixel_data })
    }

    pub fn get_image(&self, palette_index: u8, palette: &Palette) -> RgbaImage {
        image::RgbaImage::from_fn(8, 8, |x, y| {
            let pixel_index = ((y * 8) + x % 8) as usize;
            let color_index = self.pixel_data[pixel_index];
            let color = palette.get_rgb_color(palette_index, color_index);
            let alpha = if color_index == 0 { 0 } else { 255 };
            Rgba([color.red, color.green, color.blue, alpha])
        })
    }
}
