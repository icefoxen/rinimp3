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

pub struct Bs {
    buf: Vec<u8>,
    pos: usize,
    limit: usize,
}

pub struct L12ScaleInfo {
    scf: [f32;3*64],
    total_bands: u8,
    stereo_bands: u8,
    bitalloc: [u8;64],
    scfcod: [u8;64],
}

pub struct L12SubbandAlloc {
    tab_offset: u8,
    code_tab_width: u8,
    band_count: u8,
}

pub struct L3GrInfo {
    sfbtab: Vec<u8>,
    part_23_length: u16,
    big_values: u16,
    scalefac_compress: u16,
    global_gain: u8,
    block_type: u8,
    mixed_block_flag: u8,
    n_long_sfb: u8,
    n_short_sfb: u8,
    table_select: [u8;3],
    region_count: [u8;3],
    subblock_gain: [u8;3],
    preflag: u8,
    scalefac_scale: u8,
    count1_table: u8,
    scfsi: u8,
}

pub struct Mp3DecScratch {
    bs: Bs,
    maindata: [u8;MAX_BITRESERVOIR_BYTES + MAX_L3_FRAME_PAYLOAD_BYTES],
    gr_info: [L3GrInfo; 3],
    grbuf: [[f32;576]; 2],
    scf: [f32;40],
    syn: [[f32: 2*32]; 18+15],
    ist_pos: [[u8;39];2],
}

impl Bs {
    fn new(data: Vec<u8>, bytes: usize) -> Self {
        Self {
            buf: data,
            pos: 0,
            limit: bytes * 8,
        }
    }

    /// Heckin... this is way more complicated than it
    /// needs to be here...
    fn get_bits(&mut self, n: u32) -> u32 {
        let mut next: u32 = 0;
        let mut cache: u32 = 0;
        let mut s = (self.pos & 7) as u32;
        let shl: i32 = n as i32 + s as i32;
        let p = self.pos as u32 / 8;
        if self.pos + (n as usize) > self.limit {
            return 0;
        }
        self.pos += n as usize;
        p += 1;
        next = p & (255 >> s);
        while shl > 0 {
            shl -= 8;
            cache |= next << shl;
            next = p;
            p += 1;
        }
        return cache | (next >> -shl);
    }
}

fn hdr_valid(h: &[u8]) -> bool {
    h[0] == 0xFF &&
    ((h[1] & 0xF0) == 0xF0 || (h[1] & 0xFE) == 0xE2) &&
    hdr_get_layer(h) != 0 &&
    hdr_get_bitrate(h) != 15 &&
    hdr_get_sample_rate(h) != 3
}

fn hdr_compare(h1: &[u8], h2: &[u8]) -> bool {
    hdr_valid(h2) &&
    ((h1[1] ^ h2[1])& 0xFE) == 0 &&
    ((h1[2] ^ h2[2])& 0x0C) == 0 &&
    !(hdr_is_free_format(h1) ^ hdr_is_free_format(h2))
}

fn hdr_bitrate_kbps(h: &[u8]) -> u32 {
    let halfrate: [[[u32; 15]; 3]; 2] = [
        [ [ 0,4,8,12,16,20,24,28,32,40,48,56,64,72,80 ], [ 0,4,8,12,16,20,24,28,32,40,48,56,64,72,80 ], [ 0,16,24,28,32,40,48,56,64,72,80,88,96,112,128 ] ],
        [ [ 0,16,20,24,28,32,40,48,56,64,80,96,112,128,160 ], [ 0,16,24,28,32,40,48,56,64,80,96,112,128,160,192 ], [ 0,16,32,48,64,80,96,112,128,144,160,176,192,208,224 ] ],
    ];
    2 * halfrate[hdr_test_mpeg1(h) as usize][hdr_get_layer(h) as usize - 1][hdr_get_bitrate(h) as usize]
}

fn hdr_sample_rate_hz(h: &[u8]) -> u32 {
    let g_hz: [u32;3] = [44100, 48000, 32000];
    g_hz[hdr_get_sample_rate(h) as usize] >> (!hdr_test_mpeg1(h)) as u32 >> (!hdr_test_not_mpeg25(h)) as u32
}

fn hdr_frame_samples(h: &[u8]) -> u32 {
    if hdr_is_layer_1(h) {
        384
    } else {
        1152 >> (hdr_is_frame_576(h) as i32)
    }
}

fn hdr_frame_bytes(h: &[u8], free_format_size: u32) -> u32 {
    let mut frame_bytes = hdr_frame_samples(h) * hdr_bitrate_kbps(h) * 125 / hdr_sample_rate_hz(h);
    if hdr_is_layer_1(h) {
        // Slot align
        frame_bytes &= !3;
    }
    if frame_bytes != 0 {
        frame_bytes
    } else {
        free_format_size
    }
}

fn hdr_padding(h: &[u8]) -> u32 {
    if hdr_test_padding(h) {
        if hdr_is_layer_1(h) {
            4
        } else {
            1
        }
    } else {
        0
    }
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
