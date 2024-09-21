use std::{
    fs::File,
    io::{Seek, SeekFrom},
};

use byteorder::{LittleEndian, ReadBytesExt};
#[derive(Clone, Copy)]
pub enum Format{
    BPP2,
    BPP4
}

pub struct Palette {
    colors: Vec<[u8; 3]>,
}

impl Palette {
    pub fn load(rom: &mut File, offset: u32, num_colors: u32) -> Self {
        rom.seek(SeekFrom::Start(offset.into())).unwrap();
        let mut colors: Vec<[u8; 3]> = Vec::new();

        for _ in 0..num_colors {
            let word = rom.read_u16::<LittleEndian>().unwrap();

            let b = 8 * ((word & 0x7c00) >> 10) as u8;
            let g = 8 * ((word & 0x3e0) >> 5) as u8;
            let r = 8 * (word & 0x1f) as u8;

            colors.push([r, g, b]);
        }
        return Self { colors: colors };
    }

    pub fn get_color(&self, palette:u8, color:u8, format:Format)->[u8;3]{
        match format{
            Format::BPP2=>self.colors[((palette*8)+color) as usize],
            Format::BPP4=>self.colors[((palette*16)+color) as usize]
        }
    }

    
}
