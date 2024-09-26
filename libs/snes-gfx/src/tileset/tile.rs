use std::io::Read;

use image::{Rgba, RgbaImage};

use crate::palette::{Format, PaletteTrait};

pub trait Tile {
    fn load(reader: &mut impl Read, format: Format) -> TileLoadResult;
    fn get_image(&self, palette_index: u8, palette: &dyn PaletteTrait) -> RgbaImage;
}

/// TileEnum(pixel_data:[u8;64])
pub enum TileEnum {
    BPP2_8x8([u8; 64]),
    BPP4_8x8([u8; 64]),
}

pub enum TileLoadResult{
    Ok(TileEnum),
    Done
}
impl Tile for TileEnum {
    fn load(reader: &mut impl Read, format: Format) -> TileLoadResult {
        match format {
            Format::BPP2 => {
                let mut tile_data = [0u8;16];

                match reader.read(&mut tile_data){
                    Ok(0) => return TileLoadResult::Done,
                    _=>()
                }

                let mut pixel_indices = [0u8; 64];
                
                // Fetch the bitplanes for the current row of pixels
                for row in 0..8 {
                    //Get bitplane bytes. Starting with BP1 and BP2 interleaved, followed by BP3 and BP4 if its a 4BPP tile.
                    let b0 = tile_data[row * 2];
                    let b1 = tile_data[row * 2 + 1];

                    for col in 0..8 {
                        let shift = 7 - col; // Shift to align the bits
                        let bit0 = (b0 >> shift) & 1;
                        let bit1 = (b1 >> shift) & 1;

                        let pixel_value = (bit1 << 1) | bit0;

                        pixel_indices[row * 8 + col] = pixel_value;
                    }
                }
                TileLoadResult::Ok(TileEnum::BPP2_8x8(pixel_indices))
            }
            Format::BPP4 => {
                let mut tile_data = [0u8;32];

                match reader.read(&mut tile_data){
                    Ok(0) => return TileLoadResult::Done,
                    _=>()
                }

                let mut pixel_indices = [0u8; 64];

                // Fetch the bitplanes for the current row of pixels
                for row in 0..8 {
                    //Get bitplane bytes. Starting with BP1 and BP2 interleaved, followed by BP3 and BP4 if its a 4BPP tile.
                    let b0 = tile_data[row * 2];
                    let b1 = tile_data[row * 2 + 1];

                    let b2 = tile_data[16+row * 2];
                    let b3 = tile_data[16+row * 2 + 1];

                    for col in 0..8 {
                        let shift = 7 - col; // Shift to align the bits
                        let bit0 = (b0 >> shift) & 1;
                        let bit1 = (b1 >> shift) & 1;
                        let bit2 = (b2 >> shift) & 1;
                        let bit3 = (b3 >> shift) & 1;
                        let pixel_value = (bit3 << 3)|(bit2 << 2)|(bit1 << 1) | bit0;

                        pixel_indices[row * 8 + col] = pixel_value;
                    }
                }
                TileLoadResult::Ok(TileEnum::BPP4_8x8(pixel_indices))
            },
        }
    }

    fn get_image(&self, palette_index: u8, palette: &dyn PaletteTrait) -> RgbaImage {
        match self{
            TileEnum::BPP2_8x8(data)|TileEnum::BPP4_8x8(data) => {
                image::RgbaImage::from_fn(8, 8, |x, y| {
                    let pixel_index = ((y * 8) + x % 8) as usize;
                    let color_index = data[pixel_index];
                    let color = palette.get_rgb_color(palette_index, color_index);
                    let alpha = if color_index == 0 { 0 } else { 255 };
                    Rgba([color[0], color[1], color[2], alpha])
                })
            }
        }
    }
}
