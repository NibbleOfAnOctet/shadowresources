use std::io::Cursor;

use byteorder::{LittleEndian, ReadBytesExt};

#[derive(Clone, Copy)]
pub enum Format {
    BPP2,
    BPP4,
}

#[cfg_attr(test, mockall::automock)]
pub trait PaletteTrait {
    fn get_rgb_color(&self, palette_index: u8, color: u8) -> [u8; 3];
    fn count_palettes(&self) -> u8;
}

#[derive(Clone)]
pub struct Palette {
    colors: Vec<[u8; 3]>,
}

impl PaletteTrait for Palette {
    fn get_rgb_color(&self, palette_index: u8, color: u8) -> [u8; 3] {
        self.colors[((palette_index * 16) + color) as usize]
    }

    fn count_palettes(&self) -> u8 {
        (self.colors.len() / 16).try_into().unwrap()
    }
}

impl Palette {
    fn rgb15_to_rgb24(color: u16) -> [u8; 3] {
        let r = 8 * (color & 0x1f) as u16;
        let b = 8 * ((color & 0x7c00) >> 10) as u16;
        let g = 8 * ((color & 0x3e0) >> 5) as u16;

        let r = r + r / 32;
        let g = g + g / 32;
        let b = b + b / 32;

        [r as u8, g as u8, b as u8]
    }

    /// Loads little-endian palette data.
    pub fn load(palette_data: &[u8]) -> Self {
        let mut colors: Vec<[u8; 3]> = Vec::new();
        let mut cursor = Cursor::new(palette_data);
        loop {
            match cursor.read_u16::<LittleEndian>() {
                Ok(word) => colors.push(Palette::rgb15_to_rgb24(word)),
                Err(_) => break,
            }
        }
        return Self { colors: colors };
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn palette_returns_correct_number_of_palettes() {
        let palette = Palette::load(&[0xff, 0x7f].repeat(16));
        assert_eq!(palette.count_palettes(), 1);

        let palette = Palette::load(&[0xff, 0x7f].repeat(32));
        assert_eq!(palette.count_palettes(), 2);
    }

    #[test]
    fn palette_returns_correct_rgb_values() {
        let palettes = [[0xff, 0x7f].repeat(16), [0x21, 0x04].repeat(16)].concat();

        let palette = Palette::load(&palettes);
        assert_eq!(palette.get_rgb_color(0, 0), [255, 255, 255]);
        assert_eq!(palette.get_rgb_color(1, 0), [8, 8, 8]);
    }
}
