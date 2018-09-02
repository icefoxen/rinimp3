extern crate byteorder;
use byteorder::{LE, WriteBytesExt};

use std::io::{self, Read, Write, Cursor};

fn wav_header(hz: i32, ch: i16, bips: i32, data_bytes: i32) -> [u8;44] {
    // let buffer: &mut [u8;44] = b"RIFFsizeWAVEfmt \x10\x00\x00\x00\x01\x00ch_hz_abpsbabsdatasize";
    let mut buffer: [u8;44] = [0;44];
    {
        let mut c = Cursor::new(&mut buffer[..]);
        let size = 44 + data_bytes - 8; // File size - 8
        let avg_bytes_per_sec: u64 = bips as u64 * ch as u64 * hz as u64 / 8;
        let block_align = bips as u64 * ch as u64 / 8;
        // TODO: This alllll needs double checking.
        c.write(b"RIFF");               // 0x00 (offset)
        c.write_i32::<LE>(size);        // 0x04
        c.write(b"WAVE");               // 0x08
        c.write(b"fmt ");               // 0x0C
        c.write(b"\x10\x00\x00\x00");   // 0x10
        c.write_i16::<LE>(1);           // 0x14 -- Integer PCM file format.
        c.write_i16::<LE>(ch);          // 0x16
        c.write_i32::<LE>(hz);          // 0x18
        c.write_i32::<LE>(avg_bytes_per_sec as i32); // 0x1C -- TODO, better casts
        c.write_i16::<LE>(block_align as i16); // 0x20 -- TODO, better casts
        c.write_i16::<LE>(bips as i16); // 0x22 -- TODO, better casts
        c.write(b"data");               // 0x24
        c.write_i32::<LE>(data_bytes);  // 0x28
    }
    buffer
}


/// This shouldn't really be necessary in Rust, I think, since it just
/// reads from the file.  Not gonna try factoring it out right now though.
fn preload(mut file: impl Read, buf: &mut Vec<u8>) -> io::Result<usize> {
    file.read_to_end(buf)
}





#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
