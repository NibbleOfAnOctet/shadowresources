use std::io::Cursor;

use byteorder::{LittleEndian, ReadBytesExt};

#[derive(Clone, Copy)]
pub enum Format {
    BPP2,
    BPP4,
    BPP8
}
#[derive(Clone, Copy)]
pub struct RGBColor{
    pub red:u8,
    pub green:u8,
    pub blue:u8
}

#[derive(Clone)]
pub struct Palette {
    colors: Vec<RGBColor>,
}

impl Palette {
    pub fn get_rgb_color(&self, palette_index: u8, color: u8) -> RGBColor {
        self.colors[((palette_index * 16) + color) as usize]
    }

    pub fn count_palettes(&self) -> u8 {
        (self.colors.len() / 16).try_into().unwrap()
    }
}

impl Palette {
    fn rgb15_to_rgb24(color: u16) -> RGBColor {
        let red = 8 * (color & 0x1f) as u16;
        let green = 8 * ((color & 0x3e0) >> 5) as u16;
        let blue = 8 * ((color & 0x7c00) >> 10) as u16;

        let red = (red + red / 32) as u8;
        let green = (green + green / 32) as u8;
        let blue = (blue + blue / 32) as u8;

        RGBColor{red, green, blue}
    }

    /// Loads little-endian palette data.
    pub fn new(palette_data: &[u8]) -> Self {
        let mut colors: Vec<RGBColor> = Vec::new();
        let mut cursor = Cursor::new(palette_data);
        loop {
            match cursor.read_u16::<LittleEndian>() {
                Ok(word) => colors.push(Palette::rgb15_to_rgb24(word)),
                Err(_) => break,
            }
        }
        return Self { colors };
    }
}
