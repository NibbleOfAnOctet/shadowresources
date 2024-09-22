use crate::{compression, palette::Format};
use std::{
    fs::File,
    io::{BufReader, Cursor, Read},
};

pub struct Tileset {
    pub tiles: Vec<[u8; 64]>,
    pub format: Format,
}

impl Tileset {
    pub fn load(rom: &mut File, offset: u32, format: Format) -> Self {
        let decompressed = compression::decompress(rom, offset);
        Self {
            tiles: match format {
                Format::BPP2 => Tileset::create_tileset::<16>(decompressed),
                Format::BPP4 => Tileset::create_tileset::<32>(decompressed),
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

    /// Loops over the raw tiledata with a variable buffer length.
    /// BYTES_PER_TILE: 32 (4BPP) or 16 (2BPP)
    fn create_tileset<const BYTES_PER_TILE: usize>(tile_data: Vec<u8>) -> Vec<[u8; 64]> {
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
