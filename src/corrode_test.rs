extern "C" {
    fn memcpy(
        __dest: *mut ::std::os::raw::c_void,
        __src: *const ::std::os::raw::c_void,
        __n: usize,
    ) -> *mut ::std::os::raw::c_void;
}

/// Silly helper function
// fn fill<T>(slice: &mut [T], val: T)
// where
//     T: Copy,
// {
//     for v in slice {
//         *v = val;
//     }
// }

/// This function is here to take care of the common C
/// idiom of looping down a pointer to an array and bumping
/// the position of the pointer each iteration of the loop.
/// This works fine in Rust just by doing `s = s[1..];`
/// for immutable borrows, but with mutable ones this behavior
/// causes a double-borrow.  It SHOULD be safe though, so
/// this helper does takes a `&mut` to the slice(!) and does
/// it for us.  Panics if you try to increment past the end
/// of the slice.
fn increment_by_mut<T>(slice: &mut &mut [T], amount: usize) {
    let lifetime_hack = unsafe {
        let slice_ptr = slice.as_mut_ptr();
        ::std::slice::from_raw_parts_mut(slice_ptr, slice.len())
    };
    *slice = &mut lifetime_hack[amount..]
}

/// Same as `increment_by_mut()` but operates on a mutable
/// reference to an immutable slice.
///
/// This is less common than the other way around, 'cause
/// we're usually stepping down buffers that are being filled,
/// or altered, but let's follow std's convention of naming
/// const vs. mut.
fn increment_by<T>(slice: &mut &[T], amount: usize) {
    let lifetime_hack = unsafe {
        let slice_ptr = slice.as_ptr();
        ::std::slice::from_raw_parts(slice_ptr, slice.len())
    };
    *slice = &lifetime_hack[amount..]
}

static GPOW43: [f32; 145] = [
    0.0, -1.0, -2.519842, -4.326749, -6.349604, -8.549880, -10.902724, -13.390518, -16.000000,
    -18.720754, -21.544347, -24.463781, -27.473142, -30.567351, -33.741992, -36.993181, 0.0, 1.0,
    2.519842, 4.326749, 6.349604, 8.549880, 10.902724, 13.390518, 16.000000, 18.720754, 21.544347,
    24.463781, 27.473142, 30.567351, 33.741992, 36.993181, 40.317474, 43.711787, 47.173345,
    50.699631, 54.288352, 57.937408, 61.644865, 65.408941, 69.227979, 73.100443, 77.024898,
    81.000000, 85.024491, 89.097188, 93.216975, 97.382800, 101.593667, 105.848633, 110.146801,
    114.487321, 118.869381, 123.292209, 127.755065, 132.257246, 136.798076, 141.376907, 145.993119,
    150.646117, 155.335327, 160.060199, 164.820202, 169.614826, 174.443577, 179.305980, 184.201575,
    189.129918, 194.090580, 199.083145, 204.107210, 209.162385, 214.248292, 219.364564, 224.510845,
    229.686789, 234.892058, 240.126328, 245.389280, 250.680604, 256.000000, 261.347174, 266.721841,
    272.123723, 277.552547, 283.008049, 288.489971, 293.998060, 299.532071, 305.091761, 310.676898,
    316.287249, 321.922592, 327.582707, 333.267377, 338.976394, 344.709550, 350.466646, 356.247482,
    362.051866, 367.879608, 373.730522, 379.604427, 385.501143, 391.420496, 397.362314, 403.326427,
    409.312672, 415.320884, 421.350905, 427.402579, 433.475750, 439.570269, 445.685987, 451.822757,
    457.980436, 464.158883, 470.357960, 476.577530, 482.817459, 489.077615, 495.357868, 501.658090,
    507.978156, 514.317941, 520.677324, 527.056184, 533.454404, 539.871867, 546.308458, 552.764065,
    559.238575, 565.731879, 572.243870, 578.774440, 585.323483, 591.890898, 598.476581, 605.080431,
    611.702349, 618.342238, 625.000000, 631.675540, 638.368763, 645.079578,
];

#[derive(Copy)]
#[repr(C)]
pub struct Mp3Dec {
    pub mdct_overlap: [[f32; 288]; 2],
    pub qmf_state: [f32; 960],
    pub reserv: i32,
    pub free_format_bytes: i32,
    pub header: [u8; 4],
    pub reserv_buf: [u8; 511],
}

impl Clone for Mp3Dec {
    fn clone(&self) -> Self {
        *self
    }
}

impl Mp3Dec {
    pub fn new() -> Self {
        Self {
            mdct_overlap: [[0.0; 288]; 2],
            qmf_state: [0.0; 960],
            reserv: 0,
            free_format_bytes: 0,
            header: [0; 4],
            reserv_buf: [0; 511],
        }
    }
}

#[derive(Copy)]
#[repr(C)]
pub struct FrameInfo {
    pub frame_bytes: i32,
    pub channels: i32,
    pub hz: i32,
    pub layer: i32,
    pub bitrate_kbps: i32,
}

impl Clone for FrameInfo {
    fn clone(&self) -> Self {
        *self
    }
}

#[derive(Copy)]
#[repr(C)]
pub struct Bs<'a> {
    // pub buf: *const u8,
    pub buf: &'a [u8],
    // TODO: Should be usize
    pub pos: i32,
    // TODO: Should be usize
    pub limit: i32,
}

impl<'a> Bs<'a> {
    pub fn new(buf: &'a [u8], bytes: i32) -> Self {
        Self {
            buf,
            pos: 0,
            limit: bytes * 8,
        }
    }
}

impl<'a> Clone for Bs<'a> {
    fn clone(&self) -> Self {
        *self
    }
}

#[derive(Copy)]
#[repr(C)]
pub struct L3GrInfo {
    pub sfbtab: &'static [u8],
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

impl Clone for L3GrInfo {
    fn clone(&self) -> Self {
        *self
    }
}

#[derive(Copy)]
#[repr(C)]
pub struct Mp3DecScratch<'a> {
    pub bs: Bs<'a>,
    pub maindata: [u8; 2815],
    pub gr_info: [L3GrInfo; 4],
    pub grbuf: [[f32; 576]; 2],
    pub scf: [f32; 40],
    pub syn: [[f32; 64]; 33],
    pub ist_pos: [[u8; 39]; 2],
}

impl<'a> Clone for Mp3DecScratch<'a> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<'a> Mp3DecScratch<'a> {
    fn clear_grbuf(&mut self) {
        self.grbuf = [[0.0; 576]; 2];
    }
}

/*
pub struct Hdr([u8; 4]);
// TODO: Ponder unit tests for these.
impl Hdr {
    pub fn hdr_is_mono(&self) -> bool {
        // TODO: Might be nicer ways to do these bit-tests
        (self.0[3] & 0xC0) == 0xC0
    }

    pub fn hdr_is_ms_stereo(&self) -> bool {
        (self.0[3] & 0xE0) == 0x60
    }

    pub fn hdr_is_free_format(&self) -> bool {
        (self.0[2] & 0xF0) == 0
    }

    pub fn hdr_is_crc(&self) -> bool {
        // TODO: Double-check
        (self.0[1] & 1) == 0
    }

    pub fn hdr_test_padding(&self) -> bool {
        (self.0[2] & 0x2) != 0
    }

    pub fn hdr_test_mpeg1(&self) -> bool {
        (self.0[1] & 0x08) != 0
    }

    pub fn hdr_test_not_mpeg25(&self) -> bool {
        (self.0[1] & 0x10) != 0
    }

    pub fn hdr_test_i_stereo(&self) -> bool {
        (self.0[3] & 0x10) != 0
    }

    pub fn hdr_test_ms_stereo(&self) -> bool {
        (self.0[3] & 0x20) != 0
    }

    pub fn hdr_get_stereo_mode(&self) -> u8 {
        ((self.0[3] >> 6) & 3)
    }

    pub fn hdr_get_stereo_mode_ext(&self) -> u8 {
        ((self.0[3] >> 4) & 3)
    }

    pub fn hdr_get_layer(&self) -> u8 {
        ((self.0[1] >> 1) & 3)
    }

    pub fn hdr_get_bitrate(&self) -> u8 {
        (self.0[2] >> 4)
    }

    pub fn hdr_get_sample_rate(&self) -> u8 {
        ((self.0[2] >> 2) & 3)
    }

    pub fn hdr_is_frame_576(&self) -> bool {
        (self.0[1] & 14) == 2
    }

    pub fn hdr_is_layer_1(&self) -> bool {
        (self.0[1] & 6) == 6
    }

    pub fn hdr_valid(&self) -> bool {
        self.0[0] == 0xFF
            && ((self.0[1] & 0xF0) == 0xF0 || (self.0[1] & 0xFE) == 0xE2)
            && self.hdr_get_layer() != 0
            && self.hdr_get_bitrate() != 15
            && self.hdr_get_sample_rate() != 3
    }

    pub fn hdr_compare(h1: Hdr, h2: Hdr) -> bool {
        h2.hdr_valid()
            && ((h1.0[1] ^ h2.0[1]) & 0xFE) == 0
            && ((h1.0[2] ^ h2.0[2]) & 0x0C) == 0
            && !(h1.hdr_is_free_format() ^ h2.hdr_is_free_format())
    }

    pub fn hdr_bitrate_kbps(&self) -> u32 {
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
        2 * halfrate[self.hdr_test_mpeg1() as usize][self.hdr_get_layer() as usize - 1]
            [self.hdr_get_bitrate() as usize]
    }

    pub fn hdr_sample_rate_hz(&self) -> u32 {
        let g_hz: [u32; 3] = [44100, 48000, 32000];
        g_hz[self.hdr_get_sample_rate() as usize]
            >> (!self.hdr_test_mpeg1()) as u32
            >> (!self.hdr_test_not_mpeg25()) as u32
    }

    pub fn hdr_frame_samples(&self) -> u32 {
        if self.hdr_is_layer_1() {
            384
        } else {
            1152 >> (self.hdr_is_frame_576() as i32)
        }
    }

    pub fn hdr_frame_bytes(&self, free_format_size: u32) -> u32 {
        let mut frame_bytes =
            self.hdr_frame_samples() * self.hdr_bitrate_kbps() * 125 / self.hdr_sample_rate_hz();
        if self.hdr_is_layer_1() {
            // Slot align
            frame_bytes &= !3;
        }
        if frame_bytes != 0 {
            frame_bytes
        } else {
            free_format_size
        }
    }

    pub fn hdr_padding(&self) -> u32 {
        if self.hdr_test_padding() {
            if self.hdr_is_layer_1() {
                4
            } else {
                1
            }
        } else {
            0
        }
    }
}
*/
// TODO: All the horrible bit-tests in the `hdr_` functions
// are macros in the C version; can we translate them back to
// functions?
fn hdr_valid(h: &[u8]) -> i32 {
    (h[0] as (i32) == 0xffi32
        && (h[1] as (i32) & 0xf0 == 0xf0 || h[1] as (i32) & 0xfei32 == 0xe2)
        && (h[1] as (i32) >> 1 & 3 != 0)
        && (h[2] as (i32) >> 4 != 15)
        && (h[2] as (i32) >> 2 & 3 != 3)) as (i32)
}

fn hdr_compare(h1: &[u8], h2: &[u8]) -> i32 {
    (hdr_valid(h2) != 0
        && ((h1[1] as (i32) ^ h2[1] as (i32)) & 0xfei32 == 0)
        && ((h1[2] as (i32) ^ h2[2] as (i32)) & 0xci32 == 0)
        && ((h1[2] as (i32) & 0xf0 == 0) as (i32) ^ (h2[2] as (i32) & 0xf0 == 0) as (i32) == 0))
        as (i32)
}

fn hdr_frame_samples(h: &[u8]) -> u32 {
    (if h[1] as (i32) & 6 == 6 {
        384
    } else {
        1152 >> (h[1] as (i32) & 14 == 2) as (i32)
    }) as (u32)
}

pub fn hdr_bitrate_kbps(h: &[u8]) -> u32 {
    static HALFRATE: [[[u8; 15]; 3]; 2] = [
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
    let i1 = if (h[1] & 0x8) == 0 { 0 } else { 1 };
    let i2 = (((h[1] >> 1) & 3) - 1) as usize;
    let i3 = (h[2] >> 4) as usize;
    debug_assert!(i1 < HALFRATE.len());
    debug_assert!(i2 < HALFRATE[0].len());
    debug_assert!(i3 < HALFRATE[0][0].len());

    2 * HALFRATE[i1][i2][i3] as u32
}

pub fn hdr_sample_rate_hz(h: &[u8]) -> u32 {
    static G_HZ: [u32; 3] = [44100, 48000, 32000];
    G_HZ[(h[2] as (i32) >> 2 & 3) as usize]
        >> (h[1] as (i32) & 0x8 == 0) as (i32)
        >> (h[1] as (i32) & 0x10 == 0) as (i32)
}

pub fn hdr_frame_bytes(h: &[u8], free_format_size: i32) -> i32 {
    let mut frame_bytes: i32 = hdr_frame_samples(h)
        .wrapping_mul(hdr_bitrate_kbps(h))
        .wrapping_mul(125)
        .wrapping_div(hdr_sample_rate_hz(h)) as (i32);
    if h[1] as (i32) & 6 == 6 {
        frame_bytes = frame_bytes & !3;
    }
    if frame_bytes != 0 {
        frame_bytes
    } else {
        free_format_size
    }
}

pub fn hdr_padding(h: &[u8]) -> i32 {
    if h[2] as (i32) & 0x2 != 0 {
        (if h[1] as (i32) & 6 == 6 { 4 } else { 1 })
    } else {
        0
    }
}

fn mp3d_match_frame(hdr: &[u8], mp3_bytes: i32, frame_bytes: i32) -> i32 {
    let current_block;
    let mut i: i32;
    let mut nmatch: i32;
    i = 0;
    nmatch = 0;
    loop {
        if !(nmatch < 10) {
            current_block = 2;
            break;
        }
        i = i
            + (hdr_frame_bytes(&hdr[i as usize..], frame_bytes) + hdr_padding(&hdr[i as usize..]));
        if i + 4 > mp3_bytes {
            current_block = 7;
            break;
        }
        if hdr_compare(hdr, &hdr[i as usize..]) == 0 {
            current_block = 6;
            break;
        }
        nmatch = nmatch + 1;
    }
    if current_block == 2 {
        1
    } else if current_block == 6 {
        0
    } else {
        (nmatch > 0) as (i32)
    }
}

pub unsafe fn mp3d_find_frame(
    mut mp3: &[u8],
    mp3_bytes: i32,
    free_format_bytes: *mut i32,
    ptr_frame_bytes: *mut i32,
) -> i32 {
    let current_block;
    let mut i: i32;
    let mut k: i32;
    i = 0;
    let mut frame_bytes: i32;
    let mut frame_and_padding: i32 = 0;
    loop {
        if !(i < mp3_bytes - 4) {
            current_block = 2;
            break;
        }
        if hdr_valid(mp3) != 0 {
            frame_bytes = hdr_frame_bytes(mp3, *free_format_bytes);
            frame_and_padding = frame_bytes + hdr_padding(mp3);
            k = 4;
            loop {
                if !(frame_bytes == 0 && (k < 2304) && (i + 2 * k < mp3_bytes - 4)) {
                    break;
                }
                if hdr_compare(mp3, &mp3[k as usize..]) != 0 {
                    let fb: i32 = k - hdr_padding(mp3);
                    let nextfb: i32 = fb + hdr_padding(&mp3[k as usize..]);
                    // TODO: Double-check the hdr_compare()
                    if !(i + k + nextfb + 4 > mp3_bytes
                        || hdr_compare(mp3, &mp3[(k + nextfb) as usize..]) == 0)
                    {
                        frame_and_padding = k;
                        frame_bytes = fb;
                        *free_format_bytes = fb;
                    }
                }
                k = k + 1;
            }
            if frame_bytes != 0
                && (i + frame_and_padding <= mp3_bytes)
                && (mp3d_match_frame(mp3, mp3_bytes - i, frame_bytes) != 0)
                || i == 0 && (frame_and_padding == mp3_bytes)
            {
                current_block = 9;
                break;
            }
            *free_format_bytes = 0;
        }
        i = i + 1;
        mp3 = &mp3[1..];
    }
    if current_block == 2 {
        *ptr_frame_bytes = 0;
        i
    } else {
        *ptr_frame_bytes = frame_and_padding;
        i
    }
}


// Rewritten; original is on
// https://github.com/lieff/minimp3/blob/master/minimp3.h#L232
fn get_bits(bs: &mut Bs, n: u32) -> u32 {
    let n = n as i32;
    let s = bs.pos & 7;
    let mut cache: u32 = 0;
    let mut shl = n + s;
    let mut p = (bs.pos >> 3) as usize;
    bs.pos += n as i32;
    if bs.pos > bs.limit {
        return 0;
    }
    let mut next = bs.buf[p] & ((255 & s) as u8);
    p += 1;
    shl -= 8;
    while shl > 0 {
        shl -= 8;
        cache |= (next as u32).wrapping_shl(shl as u32);
        next  = bs.buf[p];
        p += 1;
    }
    
    // cache | ((next as u32) >> -shl)
    // Why on Eris's earth is this shifting by an almost-certainly-negative
    // number?  
    cache | ((next as u32).wrapping_shl(shl as u32))
}


#[derive(Copy)]
#[repr(C)]
pub struct L12ScaleInfo {
    pub scf: [f32; 192],
    pub total_bands: u8,
    pub stereo_bands: u8,
    pub bitalloc: [u8; 64],
    pub scfcod: [u8; 64],
}

impl Clone for L12ScaleInfo {
    fn clone(&self) -> Self {
        *self
    }
}

#[derive(Copy)]
#[repr(C)]
pub struct L12SubbandAlloc {
    pub tab_offset: u8,
    pub code_tab_width: u8,
    pub band_count: u8,
}

impl Clone for L12SubbandAlloc {
    fn clone(&self) -> Self {
        *self
    }
}

/// TODO: This *const it returns is actually an array,
/// make it return a proper slice if possible.
fn l12_subband_alloc_table(hdr: &[u8], sci: &mut L12ScaleInfo) -> *const L12SubbandAlloc {
    let mut alloc: *const L12SubbandAlloc;
    let mode: i32 = hdr[3] as (i32) >> 6 & 3;
    let mut nbands: i32;
    let stereo_bands: i32 = if mode == 3 {
        0
    } else if mode == 1 {
        ((hdr[3] as (i32) >> 4 & 3) << 2) + 4
    } else {
        32
    };
    if hdr[1] as (i32) & 6 == 6 {
        static G_ALLOC_L1: [L12SubbandAlloc; 1] = [L12SubbandAlloc {
            tab_offset: 76,
            code_tab_width: 4,
            band_count: 32,
        }];
        alloc = G_ALLOC_L1.as_ptr();
        nbands = 32;
    } else if hdr[1] as (i32) & 0x8 == 0 {
        static G_ALLOC_L2M2: [L12SubbandAlloc; 3] = [
            L12SubbandAlloc {
                tab_offset: 60,
                code_tab_width: 4,
                band_count: 4,
            },
            L12SubbandAlloc {
                tab_offset: 44,
                code_tab_width: 3,
                band_count: 7,
            },
            L12SubbandAlloc {
                tab_offset: 44,
                code_tab_width: 2,
                band_count: 19,
            },
        ];
        alloc = G_ALLOC_L2M2.as_ptr();
        nbands = 30;
    } else {
        static G_ALLOC_L2M1: [L12SubbandAlloc; 4] = [
            L12SubbandAlloc {
                tab_offset: 0,
                code_tab_width: 4,
                band_count: 3,
            },
            L12SubbandAlloc {
                tab_offset: 16,
                code_tab_width: 4,
                band_count: 8,
            },
            L12SubbandAlloc {
                tab_offset: 32,
                code_tab_width: 3,
                band_count: 12,
            },
            L12SubbandAlloc {
                tab_offset: 40,
                code_tab_width: 2,
                band_count: 7,
            },
        ];
        let sample_rate_idx: i32 = hdr[2] as (i32) >> 2 & 3;
        let mut kbps: u32 = hdr_bitrate_kbps(hdr) >> (mode != 3) as (i32);
        if kbps == 0 {
            kbps = 192;
        }
        alloc = G_ALLOC_L2M1.as_ptr();
        nbands = 27;
        if kbps < 56 {
            static G_ALLOC_L2M1_LOWRATE: [L12SubbandAlloc; 2] = [
                L12SubbandAlloc {
                    tab_offset: 44,
                    code_tab_width: 4,
                    band_count: 2,
                },
                L12SubbandAlloc {
                    tab_offset: 44,
                    code_tab_width: 3,
                    band_count: 10,
                },
            ];
            alloc = G_ALLOC_L2M1_LOWRATE.as_ptr();
            nbands = if sample_rate_idx == 2 { 12 } else { 8 };
        } else if kbps >= 96 && (sample_rate_idx != 1) {
            nbands = 30;
        }
    }
    (*sci).total_bands = nbands as (u8);
    (*sci).stereo_bands = if stereo_bands > nbands {
        nbands
    } else {
        stereo_bands
    } as (u8);
    alloc
}

unsafe fn l12_read_scalefactors(
    bs: &mut Bs,
    mut pba: *mut u8,
    scfcod: *mut u8,
    bands: i32,
    mut scf: *mut f32,
) {
    static G_DEQ_L12: [f32; 54] = [
        9.53674316e-07 / 3.0,
        7.56931807e-07 / 3.0,
        6.00777173e-07 / 3.0,
        9.53674316e-07 / 7.0,
        7.56931807e-07 / 7.0,
        6.00777173e-07 / 7.0,
        9.53674316e-07 / 15.0,
        7.56931807e-07 / 15.0,
        6.00777173e-07 / 15.0,
        9.53674316e-07 / 31.0,
        7.56931807e-07 / 31.0,
        6.00777173e-07 / 31.0,
        9.53674316e-07 / 63.0,
        7.56931807e-07 / 63.0,
        6.00777173e-07 / 63.0,
        9.53674316e-07 / 127.0,
        7.56931807e-07 / 127.0,
        6.00777173e-07 / 127.0,
        9.53674316e-07 / 255.0,
        7.56931807e-07 / 255.0,
        6.00777173e-07 / 255.0,
        9.53674316e-07 / 511.0,
        7.56931807e-07 / 511.0,
        6.00777173e-07 / 511.0,
        9.53674316e-07 / 1023.0,
        7.56931807e-07 / 1023.0,
        6.00777173e-07 / 1023.0,
        9.53674316e-07 / 2047.0,
        7.56931807e-07 / 2047.0,
        6.00777173e-07 / 2047.0,
        9.53674316e-07 / 4095.0,
        7.56931807e-07 / 4095.0,
        6.00777173e-07 / 4095.0,
        9.53674316e-07 / 8191.0,
        7.56931807e-07 / 8191.0,
        6.00777173e-07 / 8191.0,
        9.53674316e-07 / 16383.0,
        7.56931807e-07 / 16383.0,
        6.00777173e-07 / 16383.0,
        9.53674316e-07 / 32767.0,
        7.56931807e-07 / 32767.0,
        6.00777173e-07 / 32767.0,
        9.53674316e-07 / 65535.0,
        7.56931807e-07 / 65535.0,
        6.00777173e-07 / 65535.0,
        9.53674316e-07 / 3.0,
        7.56931807e-07 / 3.0,
        6.00777173e-07 / 3.0,
        9.53674316e-07 / 5.0,
        7.56931807e-07 / 5.0,
        6.00777173e-07 / 5.0,
        9.53674316e-07 / 9.0,
        7.56931807e-07 / 9.0,
        6.00777173e-07 / 9.0,
    ];
    let mut i: i32;
    let mut m: i32;
    i = 0;
    loop {
        if !(i < bands) {
            break;
        }
        let mut s: f32 = 0.0;
        let ba: i32 = *{
            let _old = pba;
            pba = pba.offset(1);
            _old
        } as (i32);
        let mask: i32 = if ba != 0 {
            4 + (19 >> *scfcod.offset(i as isize) as (i32) & 3)
        } else {
            0
        };
        m = 4;
        loop {
            if m == 0 {
                break;
            }
            if mask & m != 0 {
                let b: i32 = get_bits(bs, 6) as (i32);
                s = G_DEQ_L12[(ba * 3 - 6 + b % 3) as usize] * (1 << 21 >> b / 3) as f32;
            }
            *{
                let _old = scf;
                scf = scf.offset(1);
                _old
            } = s;
            m = m >> 1;
        }
        i = i + 1;
    }
}

unsafe fn l12_read_scale_info(hdr: &[u8], bs: &mut Bs, sci: &mut L12ScaleInfo) {
    static G_BITALLOC_CODE_TAB: [u8; 92] = [
        0, 17, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 0, 17, 18, 3, 19, 4, 5, 6, 7, 8, 9,
        10, 11, 12, 13, 16, 0, 17, 18, 3, 19, 4, 5, 16, 0, 17, 18, 16, 0, 17, 18, 19, 4, 5, 6, 7,
        8, 9, 10, 11, 12, 13, 14, 15, 0, 17, 18, 3, 19, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 0, 2,
        3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16,
    ];
    let mut subband_alloc: *const L12SubbandAlloc = l12_subband_alloc_table(hdr, &mut *sci);
    let mut i: i32;
    let mut k: i32 = 0;
    let mut ba_bits: u32 = 0;
    let mut ba_code_tab: *const u8 = G_BITALLOC_CODE_TAB.as_ptr();
    i = 0;
    loop {
        if !(i < (*sci).total_bands as (i32)) {
            break;
        }
        let mut ba: u8;
        if i == k {
            k = k + (*subband_alloc).band_count as (i32);
            ba_bits = (*subband_alloc).code_tab_width as (u32);
            ba_code_tab = G_BITALLOC_CODE_TAB
                .as_ptr()
                .offset((*subband_alloc).tab_offset as isize);
            subband_alloc = subband_alloc.offset(1);
        }
        ba = *ba_code_tab.offset(get_bits(bs, ba_bits) as isize);
        (*sci).bitalloc[(2 * i) as usize] = ba;
        if i < (*sci).stereo_bands as (i32) {
            ba = *ba_code_tab.offset(get_bits(bs, ba_bits) as isize);
        }
        (*sci).bitalloc[(2 * i + 1) as usize] = if (*sci).stereo_bands != 0 {
            ba as (i32)
        } else {
            0
        } as (u8);
        i = i + 1;
    }
    i = 0;
    loop {
        if !(i < 2 * (*sci).total_bands as (i32)) {
            break;
        }
        (*sci).scfcod[i as usize] = if (*sci).bitalloc[i as usize] != 0 {
            (if hdr[1] as (i32) & 6 == 6 {
                2
            } else {
                get_bits(bs, 2)
            })
        } else {
            6
        } as (u8);
        i = i + 1;
    }
    l12_read_scalefactors(
        &mut *bs,
        (*sci).bitalloc.as_mut_ptr(),
        (*sci).scfcod.as_mut_ptr(),
        (*sci).total_bands as (i32) * 2,
        (*sci).scf.as_mut_ptr(),
    );
    i = (*sci).stereo_bands as (i32);
    loop {
        if !(i < (*sci).total_bands as (i32)) {
            break;
        }
        (*sci).bitalloc[(2 * i + 1) as usize] = 0;
        i = i + 1;
    }
}

unsafe fn l12_dequantize_granule(
    grbuf: *mut f32,
    bs: &mut Bs,
    sci: &mut L12ScaleInfo,
    group_size: i32,
) -> i32 {
    let mut i: i32;
    let mut j: i32;
    let mut k: i32;
    let mut choff: i32 = 576;
    j = 0;
    loop {
        if !(j < 4) {
            break;
        }
        let mut dst: *mut f32 = grbuf.offset((group_size * j) as isize);
        i = 0;
        loop {
            if !(i < 2 * (*sci).total_bands as (i32)) {
                break;
            }
            let ba: i32 = (*sci).bitalloc[i as usize] as (i32);
            if ba != 0 {
                if ba < 17 {
                    let half: i32 = (1 << ba - 1) - 1;
                    k = 0;
                    loop {
                        if !(k < group_size) {
                            break;
                        }
                        *dst.offset(k as isize) = (get_bits(bs, ba as u32) as (i32) - half) as f32;
                        k = k + 1;
                    }
                } else {
                    let mod_: u32 = ((2 << ba - 17) + 1) as (u32);
                    let mut code: u32 =
                        get_bits(bs, mod_.wrapping_add(2).wrapping_sub(mod_ >> 3) as (u32));
                    k = 0;
                    loop {
                        if !(k < group_size) {
                            break;
                        }
                        *dst.offset(k as isize) =
                            code.wrapping_rem(mod_).wrapping_sub(mod_.wrapping_div(2)) as (i32)
                                as f32;
                        k = k + 1;
                        code = code.wrapping_div(mod_);
                    }
                }
            }
            dst = dst.offset(choff as isize);
            choff = 18 - choff;
            i = i + 1;
        }
        j = j + 1;
    }
    group_size * 4
}

unsafe fn l12_apply_scf_384(sci: &mut L12ScaleInfo, mut scf: *const f32, mut dst: *mut f32) {
    let mut i: i32;
    let mut k: i32;
    memcpy(
        dst.offset(576)
            .offset(((*sci).stereo_bands as (i32) * 18) as isize)
            as (*mut ::std::os::raw::c_void),
        dst.offset(((*sci).stereo_bands as (i32) * 18) as isize) as (*const ::std::os::raw::c_void),
        ((((*sci).total_bands as (i32) - (*sci).stereo_bands as (i32)) * 18) as usize)
            .wrapping_mul(::std::mem::size_of::<f32>()),
    );
    i = 0;
    loop {
        if !(i < (*sci).total_bands as (i32)) {
            break;
        }
        k = 0;
        loop {
            if !(k < 12) {
                break;
            }
            let _rhs = *scf.offset(0);
            let _lhs = &mut *dst.offset((k + 0) as isize);
            *_lhs = *_lhs * _rhs;
            let _rhs = *scf.offset(3);
            let _lhs = &mut *dst.offset((k + 576) as isize);
            *_lhs = *_lhs * _rhs;
            k = k + 1;
        }
        i = i + 1;
        dst = dst.offset(18);
        scf = scf.offset(6);
    }
}

#[allow(non_snake_case)]
unsafe fn mp3d_DCT_II(grbuf: *mut f32, n: i32) {
    static G_SEC: [f32; 24] = [
        10.19000816,
        0.50060302,
        0.50241929,
        3.40760851,
        0.50547093,
        0.52249861,
        2.05778098,
        0.51544732,
        0.56694406,
        1.48416460,
        0.53104258,
        0.64682180,
        1.16943991,
        0.55310392,
        0.78815460,
        0.97256821,
        0.58293498,
        1.06067765,
        0.83934963,
        0.62250412,
        1.72244716,
        0.74453628,
        0.67480832,
        5.10114861,
    ];
    let mut i: i32;
    let mut k: i32 = 0;
    loop {
        if !(k < n) {
            break;
        }
        let mut t: [[f32; 8]; 4] = [[0.0; 8]; 4];
        let mut x: *mut f32;
        let mut y: *mut f32 = grbuf.offset(k as isize);
        x = t[0].as_mut_ptr();
        i = 0;
        loop {
            if !(i < 8) {
                break;
            }
            let x0: f32 = *y.offset((i * 18) as isize);
            let x1: f32 = *y.offset(((15 - i) * 18) as isize);
            let x2: f32 = *y.offset(((16 + i) * 18) as isize);
            let x3: f32 = *y.offset(((31 - i) * 18) as isize);
            let t0: f32 = x0 + x3;
            let t1: f32 = x1 + x2;
            let t2: f32 = (x1 - x2) * G_SEC[(3 * i + 0) as usize];
            let t3: f32 = (x0 - x3) * G_SEC[(3 * i + 1) as usize];
            *x.offset(0) = t0 + t1;
            *x.offset(8) = (t0 - t1) * G_SEC[(3 * i + 2) as usize];
            *x.offset(16) = t3 + t2;
            *x.offset(24) = (t3 - t2) * G_SEC[(3 * i + 2) as usize];
            i = i + 1;
            x = x.offset(1);
        }
        x = t[0].as_mut_ptr();
        i = 0;
        loop {
            if !(i < 4) {
                break;
            }
            let mut x0: f32 = *x.offset(0);
            let mut x1: f32 = *x.offset(1);
            let mut x2: f32 = *x.offset(2);
            let mut x3: f32 = *x.offset(3);
            let mut x4: f32 = *x.offset(4);
            let mut x5: f32 = *x.offset(5);
            let mut x6: f32 = *x.offset(6);
            let mut x7: f32 = *x.offset(7);
            let mut xt: f32;
            xt = x0 - x7;
            x0 = x0 + x7;
            x7 = x1 - x6;
            x1 = x1 + x6;
            x6 = x2 - x5;
            x2 = x2 + x5;
            x5 = x3 - x4;
            x3 = x3 + x4;
            x4 = x0 - x3;
            x0 = x0 + x3;
            x3 = x1 - x2;
            x1 = x1 + x2;
            *x.offset(0) = x0 + x1;
            *x.offset(4) = (x0 - x1) * 0.70710677;
            x5 = x5 + x6;
            x6 = (x6 + x7) * 0.70710677;
            x7 = x7 + xt;
            x3 = (x3 + x4) * 0.70710677;
            x5 = x5 - x7 * 0.198912367;
            x7 = x7 + x5 * 0.382683432;
            x5 = x5 - x7 * 0.198912367;
            x0 = xt - x6;
            xt = xt + x6;
            *x.offset(1) = (xt + x7) * 0.50979561;
            *x.offset(2) = (x4 + x3) * 0.54119611;
            *x.offset(3) = (x0 - x5) * 0.60134488;
            *x.offset(5) = (x0 + x5) * 0.89997619;
            *x.offset(6) = (x4 - x3) * 1.30656302;
            *x.offset(7) = (xt - x7) * 2.56291556;
            i = i + 1;
            x = x.offset(8);
        }
        i = 0;
        loop {
            if !(i < 7) {
                break;
            }
            *y.offset((0 * 18) as isize) = t[0][i as usize];
            *y.offset((1 * 18) as isize) =
                t[2][i as usize] + t[3][i as usize] + t[3][(i + 1) as usize];
            *y.offset((2 * 18) as isize) = t[1][i as usize] + t[1][(i + 1) as usize];
            *y.offset((3 * 18) as isize) =
                t[2][(i + 1) as usize] + t[3][i as usize] + t[3][(i + 1) as usize];
            i = i + 1;
            y = y.offset((4 * 18) as isize);
        }
        *y.offset((0 * 18) as isize) = t[0][7];
        *y.offset((1 * 18) as isize) = t[2][7] + t[3][7];
        *y.offset((2 * 18) as isize) = t[1][7];
        *y.offset((3 * 18) as isize) = t[3][7];
        k = k + 1;
    }
}

fn mp3d_scale_pcm(sample: f32) -> i16 {
    if sample as (f64) >= 32766.5f64 {
        32767
    } else if sample as (f64) <= -32767.5f64 {
        -32768
    } else {
        let mut s: i16 = (sample + 0.5) as (i16);
        s = (s as (i32) - (s as (i32) < 0) as (i32)) as (i16);
        s
    }
}

unsafe fn mp3d_synth_pair(pcm: *mut i16, nch: i32, mut z: *const f32) {
    let mut a: f32;
    a = (*z.offset((14 * 64) as isize) - *z.offset(0)) * 29.0;
    a = a + (*z.offset((1 * 64) as isize) + *z.offset((13 * 64) as isize)) * 213.0;
    a = a + (*z.offset((12 * 64) as isize) - *z.offset((2 * 64) as isize)) * 459.0;
    a = a + (*z.offset((3 * 64) as isize) + *z.offset((11 * 64) as isize)) * 2037.0;
    a = a + (*z.offset((10 * 64) as isize) - *z.offset((4 * 64) as isize)) * 5153.0;
    a = a + (*z.offset((5 * 64) as isize) + *z.offset((9 * 64) as isize)) * 6574.0;
    a = a + (*z.offset((8 * 64) as isize) - *z.offset((6 * 64) as isize)) * 37489.0;
    a = a + *z.offset((7 * 64) as isize) * 75038.0;
    *pcm.offset(0) = mp3d_scale_pcm(a);
    z = z.offset(2);
    a = *z.offset((14 * 64) as isize) * 104.0;
    a = a + *z.offset((12 * 64) as isize) * 1567.0;
    a = a + *z.offset((10 * 64) as isize) * 9727.0;
    a = a + *z.offset((8 * 64) as isize) * 64019.0;
    a = a + *z.offset((6 * 64) as isize) * -9975.0;
    a = a + *z.offset((4 * 64) as isize) * -45.0;
    a = a + *z.offset((2 * 64) as isize) * 146.0;
    a = a + *z.offset((0 * 64) as isize) * -5.0;
    *pcm.offset((16 * nch) as isize) = mp3d_scale_pcm(a);
}

unsafe fn mp3d_synth(xl: *mut f32, dstl: *mut i16, nch: i32, lins: *mut f32) {
    let mut i: i32;
    let xr: *mut f32 = xl.offset((576 * (nch - 1)) as isize);
    let dstr: *mut i16 = dstl.offset((nch - 1) as isize);
    static G_WIN: [f32; 240] = [
        -1.0, 26.0, -31.0, 208.0, 218.0, 401.0, -519.0, 2063.0, 2000.0, 4788.0, -5517.0, 7134.0,
        5959.0, 35640.0, -39336.0, 74992.0, -1.0, 24.0, -35.0, 202.0, 222.0, 347.0, -581.0, 2080.0,
        1952.0, 4425.0, -5879.0, 7640.0, 5288.0, 33791.0, -41176.0, 74856.0, -1.0, 21.0, -38.0,
        196.0, 225.0, 294.0, -645.0, 2087.0, 1893.0, 4063.0, -6237.0, 8092.0, 4561.0, 31947.0,
        -43006.0, 74630.0, -1.0, 19.0, -41.0, 190.0, 227.0, 244.0, -711.0, 2085.0, 1822.0, 3705.0,
        -6589.0, 8492.0, 3776.0, 30112.0, -44821.0, 74313.0, -1.0, 17.0, -45.0, 183.0, 228.0,
        197.0, -779.0, 2075.0, 1739.0, 3351.0, -6935.0, 8840.0, 2935.0, 28289.0, -46617.0, 73908.0,
        -1.0, 16.0, -49.0, 176.0, 228.0, 153.0, -848.0, 2057.0, 1644.0, 3004.0, -7271.0, 9139.0,
        2037.0, 26482.0, -48390.0, 73415.0, -2.0, 14.0, -53.0, 169.0, 227.0, 111.0, -919.0, 2032.0,
        1535.0, 2663.0, -7597.0, 9389.0, 1082.0, 24694.0, -50137.0, 72835.0, -2.0, 13.0, -58.0,
        161.0, 224.0, 72.0, -991.0, 2001.0, 1414.0, 2330.0, -7910.0, 9592.0, 70.0, 22929.0,
        -51853.0, 72169.0, -2.0, 11.0, -63.0, 154.0, 221.0, 36.0, -1064.0, 1962.0, 1280.0, 2006.0,
        -8209.0, 9750.0, -998.0, 21189.0, -53534.0, 71420.0, -2.0, 10.0, -68.0, 147.0, 215.0, 2.0,
        -1137.0, 1919.0, 1131.0, 1692.0, -8491.0, 9863.0, -2122.0, 19478.0, -55178.0, 70590.0,
        -3.0, 9.0, -73.0, 139.0, 208.0, -29.0, -1210.0, 1870.0, 970.0, 1388.0, -8755.0, 9935.0,
        -3300.0, 17799.0, -56778.0, 69679.0, -3.0, 8.0, -79.0, 132.0, 200.0, -57.0, -1283.0,
        1817.0, 794.0, 1095.0, -8998.0, 9966.0, -4533.0, 16155.0, -58333.0, 68692.0, -4.0, 7.0,
        -85.0, 125.0, 189.0, -83.0, -1356.0, 1759.0, 605.0, 814.0, -9219.0, 9959.0, -5818.0,
        14548.0, -59838.0, 67629.0, -4.0, 7.0, -91.0, 117.0, 177.0, -106.0, -1428.0, 1698.0, 402.0,
        545.0, -9416.0, 9916.0, -7154.0, 12980.0, -61289.0, 66494.0, -5.0, 6.0, -97.0, 111.0,
        163.0, -127.0, -1498.0, 1634.0, 185.0, 288.0, -9585.0, 9838.0, -8540.0, 11455.0, -62684.0,
        65290.0,
    ];
    let zlin: *mut f32 = lins.offset((15 * 64) as isize);
    let mut w: *const f32 = G_WIN.as_ptr();
    *zlin.offset((4 * 15) as isize) = *xl.offset((18 * 16) as isize);
    *zlin.offset((4 * 15 + 1) as isize) = *xr.offset((18 * 16) as isize);
    *zlin.offset((4 * 15 + 2) as isize) = *xl.offset(0);
    *zlin.offset((4 * 15 + 3) as isize) = *xr.offset(0);
    *zlin.offset((4 * 31) as isize) = *xl.offset((1 + 18 * 16) as isize);
    *zlin.offset((4 * 31 + 1) as isize) = *xr.offset((1 + 18 * 16) as isize);
    *zlin.offset((4 * 31 + 2) as isize) = *xl.offset(1);
    *zlin.offset((4 * 31 + 3) as isize) = *xr.offset(1);
    mp3d_synth_pair(
        dstr,
        nch,
        lins.offset((4 * 15) as isize).offset(1) as (*const f32),
    );
    mp3d_synth_pair(
        dstr.offset((32 * nch) as isize),
        nch,
        lins.offset((4 * 15) as isize).offset(64).offset(1) as (*const f32),
    );
    mp3d_synth_pair(dstl, nch, lins.offset((4 * 15) as isize) as (*const f32));
    mp3d_synth_pair(
        dstl.offset((32 * nch) as isize),
        nch,
        lins.offset((4 * 15) as isize).offset(64) as (*const f32),
    );
    i = 14;
    loop {
        if !(i >= 0) {
            break;
        }
        let mut a: [f32; 4] = [0.0; 4];
        let mut b: [f32; 4] = [0.0; 4];
        *zlin.offset((4 * i) as isize) = *xl.offset((18 * (31 - i)) as isize);
        *zlin.offset((4 * i + 1) as isize) = *xr.offset((18 * (31 - i)) as isize);
        *zlin.offset((4 * i + 2) as isize) = *xl.offset((1 + 18 * (31 - i)) as isize);
        *zlin.offset((4 * i + 3) as isize) = *xr.offset((1 + 18 * (31 - i)) as isize);
        *zlin.offset((4 * (i + 16)) as isize) = *xl.offset((1 + 18 * (1 + i)) as isize);
        *zlin.offset((4 * (i + 16) + 1) as isize) = *xr.offset((1 + 18 * (1 + i)) as isize);
        *zlin.offset((4 * (i - 16) + 2) as isize) = *xl.offset((18 * (1 + i)) as isize);
        *zlin.offset((4 * (i - 16) + 3) as isize) = *xr.offset((18 * (1 + i)) as isize);
        let mut j: i32;
        let w0: f32 = *{
            let _old = w;
            w = w.offset(1);
            _old
        };
        let w1: f32 = *{
            let _old = w;
            w = w.offset(1);
            _old
        };
        let vz: *mut f32 = &mut *zlin.offset((4 * i - 0 * 64) as isize) as (*mut f32);
        let vy: *mut f32 = &mut *zlin.offset((4 * i - (15 - 0) * 64) as isize) as (*mut f32);
        j = 0;
        loop {
            if !(j < 4) {
                break;
            }
            b[j as usize] = *vz.offset(j as isize) * w1 + *vy.offset(j as isize) * w0;
            a[j as usize] = *vz.offset(j as isize) * w0 - *vy.offset(j as isize) * w1;
            j = j + 1;
        }
        let mut j: i32;
        let w0: f32 = *{
            let _old = w;
            w = w.offset(1);
            _old
        };
        let w1: f32 = *{
            let _old = w;
            w = w.offset(1);
            _old
        };
        let vz: *mut f32 = &mut *zlin.offset((4 * i - 1 * 64) as isize) as (*mut f32);
        let vy: *mut f32 = &mut *zlin.offset((4 * i - (15 - 1) * 64) as isize) as (*mut f32);
        j = 0;
        loop {
            if !(j < 4) {
                break;
            }
            let _rhs = *vz.offset(j as isize) * w1 + *vy.offset(j as isize) * w0;
            let _lhs = &mut b[j as usize];
            *_lhs = *_lhs + _rhs;
            let _rhs = *vy.offset(j as isize) * w1 - *vz.offset(j as isize) * w0;
            let _lhs = &mut a[j as usize];
            *_lhs = *_lhs + _rhs;
            j = j + 1;
        }
        let mut j: i32;
        let w0: f32 = *{
            let _old = w;
            w = w.offset(1);
            _old
        };
        let w1: f32 = *{
            let _old = w;
            w = w.offset(1);
            _old
        };
        let vz: *mut f32 = &mut *zlin.offset((4 * i - 2 * 64) as isize) as (*mut f32);
        let vy: *mut f32 = &mut *zlin.offset((4 * i - (15 - 2) * 64) as isize) as (*mut f32);
        j = 0;
        loop {
            if !(j < 4) {
                break;
            }
            let _rhs = *vz.offset(j as isize) * w1 + *vy.offset(j as isize) * w0;
            let _lhs = &mut b[j as usize];
            *_lhs = *_lhs + _rhs;
            let _rhs = *vz.offset(j as isize) * w0 - *vy.offset(j as isize) * w1;
            let _lhs = &mut a[j as usize];
            *_lhs = *_lhs + _rhs;
            j = j + 1;
        }
        let mut j: i32;
        let w0: f32 = *{
            let _old = w;
            w = w.offset(1);
            _old
        };
        let w1: f32 = *{
            let _old = w;
            w = w.offset(1);
            _old
        };
        let vz: *mut f32 = &mut *zlin.offset((4 * i - 3 * 64) as isize) as (*mut f32);
        let vy: *mut f32 = &mut *zlin.offset((4 * i - (15 - 3) * 64) as isize) as (*mut f32);
        j = 0;
        loop {
            if !(j < 4) {
                break;
            }
            let _rhs = *vz.offset(j as isize) * w1 + *vy.offset(j as isize) * w0;
            let _lhs = &mut b[j as usize];
            *_lhs = *_lhs + _rhs;
            let _rhs = *vy.offset(j as isize) * w1 - *vz.offset(j as isize) * w0;
            let _lhs = &mut a[j as usize];
            *_lhs = *_lhs + _rhs;
            j = j + 1;
        }
        let mut j: i32;
        let w0: f32 = *{
            let _old = w;
            w = w.offset(1);
            _old
        };
        let w1: f32 = *{
            let _old = w;
            w = w.offset(1);
            _old
        };
        let vz: *mut f32 = &mut *zlin.offset((4 * i - 4 * 64) as isize) as (*mut f32);
        let vy: *mut f32 = &mut *zlin.offset((4 * i - (15 - 4) * 64) as isize) as (*mut f32);
        j = 0;
        loop {
            if !(j < 4) {
                break;
            }
            let _rhs = *vz.offset(j as isize) * w1 + *vy.offset(j as isize) * w0;
            let _lhs = &mut b[j as usize];
            *_lhs = *_lhs + _rhs;
            let _rhs = *vz.offset(j as isize) * w0 - *vy.offset(j as isize) * w1;
            let _lhs = &mut a[j as usize];
            *_lhs = *_lhs + _rhs;
            j = j + 1;
        }
        let mut j: i32;
        let w0: f32 = *{
            let _old = w;
            w = w.offset(1);
            _old
        };
        let w1: f32 = *{
            let _old = w;
            w = w.offset(1);
            _old
        };
        let vz: *mut f32 = &mut *zlin.offset((4 * i - 5 * 64) as isize) as (*mut f32);
        let vy: *mut f32 = &mut *zlin.offset((4 * i - (15 - 5) * 64) as isize) as (*mut f32);
        j = 0;
        loop {
            if !(j < 4) {
                break;
            }
            let _rhs = *vz.offset(j as isize) * w1 + *vy.offset(j as isize) * w0;
            let _lhs = &mut b[j as usize];
            *_lhs = *_lhs + _rhs;
            let _rhs = *vy.offset(j as isize) * w1 - *vz.offset(j as isize) * w0;
            let _lhs = &mut a[j as usize];
            *_lhs = *_lhs + _rhs;
            j = j + 1;
        }
        let mut j: i32;
        let w0: f32 = *{
            let _old = w;
            w = w.offset(1);
            _old
        };
        let w1: f32 = *{
            let _old = w;
            w = w.offset(1);
            _old
        };
        let vz: *mut f32 = &mut *zlin.offset((4 * i - 6 * 64) as isize) as (*mut f32);
        let vy: *mut f32 = &mut *zlin.offset((4 * i - (15 - 6) * 64) as isize) as (*mut f32);
        j = 0;
        loop {
            if !(j < 4) {
                break;
            }
            let _rhs = *vz.offset(j as isize) * w1 + *vy.offset(j as isize) * w0;
            let _lhs = &mut b[j as usize];
            *_lhs = *_lhs + _rhs;
            let _rhs = *vz.offset(j as isize) * w0 - *vy.offset(j as isize) * w1;
            let _lhs = &mut a[j as usize];
            *_lhs = *_lhs + _rhs;
            j = j + 1;
        }
        let mut j: i32;
        let w0: f32 = *{
            let _old = w;
            w = w.offset(1);
            _old
        };
        let w1: f32 = *{
            let _old = w;
            w = w.offset(1);
            _old
        };
        let vz: *mut f32 = &mut *zlin.offset((4 * i - 7 * 64) as isize) as (*mut f32);
        let vy: *mut f32 = &mut *zlin.offset((4 * i - (15 - 7) * 64) as isize) as (*mut f32);
        j = 0;
        loop {
            if !(j < 4) {
                break;
            }
            let _rhs = *vz.offset(j as isize) * w1 + *vy.offset(j as isize) * w0;
            let _lhs = &mut b[j as usize];
            *_lhs = *_lhs + _rhs;
            let _rhs = *vy.offset(j as isize) * w1 - *vz.offset(j as isize) * w0;
            let _lhs = &mut a[j as usize];
            *_lhs = *_lhs + _rhs;
            j = j + 1;
        }
        *dstr.offset(((15 - i) * nch) as isize) = mp3d_scale_pcm(a[1]);
        *dstr.offset(((17 + i) * nch) as isize) = mp3d_scale_pcm(b[1]);
        *dstl.offset(((15 - i) * nch) as isize) = mp3d_scale_pcm(a[0]);
        *dstl.offset(((17 + i) * nch) as isize) = mp3d_scale_pcm(b[0]);
        *dstr.offset(((47 - i) * nch) as isize) = mp3d_scale_pcm(a[3]);
        *dstr.offset(((49 + i) * nch) as isize) = mp3d_scale_pcm(b[3]);
        *dstl.offset(((47 - i) * nch) as isize) = mp3d_scale_pcm(a[2]);
        *dstl.offset(((49 + i) * nch) as isize) = mp3d_scale_pcm(b[2]);
        i = i - 1;
    }
}

unsafe fn mp3d_synth_granule(
    qmf_state: *mut f32,
    grbuf: *mut f32,
    nbands: i32,
    nch: i32,
    pcm: &mut [i16],
    lins: *mut f32,
) {
    let mut i: i32;
    i = 0;
    loop {
        if !(i < nch) {
            break;
        }
        mp3d_DCT_II(grbuf.offset((576 * i) as isize), nbands);
        i = i + 1;
    }
    memcpy(
        lins as (*mut ::std::os::raw::c_void),
        qmf_state as (*const ::std::os::raw::c_void),
        ::std::mem::size_of::<f32>()
            .wrapping_mul(15)
            .wrapping_mul(64),
    );
    i = 0;
    loop {
        if !(i < nbands) {
            break;
        }
        let offset = (32 * nch * i) as usize;
        mp3d_synth(
            grbuf.offset(i as isize),
            pcm[offset..].as_mut_ptr(),
            nch,
            lins.offset((i * 64) as isize),
        );
        i = i + 2;
    }
    if nch == 1 {
        i = 0;
        loop {
            if !(i < 15 * 64) {
                break;
            }
            *qmf_state.offset(i as isize) = *lins.offset((nbands * 64 + i) as isize);
            i = i + 2;
        }
    } else {
        memcpy(
            qmf_state as (*mut ::std::os::raw::c_void),
            lins.offset((nbands * 64) as isize) as (*const ::std::os::raw::c_void),
            ::std::mem::size_of::<f32>()
                .wrapping_mul(15)
                .wrapping_mul(64),
        );
    }
}

/// TODO: The gr pointer is apparently actually an array
unsafe fn l3_read_side_info(bs: &mut Bs, mut gr: *mut L3GrInfo, hdr: &[u8]) -> i32 {
    let current_block;
    static G_SCF_LONG: [[u8; 23]; 8] = [
        [
            6, 6, 6, 6, 6, 6, 8, 10, 12, 14, 16, 20, 24, 28, 32, 38, 46, 52, 60, 68, 58, 54, 0,
        ],
        [
            12, 12, 12, 12, 12, 12, 16, 20, 24, 28, 32, 40, 48, 56, 64, 76, 90, 2, 2, 2, 2, 2, 0,
        ],
        [
            6, 6, 6, 6, 6, 6, 8, 10, 12, 14, 16, 20, 24, 28, 32, 38, 46, 52, 60, 68, 58, 54, 0,
        ],
        [
            6, 6, 6, 6, 6, 6, 8, 10, 12, 14, 16, 18, 22, 26, 32, 38, 46, 54, 62, 70, 76, 36, 0,
        ],
        [
            6, 6, 6, 6, 6, 6, 8, 10, 12, 14, 16, 20, 24, 28, 32, 38, 46, 52, 60, 68, 58, 54, 0,
        ],
        [
            4, 4, 4, 4, 4, 4, 6, 6, 8, 8, 10, 12, 16, 20, 24, 28, 34, 42, 50, 54, 76, 158, 0,
        ],
        [
            4, 4, 4, 4, 4, 4, 6, 6, 6, 8, 10, 12, 16, 18, 22, 28, 34, 40, 46, 54, 54, 192, 0,
        ],
        [
            4, 4, 4, 4, 4, 4, 6, 6, 8, 10, 12, 16, 20, 24, 30, 38, 46, 56, 68, 84, 102, 26, 0,
        ],
    ];
    static G_SCF_SHORT: [[u8; 40]; 8] = [
        [
            4, 4, 4, 4, 4, 4, 4, 4, 4, 6, 6, 6, 8, 8, 8, 10, 10, 10, 12, 12, 12, 14, 14, 14, 18,
            18, 18, 24, 24, 24, 30, 30, 30, 40, 40, 40, 18, 18, 18, 0,
        ],
        [
            8, 8, 8, 8, 8, 8, 8, 8, 8, 12, 12, 12, 16, 16, 16, 20, 20, 20, 24, 24, 24, 28, 28, 28,
            36, 36, 36, 2, 2, 2, 2, 2, 2, 2, 2, 2, 26, 26, 26, 0,
        ],
        [
            4, 4, 4, 4, 4, 4, 4, 4, 4, 6, 6, 6, 6, 6, 6, 8, 8, 8, 10, 10, 10, 14, 14, 14, 18, 18,
            18, 26, 26, 26, 32, 32, 32, 42, 42, 42, 18, 18, 18, 0,
        ],
        [
            4, 4, 4, 4, 4, 4, 4, 4, 4, 6, 6, 6, 8, 8, 8, 10, 10, 10, 12, 12, 12, 14, 14, 14, 18,
            18, 18, 24, 24, 24, 32, 32, 32, 44, 44, 44, 12, 12, 12, 0,
        ],
        [
            4, 4, 4, 4, 4, 4, 4, 4, 4, 6, 6, 6, 8, 8, 8, 10, 10, 10, 12, 12, 12, 14, 14, 14, 18,
            18, 18, 24, 24, 24, 30, 30, 30, 40, 40, 40, 18, 18, 18, 0,
        ],
        [
            4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 6, 6, 6, 8, 8, 8, 10, 10, 10, 12, 12, 12, 14, 14,
            14, 18, 18, 18, 22, 22, 22, 30, 30, 30, 56, 56, 56, 0,
        ],
        [
            4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 6, 6, 6, 6, 6, 6, 10, 10, 10, 12, 12, 12, 14, 14,
            14, 16, 16, 16, 20, 20, 20, 26, 26, 26, 66, 66, 66, 0,
        ],
        [
            4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 6, 6, 6, 8, 8, 8, 12, 12, 12, 16, 16, 16, 20, 20,
            20, 26, 26, 26, 34, 34, 34, 42, 42, 42, 12, 12, 12, 0,
        ],
    ];
    // TODO: These... lengths are wrong???  I jus padded them out with 0's
    static G_SCF_MIXED: [[u8; 40]; 8] = [
        [
            6, 6, 6, 6, 6, 6, 6, 6, 6, 8, 8, 8, 10, 10, 10, 12, 12, 12, 14, 14, 14, 18, 18, 18, 24,
            24, 24, 30, 30, 30, 40, 40, 40, 18, 18, 18, 0, 0, 0, 0,
        ],
        [
            12, 12, 12, 4, 4, 4, 8, 8, 8, 12, 12, 12, 16, 16, 16, 20, 20, 20, 24, 24, 24, 28, 28,
            28, 36, 36, 36, 2, 2, 2, 2, 2, 2, 2, 2, 2, 26, 26, 26, 0,
        ],
        [
            6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 8, 8, 8, 10, 10, 10, 14, 14, 14, 18, 18, 18, 26,
            26, 26, 32, 32, 32, 42, 42, 42, 18, 18, 18, 0, 0, 0, 0,
        ],
        [
            6, 6, 6, 6, 6, 6, 6, 6, 6, 8, 8, 8, 10, 10, 10, 12, 12, 12, 14, 14, 14, 18, 18, 18, 24,
            24, 24, 32, 32, 32, 44, 44, 44, 12, 12, 12, 0, 0, 0, 0,
        ],
        [
            6, 6, 6, 6, 6, 6, 6, 6, 6, 8, 8, 8, 10, 10, 10, 12, 12, 12, 14, 14, 14, 18, 18, 18, 24,
            24, 24, 30, 30, 30, 40, 40, 40, 18, 18, 18, 0, 0, 0, 0,
        ],
        [
            4, 4, 4, 4, 4, 4, 6, 6, 4, 4, 4, 6, 6, 6, 8, 8, 8, 10, 10, 10, 12, 12, 12, 14, 14, 14,
            18, 18, 18, 22, 22, 22, 30, 30, 30, 56, 56, 56, 0, 0,
        ],
        [
            4, 4, 4, 4, 4, 4, 6, 6, 4, 4, 4, 6, 6, 6, 6, 6, 6, 10, 10, 10, 12, 12, 12, 14, 14, 14,
            16, 16, 16, 20, 20, 20, 26, 26, 26, 66, 66, 66, 0, 0,
        ],
        [
            4, 4, 4, 4, 4, 4, 6, 6, 4, 4, 4, 6, 6, 6, 8, 8, 8, 12, 12, 12, 16, 16, 16, 20, 20, 20,
            26, 26, 26, 34, 34, 34, 42, 42, 42, 12, 12, 12, 0, 0,
        ],
    ];
    let mut tables: u32;
    let mut scfsi: u32 = 0;
    let main_data_begin: i32;
    let mut part_23_sum: i32 = 0;
    let mut sr_idx: i32 =
        (hdr[2] as (i32) >> 2 & 3) + ((hdr[1] as (i32) >> 3 & 1) + (hdr[1] as (i32) >> 4 & 1)) * 3;
    sr_idx = sr_idx - (sr_idx != 0) as (i32);
    let mut gr_count: u32 = if hdr[3] as (i32) & 0xc0 == 0xc0 { 1 } else { 2 };
    if hdr[1] as (i32) & 0x8 != 0 {
        gr_count = gr_count * 2;
        main_data_begin = get_bits(bs, 9) as (i32);
        scfsi = get_bits(bs, 7 + gr_count);
    } else {
        main_data_begin = (get_bits(bs, 8 + gr_count) >> gr_count) as (i32);
    }
    loop {
        if hdr[3] as (i32) & 0xc0 == 0xc0 {
            scfsi = scfsi << 4;
        }
        (*gr).part_23_length = get_bits(bs, 12) as (u16);
        part_23_sum = part_23_sum + (*gr).part_23_length as (i32);
        (*gr).big_values = get_bits(bs, 9) as (u16);
        if (*gr).big_values as (i32) > 288 {
            current_block = 20;
            break;
        }
        (*gr).global_gain = get_bits(bs, 8) as (u8);
        (*gr).scalefac_compress =
            get_bits(bs, if hdr[1] as (i32) & 0x8 != 0 { 4 } else { 9 }) as (u16);
        (*gr).sfbtab = &G_SCF_LONG[sr_idx as usize];
        (*gr).n_long_sfb = 22;
        (*gr).n_short_sfb = 0;
        if get_bits(bs, 1) != 0 {
            (*gr).block_type = get_bits(bs, 2) as (u8);
            if (*gr).block_type == 0 {
                current_block = 19;
                break;
            }
            (*gr).mixed_block_flag = get_bits(bs, 1) as (u8);
            (*gr).region_count[0] = 7;
            (*gr).region_count[1] = 255;
            if (*gr).block_type as (i32) == 2 {
                scfsi = scfsi & 0xf0fu32;
                if (*gr).mixed_block_flag == 0 {
                    (*gr).region_count[0] = 8;
                    (*gr).sfbtab = &G_SCF_SHORT[sr_idx as usize];
                    (*gr).n_long_sfb = 0;
                    (*gr).n_short_sfb = 39;
                } else {
                    (*gr).sfbtab = &G_SCF_MIXED[sr_idx as usize];
                    (*gr).n_long_sfb = if hdr[1] as (i32) & 0x8 != 0 { 8 } else { 6 } as (u8);
                    (*gr).n_short_sfb = 30;
                }
            }
            tables = get_bits(bs, 10);
            tables = tables << 5;
            (*gr).subblock_gain[0] = get_bits(bs, 3) as (u8);
            (*gr).subblock_gain[1] = get_bits(bs, 3) as (u8);
            (*gr).subblock_gain[2] = get_bits(bs, 3) as (u8);
        } else {
            (*gr).block_type = 0;
            (*gr).mixed_block_flag = 0;
            tables = get_bits(bs, 15);
            (*gr).region_count[0] = get_bits(bs, 4) as (u8);
            (*gr).region_count[1] = get_bits(bs, 3) as (u8);
            (*gr).region_count[2] = 255;
        }
        (*gr).table_select[0] = (tables >> 10) as (u8);
        (*gr).table_select[1] = (tables >> 5 & 31) as (u8);
        (*gr).table_select[2] = (tables & 31) as (u8);
        (*gr).preflag = if hdr[1] as (i32) & 0x8 != 0 {
            get_bits(bs, 1)
        } else {
            ((*gr).scalefac_compress as (i32) >= 500) as (u32)
        } as (u8);
        (*gr).scalefac_scale = get_bits(bs, 1) as (u8);
        (*gr).count1_table = get_bits(bs, 1) as (u8);
        (*gr).scfsi = (scfsi >> 12 & 15) as (u8);
        scfsi = scfsi << 4;
        gr = gr.offset(1);
        if {
            gr_count = gr_count - 1;
            gr_count
        } == 0
        {
            current_block = 16;
            break;
        }
    }
    if current_block == 16 {
        (if part_23_sum + (*bs).pos > (*bs).limit + main_data_begin * 8 {
            -1
        } else {
            main_data_begin
        })
    } else if current_block == 19 {
        -1
    } else {
        -1
    }
}

unsafe fn l3_restore_reservoir(
    h: &mut Mp3Dec,
    bs: &mut Bs,
    s: *mut Mp3DecScratch,
    main_data_begin: i32,
) -> i32 {
    let frame_bytes: i32 = ((*bs).limit - (*bs).pos) / 8;
    let bytes_have: i32 = if (*h).reserv > main_data_begin {
        main_data_begin
    } else {
        (*h).reserv
    };
    memcpy(
        (*s).maindata.as_mut_ptr() as (*mut ::std::os::raw::c_void),
        (*h).reserv_buf
            .as_mut_ptr()
            .offset(if 0 < (*h).reserv - main_data_begin {
                (*h).reserv - main_data_begin
            } else {
                0
            } as isize) as (*const ::std::os::raw::c_void),
        if (*h).reserv > main_data_begin {
            main_data_begin
        } else {
            (*h).reserv
        } as usize,
    );
    memcpy(
        (*s).maindata.as_mut_ptr().offset(bytes_have as isize) as (*mut ::std::os::raw::c_void),
        (*bs).buf.as_ptr().offset(((*bs).pos / 8) as isize) as (*const ::std::os::raw::c_void),
        frame_bytes as usize,
    );
    (*s).bs = Bs::new(&(*s).maindata[..], bytes_have + frame_bytes);
    ((*h).reserv >= main_data_begin) as (i32)
}

fn l3_read_scalefactors(
    mut scf: &mut [u8],
    mut ist_pos: &mut [u8],
    scf_size: &[u8],
    scf_count: &[u8],
    bitbuf: &mut Bs,
    mut scfsi: i32,
) {
    let mut i: usize;
    let mut k: usize;
    i = 0;
    loop {
        if !(i < 4 && (scf_count[i] != 0)) {
            break;
        }
        let cnt: usize = scf_count[i] as usize;
        if scfsi & 8 != 0 {
            // memcpy(
            //     scf as (*mut ::std::os::raw::c_void),
            //     ist_pos as (*const ::std::os::raw::c_void),
            //     cnt as usize,
            // );
            scf.copy_from_slice(&ist_pos[..cnt]);
        } else {
            let bits: u32 = scf_size[i] as (u32);
            if bits == 0 {
                // memset(scf as (*mut ::std::os::raw::c_void), 0, cnt as usize);
                // memset(
                //     ist_pos as (*mut ::std::os::raw::c_void),
                //     0,
                //     cnt as usize,
                // );
                // ::std::ptr::write_bytes(scf, 0, cnt as usize);
                // ::std::ptr::write_bytes(ist_pos, 0, cnt as usize);
                // TODO: Crude, but eh.
                for i in 0..cnt {
                    scf[i] = 0;
                    ist_pos[i] = 0;
                }
            } else {
                let max_scf: i32 = if scfsi < 0 { (1 << bits) - 1 } else { -1 };
                k = 0;
                loop {
                    if !(k < cnt) {
                        break;
                    }
                    let s: i32 = get_bits(bitbuf, bits) as (i32);
                    ist_pos[k] = if s == max_scf { -1 } else { s } as (u8);
                    scf[k] = s as (u8);
                    k = k + 1;
                }
            }
        }
        // ist_pos = ist_pos.offset(cnt as isize);
        // scf = scf.offset(cnt as isize);
        increment_by_mut(&mut ist_pos, cnt);
        increment_by_mut(&mut scf, cnt);
        i = i + 1;
        scfsi = scfsi * 2;
    }
    // TODO: Clean up this horrible post-increment-y mess.
    scf[0] = {
        let _rhs = {
            let _rhs = 0;
            let _lhs = &mut scf[2];
            *_lhs = _rhs as (u8);
            *_lhs
        };
        let _lhs = &mut scf[1];
        *_lhs = _rhs;
        *_lhs
    };
}

fn l3_ldexp_q2(mut y: f32, mut exp_q2: i32) -> f32 {
    static G_EXPFRAC: [f32; 4] = [
        9.31322575e-10,
        7.83145814e-10,
        6.58544508e-10,
        5.53767716e-10,
    ];
    let mut e: i32;
    loop {
        e = if 30 * 4 > exp_q2 { exp_q2 } else { 30 * 4 };
        y = y * (G_EXPFRAC[(e & 3) as usize] * (1 << 30 >> (e >> 2)) as f32);
        if !({
            exp_q2 = exp_q2 - e;
            exp_q2
        } > 0)
        {
            break;
        }
    }
    y
}

fn l3_decode_scalefactors(
    hdr: &[u8],
    ist_pos: &mut [u8],
    bs: &mut Bs,
    gr: &L3GrInfo,
    scf: &mut [f32],
    ch: i32,
) {
    static G_SCF_PARTITIONS: [[u8; 28]; 3] = [
        [
            6, 5, 5, 5, 6, 5, 5, 5, 6, 5, 7, 3, 11, 10, 0, 0, 7, 7, 7, 0, 6, 6, 6, 3, 8, 8, 5, 0,
        ],
        [
            8, 9, 6, 12, 6, 9, 9, 9, 6, 9, 12, 6, 15, 18, 0, 0, 6, 15, 12, 0, 6, 12, 9, 6, 6, 18,
            9, 0,
        ],
        [
            9, 9, 6, 12, 9, 9, 9, 9, 9, 9, 12, 6, 18, 18, 0, 0, 12, 12, 12, 0, 12, 9, 9, 6, 15, 12,
            9, 0,
        ],
    ];
    let scf_partition = G_SCF_PARTITIONS
        [(!((*gr).n_short_sfb == 0) as (i32) + ((*gr).n_long_sfb == 0) as (i32)) as usize];
    let mut scf_size: [u8; 4] = [0; 4];
    let mut iscf: [u8; 40] = [0; 40];
    let mut i: i32;
    let scf_shift: i32 = (*gr).scalefac_scale as (i32) + 1;
    let gain_exp: i32;
    let mut scfsi: i32 = (*gr).scfsi as (i32);
    let gain: f32;
    if hdr[1] as (i32) & 0x8 != 0 {
        static G_SCFC_DECODE: [u8; 16] = [0, 1, 2, 3, 12, 5, 6, 7, 9, 10, 11, 13, 14, 15, 18, 19];
        let part: i32 = G_SCFC_DECODE[(*gr).scalefac_compress as usize] as (i32);
        scf_size[1] = {
            let _rhs = (part >> 2) as (u8);
            let _lhs = &mut scf_size[0];
            *_lhs = _rhs;
            *_lhs
        };
        scf_size[3] = {
            let _rhs = (part & 3) as (u8);
            let _lhs = &mut scf_size[2];
            *_lhs = _rhs;
            *_lhs
        };
    } else {
        static G_MOD: [u8; 24] = [
            5, 5, 4, 4, 5, 5, 4, 1, 4, 3, 1, 1, 5, 6, 6, 1, 4, 4, 4, 1, 4, 3, 1, 1,
        ];
        let mut k: i32;
        let mut modprod: i32;
        let mut sfc: i32;
        let ist: i32 = (hdr[3] as (i32) & 0x10 != 0 && (ch != 0)) as (i32);
        sfc = (*gr).scalefac_compress as (i32) >> ist;
        k = ist * 3 * 4;
        loop {
            if !(sfc >= 0) {
                break;
            }
            modprod = 1;
            i = 3;
            loop {
                if !(i >= 0) {
                    break;
                }
                scf_size[i as usize] = (sfc / modprod % G_MOD[(k + i) as usize] as (i32)) as (u8);
                modprod = modprod * G_MOD[(k + i) as usize] as (i32);
                i = i - 1;
            }
            sfc = sfc - modprod;
            k = k + 4;
        }
        // scf_partition = scf_partition.offset(k as isize);
        increment_by(&mut &scf_partition[..], k as usize);
        scfsi = -16;
    }
    l3_read_scalefactors(
        &mut iscf[..],
        ist_pos,
        &scf_size[..],
        &scf_partition,
        &mut *bs,
        scfsi,
    );
    if (*gr).n_short_sfb != 0 {
        let sh: i32 = 3 - scf_shift;
        i = 0;
        loop {
            if !(i < (*gr).n_short_sfb as (i32)) {
                break;
            }
            {
                let _rhs = (*gr).subblock_gain[0] as (i32) << sh;
                let _lhs = &mut iscf[((*gr).n_long_sfb as (i32) + i + 0) as usize];
                *_lhs = (*_lhs as (i32) + _rhs) as (u8);
            }
            {
                let _rhs = (*gr).subblock_gain[1] as (i32) << sh;
                let _lhs = &mut iscf[((*gr).n_long_sfb as (i32) + i + 1) as usize];
                *_lhs = (*_lhs as (i32) + _rhs) as (u8);
            }
            {
                let _rhs = (*gr).subblock_gain[2] as (i32) << sh;
                let _lhs = &mut iscf[((*gr).n_long_sfb as (i32) + i + 2) as usize];
                *_lhs = (*_lhs as (i32) + _rhs) as (u8);
            }
            i = i + 3;
        }
    } else if (*gr).preflag != 0 {
        static G_PREAMP: [u8; 10] = [1, 1, 1, 1, 2, 2, 3, 3, 3, 2];
        i = 0;
        loop {
            if !(i < 10) {
                break;
            }
            let _rhs = G_PREAMP[i as usize];
            let _lhs = &mut iscf[(11 + i) as usize];
            *_lhs = (*_lhs as (i32) + _rhs as (i32)) as (u8);
            i = i + 1;
        }
    }
    gain_exp = (*gr).global_gain as (i32) + -1 * 4
        - 210
        - if hdr[3] as (i32) & 0xe0 == 0x60 { 2 } else { 0 };
    gain = l3_ldexp_q2(
        (1 << (255 + -1 * 4 - 210 + 3 & !3) / 4) as f32,
        (255 + -1 * 4 - 210 + 3 & !3) - gain_exp,
    );
    i = 0;
    loop {
        if !(i < (*gr).n_long_sfb as (i32) + (*gr).n_short_sfb as (i32)) {
            break;
        }
        scf[i as usize] = l3_ldexp_q2(gain, iscf[i as usize] as (i32) << scf_shift);
        i = i + 1;
    }
}

fn l3_pow_43(mut x: i32) -> f32 {
    let frac: f32;
    let sign: i32;
    let mut mult: i32 = 256;
    if x < 129 {
        GPOW43[(16 + x) as usize]
    } else {
        if x < 1024 {
            mult = 16;
            x = x << 3;
        }
        sign = 2 * x & 64;
        frac = ((x & 63) - sign) as f32 / ((x & !63) + sign) as f32;
        GPOW43[(16 + (x + sign >> 6)) as usize]
            * (1.0 + frac * (4.0 / 3.0 + frac * (2.0 / 9.0)))
            * mult as f32
    }
}

unsafe fn l3_huffman(
    mut dst: *mut f32,
    bs: &mut Bs,
    gr_info: &L3GrInfo,
    mut scf: *const f32,
    layer3gr_limit: i32,
) {
    static TABS: [i16; 512] = [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 785, 785, 785, 785, 784, 784, 784, 784, 513, 513, 513, 513, 513, 513, 513, 513, 256,
        256, 256, 256, 256, 256, 256, 256, 256, 256, 256, 256, 256, 256, 256, 256, -255, 1313,
        1298, 1282, 785, 785, 785, 785, 784, 784, 784, 784, 769, 769, 769, 769, 256, 256, 256, 256,
        256, 256, 256, 256, 256, 256, 256, 256, 256, 256, 256, 256, 290, 288, -255, 1313, 1298,
        1282, 769, 769, 769, 769, 529, 529, 529, 529, 529, 529, 529, 529, 528, 528, 528, 528, 528,
        528, 528, 528, 512, 512, 512, 512, 512, 512, 512, 512, 290, 288, -253, -318, -351, -367,
        785, 785, 785, 785, 784, 784, 784, 784, 769, 769, 769, 769, 256, 256, 256, 256, 256, 256,
        256, 256, 256, 256, 256, 256, 256, 256, 256, 256, 819, 818, 547, 547, 275, 275, 275, 275,
        561, 560, 515, 546, 289, 274, 288, 258, -254, -287, 1329, 1299, 1314, 1312, 1057, 1057,
        1042, 1042, 1026, 1026, 784, 784, 784, 784, 529, 529, 529, 529, 529, 529, 529, 529, 769,
        769, 769, 769, 768, 768, 768, 768, 563, 560, 306, 306, 291, 259, -252, -413, -477, -542,
        1298, -575, 1041, 1041, 784, 784, 784, 784, 769, 769, 769, 769, 256, 256, 256, 256, 256,
        256, 256, 256, 256, 256, 256, 256, 256, 256, 256, 256, -383, -399, 1107, 1092, 1106, 1061,
        849, 849, 789, 789, 1104, 1091, 773, 773, 1076, 1075, 341, 340, 325, 309, 834, 804, 577,
        577, 532, 532, 516, 516, 832, 818, 803, 816, 561, 561, 531, 531, 515, 546, 289, 289, 288,
        258, -252, -429, -493, -559, 1057, 1057, 1042, 1042, 529, 529, 529, 529, 529, 529, 529,
        529, 784, 784, 784, 784, 769, 769, 769, 769, 512, 512, 512, 512, 512, 512, 512, 512, -382,
        1077, -415, 1106, 1061, 1104, 849, 849, 789, 789, 1091, 1076, 1029, 1075, 834, 834, 597,
        581, 340, 340, 339, 324, 804, 833, 532, 532, 832, 772, 818, 803, 817, 787, 816, 771, 290,
        290, 290, 290, 288, 258, -253, -349, -414, -447, -463, 1329, 1299, -479, 1314, 1312, 1057,
        1057, 1042, 1042, 1026, 1026, 785, 785, 785, 785, 784, 784, 784, 784, 769, 769, 769, 769,
        768, 768, 768, 768, -319, 851, 821, -335, 836, 850, 805, 849, 341, 340, 325, 336, 533, 533,
        579, 579, 564, 564, 773, 832, 578, 548, 563, 516, 321, 276, 306, 291, 304, 259, -251, -572,
        -733, -830, -863, -879, 1041, 1041, 784, 784, 784, 784, 769, 769, 769, 769, 256, 256, 256,
        256, 256, 256, 256, 256, 256, 256, 256, 256, 256, 256, 256, 256, -511, -527, -543, 1396,
        1351, 1381, 1366, 1395, 1335, 1380, -559, 1334, 1138, 1138, 1063, 1063, 1350, 1392, 1031,
        1031, 1062, 1062, 1364, 1363, 1120, 1120, 1333, 1348, 881, 881, 881, 881, 375, 374, 359,
        373, 343, 358, 341, 325, 791, 791, 1123, 1122, -703, 1105, 1045, -719, 865, 865, 790, 790,
        774, 774,
    ];
    static TAB32: [u8; 28] = [
        130, 162, 193, 209, 44, 28, 76, 140, 9, 9, 9, 9, 9, 9, 9, 9, 190, 254, 222, 238, 126, 94,
        157, 157, 109, 61, 173, 205,
    ];
    static TAB33: [u8; 16] = [
        252, 236, 220, 204, 188, 172, 156, 140, 124, 108, 92, 76, 60, 44, 28, 12,
    ];
    static TABINDEX: [i16; 32] = [
        0, 32, 64, 98, 0, 132, 180, 218, 292, 364, 426, 538, 648, 746, 0, 1126, 1460, 1460, 1460,
        1460, 1460, 1460, 1460, 1460, 1842, 1842, 1842, 1842, 1842, 1842, 1842, 1842,
    ];
    static G_LINBITS: [u8; 32] = [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 2, 3, 4, 6, 8, 10, 13, 4, 5, 6, 7, 8, 9,
        11, 13,
    ];
    let mut one: f32 = 0.0;
    let mut ireg: i32 = 0;
    let mut big_val_cnt: i32 = (*gr_info).big_values as (i32);
    let mut sfb: &[u8] = (*gr_info).sfbtab;
    let mut bs_next_ptr: *const u8 = (*bs).buf.as_ptr().offset(((*bs).pos / 8) as isize);
    let mut bs_cache: u32 = (*bs_next_ptr.offset(0) as (u32))
        .wrapping_mul(256)
        .wrapping_add(*bs_next_ptr.offset(1) as (u32))
        .wrapping_mul(256)
        .wrapping_add(*bs_next_ptr.offset(2) as (u32))
        .wrapping_mul(256)
        .wrapping_add(*bs_next_ptr.offset(3) as (u32))
        << ((*bs).pos & 7);
    let mut pairs_to_decode: i32;
    let mut np: i32;
    let mut bs_sh: i32 = ((*bs).pos & 7) - 8;
    bs_next_ptr = bs_next_ptr.offset(4);
    loop {
        if !(big_val_cnt > 0) {
            break;
        }
        let tab_num: i32 = (*gr_info).table_select[ireg as usize] as (i32);
        let mut sfb_cnt: i32 =
            (*gr_info).region_count[{
                                        let _old = ireg;
                                        ireg = ireg + 1;
                                        _old
                                    } as usize] as (i32);
        let codebook: *const i16 = TABS.as_ptr().offset(TABINDEX[tab_num as usize] as isize);
        let linbits: i32 = G_LINBITS[tab_num as usize] as (i32);
        loop {
            np = {
                let _old = sfb;
                // sfb = sfb.offset(1);
                increment_by(&mut sfb, 1);
                _old[0]
            } as (i32) / 2;
            pairs_to_decode = if big_val_cnt > np { np } else { big_val_cnt };
            one = *{
                let _old = scf;
                scf = scf.offset(1);
                _old
            };
            loop {
                let mut j: i32;
                let mut w: i32 = 5;
                let mut leaf: i32 = *codebook.offset((bs_cache >> 32 - w) as isize) as (i32);
                loop {
                    if !(leaf < 0) {
                        break;
                    }
                    bs_cache = bs_cache << w;
                    bs_sh = bs_sh + w;
                    w = leaf & 7;
                    // TODO: Check that this shouldn't be `wrapping_shr(32) - w, though that doesn't seem sensible.
                    leaf = *codebook.offset(
                        (bs_cache.wrapping_shr((32 - w) as u32)).wrapping_sub((leaf >> 3) as (u32))
                            as isize,
                    ) as (i32);
                }
                // bs_cache = bs_cache << (leaf >> 8);
                bs_cache = bs_cache.wrapping_shl((leaf >> 8) as u32);
                bs_sh = bs_sh + (leaf >> 8);
                j = 0;
                loop {
                    if !(j < 2) {
                        break;
                    }
                    let mut lsb: i32 = leaf & 0xfi32;
                    if lsb == 15 && (linbits != 0) {
                        lsb = (lsb as (u32)).wrapping_add(bs_cache >> 32 - linbits) as (i32);
                        bs_cache = bs_cache << linbits;
                        bs_sh = bs_sh + linbits;
                        loop {
                            if !(bs_sh >= 0) {
                                break;
                            }
                            bs_cache = bs_cache | *{
                                let _old = bs_next_ptr;
                                bs_next_ptr = bs_next_ptr.offset(1);
                                _old
                            } as (u32) << bs_sh;
                            bs_sh = bs_sh - 8;
                        }
                        *dst = one
                            * l3_pow_43(lsb)
                            * if bs_cache as (i32) < 0 { -1 } else { 1 } as f32;
                    } else {
                        *dst = GPOW43[((16 + lsb) as (u32))
                                          .wrapping_sub(16u32.wrapping_mul(bs_cache >> 31))
                                          as usize] * one;
                    }
                    bs_cache = bs_cache << if lsb != 0 { 1 } else { 0 };
                    bs_sh = bs_sh + if lsb != 0 { 1 } else { 0 };
                    j = j + 1;
                    dst = dst.offset(1);
                    leaf = leaf >> 4;
                }
                loop {
                    if !(bs_sh >= 0) {
                        break;
                    }
                    bs_cache = (bs_cache | *{
                        let _old = bs_next_ptr;
                        bs_next_ptr = bs_next_ptr.offset(1);
                        _old
                    } as (u32))
                        .wrapping_shl(bs_sh as u32);
                    bs_sh = bs_sh - 8;
                }
                if {
                    pairs_to_decode = pairs_to_decode - 1;
                    pairs_to_decode
                } == 0
                {
                    break;
                }
            }
            if !({
                big_val_cnt = big_val_cnt - np;
                big_val_cnt
            } > 0 && ({
                sfb_cnt = sfb_cnt - 1;
                sfb_cnt
            } >= 0))
            {
                break;
            }
        }
    }
    np = 1 - big_val_cnt;
    loop {
        let codebook_count1: *const u8 = if (*gr_info).count1_table != 0 {
            TAB33.as_ptr()
        } else {
            TAB32.as_ptr()
        };
        let mut leaf: i32 = *codebook_count1.offset((bs_cache >> 32 - 4) as isize) as (i32);
        if leaf & 8 == 0 {
            leaf = *codebook_count1.offset(
                ((leaf >> 3) as (u32)).wrapping_add(bs_cache << 4 >> 32 - (leaf & 3)) as isize,
            ) as (i32);
        }
        bs_cache = bs_cache << (leaf & 7);
        bs_sh = bs_sh + (leaf & 7);
        if (bs_next_ptr as isize).wrapping_sub((*bs).buf.as_ptr() as isize)
            / ::std::mem::size_of::<u8>() as isize * 8 - 24 + bs_sh as isize
            > layer3gr_limit as isize
        {
            break;
        }
        if {
            np = np - 1;
            np
        } == 0
        {
            np = {
                let _old = sfb[0];
                increment_by(&mut sfb, 1);
                _old
            } as (i32) / 2;
            if np == 0 {
                break;
            }
            one = *{
                let _old = scf;
                scf = scf.offset(1);
                _old
            };
        }
        if leaf & 128 >> 0 != 0 {
            *dst.offset(0) = if bs_cache as (i32) < 0 { -one } else { one };
            bs_cache = bs_cache << 1;
            bs_sh = bs_sh + 1;
        }
        if leaf & 128 >> 1 != 0 {
            *dst.offset(1) = if bs_cache as (i32) < 0 { -one } else { one };
            bs_cache = bs_cache << 1;
            bs_sh = bs_sh + 1;
        }
        if {
            np = np - 1;
            np
        } == 0
        {
            np = {
                let _old = sfb[0];
                increment_by(&mut sfb, 1);
                _old
            } as (i32) / 2;
            if np == 0 {
                break;
            }
            one = *{
                let _old = scf;
                scf = scf.offset(1);
                _old
            };
        }
        if leaf & 128 >> 2 != 0 {
            *dst.offset(2) = if bs_cache as (i32) < 0 { -one } else { one };
            bs_cache = bs_cache << 1;
            bs_sh = bs_sh + 1;
        }
        if leaf & 128 >> 3 != 0 {
            *dst.offset(3) = if bs_cache as (i32) < 0 { -one } else { one };
            bs_cache = bs_cache << 1;
            bs_sh = bs_sh + 1;
        }
        loop {
            if !(bs_sh >= 0) {
                break;
            }
            bs_cache = bs_cache | *{
                let _old = bs_next_ptr;
                bs_next_ptr = bs_next_ptr.offset(1);
                _old
            } as (u32) << bs_sh;
            bs_sh = bs_sh - 8;
        }
        dst = dst.offset(4);
    }
    (*bs).pos = layer3gr_limit;
}

fn l3_midside_stereo(left: &mut [f32], n: i32) {
    let mut i: usize = 0;
    println!("LEN: {}", left.len());
    // let right = &mut left[576..];
    let (left, right) = left.split_at_mut(576);
    loop {
        if !(i < (n as usize)) {
            break;
        }
        let a: f32 = left[i];
        let b: f32 = right[i];
        left[i] = a + b;
        right[i] = a - b;
        i = i + 1;
    }
}

/// This is sometimes passed a `float[2][576]` and is sometimes,
/// it seems, passed a float array of length >= 1152.  We now
/// have two different functions for the different cases of this.
/// TODO: Can we get rid of the previous version of this?
fn l3_midside_stereo_b(left: &mut [f32], right: &mut [f32], n: i32) {
    let mut i: usize = 0;
    loop {
        if !(i < (n as usize)) {
            break;
        }
        let a: f32 = left[i];
        let b: f32 = right[i];
        left[i] = a + b;
        right[i] = a - b;
        i = i + 1;
    }
}

fn l3_stereo_top_band(mut right: &[f32], sfb: &[u8], nbands: i32, max_band: &mut [i32]) {
    let mut current_block;
    let mut i: usize;
    let mut k: usize;
    // TODO: Clean up horrible increment operations.
    max_band[0] = {
        let _rhs = {
            let _rhs = -1;
            let _lhs = &mut max_band[2];
            *_lhs = _rhs;
            *_lhs
        };
        let _lhs = &mut max_band[1];
        *_lhs = _rhs;
        *_lhs
    };
    i = 0;
    loop {
        if !(i < nbands as usize) {
            break;
        }
        k = 0;
        loop {
            if !(k < sfb[i] as usize) {
                current_block = 8;
                break;
            }
            if right[k] != 0.0 || right[k + 1] != 0.0 {
                current_block = 7;
                break;
            }
            k = k + 2;
        }
        if current_block == 7 {
            max_band[(i % 3)] = i as i32;
        }
        increment_by(&mut right, sfb[i] as usize);

        i = i + 1;
    }
}

fn l3_intensity_stereo_band(left: &mut [f32], n: i32, kl: f32, kr: f32) {
    let mut i: usize;
    i = 0;
    loop {
        if !(i < (n as usize)) {
            break;
        }
        left[i + 576] = left[i] * kr;
        left[i] = left[i] * kl;
        i = i + 1;
    }
}

fn l3_stereo_process(
    mut left: &mut [f32],
    ist_pos: &[u8],
    sfb: &[u8],
    hdr: &[u8],
    max_band: &mut [i32],
    mpeg2_sh: i32,
) {
    static L_PAN: [f32; 14] = [
        0.0, 1.0, 0.21132487, 0.78867513, 0.36602540, 0.63397460, 0.5, 0.5, 0.63397460, 0.36602540,
        0.78867513, 0.21132487, 1.0, 0.0,
    ];
    let mut i: usize;
    let max_pos: u32 = (if hdr[1] as (i32) & 0x8 != 0 { 7 } else { 64 }) as (u32);
    i = 0;
    loop {
        if sfb[i] == 0 {
            break;
        }
        let ipos: u32 = ist_pos[i] as u32;
        if i as (i32) > max_band[i.wrapping_rem(3)] && (ipos < max_pos) {
            let mut kl: f32;
            let mut kr: f32;
            let s: f32 = if hdr[3] as (i32) & 0x20 != 0 {
                1.41421356
            } else {
                1.0
            };
            if hdr[1] as (i32) & 0x8 != 0 {
                kl = L_PAN[2u32.wrapping_mul(ipos) as usize];
                kr = L_PAN[2u32.wrapping_mul(ipos).wrapping_add(1u32) as usize];
            } else {
                kl = 1.0;
                kr = l3_ldexp_q2(1.0, (ipos.wrapping_add(1) >> 1 << mpeg2_sh) as (i32));
                if ipos & 1 != 0 {
                    kl = kr;
                    kr = 1.0;
                }
            }
            l3_intensity_stereo_band(left, sfb[i] as (i32), kl * s, kr * s);
        } else if hdr[3] as (i32) & 0x20 != 0 {
            l3_midside_stereo(left, sfb[i] as i32);
        }
        increment_by_mut(&mut left, sfb[i] as usize);
        i = i.wrapping_add(1);
    }
}

fn l3_intensity_stereo(left: &mut [f32], ist_pos: &mut [u8], gr: &[L3GrInfo], hdr: &[u8]) {
    let mut max_band: [i32; 3] = [0; 3];
    let n_sfb: i32 = gr[0].n_long_sfb as (i32) + gr[0].n_short_sfb as (i32);
    let mut i: i32;
    let max_blocks: i32 = if gr[0].n_short_sfb != 0 { 3 } else { 1 };
    l3_stereo_top_band(&left[576..], gr[0].sfbtab, n_sfb, &mut max_band);
    if gr[0].n_long_sfb != 0 {
        // TODO: This can be drastically cleaned up,
        // see https://github.com/lieff/minimp3/blob/master/minimp3.h#L926
        max_band[0] = {
            let _rhs = {
                let _rhs = if if max_band[0] < max_band[1] {
                    max_band[1]
                } else {
                    max_band[0]
                } < max_band[2]
                {
                    max_band[2]
                } else if max_band[0] < max_band[1] {
                    max_band[1]
                } else {
                    max_band[0]
                };
                let _lhs = &mut max_band[2];
                *_lhs = _rhs;
                *_lhs
            };
            let _lhs = &mut max_band[1];
            *_lhs = _rhs;
            *_lhs
        };
    }
    i = 0;
    loop {
        if !(i < max_blocks) {
            break;
        }
        let default_pos: i32 = if hdr[1] as (i32) & 0x8 != 0 { 3 } else { 0 };
        let itop: i32 = n_sfb - max_blocks + i;
        let prev: i32 = itop - max_blocks;
        ist_pos[itop as usize] = if max_band[i as usize] >= prev {
            default_pos as u8
        } else {
            ist_pos[prev as usize]
        };
        i = i + 1;
    }
    l3_stereo_process(
        left,
        ist_pos,
        gr[0].sfbtab,
        hdr,
        &mut max_band,
        gr[1].scalefac_compress as (i32) & 1,
    );
}

fn l3_reorder(mut grbuf: &mut [f32], scratch: &mut [f32], mut sfb: &[u8]) {
    // TODO: This is a horrible C-ish mess of pointers that Rust profoundly
    // dislikes, and so has been rewritten.  Needs verification and testing
    // against the original.
    let mut src = 0;
    let mut dst = 0;
    while sfb[0] != 0 {
        let len = sfb[0] as usize;
        println!(
            "LEN: {}, grbuf: {}, scratch: {}",
            len,
            grbuf.len(),
            scratch.len()
        );
        for _ in 0..len {
            scratch[dst] = grbuf[src + (0 * len)];
            dst += 1;
            scratch[dst] = grbuf[src + (1 * len)];
            dst += 1;
            scratch[dst] = grbuf[src + (2 * len)];
            dst += 1;
            src += 1;
        }
        increment_by(&mut sfb, 3);
        increment_by_mut(&mut grbuf, 2 * len as usize);
    }
    let l = scratch.len();
    grbuf[..l].copy_from_slice(scratch);

    /*
    let mut i: usize;
    let mut len: usize;
    {
        let mut src = &&grbuf;
        let mut dst = &*scratch;
        loop {
            if !(0 != {
                len = sfb[0] as usize;
                len
            }) {
                break;
            }
            i = 0;
            loop {
                if !(i < len) {
                    break;
                }
                // TODO: Ugh, postincrement operators.
                // Double-check all this crap.
                // https://github.com/lieff/minimp3/blob/master/minimp3.h#L938
                // *{
                //     let _old = dst;
                //     dst = dst.offset(1);
                //     _old
                // } = *src.offset((0 * len) as isize);
                dst[0] = src[0 * len];
                increment_by_mut(&mut dst, 1);
                // *{
                //     let _old = dst;
                //     dst = dst.offset(1);
                //     _old
                // } = *src.offset((1 * len) as isize);
                dst[1] = src[1 * len];
                increment_by_mut(&mut dst, 1);
                // *{
                //     let _old = dst;
                //     dst = dst.offset(1);
                //     _old
                // } = *src.offset((2 * len) as isize);

                // src = src.offset(1);
                dst[2] = src[2 * len];
                increment_by_mut(&mut dst, 1);
                increment_by_mut(&mut src, 1);
                i = i + 1;
            }
            // sfb = sfb.offset(3);
            increment_by(&mut sfb, 3);
            // src = src.offset((2 * len) as isize);
            increment_by_mut(&mut src, 2 * len);
        }
    }
    // All this for
    // memcpy(grbuf, scratch, (dst - scratch)*sizeof(float));
    // But the implicit fucking assumption here is that dst and
    // scratch are pointers into the same array.
    // Yikes.
    // memcpy(
    //     grbuf as (*mut ::std::os::raw::c_void),
    //     scratch as (*const ::std::os::raw::c_void),
    //     (((dst as isize).wrapping_sub(scratch as isize) / ::std::mem::size_of::<f32>() as isize)
    //         as usize)
    //         .wrapping_mul(::std::mem::size_of::<f32>()),
    // );
    // let idx = (((dst as isize).wrapping_sub(scratch as isize)
    //     / ::std::mem::size_of::<f32>() as isize) as usize)
    //     .wrapping_mul(::std::mem::size_of::<f32>());
    let len = scratch.len();
    grbuf[..len].copy_from_slice(scratch);
    */
}

fn l3_antialias(mut grbuf: &mut [f32], mut nbands: i32) {
    static G_AA: [[f32; 8]; 2] = [
        [
            0.85749293, 0.88174200, 0.94962865, 0.98331459, 0.99551782, 0.99916056, 0.99989920,
            0.99999316,
        ],
        [
            0.51449576, 0.47173197, 0.31337745, 0.18191320, 0.09457419, 0.04096558, 0.01419856,
            0.00369997,
        ],
    ];
    loop {
        if !(nbands > 0) {
            break;
        }
        let mut i: usize = 0;
        loop {
            if !(i < 8) {
                break;
            }
            let u: f32 = grbuf[18 + i];
            let d: f32 = grbuf[17 - i];
            grbuf[18 + i] = u * G_AA[0][i] - d * G_AA[1][i];
            grbuf[17 - i] = u * G_AA[1][i] + d * G_AA[0][i];
            i = i + 1;
        }
        nbands = nbands - 1;
        increment_by_mut(&mut grbuf, 18);
    }
}

/// Y is apparently an [f32;9] ?
fn l3_dct3_9(y: &mut [f32; 9]) {
    let mut s0: f32;
    let mut s1: f32;
    let mut s2: f32;
    let mut s3: f32;
    let mut s4: f32;
    let mut s5: f32;
    let mut s6: f32;
    let mut s7: f32;
    let mut s8: f32;
    let mut t0: f32;
    let mut t2: f32;
    let mut t4: f32;
    s0 = y[0];
    s2 = y[2];
    s4 = y[4];
    s6 = y[6];
    s8 = y[8];
    t0 = s0 + s6 * 0.5;
    s0 = s0 - s6;
    t4 = (s4 + s2) * 0.93969262;
    t2 = (s8 + s2) * 0.76604444;
    s6 = (s4 - s8) * 0.17364818;
    s4 = s4 + (s8 - s2);
    s2 = s0 - s4 * 0.5;
    y[4] = s4 + s0;
    s8 = t0 - t2 + s6;
    s0 = t0 - t4 + t2;
    s4 = t0 + t4 - s6;
    s1 = y[1];
    s3 = y[3];
    s5 = y[5];
    s7 = y[7];
    s3 = s3 * 0.86602540;
    t0 = (s5 + s1) * 0.98480775;
    t4 = (s5 - s7) * 0.34202014;
    t2 = (s1 + s7) * 0.64278761;
    s1 = (s1 - s5 - s7) * 0.86602540;
    s5 = t0 - s3 - t2;
    s7 = t4 - s3 - t0;
    s3 = t4 + s3 - t2;
    y[0] = s4 - s7;
    y[1] = s2 + s1;
    y[2] = s0 - s3;
    y[3] = s8 + s5;
    y[5] = s8 - s5;
    y[6] = s0 + s3;
    y[7] = s2 - s1;
    y[8] = s4 + s7;
}

fn l3_imdct36(mut grbuf: &mut [f32], mut overlap: &mut [f32], window: &[f32], nbands: i32) {
    let mut i: usize;
    let mut j: i32;
    static G_TWID9: [f32; 18] = [
        0.73727734, 0.79335334, 0.84339145, 0.88701083, 0.92387953, 0.95371695, 0.97629601,
        0.99144486, 0.99904822, 0.67559021, 0.60876143, 0.53729961, 0.46174861, 0.38268343,
        0.30070580, 0.21643961, 0.13052619, 0.04361938,
    ];
    j = 0;
    loop {
        if !(j < nbands) {
            break;
        }
        let mut co: [f32; 9] = [0.0; 9];
        let mut si: [f32; 9] = [0.0; 9];
        co[0] = -grbuf[0];
        si[0] = grbuf[17];
        i = 0;
        loop {
            if !(i < 4) {
                break;
            }
            si[(8 - 2 * i)] = grbuf[(4 * i + 1)] - grbuf[(4 * i + 2)];
            co[(1 + 2 * i)] = grbuf[(4 * i + 1)] + grbuf[(4 * i + 2)];
            si[(7 - 2 * i)] = grbuf[(4 * i + 4)] - grbuf[(4 * i + 3)];
            co[(2 + 2 * i)] = -(grbuf[(4 * i + 3)] + grbuf[(4 * i + 4)]);
            i = i + 1;
        }
        l3_dct3_9(&mut co);
        l3_dct3_9(&mut si);
        si[1] = -si[1];
        si[3] = -si[3];
        si[5] = -si[5];
        si[7] = -si[7];
        i = 0;
        loop {
            if !(i < 9) {
                break;
            }
            let ovl: f32 = overlap[i];
            let sum: f32 = co[i] * G_TWID9[(9 + i)] + si[i] * G_TWID9[(0 + i)];
            overlap[i] = co[i] * G_TWID9[(0 + i)] - si[i] * G_TWID9[(9 + i)];
            grbuf[i] = ovl * window[(0 + i)] - sum * window[(9 + i)];
            grbuf[(17 - i)] = ovl * window[(9 + i)] + sum * window[(0 + i)];
            i = i + 1;
        }
        j = j + 1;
        increment_by_mut(&mut grbuf, 18);
        increment_by_mut(&mut overlap, 9);
    }
}

fn l3_idct3(x0: f32, x1: f32, x2: f32, dst: &mut [f32; 3]) {
    let m1: f32 = x1 * 0.86602540;
    let a1: f32 = x0 - x2 * 0.5;
    dst[1] = x0 + x2;
    dst[0] = a1 + m1;
    dst[2] = a1 - m1;
}

fn l3_imdct12(x: &mut [f32], dst: &mut [f32], overlap: &mut [f32]) {
    static G_TWID3: [f32; 6] = [
        0.79335334, 0.92387953, 0.99144486, 0.60876143, 0.38268343, 0.13052619,
    ];
    let mut co: [f32; 3] = [0.0; 3];
    let mut si: [f32; 3] = [0.0; 3];
    let mut i: usize;
    l3_idct3(-x[0], x[6] + x[3], x[12] + x[9], &mut co);
    l3_idct3(x[15], x[12] - x[9], x[6] - x[3], &mut si);
    si[1] = -si[1];
    i = 0;
    loop {
        if !(i < 3) {
            break;
        }
        let ovl: f32 = overlap[i];
        let sum: f32 = co[i] * G_TWID3[(3 + i)] + si[i] * G_TWID3[(0 + i)];
        overlap[i] = co[i] * G_TWID3[(0 + i)] - si[i] * G_TWID3[(3 + i)];
        dst[i] = ovl * G_TWID3[(2 - i)] - sum * G_TWID3[(5 - i)];
        dst[5 - i] = ovl * G_TWID3[(5 - i)] + sum * G_TWID3[(2 - i)];
        i = i + 1;
    }
}

fn l3_imdct_short(mut grbuf: &mut [f32], mut overlap: &mut [f32], mut nbands: i32) {
    loop {
        if !(nbands > 0) {
            break;
        }
        let mut tmp: [f32; 18] = [0.0; 18];
        // memcpy(
        //     tmp.as_mut_ptr() as (*mut ::std::os::raw::c_void),
        //     grbuf.as_ptr() as (*const ::std::os::raw::c_void),
        //     ::std::mem::size_of::<[f32; 18]>(),
        // );
        tmp.copy_from_slice(&grbuf[..18]);
        // memcpy(
        //     grbuf.as_mut_ptr() as (*mut ::std::os::raw::c_void),
        //     overlap.as_ptr() as (*const ::std::os::raw::c_void),
        //     6_usize.wrapping_mul(::std::mem::size_of::<f32>()),
        // );
        grbuf.copy_from_slice(&overlap[..6]);
        l3_imdct12(&mut tmp[..], &mut grbuf[6..], &mut overlap[6..]);
        l3_imdct12(&mut tmp[1..], &mut grbuf[12..], &mut overlap[6..]);
        {
            let (a, b) = overlap.split_at_mut(6);
            l3_imdct12(&mut tmp[2..], a, b);
        }
        nbands = nbands - 1;
        increment_by_mut(&mut overlap, 9);
        increment_by_mut(&mut grbuf, 18);
    }
}

fn l3_imdct_gr(mut grbuf: &mut [f32], mut overlap: &mut [f32], block_type: u32, n_long_bands: u32) {
    static G_MDCT_WINDOW: [[f32; 18]; 2] = [
        [
            0.99904822, 0.99144486, 0.97629601, 0.95371695, 0.92387953, 0.88701083, 0.84339145,
            0.79335334, 0.73727734, 0.04361938, 0.13052619, 0.21643961, 0.30070580, 0.38268343,
            0.46174861, 0.53729961, 0.60876143, 0.67559021,
        ],
        [
            1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 0.99144486, 0.92387953, 0.79335334, 0.0, 0.0, 0.0, 0.0,
            0.0, 0.0, 0.13052619, 0.38268343, 0.60876143,
        ],
    ];
    if n_long_bands != 0 {
        l3_imdct36(grbuf, overlap, &G_MDCT_WINDOW[0], n_long_bands as (i32));
        increment_by_mut(&mut grbuf, 18u32.wrapping_mul(n_long_bands) as usize);
        increment_by_mut(&mut overlap, 9u32.wrapping_mul(n_long_bands) as usize);
    }
    if block_type == 2u32 {
        l3_imdct_short(grbuf, overlap, 32u32.wrapping_sub(n_long_bands) as (i32));
    } else {
        l3_imdct36(
            grbuf,
            overlap,
            &G_MDCT_WINDOW[(block_type == 3u32) as usize],
            32u32.wrapping_sub(n_long_bands) as (i32),
        );
    }
}

/// I am rather confused about why this function uses two
/// nested loops instead of one.
fn l3_change_sign(mut grbuf: &mut [f32]) {
    // let mut b: i32;
    // let mut i: usize;
    // b = 0;
    // increment_by_mut(&mut grbuf, 18);
    // loop {
    //     if !(b < 32) {
    //         break;
    //     }
    //     i = 1;
    //     loop {
    //         if !(i < 18) {
    //             break;
    //         }
    //         grbuf[i] = -grbuf[i];
    //         i = i + 2;
    //     }
    //     b = b + 2;
    //     increment_by_mut(&mut grbuf, 36);
    // }
    let mut b = 0;
    increment_by_mut(&mut grbuf, 18);
    while b < 32 {
        let mut i = 0;
        while i < 18 {
            grbuf[i] = -grbuf[i];
            i += 2;
        }
        b = b + 2;
        if grbuf.len() >= 36 {
            increment_by_mut(&mut grbuf, 36);
        }
    }
}

fn l3_decode(h: &mut Mp3Dec, s: &mut Mp3DecScratch, mut gr_info: &mut [L3GrInfo], nch: i32) {
    let mut ch: i32;
    ch = 0;
    loop {
        if !(ch < nch) {
            break;
        }
        let layer3gr_limit: i32 = (*s).bs.pos + (gr_info[ch as usize]).part_23_length as (i32);
        l3_decode_scalefactors(
            &h.header,
            &mut s.ist_pos[ch as usize],
            &mut (*s).bs,
            &gr_info[ch as usize],
            &mut s.scf,
            ch,
        );
        unsafe {
        l3_huffman(
            (*s).grbuf[ch as usize].as_mut_ptr(),
            &mut (*s).bs,
            &gr_info[ch as usize],
            (*s).scf.as_mut_ptr() as (*const f32),
            layer3gr_limit,
        );
        }
        ch = ch + 1;
    }
    if (*h).header[3] as (i32) & 0x10 != 0 {
        l3_intensity_stereo(&mut s.grbuf[0], &mut s.ist_pos[1], gr_info, &h.header);
    } else if (*h).header[3] as (i32) & 0xe0 == 0x60 {
        let (left, right) = s.grbuf.split_at_mut(1);
        l3_midside_stereo_b(&mut left[0], &mut right[0], 576);
    }
    ch = 0;
    loop {
        if !(ch < nch) {
            break;
        }
        let mut aa_bands: i32 = 31;
        let n_long_bands: i32 = (if gr_info[0].mixed_block_flag != 0 {
            2
        } else {
            0
        })
            << (((*h).header[2] as (i32) >> 2 & 3)
                + (((*h).header[1] as (i32) >> 3 & 1) + ((*h).header[1] as (i32) >> 4 & 1)) * 3
                == 2) as (i32);
        if gr_info[0].n_short_sfb != 0 {
            aa_bands = n_long_bands - 1;
            l3_reorder(
                &mut s.grbuf[ch as usize][..(n_long_bands * 18) as usize],
                &mut s.syn[0][..],
                &gr_info[0].sfbtab[gr_info[0].n_long_sfb as usize..],
            );
        }
        l3_antialias(&mut s.grbuf[ch as usize][..], aa_bands);
        l3_imdct_gr(
            &mut s.grbuf[ch as usize][..],
            &mut h.mdct_overlap[ch as usize][..],
            gr_info[0].block_type as (u32),
            n_long_bands as (u32),
        );
        l3_change_sign(&mut s.grbuf[ch as usize]);
        ch = ch + 1;
        // gr_info = gr_info.offset(1);
        increment_by_mut(&mut gr_info, 1);
    }
}

fn l3_save_reservoir(h: &mut Mp3Dec, s: &mut Mp3DecScratch) {
    let mut pos: i32 = (((*s).bs.pos + 7) as (u32)).wrapping_div(8) as (i32);
    let mut remains: i32 = ((*s).bs.limit as (u32))
        .wrapping_div(8)
        .wrapping_sub(pos as (u32)) as (i32);
    // TODO: remains can probably be way simpler.
    if remains > 511 {
        pos = pos + (remains - 511);
        remains = 511;
    }
    if remains > 0 {
        // memmove(
        //     (*h).reserv_buf.as_mut_ptr() as (*mut ::std::os::raw::c_void),
        //     (*s).maindata.as_mut_ptr().offset(pos as isize) as (*const ::std::os::raw::c_void),
        //     remains as usize,
        // );
        let slice_end = (pos + remains) as usize;
        let from_slice = &s.maindata[pos as usize..slice_end];
        h.reserv_buf[..from_slice.len()].copy_from_slice(from_slice);
    }
    (*h).reserv = remains;
}

#[no_mangle]
pub unsafe fn mp3dec_decode_frame(
    dec: &mut Mp3Dec,
    mp3: &[u8],
    mut pcm: &mut [i16],
    info: &mut FrameInfo,
) -> i32 {
    let mp3_bytes = mp3.len() as i32;
    let current_block;
    let mut i: i32 = 0;
    let mut igr: i32;
    let mut frame_size: i32 = 0;
    let mut success: i32 = 1;
    let hdr: &[u8];
    let mut bs_frame: Bs = Bs::new(&[], 0);
    let mut scratch: Mp3DecScratch = Mp3DecScratch {
        bs: bs_frame,
        maindata: [0; 2815],
        gr_info: ::std::mem::zeroed(),
        grbuf: [[0.0; 576]; 2],
        scf: [0.0; 40],
        syn: [[0.0; 64]; 33],
        ist_pos: [[0; 39]; 2],
    };
    if mp3_bytes > 4 && ((*dec).header[0] as (i32) == 0xff) && (hdr_compare(&dec.header, mp3) != 0)
    {
        frame_size = hdr_frame_bytes(mp3, (*dec).free_format_bytes) + hdr_padding(mp3);
        if frame_size != mp3_bytes
            && (frame_size + 4 > mp3_bytes || hdr_compare(mp3, &mp3[frame_size as usize..]) == 0)
        {
            frame_size = 0;
        }
    }
    if frame_size == 0 {
        // memset(
        //     dec as (*mut ::std::os::raw::c_void),
        //     0,
        //     ::std::mem::size_of::<Mp3Dec>(),
        // );
        *dec = Mp3Dec::new();
        i = mp3d_find_frame(
            mp3,
            mp3_bytes,
            &mut (*dec).free_format_bytes as (*mut i32),
            &mut frame_size as (*mut i32),
        );
        if frame_size == 0 || i + frame_size > mp3_bytes {
            (*info).frame_bytes = i;
            return 0;
        }
    }
    hdr = &mp3[i as usize..];
    // memcpy(
    //     (*dec).header.as_mut_ptr() as (*mut ::std::os::raw::c_void),
    //     hdr.as_mut_ptr() as (*const ::std::os::raw::c_void),
    //     4,
    // );
    dec.header[0..4].copy_from_slice(&hdr[0..4]);
    (*info).frame_bytes = i + frame_size;
    (*info).channels = if hdr[3] & 0xc0 == 0xc0 { 1 } else { 2 };
    (*info).hz = hdr_sample_rate_hz(hdr) as (i32);
    (*info).layer = 4 - (hdr[1] as (i32) >> 1 & 3);
    (*info).bitrate_kbps = hdr_bitrate_kbps(hdr) as (i32);
    // This was pcm.is_null()... does this work?
    if pcm.len() == 0 {
        hdr_frame_samples(hdr) as (i32)
    } else {
        bs_frame = Bs::new(&hdr[4..], frame_size - 4);
        if hdr[1] as (i32) & 1 == 0 {
            get_bits(&mut bs_frame, 16);
        }
        if (*info).layer == 3 {
            let main_data_begin: i32 =
                l3_read_side_info(&mut bs_frame, scratch.gr_info.as_mut_ptr(), hdr);
            if main_data_begin < 0 || bs_frame.pos > bs_frame.limit {
                *dec = Mp3Dec::new();
                return 0;
            } else {
                success = l3_restore_reservoir(
                    &mut *dec,
                    &mut bs_frame,
                    &mut scratch as (*mut Mp3DecScratch),
                    main_data_begin,
                );
                if success != 0 {
                    igr = 0;
                    loop {
                        if !(igr < if hdr[1] as (i32) & 0x8 != 0 { 2 } else { 1 }) {
                            break;
                        }
                        // memset(
                        //     scratch.grbuf[0].as_mut_ptr() as (*mut ::std::os::raw::c_void),
                        //     0,
                        //     ((576 * 2) as usize).wrapping_mul(::std::mem::size_of::<f32>()),
                        // );
                        scratch.clear_grbuf();
                        let gr_offset = (igr * (*info).channels) as usize;
                        l3_decode(
                            &mut *dec,
                            // BUGGO: Defeat borrow checker
                            &mut *(&mut scratch as *mut Mp3DecScratch),
                            &mut scratch.gr_info[gr_offset..],
                            (*info).channels,
                        );
                        mp3d_synth_granule(
                            (*dec).qmf_state.as_mut_ptr(),
                            scratch.grbuf[0].as_mut_ptr(),
                            18,
                            (*info).channels,
                            pcm,
                            scratch.syn[0].as_mut_ptr(),
                        );
                        igr = igr + 1;
                        // pcm = pcm.offset((576 * (*info).channels) as isize);
                        // let pcm_lifetime_hack = unsafe {
                        //     let pcm_ptr = pcm.as_mut_ptr();
                        //     ::std::slice::from_raw_parts_mut(pcm_ptr, pcm.len())
                        // };
                        // pcm = &mut pcm_lifetime_hack[(576 * info.channels) as usize..];
                        increment_by_mut(&mut pcm, (576 * info.channels) as usize);
                    }
                }
                l3_save_reservoir(&mut *dec, &mut scratch);
            }
        } else {
            let mut sci: L12ScaleInfo = ::std::mem::uninitialized();
            l12_read_scale_info(hdr, &mut bs_frame, &mut sci);
            // memset(
            //     scratch.grbuf[0].as_mut_ptr() as (*mut ::std::os::raw::c_void),
            //     0,
            //     ((576 * 2) as usize).wrapping_mul(::std::mem::size_of::<f32>()),
            // );

            scratch.clear_grbuf();
            i = 0;
            igr = 0;
            loop {
                if !(igr < 3) {
                    current_block = 21;
                    break;
                }
                if 12 == {
                    i = i + l12_dequantize_granule(
                        scratch.grbuf[0].as_mut_ptr().offset(i as isize),
                        &mut bs_frame,
                        &mut sci,
                        (*info).layer | 1,
                    );
                    i
                } {
                    i = 0;
                    // BUGGO Gotta defeat the borrow checker here;
                    // borrowing both sci and sci.scf
                    l12_apply_scf_384(
                        &mut *(&mut sci as *mut L12ScaleInfo),
                        sci.scf.as_mut_ptr().offset(igr as isize) as (*const f32),
                        scratch.grbuf[0].as_mut_ptr(),
                    );
                    mp3d_synth_granule(
                        (*dec).qmf_state.as_mut_ptr(),
                        scratch.grbuf[0].as_mut_ptr(),
                        12,
                        (*info).channels,
                        pcm,
                        scratch.syn[0].as_mut_ptr(),
                    );
                    // memset(
                    //     scratch.grbuf[0].as_mut_ptr() as (*mut ::std::os::raw::c_void),
                    //     0,
                    //     ((576 * 2) as usize).wrapping_mul(::std::mem::size_of::<f32>()),
                    // );
                    scratch.clear_grbuf();
                    // pcm = pcm.offset((384 * (*info).channels) as isize);
                    // BUGGO: Borrow checker defeat here.
                    // let pcm_lifetime_hack = unsafe {
                    //     let pcm_ptr = pcm.as_mut_ptr();
                    //     ::std::slice::from_raw_parts_mut(pcm_ptr, pcm.len())
                    // };
                    // pcm = &mut pcm_lifetime_hack[(384 * info.channels) as usize..]
                    increment_by_mut(&mut pcm, (384 * info.channels) as usize);
                }
                if bs_frame.pos > bs_frame.limit {
                    current_block = 15;
                    break;
                }
                igr = igr + 1;
            }
            if current_block == 21 {
            } else {
                *dec = Mp3Dec::new();
                return 0;
            }
        }
        (success as (u32)).wrapping_mul(hdr_frame_samples(&dec.header)) as (i32)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    use super::*;
    #[test]
    fn test_increment_by_mut() {
        let slice: Vec<i32> = vec![1, 2, 3, 4, 5, 6];
        let mut slice_to_mongle: &mut [i32] = &mut slice.clone();
        increment_by_mut(&mut slice_to_mongle, 1);
        assert_eq!(&slice[1..], slice_to_mongle);
        increment_by_mut(&mut slice_to_mongle, 1);
        assert_eq!(&slice[2..], slice_to_mongle);
        increment_by_mut(&mut slice_to_mongle, 2);
        assert_eq!(&slice[4..], slice_to_mongle);
        increment_by_mut(&mut slice_to_mongle, 2);
        let empty: &[i32] = &[];
        assert_eq!(empty, slice_to_mongle);
        assert_eq!(&slice[6..], slice_to_mongle);
    }

    #[test]
    #[should_panic]
    fn test_increment_too_far() {
        let slice: Vec<i32> = vec![1, 2, 3, 4, 5, 6];
        let mut slice_to_mongle: &mut [i32] = &mut slice.clone();
        increment_by_mut(&mut slice_to_mongle, 99);
    }

    #[test]
    fn test_rewritten_get_bits() {
        unsafe fn get_bits_corroded(bs: &mut Bs, n: i32) -> u32 {
            let mut next: u32;
            let mut cache: u32 = 0;
            let s: u32 = (bs.pos & 7) as (u32);
            let mut shl: i32 = (n as (u32)).wrapping_add(s) as (i32);
            let mut p: *const u8 = (*bs).buf.as_ptr().offset(((*bs).pos >> 3) as isize);
            if {
                (*bs).pos = (*bs).pos + n;
                (*bs).pos
            } > (*bs).limit
            {
                0
            } else {
                next = (*{
                    let _old = p;
                    p = p.offset(1);
                    _old
                } as (i32) & 255 >> s) as (u32);
                loop {
                    if !({
                        shl = shl - 8;
                        shl
                    } > 0)
                    {
                        break;
                    }
                    cache = cache | next << shl;
                    next = *{
                        let _old = p;
                        p = p.offset(1);
                        _old
                    } as (u32);
                }
                cache | next >> -shl
            }
        }

        let data = vec![1,2,3,4,5,6,7,8,9,10];
        let bs = &mut Bs::new(&data, 0);
        // I'm going to crudely assume 0x8FFF is
        // Big Enough, I suppose.
        for i in 0..0x8FFF {
            let orig = unsafe { get_bits_corroded(bs, i as i32) };
            let rewritten = get_bits(bs, i);
            assert_eq!(orig, rewritten);
        }
    }
}
