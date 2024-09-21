use std::{
    fs::File,
    io::{BufReader, Cursor, Read},
};
use crate::{compression, palette::Format};

pub struct Tileset {
    pub tiles: Vec<[u8; 64]>,
    pub format: Format,
}

impl Tileset {
    pub fn new(format: Format) -> Self {
        Self {
            tiles: Vec::new(),
            format,
        }
    }

    pub fn load(&mut self, rom: &mut File, offset: u32, format: Format) {
        let decompressed = compression::decompress(rom, offset);
        self.create_tileset(decompressed, format);
    }

    fn bitplanes_to_tile(&mut self, tile_data: &[u8], format: Format) -> [u8; 64] {
        match format {
            Format::BPP4 => {
                let mut pixel_indices = [0u8; 64];
                for row in 0..8 {
                    // Fetch the bytes for the current row
                    let b0 = tile_data[row * 2]; // Bitplane 0
                    let b1 = tile_data[row * 2 + 1]; // Bitplane 1
                    let b2 = tile_data[16 + row * 2]; // Bitplane 2
                    let b3 = tile_data[16 + row * 2 + 1]; // Bitplane 3

                    for col in 0..8 {
                        let shift = 7 - col; // Shift to align the bits
                        let bit0 = (b0 >> shift) & 1;
                        let bit1 = (b1 >> shift) & 1;
                        let bit2 = (b2 >> shift) & 1;
                        let bit3 = (b3 >> shift) & 1;

                        // Combine bits to form the pixel value
                        let pixel_value = (bit3 << 3) | (bit2 << 2) | (bit1 << 1) | bit0;

                        pixel_indices[row * 8 + col] = pixel_value;
                    }
                }
                return pixel_indices;
            }
            Format::BPP2 => {
                let mut pixel_indices = [0u8; 64];
                for row in 0..8 {
                    // Fetch the bytes for the current row
                    let b0 = tile_data[row * 2]; // Bitplane 0
                    let b1 = tile_data[(row * 2) + 1]; // Bitplane 1

                    for col in 0..8 {
                        let shift = 7 - col; // Shift to align the bits
                        let bit0 = (b0 >> shift) & 1;
                        let bit1 = (b1 >> shift) & 1;

                        // Combine bits to form the pixel value
                        let pixel_value = (bit1 << 1) | bit0;

                        pixel_indices[row * 8 + col] = pixel_value;
                    }
                }

                return pixel_indices;
            }
        }
    }

    fn create_tileset(&mut self, tile_data: Vec<u8>, format: Format) {
        let cursor = Cursor::new(tile_data);
        let mut reader = BufReader::new(cursor);

        loop {
            match format {
                Format::BPP4 => {
                    let mut buf: [u8; 32] = [0; 32];
                    match reader.read(&mut buf) {
                        Ok(0) => break,
                        Ok(_) => {
                            let tile = self.bitplanes_to_tile(&buf, format);
                            self.tiles.push(tile);
                        }
                        Err(e) => panic!("{}", e),
                    }
                },
                Format::BPP2=>{
                    let mut buf: [u8; 16] = [0; 16];
                    match reader.read(&mut buf) {
                        Ok(0) => break,
                        Ok(_) => {
                            let tile = self.bitplanes_to_tile(&buf, format);
                            self.tiles.push(tile);
                        }
                        Err(e) => panic!("{}", e),
                    }
                }
            }
        }
    }
}
