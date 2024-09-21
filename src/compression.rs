use std::{fs::File, io::{Cursor, Read, Seek}};

use bitstream_io::{BigEndian, BitRead, BitReader};

trait UIE {
    fn read_uie(&mut self) -> u32;
}

impl<R: std::io::Read, E: bitstream_io::Endianness> UIE for BitReader<R, E> {
    fn read_uie(&mut self) -> u32 {
        let mut result: u32 = 1;
        loop {
            match self.read_bit() {
                Ok(false) => {
                    //Continue current number
                    let bit = self.read::<u32>(1).expect("Could not read expected bit.");
                    result <<= 1;
                    result |= bit;
                }
                Ok(true) => {
                    //End of current number
                    break;
                }
                _ => {
                    //End of file
                    break;
                }
            }
        }
        return result;
    }
}


/// Decompression algorithm
/// Ported from https://github.com/Osteoclave/game-tools/blob/main/snes/shadowrun_decomp.py
pub fn decompress(rom_file: &mut File, offset: u32)->Vec<u8> {
    rom_file.seek(std::io::SeekFrom::Start(offset.into())).expect("Could not seek offset to compressed tiles!");
    let mut rom_bytes:Vec<u8> = Vec::new();
    rom_file.read_to_end(&mut rom_bytes).expect("Could not read compressed tiles!");
    // Create a cursor for reading from the buffer
    let cursor = Cursor::new(rom_bytes);
    // Set up the bit reader for control bits
    let mut in_stream = BitReader::endian(cursor, BigEndian);

    let uncompressed_length = in_stream.read::<u16>(16).unwrap().swap_bytes() as usize;
    let data_length = in_stream.read::<u16>(16).unwrap().swap_bytes() as usize - 2;
    

    // Read the data bytes
    let mut data = vec![0u8; data_length];
    in_stream.read_bytes(data.as_mut_slice()).unwrap();
    

    // Prepare the decompression buffer
    let mut decomp = vec![0u8; uncompressed_length];
    let mut decomp_pos = 0;
    let mut data_pos = 0;

    

    // The first command is always BIT_LITERAL (0)
    let mut next_command = 0u8;

    loop {
        if next_command == 0 {
            // Literal case
            let copy_amount = in_stream.read_uie();
            // Truncate if necessary
            let remaining = uncompressed_length - decomp_pos;
            let copy_amount = std::cmp::min(copy_amount as usize, remaining);

            // Copy the bytes from data to decompression buffer
            decomp[decomp_pos..decomp_pos + copy_amount]
                .copy_from_slice(&data[data_pos..data_pos + copy_amount]);
            decomp_pos += copy_amount;
            data_pos += copy_amount;

            // Check if decompression is complete
            if decomp_pos == uncompressed_length {
                break;
            }
        }

        // Pastcopy case
        // Calculate the bit length of decomp_pos
        let copy_source_length = if decomp_pos == 0 {
            0
        } else {
            32 - (decomp_pos as u32).leading_zeros()
        } as u32;

        // Read the copy source index
        let copy_source = if copy_source_length > 0 {
            in_stream
                .read::<u32>(copy_source_length)
                .expect("Failed to read copy source.") as usize
        } else {
            0
        };
        // Read the amount to copy
        let copy_amount = in_stream.read_uie() + 2;

        // Truncate if necessary
        let remaining = uncompressed_length - decomp_pos;
        let copy_amount = std::cmp::min(copy_amount as usize, remaining);

        // Copy the bytes, handling potential overlap
        let mut src = copy_source;
        for _ in 0..copy_amount {
            decomp[decomp_pos] = decomp[src];
            decomp_pos += 1;
            src += 1;
        }

        // Check if decompression is complete
        if decomp_pos == uncompressed_length {
            break;
        }

        // Read the next command bit
        next_command = in_stream
            .read::<u8>(1)
            .expect("Failed to read next command.");
    }

    decomp  
}