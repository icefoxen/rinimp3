extern crate byteorder;
use byteorder::{LE, WriteBytesExt};

use std::io::{self, Read, Write, Cursor};

pub const MAX_SAMPLES_PER_FRAME: usize = 1152*2;
/// More than ISO spec's
pub const MAX_FREE_FORMAT_FRAME_SIZE: usize = 2304;
pub const MAX_FRAME_SYNC_MATCHES: usize = 10;

/// MUST be >= 320000/8/32000*1152 = 1440
pub const MAX_L3_FRAME_PAYLOAD_BYTES: usize = MAX_FREE_FORMAT_FRAME_SIZE;

pub const MAX_BITRESERVOIR_BYTES: usize = 511;
pub const SHORT_BLOCK_TYPE: usize = 2;
pub const STOP_BLOCK_TYPE: usize = 3;
pub const MODE_MONO: usize = 3;
pub const MODE_JOINT_STEREO: usize = 1;
pub const HDR_SIZE: usize = 4;

fn hdr_is_mono(h: &[u8]) -> bool {
    // TODO: Might be nicer ways to do these bit-tests
    (h[3] & 0xC0) == 0xC0
}

fn hdr_is_ms_stereo(h: &[u8]) -> bool {
    (h[3] & 0xE0) == 0x60
}


fn hdr_is_free_format(h: &[u8]) -> bool {
    (h[2] & 0xF0) == 0
}


fn hdr_is_crc(h: &[u8]) -> bool {
    // TODO: Double-check
    (h[1] & 1) == 0
}

fn hdr_test_padding(h: &[u8]) -> bool {
    (h[2] & 0x2) != 0
}

fn hdr_test_mpeg1(h: &[u8]) -> bool {
    (h[1] & 0x08) != 0
}

fn hdr_test_not_mpeg25(h: &[u8]) -> bool {
    (h[1] & 0x10) != 0
}

fn hdr_test_i_stereo(h: &[u8]) -> bool {
    (h[3] & 0x10) != 0
}

fn hdr_test_ms_stereo(h: &[u8]) -> bool {
    (h[3] & 0x20) != 0
}

fn hdr_get_stereo_mode(h: &[u8]) -> u8 {
    ((h[3] >> 6) & 3)
}

fn hdr_get_stereo_mode_ext(h: &[u8]) -> u8 {
    ((h[3] >> 4) & 3)
}

fn hdr_get_layer(h: &[u8]) -> u8 {
    ((h[1] >> 1) & 3)
}


fn hdr_get_bitrate(h: &[u8]) -> u8 {
    (h[2] >> 4)
}

fn hdr_get_sample_rate(h: &[u8]) -> u8 {
    ((h[2] >> 2) & 3)
}


fn hdr_is_frame_576(h: &[u8]) -> bool {
    (h[1] & 14) == 2
}


fn hdr_is_layer_1(h: &[u8]) -> bool {
    (h[1] & 6) == 6
}

const BITS_DEQUANTIZER_OUT: i32 = -1;
const MAX_SCF: i32 = 255 + BITS_DEQUANTIZER_OUT * 4 - 210;
const MAX_SCFI: i32 = (MAX_SCF + 3) & !3;



pub struct FrameInfo {
    frame_bytes: i32,
    channels: i32,
    hz: i32,
    layers: i32,
    bitrate_kbps: i32,
}

pub struct Mp3Dec {
    mdct_overlap: [[f32;2]; 9*32],
    qmf_state: [f32;15 * 2 * 32],
    reserv: i32,
    free_format_bytes: i32,
    header: [u8;4],
    reserv_buf: [u8;511],
}

// TODO: float vs. int16 output?
type Mp3Sample = i16;

fn decode_frame(dec: &Mp3Dec, mp3: &[u8], mp3_bytes: usize, pcm: &[Mp3Sample], info: &FrameInfo) -> i32 {
    0
}



#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }


/*
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
*/
}
