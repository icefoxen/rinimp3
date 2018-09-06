extern crate byteorder;
// use byteorder::{WriteBytesExt, LE};

// use std::io::{self, Cursor, Read, Write};

pub const MAX_SAMPLES_PER_FRAME: usize = 1152 * 2;
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

pub mod corrode_test;

pub fn hdr_is_mono(h: &[u8]) -> bool {
    // TODO: Might be nicer ways to do these bit-tests
    (h[3] & 0xC0) == 0xC0
}

pub fn hdr_is_ms_stereo(h: &[u8]) -> bool {
    (h[3] & 0xE0) == 0x60
}

pub fn hdr_is_free_format(h: &[u8]) -> bool {
    (h[2] & 0xF0) == 0
}

pub fn hdr_is_crc(h: &[u8]) -> bool {
    // TODO: Double-check
    (h[1] & 1) == 0
}

pub fn hdr_test_padding(h: &[u8]) -> bool {
    (h[2] & 0x2) != 0
}

pub fn hdr_test_mpeg1(h: &[u8]) -> bool {
    (h[1] & 0x08) != 0
}

pub fn hdr_test_not_mpeg25(h: &[u8]) -> bool {
    (h[1] & 0x10) != 0
}

pub fn hdr_test_i_stereo(h: &[u8]) -> bool {
    (h[3] & 0x10) != 0
}

pub fn hdr_test_ms_stereo(h: &[u8]) -> bool {
    (h[3] & 0x20) != 0
}

pub fn hdr_get_stereo_mode(h: &[u8]) -> u8 {
    ((h[3] >> 6) & 3)
}

pub fn hdr_get_stereo_mode_ext(h: &[u8]) -> u8 {
    ((h[3] >> 4) & 3)
}

pub fn hdr_get_layer(h: &[u8]) -> u8 {
    ((h[1] >> 1) & 3)
}

pub fn hdr_get_bitrate(h: &[u8]) -> u8 {
    (h[2] >> 4)
}

pub fn hdr_get_sample_rate(h: &[u8]) -> u8 {
    ((h[2] >> 2) & 3)
}

pub fn hdr_is_frame_576(h: &[u8]) -> bool {
    (h[1] & 14) == 2
}

pub fn hdr_is_layer_1(h: &[u8]) -> bool {
    (h[1] & 6) == 6
}

pub const BITS_DEQUANTIZER_OUT: i32 = -1;
pub const MAX_SCF: i32 = 255 + BITS_DEQUANTIZER_OUT * 4 - 210;
pub const MAX_SCFI: i32 = (MAX_SCF + 3) & !3;

pub struct FrameInfo {
    pub frame_bytes: i32,
    pub channels: i32,
    pub hz: i32,
    pub layers: i32,
    pub bitrate_kbps: i32,
}

pub struct Mp3Dec {
    pub mdct_overlap: [[f32; 2]; 9 * 32],
    pub qmf_state: [f32; 15 * 2 * 32],
    pub reserv: i32,
    pub free_format_bytes: i32,
    pub header: [u8; 4],
    pub reserv_buf: [u8; 511],
}

// TODO: float vs. int16 output?
// type Mp3Sample = i16;

// pub fn decode_frame(
//     dec: &Mp3Dec,
//     mp3: &[u8],
//     mp3_bytes: usize,
//     pcm: &[Mp3Sample],
//     info: &FrameInfo,
// ) -> i32 {
//     0
// }

pub struct Bs {
    pub buf: Vec<u8>,
    pub pos: usize,
    pub limit: usize,
}

pub struct L12ScaleInfo {
    pub scf: [f32; 3 * 64],
    pub total_bands: u8,
    pub stereo_bands: u8,
    pub bitalloc: [u8; 64],
    pub scfcod: [u8; 64],
}

pub struct L12SubbandAlloc {
    pub tab_offset: u8,
    pub code_tab_width: u8,
    pub band_count: u8,
}

pub struct L3GrInfo {
    pub sfbtab: Vec<u8>,
    pub part_23_length: u16,
    pub big_values: u16,
    pub scalefac_compress: u16,
    pub global_gain: u8,
    pub block_type: u8,
    pub mixed_block_flag: u8,
    pub n_long_sfb: u8,
    pub n_short_sfb: u8,
    pub table_select: [u8; 3],
    pub region_count: [u8; 3],
    pub subblock_gain: [u8; 3],
    pub preflag: u8,
    pub scalefac_scale: u8,
    pub count1_table: u8,
    pub scfsi: u8,
}

pub struct Mp3DecScratch {
    pub bs: Bs,
    pub maindata: [u8; MAX_BITRESERVOIR_BYTES + MAX_L3_FRAME_PAYLOAD_BYTES],
    pub gr_info: [L3GrInfo; 3],
    pub grbuf: [[f32; 576]; 2],
    pub scf: [f32; 40],
    pub syn: [[f32; 2 * 32]; 18 + 15],
    pub ist_pos: [[u8; 39]; 2],
}

impl Bs {
    pub fn new(data: Vec<u8>, bytes: usize) -> Self {
        Self {
            buf: data,
            pos: 0,
            limit: bytes * 8,
        }
    }

    /// Heckin... this is way more complicated than it
    /// needs to be here...
    pub fn get_bits(&mut self, n: u32) -> u32 {
        let mut next: u32;
        let mut cache: u32 = 0;
        let s = (self.pos & 7) as u32;
        let mut shl: i32 = n as i32 + s as i32;
        let mut p = self.pos as u32 / 8;
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

/*
pub fn hdr_valid(h: &[u8]) -> bool {
    h[0] == 0xFF
        && ((h[1] & 0xF0) == 0xF0 || (h[1] & 0xFE) == 0xE2)
        && hdr_get_layer(h) != 0
        && hdr_get_bitrate(h) != 15
        && hdr_get_sample_rate(h) != 3
}

pub fn hdr_compare(h1: &[u8], h2: &[u8]) -> bool {
    hdr_valid(h2)
        && ((h1[1] ^ h2[1]) & 0xFE) == 0
        && ((h1[2] ^ h2[2]) & 0x0C) == 0
        && !(hdr_is_free_format(h1) ^ hdr_is_free_format(h2))
}

pub fn hdr_bitrate_kbps(h: &[u8]) -> u32 {
    let halfrate: [[[u32; 15]; 3]; 2] = [
        [
            [0, 4, 8, 12, 16, 20, 24, 28, 32, 40, 48, 56, 64, 72, 80],
            [0, 4, 8, 12, 16, 20, 24, 28, 32, 40, 48, 56, 64, 72, 80],
            [0, 16, 24, 28, 32, 40, 48, 56, 64, 72, 80, 88, 96, 112, 128],
        ],
        [
            [0, 16, 20, 24, 28, 32, 40, 48, 56, 64, 80, 96, 112, 128, 160],
            [
                0, 16, 24, 28, 32, 40, 48, 56, 64, 80, 96, 112, 128, 160, 192,
            ],
            [
                0, 16, 32, 48, 64, 80, 96, 112, 128, 144, 160, 176, 192, 208, 224,
            ],
        ],
    ];
    2 * halfrate[hdr_test_mpeg1(h) as usize][hdr_get_layer(h) as usize - 1]
        [hdr_get_bitrate(h) as usize]
}

pub fn hdr_sample_rate_hz(h: &[u8]) -> u32 {
    let g_hz: [u32; 3] = [44100, 48000, 32000];
    g_hz[hdr_get_sample_rate(h) as usize]
        >> (!hdr_test_mpeg1(h)) as u32
        >> (!hdr_test_not_mpeg25(h)) as u32
}

pub fn hdr_frame_samples(h: &[u8]) -> u32 {
    if hdr_is_layer_1(h) {
        384
    } else {
        1152 >> (hdr_is_frame_576(h) as i32)
    }
}

pub fn hdr_frame_bytes(h: &[u8], free_format_size: u32) -> u32 {
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

pub fn hdr_padding(h: &[u8]) -> u32 {
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
pub fn L12_subband_alloc_table(hdr: &[u8], sci: &mut L12ScaleInfo) -> Vec<L12SubbandAlloc> {
    let mode = hdr_get_stereo_mode(hdr) as usize;
    let mut nbands;
    let mut alloc: Vec<L12SubbandAlloc> = vec![];
    let stereo_bands = if mode == MODE_MONO {
        0
    } else if mode == MODE_JOINT_STEREO {
        (hdr_get_stereo_mode_ext(hdr) << 2) + 4
    } else {
        32
    };
    if hdr_is_layer_1(hdr) {
        alloc.push(L12SubbandAlloc {
            tab_offset: 76,
            code_tab_width: 4,
            band_count: 32,
        });
        nbands = 32;
    } else if !hdr_test_mpeg1(hdr) {
        alloc.push(L12SubbandAlloc {
            tab_offset: 60,
            code_tab_width: 4,
            band_count: 4,
        });
        alloc.push(L12SubbandAlloc {
            tab_offset: 44,
            code_tab_width: 3,
            band_count: 7,
        });
        alloc.push(L12SubbandAlloc {
            tab_offset: 44,
            code_tab_width: 2,
            band_count: 19,
        });
        nbands = 30;
    } else {
        let sample_rate_idx = hdr_get_sample_rate(hdr);
        // TODO: Clean up this comparison
        let mut kbps = hdr_bitrate_kbps(hdr) >> ((mode != MODE_MONO) as u32);
        if kbps == 0 {
            kbps = 192;
        }
        alloc.push(L12SubbandAlloc {
            tab_offset: 0,
            code_tab_width: 4,
            band_count: 3,
        });
        alloc.push(L12SubbandAlloc {
            tab_offset: 16,
            code_tab_width: 4,
            band_count: 8,
        });
        alloc.push(L12SubbandAlloc {
            tab_offset: 32,
            code_tab_width: 3,
            band_count: 12,
        });
        alloc.push(L12SubbandAlloc {
            tab_offset: 40,
            code_tab_width: 2,
            band_count: 7,
        });
        nbands = 27;
        if kbps < 56 {
            alloc.clear();
            alloc.push(L12SubbandAlloc {
                tab_offset: 44,
                code_tab_width: 4,
                band_count: 2,
            });
            alloc.push(L12SubbandAlloc {
                tab_offset: 44,
                code_tab_width: 3,
                band_count: 10,
            });
            nbands = if sample_rate_idx == 2 { 12 } else { 8 };
        } else if (kbps >= 96 && sample_rate_idx != 1) {
            // TODO: sigh, and possibly weep.
            // I think this basically just chops off the last few
            // entries in the alloc defined above the previous if
            // statement.
            nbands = 30;
        }
    }
    sci.total_bands = nbands;
    sci.stereo_bands = u8::min(stereo_bands, nbands);
    alloc
}

pub fn L12_read_scalefactors(bs: &mut Bs, pba: &[u8], scfcod: &[u8], bands: usize, scf: &mut [f32]) {
    // TODO: The C version uses macros to build this array statically,
    // which is a PITA so for now we just do it the simple and slower way.
    let mut g_deq_L12: Vec<f32> = vec![];
    {
        let mut DQ = |x: f32| {
            g_deq_L12.push(9.53674316e-07 / x);
            g_deq_L12.push(7.56931807e-07 / x);
            g_deq_L12.push(6.00777173e-07 / x);
        };

        DQ(3.0);
        DQ(7.0);
        DQ(15.0);
        DQ(31.0);
        DQ(63.0);
        DQ(127.0);
        DQ(255.0);
        DQ(511.0);
        DQ(1023.0);
        DQ(2047.0);
        DQ(4095.0);
        DQ(8191.0);
        DQ(16383.0);
        DQ(32767.0);
        DQ(65535.0);
        DQ(3.0);
        DQ(5.0);
        DQ(9.0);
    }
    let mut scf_idx = 0;
    for i in 0..bands {
        let ba = pba[i];
        let mask = if ba != 0 {
            4 + ((19 >> scfcod[i]) & 3)
        } else {
            0
        };
        let mut m = 4;
        while m != 0 {
            let s;
            if (mask & m) != 0 {
                let b = bs.get_bits(6);
                let idx = (ba as u32 * 3 - 6 + b % 3) as usize;
                s = g_deq_L12[idx] * (1 << 21 >> (b / 3)) as f32;
            } else {
                s = 0.0;
            }
            // TODO: Check the post and pre-increment order here!!!
            scf[scf_idx] = s;
            scf_idx += 1;
        }
    }
}

pub fn L12_read_scale_info(hdr: &[u8], bs: &mut Bs, sci: &mut L12ScaleInfo) {
    let g_bitalloc_code_tab: &[u8] = &[
        0, 17, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 0, 17, 18, 3, 19, 4, 5, 6, 7, 8, 9,
        10, 11, 12, 13, 16, 0, 17, 18, 3, 19, 4, 5, 16, 0, 17, 18, 16, 0, 17, 18, 19, 4, 5, 6, 7,
        8, 9, 10, 11, 12, 13, 14, 15, 0, 17, 18, 3, 19, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 0, 2,
        3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16,
    ];
    let subband_alloc = L12_subband_alloc_table(hdr, sci);
    let mut subband_alloc_idx = 0;
    let mut k: usize = 0;
    let mut ba_bits = 0;
    let mut ba_code_tab_idx: usize = 0;
    for i in 0..(sci.total_bands as usize) {
        let ba: u8;
        if i == k {
            let sb = &subband_alloc[subband_alloc_idx];
            k += sb.band_count as usize;
            ba_bits = sb.code_tab_width;
            ba_code_tab_idx = sb.tab_offset as usize;
            subband_alloc_idx += 1;
        }
        let ba_idx: usize = ba_code_tab_idx + (bs.get_bits(ba_bits as u32) as usize);
        ba = g_bitalloc_code_tab[ba_idx];
        sci.bitalloc[2 * i + 1] = if sci.stereo_bands != 0 { ba } else { 0 };
    }

    for i in 0..(2 * sci.total_bands as usize) {
        sci.scfcod[i] = if sci.bitalloc[i] != 0 {
            if hdr_is_layer_1(hdr) {
                2
            } else {
                bs.get_bits(2) as u8
            }
        } else {
            6
        };
    }

    L12_read_scalefactors(
        bs,
        &sci.bitalloc,
        &sci.scfcod,
        (sci.total_bands * 2) as usize,
        &mut sci.scf,
    );
    // TODO: This clear can probably be better.
    for i in sci.stereo_bands..sci.total_bands {
        let i = i as usize;
        sci.bitalloc[2 * i + 1] = 0;
    }
}
*/

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    /*
pub fn wav_header(hz: i32, ch: i16, bips: i32, data_bytes: i32) -> [u8;44] {
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
pub fn preload(mut file: impl Read, buf: &mut Vec<u8>) -> io::Result<usize> {
    file.read_to_end(buf)
}
*/
}
