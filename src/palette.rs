use std::{
    fs::File,
    io::{Seek, SeekFrom},
};

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
        let r = 8 * (color & 0x1f) as u8;
        let b = 8 * ((color & 0x7c00) >> 10) as u8;
        let g = 8 * ((color & 0x3e0) >> 5) as u8;

        [r, g, b]
    }

    pub fn load(rom: &mut File, offset: u32, num_colors: u32) -> Self {
        rom.seek(SeekFrom::Start(offset.into())).unwrap();
        let mut colors: Vec<[u8; 3]> = Vec::new();

        for _ in 0..num_colors {
            let word = rom.read_u16::<LittleEndian>().unwrap();
            colors.push(Palette::rgb15_to_rgb24(word));
        }
        return Self { colors: colors };
    }

    pub fn get_color(&self, palette: u8, color: u8, format: Format) -> [u8; 3] {
        match format {
            Format::BPP2 => self.colors[((palette * 8) + color) as usize],
            Format::BPP4 => self.colors[((palette * 16) + color) as usize],
        }
    }
}
