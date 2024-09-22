use std::io::Cursor;

use byteorder::{LittleEndian, ReadBytesExt};

#[derive(Clone, Copy)]
pub enum Format {
    BPP2,
    BPP4,
}

#[derive(Clone)]
pub struct Palette {
    colors: Vec<[u8; 3]>,
}

impl Palette {
    /// SNES colors are 15bit RGB values. 0b0RRRRRBBBBBGGGGG
    fn rgb15_to_rgb24(color: u16) -> [u8; 3] {
        let r = 8 * (color & 0x1f) as u16;
        let b = 8 * ((color & 0x7c00) >> 10) as u16;
        let g = 8 * ((color & 0x3e0) >> 5) as u16;

        let r=r+r/32;
        let g=g+g/32;
        let b=b+b/32;
        
        [r as u8, g as u8, b as u8]
    }

    pub fn load(palette_data: &Vec<u8>) -> Self {
        let mut colors: Vec<[u8; 3]> = Vec::new();
        let mut cursor = Cursor::new(&palette_data);
        loop {
            match cursor.read_u16::<LittleEndian>(){
                Ok(word) => colors.push(Palette::rgb15_to_rgb24(word)),
                Err(_) => break,
            }
        }
        return Self { colors: colors };
    }

    pub fn get_rgb_color(&self, palette_index: u8, color: u8) -> [u8; 3] {
        self.colors[((palette_index * 16) + color) as usize]
    }
    
    pub fn count_palettes(&self)->u8{
        (self.colors.len()/16).try_into().unwrap()
    }
}