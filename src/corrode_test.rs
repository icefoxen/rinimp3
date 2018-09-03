extern "C" {
    fn memcpy(
        __dest: *mut ::std::os::raw::c_void,
        __src: *const ::std::os::raw::c_void,
        __n: usize,
    ) -> *mut ::std::os::raw::c_void;
    fn memmove(
        __dest: *mut ::std::os::raw::c_void,
        __src: *const ::std::os::raw::c_void,
        __n: usize,
    ) -> *mut ::std::os::raw::c_void;
    fn memset(
        __s: *mut ::std::os::raw::c_void,
        __c: i32,
        __n: usize,
    ) -> *mut ::std::os::raw::c_void;
}

static mut g_pow43: [f32; 145] = [
    0i32 as (f32),
    -1i32 as (f32),
    -2.519842f32,
    -4.326749f32,
    -6.349604f32,
    -8.549880f32,
    -10.902724f32,
    -13.390518f32,
    -16.000000f32,
    -18.720754f32,
    -21.544347f32,
    -24.463781f32,
    -27.473142f32,
    -30.567351f32,
    -33.741992f32,
    -36.993181f32,
    0i32 as (f32),
    1i32 as (f32),
    2.519842f32,
    4.326749f32,
    6.349604f32,
    8.549880f32,
    10.902724f32,
    13.390518f32,
    16.000000f32,
    18.720754f32,
    21.544347f32,
    24.463781f32,
    27.473142f32,
    30.567351f32,
    33.741992f32,
    36.993181f32,
    40.317474f32,
    43.711787f32,
    47.173345f32,
    50.699631f32,
    54.288352f32,
    57.937408f32,
    61.644865f32,
    65.408941f32,
    69.227979f32,
    73.100443f32,
    77.024898f32,
    81.000000f32,
    85.024491f32,
    89.097188f32,
    93.216975f32,
    97.382800f32,
    101.593667f32,
    105.848633f32,
    110.146801f32,
    114.487321f32,
    118.869381f32,
    123.292209f32,
    127.755065f32,
    132.257246f32,
    136.798076f32,
    141.376907f32,
    145.993119f32,
    150.646117f32,
    155.335327f32,
    160.060199f32,
    164.820202f32,
    169.614826f32,
    174.443577f32,
    179.305980f32,
    184.201575f32,
    189.129918f32,
    194.090580f32,
    199.083145f32,
    204.107210f32,
    209.162385f32,
    214.248292f32,
    219.364564f32,
    224.510845f32,
    229.686789f32,
    234.892058f32,
    240.126328f32,
    245.389280f32,
    250.680604f32,
    256.000000f32,
    261.347174f32,
    266.721841f32,
    272.123723f32,
    277.552547f32,
    283.008049f32,
    288.489971f32,
    293.998060f32,
    299.532071f32,
    305.091761f32,
    310.676898f32,
    316.287249f32,
    321.922592f32,
    327.582707f32,
    333.267377f32,
    338.976394f32,
    344.709550f32,
    350.466646f32,
    356.247482f32,
    362.051866f32,
    367.879608f32,
    373.730522f32,
    379.604427f32,
    385.501143f32,
    391.420496f32,
    397.362314f32,
    403.326427f32,
    409.312672f32,
    415.320884f32,
    421.350905f32,
    427.402579f32,
    433.475750f32,
    439.570269f32,
    445.685987f32,
    451.822757f32,
    457.980436f32,
    464.158883f32,
    470.357960f32,
    476.577530f32,
    482.817459f32,
    489.077615f32,
    495.357868f32,
    501.658090f32,
    507.978156f32,
    514.317941f32,
    520.677324f32,
    527.056184f32,
    533.454404f32,
    539.871867f32,
    546.308458f32,
    552.764065f32,
    559.238575f32,
    565.731879f32,
    572.243870f32,
    578.774440f32,
    585.323483f32,
    591.890898f32,
    598.476581f32,
    605.080431f32,
    611.702349f32,
    618.342238f32,
    625.000000f32,
    631.675540f32,
    638.368763f32,
    645.079578f32,
];

#[derive(Copy)]
#[repr(C)]
pub struct Struct1 {
    pub mdct_overlap: [[f32; 288]; 2],
    pub qmf_state: [f32; 960],
    pub reserv: i32,
    pub free_format_bytes: i32,
    pub header: [u8; 4],
    pub reserv_buf: [u8; 511],
}

impl Clone for Struct1 {
    fn clone(&self) -> Self {
        *self
    }
}

#[no_mangle]
pub unsafe extern "C" fn mp3dec_init(mut dec: *mut Struct1) {
    (*dec).header[0usize] = 0u8;
}

#[derive(Copy)]
#[repr(C)]
pub struct Struct2 {
    pub frame_bytes: i32,
    pub channels: i32,
    pub hz: i32,
    pub layer: i32,
    pub bitrate_kbps: i32,
}

impl Clone for Struct2 {
    fn clone(&self) -> Self {
        *self
    }
}

#[derive(Copy)]
#[repr(C)]
pub struct Struct3 {
    pub buf: *const u8,
    pub pos: i32,
    pub limit: i32,
}

impl Clone for Struct3 {
    fn clone(&self) -> Self {
        *self
    }
}

#[derive(Copy)]
#[repr(C)]
pub struct Struct5 {
    pub sfbtab: *const u8,
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

impl Clone for Struct5 {
    fn clone(&self) -> Self {
        *self
    }
}

#[derive(Copy)]
#[repr(C)]
pub struct Struct4 {
    pub bs: Struct3,
    pub maindata: [u8; 2815],
    pub gr_info: [Struct5; 4],
    pub grbuf: [[f32; 576]; 2],
    pub scf: [f32; 40],
    pub syn: [[f32; 64]; 33],
    pub ist_pos: [[u8; 39]; 2],
}

impl Clone for Struct4 {
    fn clone(&self) -> Self {
        *self
    }
}

unsafe extern "C" fn hdr_valid(mut h: *const u8) -> i32 {
    (*h.offset(0isize) as (i32) == 0xffi32
        && (*h.offset(1isize) as (i32) & 0xf0i32 == 0xf0i32
            || *h.offset(1isize) as (i32) & 0xfei32 == 0xe2i32)
        && (*h.offset(1isize) as (i32) >> 1i32 & 3i32 != 0i32)
        && (*h.offset(2isize) as (i32) >> 4i32 != 15i32)
        && (*h.offset(2isize) as (i32) >> 2i32 & 3i32 != 3i32)) as (i32)
}

unsafe extern "C" fn hdr_compare(mut h1: *const u8, mut h2: *const u8) -> i32 {
    (hdr_valid(h2) != 0
        && ((*h1.offset(1isize) as (i32) ^ *h2.offset(1isize) as (i32)) & 0xfei32 == 0i32)
        && ((*h1.offset(2isize) as (i32) ^ *h2.offset(2isize) as (i32)) & 0xci32 == 0i32)
        && ((*h1.offset(2isize) as (i32) & 0xf0i32 == 0i32) as (i32)
            ^ (*h2.offset(2isize) as (i32) & 0xf0i32 == 0i32) as (i32) == 0)) as (i32)
}

unsafe extern "C" fn hdr_frame_samples(mut h: *const u8) -> u32 {
    (if *h.offset(1isize) as (i32) & 6i32 == 6i32 {
        384i32
    } else {
        1152i32 >> (*h.offset(1isize) as (i32) & 14i32 == 2i32) as (i32)
    }) as (u32)
}

pub unsafe extern "C" fn hdr_bitrate_kbps(mut h: *const u8) -> u32 {
    static mut halfrate: [[[u8; 15]; 3]; 2] = [
        [
            [
                0u8, 4u8, 8u8, 12u8, 16u8, 20u8, 24u8, 28u8, 32u8, 40u8, 48u8, 56u8, 64u8, 72u8,
                80u8,
            ],
            [
                0u8, 4u8, 8u8, 12u8, 16u8, 20u8, 24u8, 28u8, 32u8, 40u8, 48u8, 56u8, 64u8, 72u8,
                80u8,
            ],
            [
                0u8, 16u8, 24u8, 28u8, 32u8, 40u8, 48u8, 56u8, 64u8, 72u8, 80u8, 88u8, 96u8, 112u8,
                128u8,
            ],
        ],
        [
            [
                0u8, 16u8, 20u8, 24u8, 28u8, 32u8, 40u8, 48u8, 56u8, 64u8, 80u8, 96u8, 112u8,
                128u8, 160u8,
            ],
            [
                0u8, 16u8, 24u8, 28u8, 32u8, 40u8, 48u8, 56u8, 64u8, 80u8, 96u8, 112u8, 128u8,
                160u8, 192u8,
            ],
            [
                0u8, 16u8, 32u8, 48u8, 64u8, 80u8, 96u8, 112u8, 128u8, 144u8, 160u8, 176u8, 192u8,
                208u8, 224u8,
            ],
        ],
    ];
    (2i32 * halfrate[!(*h.offset(1isize) as (i32) & 0x8i32 == 0) as (usize)]
        [((*h.offset(1isize) as (i32) >> 1i32 & 3i32) - 1i32) as (usize)]
        [(*h.offset(2isize) as (i32) >> 4i32) as (usize)] as (i32)) as (u32)
}

pub unsafe extern "C" fn hdr_sample_rate_hz(mut h: *const u8) -> u32 {
    static mut g_hz: [u32; 3] = [44100u32, 48000u32, 32000u32];
    g_hz[(*h.offset(2isize) as (i32) >> 2i32 & 3i32) as (usize)]
        >> (*h.offset(1isize) as (i32) & 0x8i32 == 0) as (i32)
        >> (*h.offset(1isize) as (i32) & 0x10i32 == 0) as (i32)
}

pub unsafe extern "C" fn hdr_frame_bytes(mut h: *const u8, mut free_format_size: i32) -> i32 {
    let mut frame_bytes: i32 = hdr_frame_samples(h)
        .wrapping_mul(hdr_bitrate_kbps(h))
        .wrapping_mul(125u32)
        .wrapping_div(hdr_sample_rate_hz(h)) as (i32);
    if *h.offset(1isize) as (i32) & 6i32 == 6i32 {
        frame_bytes = frame_bytes & !3i32;
    }
    if frame_bytes != 0 {
        frame_bytes
    } else {
        free_format_size
    }
}

pub unsafe extern "C" fn hdr_padding(mut h: *const u8) -> i32 {
    if *h.offset(2isize) as (i32) & 0x2i32 != 0 {
        (if *h.offset(1isize) as (i32) & 6i32 == 6i32 {
            4i32
        } else {
            1i32
        })
    } else {
        0i32
    }
}

unsafe extern "C" fn mp3d_match_frame(
    mut hdr: *const u8,
    mut mp3_bytes: i32,
    mut frame_bytes: i32,
) -> i32 {
    let mut _currentBlock;
    let mut i: i32;
    let mut nmatch: i32;
    i = 0i32;
    nmatch = 0i32;
    'loop1: loop {
        if !(nmatch < 10i32) {
            _currentBlock = 2;
            break;
        }
        i = i + (hdr_frame_bytes(hdr.offset(i as (isize)), frame_bytes)
            + hdr_padding(hdr.offset(i as (isize))));
        if i + 4i32 > mp3_bytes {
            _currentBlock = 7;
            break;
        }
        if hdr_compare(hdr, hdr.offset(i as (isize))) == 0 {
            _currentBlock = 6;
            break;
        }
        nmatch = nmatch + 1;
    }
    if _currentBlock == 2 {
        1i32
    } else if _currentBlock == 6 {
        0i32
    } else {
        (nmatch > 0i32) as (i32)
    }
}

pub unsafe extern "C" fn mp3d_find_frame(
    mut mp3: *const u8,
    mut mp3_bytes: i32,
    mut free_format_bytes: *mut i32,
    mut ptr_frame_bytes: *mut i32,
) -> i32 {
    let mut _currentBlock;
    let mut i: i32;
    let mut k: i32;
    i = 0i32;
    let mut frame_bytes: i32 = hdr_frame_bytes(mp3, *free_format_bytes);
    let mut frame_and_padding: i32 = frame_bytes + hdr_padding(mp3);
    'loop1: loop {
        if !(i < mp3_bytes - 4i32) {
            _currentBlock = 2;
            break;
        }
        if hdr_valid(mp3) != 0 {
            k = 4i32;
            'loop5: loop {
                if !(frame_bytes == 0 && (k < 2304i32) && (i + 2i32 * k < mp3_bytes - 4i32)) {
                    break;
                }
                if hdr_compare(mp3, mp3.offset(k as (isize))) != 0 {
                    let mut fb: i32 = k - hdr_padding(mp3);
                    let mut nextfb: i32 = fb + hdr_padding(mp3.offset(k as (isize)));
                    if !(i + k + nextfb + 4i32 > mp3_bytes
                        || hdr_compare(mp3, mp3.offset(k as (isize)).offset(nextfb as (isize)))
                            == 0)
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
                _currentBlock = 9;
                break;
            }
            *free_format_bytes = 0i32;
        }
        i = i + 1;
        mp3 = mp3.offset(1isize);
    }
    if _currentBlock == 2 {
        *ptr_frame_bytes = 0i32;
        i
    } else {
        *ptr_frame_bytes = frame_and_padding;
        i
    }
}

unsafe extern "C" fn bs_init(mut bs: *mut Struct3, mut data: *const u8, mut bytes: i32) {
    (*bs).buf = data;
    (*bs).pos = 0i32;
    (*bs).limit = bytes * 8i32;
}

unsafe extern "C" fn get_bits(mut bs: *mut Struct3, mut n: i32) -> u32 {
    let mut next: u32;
    let mut cache: u32 = 0u32;
    let mut s: u32 = ((*bs).pos & 7i32) as (u32);
    let mut shl: i32 = (n as (u32)).wrapping_add(s) as (i32);
    let mut p: *const u8 = (*bs).buf.offset(((*bs).pos >> 3i32) as (isize));
    if {
        (*bs).pos = (*bs).pos + n;
        (*bs).pos
    } > (*bs).limit
    {
        0u32
    } else {
        next = (*{
            let _old = p;
            p = p.offset(1isize);
            _old
        } as (i32) & 255i32 >> s) as (u32);
        'loop2: loop {
            if !({
                shl = shl - 8i32;
                shl
            } > 0i32)
            {
                break;
            }
            cache = cache | next << shl;
            next = *{
                let _old = p;
                p = p.offset(1isize);
                _old
            } as (u32);
        }
        cache | next >> -shl
    }
}

#[derive(Copy)]
#[repr(C)]
pub struct Struct6 {
    pub scf: [f32; 192],
    pub total_bands: u8,
    pub stereo_bands: u8,
    pub bitalloc: [u8; 64],
    pub scfcod: [u8; 64],
}

impl Clone for Struct6 {
    fn clone(&self) -> Self {
        *self
    }
}

#[derive(Copy)]
#[repr(C)]
pub struct Struct7 {
    pub tab_offset: u8,
    pub code_tab_width: u8,
    pub band_count: u8,
}

impl Clone for Struct7 {
    fn clone(&self) -> Self {
        *self
    }
}

unsafe extern "C" fn L12_subband_alloc_table(
    mut hdr: *const u8,
    mut sci: *mut Struct6,
) -> *const Struct7 {
    let mut alloc: *const Struct7;
    let mut mode: i32 = *hdr.offset(3isize) as (i32) >> 6i32 & 3i32;
    let mut nbands: i32;
    let mut stereo_bands: i32 = if mode == 3i32 {
        0i32
    } else if mode == 1i32 {
        ((*hdr.offset(3isize) as (i32) >> 4i32 & 3i32) << 2i32) + 4i32
    } else {
        32i32
    };
    if *hdr.offset(1isize) as (i32) & 6i32 == 6i32 {
        static mut g_alloc_L1: [Struct7; 1] = [Struct7 {
            tab_offset: 76u8,
            code_tab_width: 4u8,
            band_count: 32u8,
        }];
        alloc = g_alloc_L1.as_ptr();
        nbands = 32i32;
    } else if *hdr.offset(1isize) as (i32) & 0x8i32 == 0 {
        static mut g_alloc_L2M2: [Struct7; 3] = [
            Struct7 {
                tab_offset: 60u8,
                code_tab_width: 4u8,
                band_count: 4u8,
            },
            Struct7 {
                tab_offset: 44u8,
                code_tab_width: 3u8,
                band_count: 7u8,
            },
            Struct7 {
                tab_offset: 44u8,
                code_tab_width: 2u8,
                band_count: 19u8,
            },
        ];
        alloc = g_alloc_L2M2.as_ptr();
        nbands = 30i32;
    } else {
        static mut g_alloc_L2M1: [Struct7; 4] = [
            Struct7 {
                tab_offset: 0u8,
                code_tab_width: 4u8,
                band_count: 3u8,
            },
            Struct7 {
                tab_offset: 16u8,
                code_tab_width: 4u8,
                band_count: 8u8,
            },
            Struct7 {
                tab_offset: 32u8,
                code_tab_width: 3u8,
                band_count: 12u8,
            },
            Struct7 {
                tab_offset: 40u8,
                code_tab_width: 2u8,
                band_count: 7u8,
            },
        ];
        let mut sample_rate_idx: i32 = *hdr.offset(2isize) as (i32) >> 2i32 & 3i32;
        let mut kbps: u32 = hdr_bitrate_kbps(hdr) >> (mode != 3i32) as (i32);
        if kbps == 0 {
            kbps = 192u32;
        }
        alloc = g_alloc_L2M1.as_ptr();
        nbands = 27i32;
        if kbps < 56u32 {
            static mut g_alloc_L2M1_lowrate: [Struct7; 2] = [
                Struct7 {
                    tab_offset: 44u8,
                    code_tab_width: 4u8,
                    band_count: 2u8,
                },
                Struct7 {
                    tab_offset: 44u8,
                    code_tab_width: 3u8,
                    band_count: 10u8,
                },
            ];
            alloc = g_alloc_L2M1_lowrate.as_ptr();
            nbands = if sample_rate_idx == 2i32 { 12i32 } else { 8i32 };
        } else if kbps >= 96u32 && (sample_rate_idx != 1i32) {
            nbands = 30i32;
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

unsafe extern "C" fn L12_read_scalefactors(
    mut bs: *mut Struct3,
    mut pba: *mut u8,
    mut scfcod: *mut u8,
    mut bands: i32,
    mut scf: *mut f32,
) {
    static mut g_deq_L12: [f32; 54] = [
        9.53674316e-07f32 / 3i32 as (f32),
        7.56931807e-07f32 / 3i32 as (f32),
        6.00777173e-07f32 / 3i32 as (f32),
        9.53674316e-07f32 / 7i32 as (f32),
        7.56931807e-07f32 / 7i32 as (f32),
        6.00777173e-07f32 / 7i32 as (f32),
        9.53674316e-07f32 / 15i32 as (f32),
        7.56931807e-07f32 / 15i32 as (f32),
        6.00777173e-07f32 / 15i32 as (f32),
        9.53674316e-07f32 / 31i32 as (f32),
        7.56931807e-07f32 / 31i32 as (f32),
        6.00777173e-07f32 / 31i32 as (f32),
        9.53674316e-07f32 / 63i32 as (f32),
        7.56931807e-07f32 / 63i32 as (f32),
        6.00777173e-07f32 / 63i32 as (f32),
        9.53674316e-07f32 / 127i32 as (f32),
        7.56931807e-07f32 / 127i32 as (f32),
        6.00777173e-07f32 / 127i32 as (f32),
        9.53674316e-07f32 / 255i32 as (f32),
        7.56931807e-07f32 / 255i32 as (f32),
        6.00777173e-07f32 / 255i32 as (f32),
        9.53674316e-07f32 / 511i32 as (f32),
        7.56931807e-07f32 / 511i32 as (f32),
        6.00777173e-07f32 / 511i32 as (f32),
        9.53674316e-07f32 / 1023i32 as (f32),
        7.56931807e-07f32 / 1023i32 as (f32),
        6.00777173e-07f32 / 1023i32 as (f32),
        9.53674316e-07f32 / 2047i32 as (f32),
        7.56931807e-07f32 / 2047i32 as (f32),
        6.00777173e-07f32 / 2047i32 as (f32),
        9.53674316e-07f32 / 4095i32 as (f32),
        7.56931807e-07f32 / 4095i32 as (f32),
        6.00777173e-07f32 / 4095i32 as (f32),
        9.53674316e-07f32 / 8191i32 as (f32),
        7.56931807e-07f32 / 8191i32 as (f32),
        6.00777173e-07f32 / 8191i32 as (f32),
        9.53674316e-07f32 / 16383i32 as (f32),
        7.56931807e-07f32 / 16383i32 as (f32),
        6.00777173e-07f32 / 16383i32 as (f32),
        9.53674316e-07f32 / 32767i32 as (f32),
        7.56931807e-07f32 / 32767i32 as (f32),
        6.00777173e-07f32 / 32767i32 as (f32),
        9.53674316e-07f32 / 65535i32 as (f32),
        7.56931807e-07f32 / 65535i32 as (f32),
        6.00777173e-07f32 / 65535i32 as (f32),
        9.53674316e-07f32 / 3i32 as (f32),
        7.56931807e-07f32 / 3i32 as (f32),
        6.00777173e-07f32 / 3i32 as (f32),
        9.53674316e-07f32 / 5i32 as (f32),
        7.56931807e-07f32 / 5i32 as (f32),
        6.00777173e-07f32 / 5i32 as (f32),
        9.53674316e-07f32 / 9i32 as (f32),
        7.56931807e-07f32 / 9i32 as (f32),
        6.00777173e-07f32 / 9i32 as (f32),
    ];
    let mut i: i32;
    let mut m: i32;
    i = 0i32;
    'loop1: loop {
        if !(i < bands) {
            break;
        }
        let mut s: f32 = 0i32 as (f32);
        let mut ba: i32 = *{
            let _old = pba;
            pba = pba.offset(1isize);
            _old
        } as (i32);
        let mut mask: i32 = if ba != 0 {
            4i32 + (19i32 >> *scfcod.offset(i as (isize)) as (i32) & 3i32)
        } else {
            0i32
        };
        m = 4i32;
        'loop4: loop {
            if m == 0 {
                break;
            }
            if mask & m != 0 {
                let mut b: i32 = get_bits(bs, 6i32) as (i32);
                s = g_deq_L12[(ba * 3i32 - 6i32 + b % 3i32) as (usize)]
                    * (1i32 << 21i32 >> b / 3i32) as (f32);
            }
            *{
                let _old = scf;
                scf = scf.offset(1isize);
                _old
            } = s;
            m = m >> 1i32;
        }
        i = i + 1;
    }
}

unsafe extern "C" fn L12_read_scale_info(
    mut hdr: *const u8,
    mut bs: *mut Struct3,
    mut sci: *mut Struct6,
) {
    static mut g_bitalloc_code_tab: [u8; 92] = [
        0u8, 17u8, 3u8, 4u8, 5u8, 6u8, 7u8, 8u8, 9u8, 10u8, 11u8, 12u8, 13u8, 14u8, 15u8, 16u8,
        0u8, 17u8, 18u8, 3u8, 19u8, 4u8, 5u8, 6u8, 7u8, 8u8, 9u8, 10u8, 11u8, 12u8, 13u8, 16u8,
        0u8, 17u8, 18u8, 3u8, 19u8, 4u8, 5u8, 16u8, 0u8, 17u8, 18u8, 16u8, 0u8, 17u8, 18u8, 19u8,
        4u8, 5u8, 6u8, 7u8, 8u8, 9u8, 10u8, 11u8, 12u8, 13u8, 14u8, 15u8, 0u8, 17u8, 18u8, 3u8,
        19u8, 4u8, 5u8, 6u8, 7u8, 8u8, 9u8, 10u8, 11u8, 12u8, 13u8, 14u8, 0u8, 2u8, 3u8, 4u8, 5u8,
        6u8, 7u8, 8u8, 9u8, 10u8, 11u8, 12u8, 13u8, 14u8, 15u8, 16u8,
    ];
    let mut subband_alloc: *const Struct7 = L12_subband_alloc_table(hdr, sci);
    let mut i: i32;
    let mut k: i32 = 0i32;
    let mut ba_bits: i32 = 0i32;
    let mut ba_code_tab: *const u8 = g_bitalloc_code_tab.as_ptr();
    i = 0i32;
    'loop1: loop {
        if !(i < (*sci).total_bands as (i32)) {
            break;
        }
        let mut ba: u8;
        if i == k {
            k = k + (*subband_alloc).band_count as (i32);
            ba_bits = (*subband_alloc).code_tab_width as (i32);
            ba_code_tab = g_bitalloc_code_tab
                .as_ptr()
                .offset((*subband_alloc).tab_offset as (isize));
            subband_alloc = subband_alloc.offset(1isize);
        }
        ba = *ba_code_tab.offset(get_bits(bs, ba_bits) as (isize));
        (*sci).bitalloc[(2i32 * i) as (usize)] = ba;
        if i < (*sci).stereo_bands as (i32) {
            ba = *ba_code_tab.offset(get_bits(bs, ba_bits) as (isize));
        }
        (*sci).bitalloc[(2i32 * i + 1i32) as (usize)] = if (*sci).stereo_bands != 0 {
            ba as (i32)
        } else {
            0i32
        } as (u8);
        i = i + 1;
    }
    i = 0i32;
    'loop3: loop {
        if !(i < 2i32 * (*sci).total_bands as (i32)) {
            break;
        }
        (*sci).scfcod[i as (usize)] = if (*sci).bitalloc[i as (usize)] != 0 {
            (if *hdr.offset(1isize) as (i32) & 6i32 == 6i32 {
                2u32
            } else {
                get_bits(bs, 2i32)
            })
        } else {
            6u32
        } as (u8);
        i = i + 1;
    }
    L12_read_scalefactors(
        bs,
        (*sci).bitalloc.as_mut_ptr(),
        (*sci).scfcod.as_mut_ptr(),
        (*sci).total_bands as (i32) * 2i32,
        (*sci).scf.as_mut_ptr(),
    );
    i = (*sci).stereo_bands as (i32);
    'loop5: loop {
        if !(i < (*sci).total_bands as (i32)) {
            break;
        }
        (*sci).bitalloc[(2i32 * i + 1i32) as (usize)] = 0u8;
        i = i + 1;
    }
}

unsafe extern "C" fn L12_dequantize_granule(
    mut grbuf: *mut f32,
    mut bs: *mut Struct3,
    mut sci: *mut Struct6,
    mut group_size: i32,
) -> i32 {
    let mut i: i32;
    let mut j: i32;
    let mut k: i32;
    let mut choff: i32 = 576i32;
    j = 0i32;
    'loop1: loop {
        if !(j < 4i32) {
            break;
        }
        let mut dst: *mut f32 = grbuf.offset((group_size * j) as (isize));
        i = 0i32;
        'loop4: loop {
            if !(i < 2i32 * (*sci).total_bands as (i32)) {
                break;
            }
            let mut ba: i32 = (*sci).bitalloc[i as (usize)] as (i32);
            if ba != 0i32 {
                if ba < 17i32 {
                    let mut half: i32 = (1i32 << ba - 1i32) - 1i32;
                    k = 0i32;
                    'loop13: loop {
                        if !(k < group_size) {
                            break;
                        }
                        *dst.offset(k as (isize)) = (get_bits(bs, ba) as (i32) - half) as (f32);
                        k = k + 1;
                    }
                } else {
                    let mut mod_: u32 = ((2i32 << ba - 17i32) + 1i32) as (u32);
                    let mut code: u32 = get_bits(
                        bs,
                        mod_.wrapping_add(2u32).wrapping_sub(mod_ >> 3i32) as (i32),
                    );
                    k = 0i32;
                    'loop9: loop {
                        if !(k < group_size) {
                            break;
                        }
                        *dst.offset(k as (isize)) = code
                            .wrapping_rem(mod_)
                            .wrapping_sub(mod_.wrapping_div(2u32))
                            as (i32) as (f32);
                        k = k + 1;
                        code = code.wrapping_div(mod_);
                    }
                }
            }
            dst = dst.offset(choff as (isize));
            choff = 18i32 - choff;
            i = i + 1;
        }
        j = j + 1;
    }
    group_size * 4i32
}

unsafe extern "C" fn L12_apply_scf_384(
    mut sci: *mut Struct6,
    mut scf: *const f32,
    mut dst: *mut f32,
) {
    let mut i: i32;
    let mut k: i32;
    memcpy(
        dst.offset(576isize)
            .offset(((*sci).stereo_bands as (i32) * 18i32) as (isize))
            as (*mut ::std::os::raw::c_void),
        dst.offset(((*sci).stereo_bands as (i32) * 18i32) as (isize))
            as (*const ::std::os::raw::c_void),
        ((((*sci).total_bands as (i32) - (*sci).stereo_bands as (i32)) * 18i32) as (usize))
            .wrapping_mul(::std::mem::size_of::<f32>()),
    );
    i = 0i32;
    'loop1: loop {
        if !(i < (*sci).total_bands as (i32)) {
            break;
        }
        k = 0i32;
        'loop4: loop {
            if !(k < 12i32) {
                break;
            }
            let _rhs = *scf.offset(0isize);
            let _lhs = &mut *dst.offset((k + 0i32) as (isize));
            *_lhs = *_lhs * _rhs;
            let _rhs = *scf.offset(3isize);
            let _lhs = &mut *dst.offset((k + 576i32) as (isize));
            *_lhs = *_lhs * _rhs;
            k = k + 1;
        }
        i = i + 1;
        dst = dst.offset(18isize);
        scf = scf.offset(6isize);
    }
}

unsafe extern "C" fn mp3d_DCT_II(mut grbuf: *mut f32, mut n: i32) {
    static mut g_sec: [f32; 24] = [
        10.19000816f32,
        0.50060302f32,
        0.50241929f32,
        3.40760851f32,
        0.50547093f32,
        0.52249861f32,
        2.05778098f32,
        0.51544732f32,
        0.56694406f32,
        1.48416460f32,
        0.53104258f32,
        0.64682180f32,
        1.16943991f32,
        0.55310392f32,
        0.78815460f32,
        0.97256821f32,
        0.58293498f32,
        1.06067765f32,
        0.83934963f32,
        0.62250412f32,
        1.72244716f32,
        0.74453628f32,
        0.67480832f32,
        5.10114861f32,
    ];
    let mut i: i32;
    let mut k: i32 = 0i32;
    'loop1: loop {
        if !(k < n) {
            break;
        }
        let mut t: [[f32; 8]; 4] = [[0.0; 8]; 4];
        let mut x: *mut f32;
        let mut y: *mut f32 = grbuf.offset(k as (isize));
        x = t[0usize].as_mut_ptr();
        i = 0i32;
        'loop4: loop {
            if !(i < 8i32) {
                break;
            }
            let mut x0: f32 = *y.offset((i * 18i32) as (isize));
            let mut x1: f32 = *y.offset(((15i32 - i) * 18i32) as (isize));
            let mut x2: f32 = *y.offset(((16i32 + i) * 18i32) as (isize));
            let mut x3: f32 = *y.offset(((31i32 - i) * 18i32) as (isize));
            let mut t0: f32 = x0 + x3;
            let mut t1: f32 = x1 + x2;
            let mut t2: f32 = (x1 - x2) * g_sec[(3i32 * i + 0i32) as (usize)];
            let mut t3: f32 = (x0 - x3) * g_sec[(3i32 * i + 1i32) as (usize)];
            *x.offset(0isize) = t0 + t1;
            *x.offset(8isize) = (t0 - t1) * g_sec[(3i32 * i + 2i32) as (usize)];
            *x.offset(16isize) = t3 + t2;
            *x.offset(24isize) = (t3 - t2) * g_sec[(3i32 * i + 2i32) as (usize)];
            i = i + 1;
            x = x.offset(1isize);
        }
        x = t[0usize].as_mut_ptr();
        i = 0i32;
        'loop6: loop {
            if !(i < 4i32) {
                break;
            }
            let mut x0: f32 = *x.offset(0isize);
            let mut x1: f32 = *x.offset(1isize);
            let mut x2: f32 = *x.offset(2isize);
            let mut x3: f32 = *x.offset(3isize);
            let mut x4: f32 = *x.offset(4isize);
            let mut x5: f32 = *x.offset(5isize);
            let mut x6: f32 = *x.offset(6isize);
            let mut x7: f32 = *x.offset(7isize);
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
            *x.offset(0isize) = x0 + x1;
            *x.offset(4isize) = (x0 - x1) * 0.70710677f32;
            x5 = x5 + x6;
            x6 = (x6 + x7) * 0.70710677f32;
            x7 = x7 + xt;
            x3 = (x3 + x4) * 0.70710677f32;
            x5 = x5 - x7 * 0.198912367f32;
            x7 = x7 + x5 * 0.382683432f32;
            x5 = x5 - x7 * 0.198912367f32;
            x0 = xt - x6;
            xt = xt + x6;
            *x.offset(1isize) = (xt + x7) * 0.50979561f32;
            *x.offset(2isize) = (x4 + x3) * 0.54119611f32;
            *x.offset(3isize) = (x0 - x5) * 0.60134488f32;
            *x.offset(5isize) = (x0 + x5) * 0.89997619f32;
            *x.offset(6isize) = (x4 - x3) * 1.30656302f32;
            *x.offset(7isize) = (xt - x7) * 2.56291556f32;
            i = i + 1;
            x = x.offset(8isize);
        }
        i = 0i32;
        'loop8: loop {
            if !(i < 7i32) {
                break;
            }
            *y.offset((0i32 * 18i32) as (isize)) = t[0usize][i as (usize)];
            *y.offset((1i32 * 18i32) as (isize)) = t[2usize][i as (usize)]
                + t[3usize][i as (usize)]
                + t[3usize][(i + 1i32) as (usize)];
            *y.offset((2i32 * 18i32) as (isize)) =
                t[1usize][i as (usize)] + t[1usize][(i + 1i32) as (usize)];
            *y.offset((3i32 * 18i32) as (isize)) = t[2usize][(i + 1i32) as (usize)]
                + t[3usize][i as (usize)]
                + t[3usize][(i + 1i32) as (usize)];
            i = i + 1;
            y = y.offset((4i32 * 18i32) as (isize));
        }
        *y.offset((0i32 * 18i32) as (isize)) = t[0usize][7usize];
        *y.offset((1i32 * 18i32) as (isize)) = t[2usize][7usize] + t[3usize][7usize];
        *y.offset((2i32 * 18i32) as (isize)) = t[1usize][7usize];
        *y.offset((3i32 * 18i32) as (isize)) = t[3usize][7usize];
        k = k + 1;
    }
}

unsafe extern "C" fn mp3d_scale_pcm(mut sample: f32) -> i16 {
    if sample as (f64) >= 32766.5f64 {
        32767i16
    } else if sample as (f64) <= -32767.5f64 {
        -32768i16
    } else {
        let mut s: i16 = (sample + 0.5f32) as (i16);
        s = (s as (i32) - (s as (i32) < 0i32) as (i32)) as (i16);
        s
    }
}

unsafe extern "C" fn mp3d_synth_pair(mut pcm: *mut i16, mut nch: i32, mut z: *const f32) {
    let mut a: f32;
    a = (*z.offset((14i32 * 64i32) as (isize)) - *z.offset(0isize)) * 29i32 as (f32);
    a = a + (*z.offset((1i32 * 64i32) as (isize)) + *z.offset((13i32 * 64i32) as (isize)))
        * 213i32 as (f32);
    a = a + (*z.offset((12i32 * 64i32) as (isize)) - *z.offset((2i32 * 64i32) as (isize)))
        * 459i32 as (f32);
    a = a + (*z.offset((3i32 * 64i32) as (isize)) + *z.offset((11i32 * 64i32) as (isize)))
        * 2037i32 as (f32);
    a = a + (*z.offset((10i32 * 64i32) as (isize)) - *z.offset((4i32 * 64i32) as (isize)))
        * 5153i32 as (f32);
    a = a + (*z.offset((5i32 * 64i32) as (isize)) + *z.offset((9i32 * 64i32) as (isize)))
        * 6574i32 as (f32);
    a = a + (*z.offset((8i32 * 64i32) as (isize)) - *z.offset((6i32 * 64i32) as (isize)))
        * 37489i32 as (f32);
    a = a + *z.offset((7i32 * 64i32) as (isize)) * 75038i32 as (f32);
    *pcm.offset(0isize) = mp3d_scale_pcm(a);
    z = z.offset(2isize);
    a = *z.offset((14i32 * 64i32) as (isize)) * 104i32 as (f32);
    a = a + *z.offset((12i32 * 64i32) as (isize)) * 1567i32 as (f32);
    a = a + *z.offset((10i32 * 64i32) as (isize)) * 9727i32 as (f32);
    a = a + *z.offset((8i32 * 64i32) as (isize)) * 64019i32 as (f32);
    a = a + *z.offset((6i32 * 64i32) as (isize)) * -9975i32 as (f32);
    a = a + *z.offset((4i32 * 64i32) as (isize)) * -45i32 as (f32);
    a = a + *z.offset((2i32 * 64i32) as (isize)) * 146i32 as (f32);
    a = a + *z.offset((0i32 * 64i32) as (isize)) * -5i32 as (f32);
    *pcm.offset((16i32 * nch) as (isize)) = mp3d_scale_pcm(a);
}

unsafe extern "C" fn mp3d_synth(
    mut xl: *mut f32,
    mut dstl: *mut i16,
    mut nch: i32,
    mut lins: *mut f32,
) {
    let mut i: i32;
    let mut xr: *mut f32 = xl.offset((576i32 * (nch - 1i32)) as (isize));
    let mut dstr: *mut i16 = dstl.offset((nch - 1i32) as (isize));
    static mut g_win: [f32; 240] = [
        -1i32 as (f32),
        26i32 as (f32),
        -31i32 as (f32),
        208i32 as (f32),
        218i32 as (f32),
        401i32 as (f32),
        -519i32 as (f32),
        2063i32 as (f32),
        2000i32 as (f32),
        4788i32 as (f32),
        -5517i32 as (f32),
        7134i32 as (f32),
        5959i32 as (f32),
        35640i32 as (f32),
        -39336i32 as (f32),
        74992i32 as (f32),
        -1i32 as (f32),
        24i32 as (f32),
        -35i32 as (f32),
        202i32 as (f32),
        222i32 as (f32),
        347i32 as (f32),
        -581i32 as (f32),
        2080i32 as (f32),
        1952i32 as (f32),
        4425i32 as (f32),
        -5879i32 as (f32),
        7640i32 as (f32),
        5288i32 as (f32),
        33791i32 as (f32),
        -41176i32 as (f32),
        74856i32 as (f32),
        -1i32 as (f32),
        21i32 as (f32),
        -38i32 as (f32),
        196i32 as (f32),
        225i32 as (f32),
        294i32 as (f32),
        -645i32 as (f32),
        2087i32 as (f32),
        1893i32 as (f32),
        4063i32 as (f32),
        -6237i32 as (f32),
        8092i32 as (f32),
        4561i32 as (f32),
        31947i32 as (f32),
        -43006i32 as (f32),
        74630i32 as (f32),
        -1i32 as (f32),
        19i32 as (f32),
        -41i32 as (f32),
        190i32 as (f32),
        227i32 as (f32),
        244i32 as (f32),
        -711i32 as (f32),
        2085i32 as (f32),
        1822i32 as (f32),
        3705i32 as (f32),
        -6589i32 as (f32),
        8492i32 as (f32),
        3776i32 as (f32),
        30112i32 as (f32),
        -44821i32 as (f32),
        74313i32 as (f32),
        -1i32 as (f32),
        17i32 as (f32),
        -45i32 as (f32),
        183i32 as (f32),
        228i32 as (f32),
        197i32 as (f32),
        -779i32 as (f32),
        2075i32 as (f32),
        1739i32 as (f32),
        3351i32 as (f32),
        -6935i32 as (f32),
        8840i32 as (f32),
        2935i32 as (f32),
        28289i32 as (f32),
        -46617i32 as (f32),
        73908i32 as (f32),
        -1i32 as (f32),
        16i32 as (f32),
        -49i32 as (f32),
        176i32 as (f32),
        228i32 as (f32),
        153i32 as (f32),
        -848i32 as (f32),
        2057i32 as (f32),
        1644i32 as (f32),
        3004i32 as (f32),
        -7271i32 as (f32),
        9139i32 as (f32),
        2037i32 as (f32),
        26482i32 as (f32),
        -48390i32 as (f32),
        73415i32 as (f32),
        -2i32 as (f32),
        14i32 as (f32),
        -53i32 as (f32),
        169i32 as (f32),
        227i32 as (f32),
        111i32 as (f32),
        -919i32 as (f32),
        2032i32 as (f32),
        1535i32 as (f32),
        2663i32 as (f32),
        -7597i32 as (f32),
        9389i32 as (f32),
        1082i32 as (f32),
        24694i32 as (f32),
        -50137i32 as (f32),
        72835i32 as (f32),
        -2i32 as (f32),
        13i32 as (f32),
        -58i32 as (f32),
        161i32 as (f32),
        224i32 as (f32),
        72i32 as (f32),
        -991i32 as (f32),
        2001i32 as (f32),
        1414i32 as (f32),
        2330i32 as (f32),
        -7910i32 as (f32),
        9592i32 as (f32),
        70i32 as (f32),
        22929i32 as (f32),
        -51853i32 as (f32),
        72169i32 as (f32),
        -2i32 as (f32),
        11i32 as (f32),
        -63i32 as (f32),
        154i32 as (f32),
        221i32 as (f32),
        36i32 as (f32),
        -1064i32 as (f32),
        1962i32 as (f32),
        1280i32 as (f32),
        2006i32 as (f32),
        -8209i32 as (f32),
        9750i32 as (f32),
        -998i32 as (f32),
        21189i32 as (f32),
        -53534i32 as (f32),
        71420i32 as (f32),
        -2i32 as (f32),
        10i32 as (f32),
        -68i32 as (f32),
        147i32 as (f32),
        215i32 as (f32),
        2i32 as (f32),
        -1137i32 as (f32),
        1919i32 as (f32),
        1131i32 as (f32),
        1692i32 as (f32),
        -8491i32 as (f32),
        9863i32 as (f32),
        -2122i32 as (f32),
        19478i32 as (f32),
        -55178i32 as (f32),
        70590i32 as (f32),
        -3i32 as (f32),
        9i32 as (f32),
        -73i32 as (f32),
        139i32 as (f32),
        208i32 as (f32),
        -29i32 as (f32),
        -1210i32 as (f32),
        1870i32 as (f32),
        970i32 as (f32),
        1388i32 as (f32),
        -8755i32 as (f32),
        9935i32 as (f32),
        -3300i32 as (f32),
        17799i32 as (f32),
        -56778i32 as (f32),
        69679i32 as (f32),
        -3i32 as (f32),
        8i32 as (f32),
        -79i32 as (f32),
        132i32 as (f32),
        200i32 as (f32),
        -57i32 as (f32),
        -1283i32 as (f32),
        1817i32 as (f32),
        794i32 as (f32),
        1095i32 as (f32),
        -8998i32 as (f32),
        9966i32 as (f32),
        -4533i32 as (f32),
        16155i32 as (f32),
        -58333i32 as (f32),
        68692i32 as (f32),
        -4i32 as (f32),
        7i32 as (f32),
        -85i32 as (f32),
        125i32 as (f32),
        189i32 as (f32),
        -83i32 as (f32),
        -1356i32 as (f32),
        1759i32 as (f32),
        605i32 as (f32),
        814i32 as (f32),
        -9219i32 as (f32),
        9959i32 as (f32),
        -5818i32 as (f32),
        14548i32 as (f32),
        -59838i32 as (f32),
        67629i32 as (f32),
        -4i32 as (f32),
        7i32 as (f32),
        -91i32 as (f32),
        117i32 as (f32),
        177i32 as (f32),
        -106i32 as (f32),
        -1428i32 as (f32),
        1698i32 as (f32),
        402i32 as (f32),
        545i32 as (f32),
        -9416i32 as (f32),
        9916i32 as (f32),
        -7154i32 as (f32),
        12980i32 as (f32),
        -61289i32 as (f32),
        66494i32 as (f32),
        -5i32 as (f32),
        6i32 as (f32),
        -97i32 as (f32),
        111i32 as (f32),
        163i32 as (f32),
        -127i32 as (f32),
        -1498i32 as (f32),
        1634i32 as (f32),
        185i32 as (f32),
        288i32 as (f32),
        -9585i32 as (f32),
        9838i32 as (f32),
        -8540i32 as (f32),
        11455i32 as (f32),
        -62684i32 as (f32),
        65290i32 as (f32),
    ];
    let mut zlin: *mut f32 = lins.offset((15i32 * 64i32) as (isize));
    let mut w: *const f32 = g_win.as_ptr();
    *zlin.offset((4i32 * 15i32) as (isize)) = *xl.offset((18i32 * 16i32) as (isize));
    *zlin.offset((4i32 * 15i32 + 1i32) as (isize)) = *xr.offset((18i32 * 16i32) as (isize));
    *zlin.offset((4i32 * 15i32 + 2i32) as (isize)) = *xl.offset(0isize);
    *zlin.offset((4i32 * 15i32 + 3i32) as (isize)) = *xr.offset(0isize);
    *zlin.offset((4i32 * 31i32) as (isize)) = *xl.offset((1i32 + 18i32 * 16i32) as (isize));
    *zlin.offset((4i32 * 31i32 + 1i32) as (isize)) = *xr.offset((1i32 + 18i32 * 16i32) as (isize));
    *zlin.offset((4i32 * 31i32 + 2i32) as (isize)) = *xl.offset(1isize);
    *zlin.offset((4i32 * 31i32 + 3i32) as (isize)) = *xr.offset(1isize);
    mp3d_synth_pair(
        dstr,
        nch,
        lins.offset((4i32 * 15i32) as (isize)).offset(1isize) as (*const f32),
    );
    mp3d_synth_pair(
        dstr.offset((32i32 * nch) as (isize)),
        nch,
        lins.offset((4i32 * 15i32) as (isize))
            .offset(64isize)
            .offset(1isize) as (*const f32),
    );
    mp3d_synth_pair(
        dstl,
        nch,
        lins.offset((4i32 * 15i32) as (isize)) as (*const f32),
    );
    mp3d_synth_pair(
        dstl.offset((32i32 * nch) as (isize)),
        nch,
        lins.offset((4i32 * 15i32) as (isize)).offset(64isize) as (*const f32),
    );
    i = 14i32;
    'loop1: loop {
        if !(i >= 0i32) {
            break;
        }
        let mut a: [f32; 4] = [0.0; 4];
        let mut b: [f32; 4] = [0.0; 4];
        *zlin.offset((4i32 * i) as (isize)) = *xl.offset((18i32 * (31i32 - i)) as (isize));
        *zlin.offset((4i32 * i + 1i32) as (isize)) = *xr.offset((18i32 * (31i32 - i)) as (isize));
        *zlin.offset((4i32 * i + 2i32) as (isize)) =
            *xl.offset((1i32 + 18i32 * (31i32 - i)) as (isize));
        *zlin.offset((4i32 * i + 3i32) as (isize)) =
            *xr.offset((1i32 + 18i32 * (31i32 - i)) as (isize));
        *zlin.offset((4i32 * (i + 16i32)) as (isize)) =
            *xl.offset((1i32 + 18i32 * (1i32 + i)) as (isize));
        *zlin.offset((4i32 * (i + 16i32) + 1i32) as (isize)) =
            *xr.offset((1i32 + 18i32 * (1i32 + i)) as (isize));
        *zlin.offset((4i32 * (i - 16i32) + 2i32) as (isize)) =
            *xl.offset((18i32 * (1i32 + i)) as (isize));
        *zlin.offset((4i32 * (i - 16i32) + 3i32) as (isize)) =
            *xr.offset((18i32 * (1i32 + i)) as (isize));
        let mut j: i32;
        let mut w0: f32 = *{
            let _old = w;
            w = w.offset(1isize);
            _old
        };
        let mut w1: f32 = *{
            let _old = w;
            w = w.offset(1isize);
            _old
        };
        let mut vz: *mut f32 =
            &mut *zlin.offset((4i32 * i - 0i32 * 64i32) as (isize)) as (*mut f32);
        let mut vy: *mut f32 =
            &mut *zlin.offset((4i32 * i - (15i32 - 0i32) * 64i32) as (isize)) as (*mut f32);
        j = 0i32;
        'loop4: loop {
            if !(j < 4i32) {
                break;
            }
            b[j as (usize)] = *vz.offset(j as (isize)) * w1 + *vy.offset(j as (isize)) * w0;
            a[j as (usize)] = *vz.offset(j as (isize)) * w0 - *vy.offset(j as (isize)) * w1;
            j = j + 1;
        }
        let mut j: i32;
        let mut w0: f32 = *{
            let _old = w;
            w = w.offset(1isize);
            _old
        };
        let mut w1: f32 = *{
            let _old = w;
            w = w.offset(1isize);
            _old
        };
        let mut vz: *mut f32 =
            &mut *zlin.offset((4i32 * i - 1i32 * 64i32) as (isize)) as (*mut f32);
        let mut vy: *mut f32 =
            &mut *zlin.offset((4i32 * i - (15i32 - 1i32) * 64i32) as (isize)) as (*mut f32);
        j = 0i32;
        'loop6: loop {
            if !(j < 4i32) {
                break;
            }
            let _rhs = *vz.offset(j as (isize)) * w1 + *vy.offset(j as (isize)) * w0;
            let _lhs = &mut b[j as (usize)];
            *_lhs = *_lhs + _rhs;
            let _rhs = *vy.offset(j as (isize)) * w1 - *vz.offset(j as (isize)) * w0;
            let _lhs = &mut a[j as (usize)];
            *_lhs = *_lhs + _rhs;
            j = j + 1;
        }
        let mut j: i32;
        let mut w0: f32 = *{
            let _old = w;
            w = w.offset(1isize);
            _old
        };
        let mut w1: f32 = *{
            let _old = w;
            w = w.offset(1isize);
            _old
        };
        let mut vz: *mut f32 =
            &mut *zlin.offset((4i32 * i - 2i32 * 64i32) as (isize)) as (*mut f32);
        let mut vy: *mut f32 =
            &mut *zlin.offset((4i32 * i - (15i32 - 2i32) * 64i32) as (isize)) as (*mut f32);
        j = 0i32;
        'loop8: loop {
            if !(j < 4i32) {
                break;
            }
            let _rhs = *vz.offset(j as (isize)) * w1 + *vy.offset(j as (isize)) * w0;
            let _lhs = &mut b[j as (usize)];
            *_lhs = *_lhs + _rhs;
            let _rhs = *vz.offset(j as (isize)) * w0 - *vy.offset(j as (isize)) * w1;
            let _lhs = &mut a[j as (usize)];
            *_lhs = *_lhs + _rhs;
            j = j + 1;
        }
        let mut j: i32;
        let mut w0: f32 = *{
            let _old = w;
            w = w.offset(1isize);
            _old
        };
        let mut w1: f32 = *{
            let _old = w;
            w = w.offset(1isize);
            _old
        };
        let mut vz: *mut f32 =
            &mut *zlin.offset((4i32 * i - 3i32 * 64i32) as (isize)) as (*mut f32);
        let mut vy: *mut f32 =
            &mut *zlin.offset((4i32 * i - (15i32 - 3i32) * 64i32) as (isize)) as (*mut f32);
        j = 0i32;
        'loop10: loop {
            if !(j < 4i32) {
                break;
            }
            let _rhs = *vz.offset(j as (isize)) * w1 + *vy.offset(j as (isize)) * w0;
            let _lhs = &mut b[j as (usize)];
            *_lhs = *_lhs + _rhs;
            let _rhs = *vy.offset(j as (isize)) * w1 - *vz.offset(j as (isize)) * w0;
            let _lhs = &mut a[j as (usize)];
            *_lhs = *_lhs + _rhs;
            j = j + 1;
        }
        let mut j: i32;
        let mut w0: f32 = *{
            let _old = w;
            w = w.offset(1isize);
            _old
        };
        let mut w1: f32 = *{
            let _old = w;
            w = w.offset(1isize);
            _old
        };
        let mut vz: *mut f32 =
            &mut *zlin.offset((4i32 * i - 4i32 * 64i32) as (isize)) as (*mut f32);
        let mut vy: *mut f32 =
            &mut *zlin.offset((4i32 * i - (15i32 - 4i32) * 64i32) as (isize)) as (*mut f32);
        j = 0i32;
        'loop12: loop {
            if !(j < 4i32) {
                break;
            }
            let _rhs = *vz.offset(j as (isize)) * w1 + *vy.offset(j as (isize)) * w0;
            let _lhs = &mut b[j as (usize)];
            *_lhs = *_lhs + _rhs;
            let _rhs = *vz.offset(j as (isize)) * w0 - *vy.offset(j as (isize)) * w1;
            let _lhs = &mut a[j as (usize)];
            *_lhs = *_lhs + _rhs;
            j = j + 1;
        }
        let mut j: i32;
        let mut w0: f32 = *{
            let _old = w;
            w = w.offset(1isize);
            _old
        };
        let mut w1: f32 = *{
            let _old = w;
            w = w.offset(1isize);
            _old
        };
        let mut vz: *mut f32 =
            &mut *zlin.offset((4i32 * i - 5i32 * 64i32) as (isize)) as (*mut f32);
        let mut vy: *mut f32 =
            &mut *zlin.offset((4i32 * i - (15i32 - 5i32) * 64i32) as (isize)) as (*mut f32);
        j = 0i32;
        'loop14: loop {
            if !(j < 4i32) {
                break;
            }
            let _rhs = *vz.offset(j as (isize)) * w1 + *vy.offset(j as (isize)) * w0;
            let _lhs = &mut b[j as (usize)];
            *_lhs = *_lhs + _rhs;
            let _rhs = *vy.offset(j as (isize)) * w1 - *vz.offset(j as (isize)) * w0;
            let _lhs = &mut a[j as (usize)];
            *_lhs = *_lhs + _rhs;
            j = j + 1;
        }
        let mut j: i32;
        let mut w0: f32 = *{
            let _old = w;
            w = w.offset(1isize);
            _old
        };
        let mut w1: f32 = *{
            let _old = w;
            w = w.offset(1isize);
            _old
        };
        let mut vz: *mut f32 =
            &mut *zlin.offset((4i32 * i - 6i32 * 64i32) as (isize)) as (*mut f32);
        let mut vy: *mut f32 =
            &mut *zlin.offset((4i32 * i - (15i32 - 6i32) * 64i32) as (isize)) as (*mut f32);
        j = 0i32;
        'loop16: loop {
            if !(j < 4i32) {
                break;
            }
            let _rhs = *vz.offset(j as (isize)) * w1 + *vy.offset(j as (isize)) * w0;
            let _lhs = &mut b[j as (usize)];
            *_lhs = *_lhs + _rhs;
            let _rhs = *vz.offset(j as (isize)) * w0 - *vy.offset(j as (isize)) * w1;
            let _lhs = &mut a[j as (usize)];
            *_lhs = *_lhs + _rhs;
            j = j + 1;
        }
        let mut j: i32;
        let mut w0: f32 = *{
            let _old = w;
            w = w.offset(1isize);
            _old
        };
        let mut w1: f32 = *{
            let _old = w;
            w = w.offset(1isize);
            _old
        };
        let mut vz: *mut f32 =
            &mut *zlin.offset((4i32 * i - 7i32 * 64i32) as (isize)) as (*mut f32);
        let mut vy: *mut f32 =
            &mut *zlin.offset((4i32 * i - (15i32 - 7i32) * 64i32) as (isize)) as (*mut f32);
        j = 0i32;
        'loop18: loop {
            if !(j < 4i32) {
                break;
            }
            let _rhs = *vz.offset(j as (isize)) * w1 + *vy.offset(j as (isize)) * w0;
            let _lhs = &mut b[j as (usize)];
            *_lhs = *_lhs + _rhs;
            let _rhs = *vy.offset(j as (isize)) * w1 - *vz.offset(j as (isize)) * w0;
            let _lhs = &mut a[j as (usize)];
            *_lhs = *_lhs + _rhs;
            j = j + 1;
        }
        *dstr.offset(((15i32 - i) * nch) as (isize)) = mp3d_scale_pcm(a[1usize]);
        *dstr.offset(((17i32 + i) * nch) as (isize)) = mp3d_scale_pcm(b[1usize]);
        *dstl.offset(((15i32 - i) * nch) as (isize)) = mp3d_scale_pcm(a[0usize]);
        *dstl.offset(((17i32 + i) * nch) as (isize)) = mp3d_scale_pcm(b[0usize]);
        *dstr.offset(((47i32 - i) * nch) as (isize)) = mp3d_scale_pcm(a[3usize]);
        *dstr.offset(((49i32 + i) * nch) as (isize)) = mp3d_scale_pcm(b[3usize]);
        *dstl.offset(((47i32 - i) * nch) as (isize)) = mp3d_scale_pcm(a[2usize]);
        *dstl.offset(((49i32 + i) * nch) as (isize)) = mp3d_scale_pcm(b[2usize]);
        i = i - 1;
    }
}

unsafe extern "C" fn mp3d_synth_granule(
    mut qmf_state: *mut f32,
    mut grbuf: *mut f32,
    mut nbands: i32,
    mut nch: i32,
    mut pcm: *mut i16,
    mut lins: *mut f32,
) {
    let mut i: i32;
    i = 0i32;
    'loop1: loop {
        if !(i < nch) {
            break;
        }
        mp3d_DCT_II(grbuf.offset((576i32 * i) as (isize)), nbands);
        i = i + 1;
    }
    memcpy(
        lins as (*mut ::std::os::raw::c_void),
        qmf_state as (*const ::std::os::raw::c_void),
        ::std::mem::size_of::<f32>()
            .wrapping_mul(15usize)
            .wrapping_mul(64usize),
    );
    i = 0i32;
    'loop3: loop {
        if !(i < nbands) {
            break;
        }
        mp3d_synth(
            grbuf.offset(i as (isize)),
            pcm.offset((32i32 * nch * i) as (isize)),
            nch,
            lins.offset((i * 64i32) as (isize)),
        );
        i = i + 2i32;
    }
    if nch == 1i32 {
        i = 0i32;
        'loop7: loop {
            if !(i < 15i32 * 64i32) {
                break;
            }
            *qmf_state.offset(i as (isize)) = *lins.offset((nbands * 64i32 + i) as (isize));
            i = i + 2i32;
        }
    } else {
        memcpy(
            qmf_state as (*mut ::std::os::raw::c_void),
            lins.offset((nbands * 64i32) as (isize)) as (*const ::std::os::raw::c_void),
            ::std::mem::size_of::<f32>()
                .wrapping_mul(15usize)
                .wrapping_mul(64usize),
        );
    }
}

unsafe extern "C" fn L3_read_side_info(
    mut bs: *mut Struct3,
    mut gr: *mut Struct5,
    mut hdr: *const u8,
) -> i32 {
    let mut _currentBlock;
    static mut g_scf_long: [[u8; 23]; 8] = [
        [
            6u8, 6u8, 6u8, 6u8, 6u8, 6u8, 8u8, 10u8, 12u8, 14u8, 16u8, 20u8, 24u8, 28u8, 32u8,
            38u8, 46u8, 52u8, 60u8, 68u8, 58u8, 54u8, 0u8,
        ],
        [
            12u8, 12u8, 12u8, 12u8, 12u8, 12u8, 16u8, 20u8, 24u8, 28u8, 32u8, 40u8, 48u8, 56u8,
            64u8, 76u8, 90u8, 2u8, 2u8, 2u8, 2u8, 2u8, 0u8,
        ],
        [
            6u8, 6u8, 6u8, 6u8, 6u8, 6u8, 8u8, 10u8, 12u8, 14u8, 16u8, 20u8, 24u8, 28u8, 32u8,
            38u8, 46u8, 52u8, 60u8, 68u8, 58u8, 54u8, 0u8,
        ],
        [
            6u8, 6u8, 6u8, 6u8, 6u8, 6u8, 8u8, 10u8, 12u8, 14u8, 16u8, 18u8, 22u8, 26u8, 32u8,
            38u8, 46u8, 54u8, 62u8, 70u8, 76u8, 36u8, 0u8,
        ],
        [
            6u8, 6u8, 6u8, 6u8, 6u8, 6u8, 8u8, 10u8, 12u8, 14u8, 16u8, 20u8, 24u8, 28u8, 32u8,
            38u8, 46u8, 52u8, 60u8, 68u8, 58u8, 54u8, 0u8,
        ],
        [
            4u8, 4u8, 4u8, 4u8, 4u8, 4u8, 6u8, 6u8, 8u8, 8u8, 10u8, 12u8, 16u8, 20u8, 24u8, 28u8,
            34u8, 42u8, 50u8, 54u8, 76u8, 158u8, 0u8,
        ],
        [
            4u8, 4u8, 4u8, 4u8, 4u8, 4u8, 6u8, 6u8, 6u8, 8u8, 10u8, 12u8, 16u8, 18u8, 22u8, 28u8,
            34u8, 40u8, 46u8, 54u8, 54u8, 192u8, 0u8,
        ],
        [
            4u8, 4u8, 4u8, 4u8, 4u8, 4u8, 6u8, 6u8, 8u8, 10u8, 12u8, 16u8, 20u8, 24u8, 30u8, 38u8,
            46u8, 56u8, 68u8, 84u8, 102u8, 26u8, 0u8,
        ],
    ];
    static mut g_scf_short: [[u8; 40]; 8] = [
        [
            4u8, 4u8, 4u8, 4u8, 4u8, 4u8, 4u8, 4u8, 4u8, 6u8, 6u8, 6u8, 8u8, 8u8, 8u8, 10u8, 10u8,
            10u8, 12u8, 12u8, 12u8, 14u8, 14u8, 14u8, 18u8, 18u8, 18u8, 24u8, 24u8, 24u8, 30u8,
            30u8, 30u8, 40u8, 40u8, 40u8, 18u8, 18u8, 18u8, 0u8,
        ],
        [
            8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 8u8, 12u8, 12u8, 12u8, 16u8, 16u8, 16u8, 20u8,
            20u8, 20u8, 24u8, 24u8, 24u8, 28u8, 28u8, 28u8, 36u8, 36u8, 36u8, 2u8, 2u8, 2u8, 2u8,
            2u8, 2u8, 2u8, 2u8, 2u8, 26u8, 26u8, 26u8, 0u8,
        ],
        [
            4u8, 4u8, 4u8, 4u8, 4u8, 4u8, 4u8, 4u8, 4u8, 6u8, 6u8, 6u8, 6u8, 6u8, 6u8, 8u8, 8u8,
            8u8, 10u8, 10u8, 10u8, 14u8, 14u8, 14u8, 18u8, 18u8, 18u8, 26u8, 26u8, 26u8, 32u8,
            32u8, 32u8, 42u8, 42u8, 42u8, 18u8, 18u8, 18u8, 0u8,
        ],
        [
            4u8, 4u8, 4u8, 4u8, 4u8, 4u8, 4u8, 4u8, 4u8, 6u8, 6u8, 6u8, 8u8, 8u8, 8u8, 10u8, 10u8,
            10u8, 12u8, 12u8, 12u8, 14u8, 14u8, 14u8, 18u8, 18u8, 18u8, 24u8, 24u8, 24u8, 32u8,
            32u8, 32u8, 44u8, 44u8, 44u8, 12u8, 12u8, 12u8, 0u8,
        ],
        [
            4u8, 4u8, 4u8, 4u8, 4u8, 4u8, 4u8, 4u8, 4u8, 6u8, 6u8, 6u8, 8u8, 8u8, 8u8, 10u8, 10u8,
            10u8, 12u8, 12u8, 12u8, 14u8, 14u8, 14u8, 18u8, 18u8, 18u8, 24u8, 24u8, 24u8, 30u8,
            30u8, 30u8, 40u8, 40u8, 40u8, 18u8, 18u8, 18u8, 0u8,
        ],
        [
            4u8, 4u8, 4u8, 4u8, 4u8, 4u8, 4u8, 4u8, 4u8, 4u8, 4u8, 4u8, 6u8, 6u8, 6u8, 8u8, 8u8,
            8u8, 10u8, 10u8, 10u8, 12u8, 12u8, 12u8, 14u8, 14u8, 14u8, 18u8, 18u8, 18u8, 22u8,
            22u8, 22u8, 30u8, 30u8, 30u8, 56u8, 56u8, 56u8, 0u8,
        ],
        [
            4u8, 4u8, 4u8, 4u8, 4u8, 4u8, 4u8, 4u8, 4u8, 4u8, 4u8, 4u8, 6u8, 6u8, 6u8, 6u8, 6u8,
            6u8, 10u8, 10u8, 10u8, 12u8, 12u8, 12u8, 14u8, 14u8, 14u8, 16u8, 16u8, 16u8, 20u8,
            20u8, 20u8, 26u8, 26u8, 26u8, 66u8, 66u8, 66u8, 0u8,
        ],
        [
            4u8, 4u8, 4u8, 4u8, 4u8, 4u8, 4u8, 4u8, 4u8, 4u8, 4u8, 4u8, 6u8, 6u8, 6u8, 8u8, 8u8,
            8u8, 12u8, 12u8, 12u8, 16u8, 16u8, 16u8, 20u8, 20u8, 20u8, 26u8, 26u8, 26u8, 34u8,
            34u8, 34u8, 42u8, 42u8, 42u8, 12u8, 12u8, 12u8, 0u8,
        ],
    ];
    // TODO: These... lengths are wrong???  I jus padded them out with 0's
    static mut g_scf_mixed: [[u8; 40]; 8] = [
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
    let mut scfsi: u32 = 0u32;
    let mut main_data_begin: i32;
    let mut part_23_sum: i32 = 0i32;
    let mut sr_idx: i32 = (*hdr.offset(2isize) as (i32) >> 2i32 & 3i32)
        + ((*hdr.offset(1isize) as (i32) >> 3i32 & 1i32)
            + (*hdr.offset(1isize) as (i32) >> 4i32 & 1i32)) * 3i32;
    sr_idx = sr_idx - (sr_idx != 0i32) as (i32);
    let mut gr_count: i32 = if *hdr.offset(3isize) as (i32) & 0xc0i32 == 0xc0i32 {
        1i32
    } else {
        2i32
    };
    if *hdr.offset(1isize) as (i32) & 0x8i32 != 0 {
        gr_count = gr_count * 2i32;
        main_data_begin = get_bits(bs, 9i32) as (i32);
        scfsi = get_bits(bs, 7i32 + gr_count);
    } else {
        main_data_begin = (get_bits(bs, 8i32 + gr_count) >> gr_count) as (i32);
    }
    'loop3: loop {
        if *hdr.offset(3isize) as (i32) & 0xc0i32 == 0xc0i32 {
            scfsi = scfsi << 4i32;
        }
        (*gr).part_23_length = get_bits(bs, 12i32) as (u16);
        part_23_sum = part_23_sum + (*gr).part_23_length as (i32);
        (*gr).big_values = get_bits(bs, 9i32) as (u16);
        if (*gr).big_values as (i32) > 288i32 {
            _currentBlock = 20;
            break;
        }
        (*gr).global_gain = get_bits(bs, 8i32) as (u8);
        (*gr).scalefac_compress = get_bits(
            bs,
            if *hdr.offset(1isize) as (i32) & 0x8i32 != 0 {
                4i32
            } else {
                9i32
            },
        ) as (u16);
        (*gr).sfbtab = g_scf_long[sr_idx as (usize)].as_ptr();
        (*gr).n_long_sfb = 22u8;
        (*gr).n_short_sfb = 0u8;
        if get_bits(bs, 1i32) != 0 {
            (*gr).block_type = get_bits(bs, 2i32) as (u8);
            if (*gr).block_type == 0 {
                _currentBlock = 19;
                break;
            }
            (*gr).mixed_block_flag = get_bits(bs, 1i32) as (u8);
            (*gr).region_count[0usize] = 7u8;
            (*gr).region_count[1usize] = 255u8;
            if (*gr).block_type as (i32) == 2i32 {
                scfsi = scfsi & 0xf0fu32;
                if (*gr).mixed_block_flag == 0 {
                    (*gr).region_count[0usize] = 8u8;
                    (*gr).sfbtab = g_scf_short[sr_idx as (usize)].as_ptr();
                    (*gr).n_long_sfb = 0u8;
                    (*gr).n_short_sfb = 39u8;
                } else {
                    (*gr).sfbtab = g_scf_mixed[sr_idx as (usize)].as_ptr();
                    (*gr).n_long_sfb = if *hdr.offset(1isize) as (i32) & 0x8i32 != 0 {
                        8i32
                    } else {
                        6i32
                    } as (u8);
                    (*gr).n_short_sfb = 30u8;
                }
            }
            tables = get_bits(bs, 10i32);
            tables = tables << 5i32;
            (*gr).subblock_gain[0usize] = get_bits(bs, 3i32) as (u8);
            (*gr).subblock_gain[1usize] = get_bits(bs, 3i32) as (u8);
            (*gr).subblock_gain[2usize] = get_bits(bs, 3i32) as (u8);
        } else {
            (*gr).block_type = 0u8;
            (*gr).mixed_block_flag = 0u8;
            tables = get_bits(bs, 15i32);
            (*gr).region_count[0usize] = get_bits(bs, 4i32) as (u8);
            (*gr).region_count[1usize] = get_bits(bs, 3i32) as (u8);
            (*gr).region_count[2usize] = 255u8;
        }
        (*gr).table_select[0usize] = (tables >> 10i32) as (u8);
        (*gr).table_select[1usize] = (tables >> 5i32 & 31u32) as (u8);
        (*gr).table_select[2usize] = (tables & 31u32) as (u8);
        (*gr).preflag = if *hdr.offset(1isize) as (i32) & 0x8i32 != 0 {
            get_bits(bs, 1i32)
        } else {
            ((*gr).scalefac_compress as (i32) >= 500i32) as (u32)
        } as (u8);
        (*gr).scalefac_scale = get_bits(bs, 1i32) as (u8);
        (*gr).count1_table = get_bits(bs, 1i32) as (u8);
        (*gr).scfsi = (scfsi >> 12i32 & 15u32) as (u8);
        scfsi = scfsi << 4i32;
        gr = gr.offset(1isize);
        if {
            gr_count = gr_count - 1;
            gr_count
        } == 0
        {
            _currentBlock = 16;
            break;
        }
    }
    if _currentBlock == 16 {
        (if part_23_sum + (*bs).pos > (*bs).limit + main_data_begin * 8i32 {
            -1i32
        } else {
            main_data_begin
        })
    } else if _currentBlock == 19 {
        -1i32
    } else {
        -1i32
    }
}

unsafe extern "C" fn L3_restore_reservoir(
    mut h: *mut Struct1,
    mut bs: *mut Struct3,
    mut s: *mut Struct4,
    mut main_data_begin: i32,
) -> i32 {
    let mut frame_bytes: i32 = ((*bs).limit - (*bs).pos) / 8i32;
    let mut bytes_have: i32 = if (*h).reserv > main_data_begin {
        main_data_begin
    } else {
        (*h).reserv
    };
    memcpy(
        (*s).maindata.as_mut_ptr() as (*mut ::std::os::raw::c_void),
        (*h).reserv_buf
            .as_mut_ptr()
            .offset(if 0i32 < (*h).reserv - main_data_begin {
                (*h).reserv - main_data_begin
            } else {
                0i32
            } as (isize)) as (*const ::std::os::raw::c_void),
        if (*h).reserv > main_data_begin {
            main_data_begin
        } else {
            (*h).reserv
        } as (usize),
    );
    memcpy(
        (*s).maindata.as_mut_ptr().offset(bytes_have as (isize)) as (*mut ::std::os::raw::c_void),
        (*bs).buf.offset(((*bs).pos / 8i32) as (isize)) as (*const ::std::os::raw::c_void),
        frame_bytes as (usize),
    );
    bs_init(
        &mut (*s).bs as (*mut Struct3),
        (*s).maindata.as_mut_ptr() as (*const u8),
        bytes_have + frame_bytes,
    );
    ((*h).reserv >= main_data_begin) as (i32)
}

unsafe extern "C" fn L3_read_scalefactors(
    mut scf: *mut u8,
    mut ist_pos: *mut u8,
    mut scf_size: *const u8,
    mut scf_count: *const u8,
    mut bitbuf: *mut Struct3,
    mut scfsi: i32,
) {
    let mut i: i32;
    let mut k: i32;
    i = 0i32;
    'loop1: loop {
        if !(i < 4i32 && (*scf_count.offset(i as (isize)) != 0)) {
            break;
        }
        let mut cnt: i32 = *scf_count.offset(i as (isize)) as (i32);
        if scfsi & 8i32 != 0 {
            memcpy(
                scf as (*mut ::std::os::raw::c_void),
                ist_pos as (*const ::std::os::raw::c_void),
                cnt as (usize),
            );
        } else {
            let mut bits: i32 = *scf_size.offset(i as (isize)) as (i32);
            if bits == 0 {
                memset(scf as (*mut ::std::os::raw::c_void), 0i32, cnt as (usize));
                memset(
                    ist_pos as (*mut ::std::os::raw::c_void),
                    0i32,
                    cnt as (usize),
                );
            } else {
                let mut max_scf: i32 = if scfsi < 0i32 {
                    (1i32 << bits) - 1i32
                } else {
                    -1i32
                };
                k = 0i32;
                'loop6: loop {
                    if !(k < cnt) {
                        break;
                    }
                    let mut s: i32 = get_bits(bitbuf, bits) as (i32);
                    *ist_pos.offset(k as (isize)) = if s == max_scf { -1i32 } else { s } as (u8);
                    *scf.offset(k as (isize)) = s as (u8);
                    k = k + 1;
                }
            }
        }
        ist_pos = ist_pos.offset(cnt as (isize));
        scf = scf.offset(cnt as (isize));
        i = i + 1;
        scfsi = scfsi * 2i32;
    }
    *scf.offset(0isize) = {
        let _rhs = {
            let _rhs = 0i32;
            let _lhs = &mut *scf.offset(2isize);
            *_lhs = _rhs as (u8);
            *_lhs
        };
        let _lhs = &mut *scf.offset(1isize);
        *_lhs = _rhs;
        *_lhs
    };
}

unsafe extern "C" fn L3_ldexp_q2(mut y: f32, mut exp_q2: i32) -> f32 {
    static mut g_expfrac: [f32; 4] = [
        9.31322575e-10f32,
        7.83145814e-10f32,
        6.58544508e-10f32,
        5.53767716e-10f32,
    ];
    let mut e: i32;
    'loop1: loop {
        e = if 30i32 * 4i32 > exp_q2 {
            exp_q2
        } else {
            30i32 * 4i32
        };
        y = y * (g_expfrac[(e & 3i32) as (usize)] * (1i32 << 30i32 >> (e >> 2i32)) as (f32));
        if !({
            exp_q2 = exp_q2 - e;
            exp_q2
        } > 0i32)
        {
            break;
        }
    }
    y
}

unsafe extern "C" fn L3_decode_scalefactors(
    mut hdr: *const u8,
    mut ist_pos: *mut u8,
    mut bs: *mut Struct3,
    mut gr: *const Struct5,
    mut scf: *mut f32,
    mut ch: i32,
) {
    static mut g_scf_partitions: [[u8; 28]; 3] = [
        [
            6u8, 5u8, 5u8, 5u8, 6u8, 5u8, 5u8, 5u8, 6u8, 5u8, 7u8, 3u8, 11u8, 10u8, 0u8, 0u8, 7u8,
            7u8, 7u8, 0u8, 6u8, 6u8, 6u8, 3u8, 8u8, 8u8, 5u8, 0u8,
        ],
        [
            8u8, 9u8, 6u8, 12u8, 6u8, 9u8, 9u8, 9u8, 6u8, 9u8, 12u8, 6u8, 15u8, 18u8, 0u8, 0u8,
            6u8, 15u8, 12u8, 0u8, 6u8, 12u8, 9u8, 6u8, 6u8, 18u8, 9u8, 0u8,
        ],
        [
            9u8, 9u8, 6u8, 12u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 12u8, 6u8, 18u8, 18u8, 0u8, 0u8,
            12u8, 12u8, 12u8, 0u8, 12u8, 9u8, 9u8, 6u8, 15u8, 12u8, 9u8, 0u8,
        ],
    ];
    let mut scf_partition: *const u8 = g_scf_partitions
        [(!((*gr).n_short_sfb == 0) as (i32) + ((*gr).n_long_sfb == 0) as (i32)) as (usize)]
        .as_ptr();
    let mut scf_size: [u8; 4] = [0; 4];
    let mut iscf: [u8; 40] = [0; 40];
    let mut i: i32;
    let mut scf_shift: i32 = (*gr).scalefac_scale as (i32) + 1i32;
    let mut gain_exp: i32;
    let mut scfsi: i32 = (*gr).scfsi as (i32);
    let mut gain: f32;
    if *hdr.offset(1isize) as (i32) & 0x8i32 != 0 {
        static mut g_scfc_decode: [u8; 16] = [
            0u8, 1u8, 2u8, 3u8, 12u8, 5u8, 6u8, 7u8, 9u8, 10u8, 11u8, 13u8, 14u8, 15u8, 18u8, 19u8,
        ];
        let mut part: i32 = g_scfc_decode[(*gr).scalefac_compress as (usize)] as (i32);
        scf_size[1usize] = {
            let _rhs = (part >> 2i32) as (u8);
            let _lhs = &mut scf_size[0usize];
            *_lhs = _rhs;
            *_lhs
        };
        scf_size[3usize] = {
            let _rhs = (part & 3i32) as (u8);
            let _lhs = &mut scf_size[2usize];
            *_lhs = _rhs;
            *_lhs
        };
    } else {
        static mut g_mod: [u8; 24] = [
            5u8, 5u8, 4u8, 4u8, 5u8, 5u8, 4u8, 1u8, 4u8, 3u8, 1u8, 1u8, 5u8, 6u8, 6u8, 1u8, 4u8,
            4u8, 4u8, 1u8, 4u8, 3u8, 1u8, 1u8,
        ];
        let mut k: i32;
        let mut modprod: i32;
        let mut sfc: i32;
        let mut ist: i32 = (*hdr.offset(3isize) as (i32) & 0x10i32 != 0 && (ch != 0)) as (i32);
        sfc = (*gr).scalefac_compress as (i32) >> ist;
        k = ist * 3i32 * 4i32;
        'loop2: loop {
            if !(sfc >= 0i32) {
                break;
            }
            modprod = 1i32;
            i = 3i32;
            'loop5: loop {
                if !(i >= 0i32) {
                    break;
                }
                scf_size[i as (usize)] =
                    (sfc / modprod % g_mod[(k + i) as (usize)] as (i32)) as (u8);
                modprod = modprod * g_mod[(k + i) as (usize)] as (i32);
                i = i - 1;
            }
            sfc = sfc - modprod;
            k = k + 4i32;
        }
        scf_partition = scf_partition.offset(k as (isize));
        scfsi = -16i32;
    }
    L3_read_scalefactors(
        iscf.as_mut_ptr(),
        ist_pos,
        scf_size.as_mut_ptr() as (*const u8),
        scf_partition,
        bs,
        scfsi,
    );
    if (*gr).n_short_sfb != 0 {
        let mut sh: i32 = 3i32 - scf_shift;
        i = 0i32;
        'loop17: loop {
            if !(i < (*gr).n_short_sfb as (i32)) {
                break;
            }
            {
                let _rhs = (*gr).subblock_gain[0usize] as (i32) << sh;
                let _lhs = &mut iscf[((*gr).n_long_sfb as (i32) + i + 0i32) as (usize)];
                *_lhs = (*_lhs as (i32) + _rhs) as (u8);
            }
            {
                let _rhs = (*gr).subblock_gain[1usize] as (i32) << sh;
                let _lhs = &mut iscf[((*gr).n_long_sfb as (i32) + i + 1i32) as (usize)];
                *_lhs = (*_lhs as (i32) + _rhs) as (u8);
            }
            {
                let _rhs = (*gr).subblock_gain[2usize] as (i32) << sh;
                let _lhs = &mut iscf[((*gr).n_long_sfb as (i32) + i + 2i32) as (usize)];
                *_lhs = (*_lhs as (i32) + _rhs) as (u8);
            }
            i = i + 3i32;
        }
    } else if (*gr).preflag != 0 {
        static mut g_preamp: [u8; 10] = [1u8, 1u8, 1u8, 1u8, 2u8, 2u8, 3u8, 3u8, 3u8, 2u8];
        i = 0i32;
        'loop13: loop {
            if !(i < 10i32) {
                break;
            }
            let _rhs = g_preamp[i as (usize)];
            let _lhs = &mut iscf[(11i32 + i) as (usize)];
            *_lhs = (*_lhs as (i32) + _rhs as (i32)) as (u8);
            i = i + 1;
        }
    }
    gain_exp = (*gr).global_gain as (i32) + -1i32 * 4i32 - 210i32
        - if *hdr.offset(3isize) as (i32) & 0xe0i32 == 0x60i32 {
            2i32
        } else {
            0i32
        };
    gain = L3_ldexp_q2(
        (1i32 << (255i32 + -1i32 * 4i32 - 210i32 + 3i32 & !3i32) / 4i32) as (f32),
        (255i32 + -1i32 * 4i32 - 210i32 + 3i32 & !3i32) - gain_exp,
    );
    i = 0i32;
    'loop19: loop {
        if !(i < (*gr).n_long_sfb as (i32) + (*gr).n_short_sfb as (i32)) {
            break;
        }
        *scf.offset(i as (isize)) = L3_ldexp_q2(gain, iscf[i as (usize)] as (i32) << scf_shift);
        i = i + 1;
    }
}

unsafe extern "C" fn L3_pow_43(mut x: i32) -> f32 {
    let mut frac: f32;
    let mut sign: i32;
    let mut mult: i32 = 256i32;
    if x < 129i32 {
        g_pow43[(16i32 + x) as (usize)]
    } else {
        if x < 1024i32 {
            mult = 16i32;
            x = x << 3i32;
        }
        sign = 2i32 * x & 64i32;
        frac = ((x & 63i32) - sign) as (f32) / ((x & !63i32) + sign) as (f32);
        g_pow43[(16i32 + (x + sign >> 6i32)) as (usize)]
            * (1f32 + frac * (4f32 / 3i32 as (f32) + frac * (2f32 / 9i32 as (f32))))
            * mult as (f32)
    }
}

unsafe extern "C" fn L3_huffman(
    mut dst: *mut f32,
    mut bs: *mut Struct3,
    mut gr_info: *const Struct5,
    mut scf: *const f32,
    mut layer3gr_limit: i32,
) {
    static mut tabs: [i16; 512] = [
        0i16, 0i16, 0i16, 0i16, 0i16, 0i16, 0i16, 0i16, 0i16, 0i16, 0i16, 0i16, 0i16, 0i16, 0i16,
        0i16, 0i16, 0i16, 0i16, 0i16, 0i16, 0i16, 0i16, 0i16, 0i16, 0i16, 0i16, 0i16, 0i16, 0i16,
        0i16, 0i16, 785i16, 785i16, 785i16, 785i16, 784i16, 784i16, 784i16, 784i16, 513i16, 513i16,
        513i16, 513i16, 513i16, 513i16, 513i16, 513i16, 256i16, 256i16, 256i16, 256i16, 256i16,
        256i16, 256i16, 256i16, 256i16, 256i16, 256i16, 256i16, 256i16, 256i16, 256i16, 256i16,
        -255i16, 1313i16, 1298i16, 1282i16, 785i16, 785i16, 785i16, 785i16, 784i16, 784i16, 784i16,
        784i16, 769i16, 769i16, 769i16, 769i16, 256i16, 256i16, 256i16, 256i16, 256i16, 256i16,
        256i16, 256i16, 256i16, 256i16, 256i16, 256i16, 256i16, 256i16, 256i16, 256i16, 290i16,
        288i16, -255i16, 1313i16, 1298i16, 1282i16, 769i16, 769i16, 769i16, 769i16, 529i16, 529i16,
        529i16, 529i16, 529i16, 529i16, 529i16, 529i16, 528i16, 528i16, 528i16, 528i16, 528i16,
        528i16, 528i16, 528i16, 512i16, 512i16, 512i16, 512i16, 512i16, 512i16, 512i16, 512i16,
        290i16, 288i16, -253i16, -318i16, -351i16, -367i16, 785i16, 785i16, 785i16, 785i16, 784i16,
        784i16, 784i16, 784i16, 769i16, 769i16, 769i16, 769i16, 256i16, 256i16, 256i16, 256i16,
        256i16, 256i16, 256i16, 256i16, 256i16, 256i16, 256i16, 256i16, 256i16, 256i16, 256i16,
        256i16, 819i16, 818i16, 547i16, 547i16, 275i16, 275i16, 275i16, 275i16, 561i16, 560i16,
        515i16, 546i16, 289i16, 274i16, 288i16, 258i16, -254i16, -287i16, 1329i16, 1299i16,
        1314i16, 1312i16, 1057i16, 1057i16, 1042i16, 1042i16, 1026i16, 1026i16, 784i16, 784i16,
        784i16, 784i16, 529i16, 529i16, 529i16, 529i16, 529i16, 529i16, 529i16, 529i16, 769i16,
        769i16, 769i16, 769i16, 768i16, 768i16, 768i16, 768i16, 563i16, 560i16, 306i16, 306i16,
        291i16, 259i16, -252i16, -413i16, -477i16, -542i16, 1298i16, -575i16, 1041i16, 1041i16,
        784i16, 784i16, 784i16, 784i16, 769i16, 769i16, 769i16, 769i16, 256i16, 256i16, 256i16,
        256i16, 256i16, 256i16, 256i16, 256i16, 256i16, 256i16, 256i16, 256i16, 256i16, 256i16,
        256i16, 256i16, -383i16, -399i16, 1107i16, 1092i16, 1106i16, 1061i16, 849i16, 849i16,
        789i16, 789i16, 1104i16, 1091i16, 773i16, 773i16, 1076i16, 1075i16, 341i16, 340i16, 325i16,
        309i16, 834i16, 804i16, 577i16, 577i16, 532i16, 532i16, 516i16, 516i16, 832i16, 818i16,
        803i16, 816i16, 561i16, 561i16, 531i16, 531i16, 515i16, 546i16, 289i16, 289i16, 288i16,
        258i16, -252i16, -429i16, -493i16, -559i16, 1057i16, 1057i16, 1042i16, 1042i16, 529i16,
        529i16, 529i16, 529i16, 529i16, 529i16, 529i16, 529i16, 784i16, 784i16, 784i16, 784i16,
        769i16, 769i16, 769i16, 769i16, 512i16, 512i16, 512i16, 512i16, 512i16, 512i16, 512i16,
        512i16, -382i16, 1077i16, -415i16, 1106i16, 1061i16, 1104i16, 849i16, 849i16, 789i16,
        789i16, 1091i16, 1076i16, 1029i16, 1075i16, 834i16, 834i16, 597i16, 581i16, 340i16, 340i16,
        339i16, 324i16, 804i16, 833i16, 532i16, 532i16, 832i16, 772i16, 818i16, 803i16, 817i16,
        787i16, 816i16, 771i16, 290i16, 290i16, 290i16, 290i16, 288i16, 258i16, -253i16, -349i16,
        -414i16, -447i16, -463i16, 1329i16, 1299i16, -479i16, 1314i16, 1312i16, 1057i16, 1057i16,
        1042i16, 1042i16, 1026i16, 1026i16, 785i16, 785i16, 785i16, 785i16, 784i16, 784i16, 784i16,
        784i16, 769i16, 769i16, 769i16, 769i16, 768i16, 768i16, 768i16, 768i16, -319i16, 851i16,
        821i16, -335i16, 836i16, 850i16, 805i16, 849i16, 341i16, 340i16, 325i16, 336i16, 533i16,
        533i16, 579i16, 579i16, 564i16, 564i16, 773i16, 832i16, 578i16, 548i16, 563i16, 516i16,
        321i16, 276i16, 306i16, 291i16, 304i16, 259i16, -251i16, -572i16, -733i16, -830i16,
        -863i16, -879i16, 1041i16, 1041i16, 784i16, 784i16, 784i16, 784i16, 769i16, 769i16, 769i16,
        769i16, 256i16, 256i16, 256i16, 256i16, 256i16, 256i16, 256i16, 256i16, 256i16, 256i16,
        256i16, 256i16, 256i16, 256i16, 256i16, 256i16, -511i16, -527i16, -543i16, 1396i16,
        1351i16, 1381i16, 1366i16, 1395i16, 1335i16, 1380i16, -559i16, 1334i16, 1138i16, 1138i16,
        1063i16, 1063i16, 1350i16, 1392i16, 1031i16, 1031i16, 1062i16, 1062i16, 1364i16, 1363i16,
        1120i16, 1120i16, 1333i16, 1348i16, 881i16, 881i16, 881i16, 881i16, 375i16, 374i16, 359i16,
        373i16, 343i16, 358i16, 341i16, 325i16, 791i16, 791i16, 1123i16, 1122i16, -703i16, 1105i16,
        1045i16, -719i16, 865i16, 865i16, 790i16, 790i16, 774i16, 774i16,
    ];
    static mut tab32: [u8; 28] = [
        130u8, 162u8, 193u8, 209u8, 44u8, 28u8, 76u8, 140u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8,
        9u8, 190u8, 254u8, 222u8, 238u8, 126u8, 94u8, 157u8, 157u8, 109u8, 61u8, 173u8, 205u8,
    ];
    static mut tab33: [u8; 16] = [
        252u8, 236u8, 220u8, 204u8, 188u8, 172u8, 156u8, 140u8, 124u8, 108u8, 92u8, 76u8, 60u8,
        44u8, 28u8, 12u8,
    ];
    static mut tabindex: [i16; 32] = [
        0i16, 32i16, 64i16, 98i16, 0i16, 132i16, 180i16, 218i16, 292i16, 364i16, 426i16, 538i16,
        648i16, 746i16, 0i16, 1126i16, 1460i16, 1460i16, 1460i16, 1460i16, 1460i16, 1460i16,
        1460i16, 1460i16, 1842i16, 1842i16, 1842i16, 1842i16, 1842i16, 1842i16, 1842i16, 1842i16,
    ];
    static mut g_linbits: [u8; 32] = [
        0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 1u8, 2u8,
        3u8, 4u8, 6u8, 8u8, 10u8, 13u8, 4u8, 5u8, 6u8, 7u8, 8u8, 9u8, 11u8, 13u8,
    ];
    let mut one: f32 = 0.0f32;
    let mut ireg: i32 = 0i32;
    let mut big_val_cnt: i32 = (*gr_info).big_values as (i32);
    let mut sfb: *const u8 = (*gr_info).sfbtab;
    let mut bs_next_ptr: *const u8 = (*bs).buf.offset(((*bs).pos / 8i32) as (isize));
    let mut bs_cache: u32 = (*bs_next_ptr.offset(0isize) as (u32))
        .wrapping_mul(256u32)
        .wrapping_add(*bs_next_ptr.offset(1isize) as (u32))
        .wrapping_mul(256u32)
        .wrapping_add(*bs_next_ptr.offset(2isize) as (u32))
        .wrapping_mul(256u32)
        .wrapping_add(*bs_next_ptr.offset(3isize) as (u32))
        << ((*bs).pos & 7i32);
    let mut pairs_to_decode: i32;
    let mut np: i32;
    let mut bs_sh: i32 = ((*bs).pos & 7i32) - 8i32;
    bs_next_ptr = bs_next_ptr.offset(4isize);
    'loop1: loop {
        if !(big_val_cnt > 0i32) {
            break;
        }
        let mut tab_num: i32 = (*gr_info).table_select[ireg as (usize)] as (i32);
        let mut sfb_cnt: i32 =
            (*gr_info).region_count[{
                                        let _old = ireg;
                                        ireg = ireg + 1;
                                        _old
                                    } as (usize)] as (i32);
        let mut codebook: *const i16 = tabs
            .as_ptr()
            .offset(tabindex[tab_num as (usize)] as (isize));
        let mut linbits: i32 = g_linbits[tab_num as (usize)] as (i32);
        'loop25: loop {
            np = *{
                let _old = sfb;
                sfb = sfb.offset(1isize);
                _old
            } as (i32) / 2i32;
            pairs_to_decode = if big_val_cnt > np { np } else { big_val_cnt };
            one = *{
                let _old = scf;
                scf = scf.offset(1isize);
                _old
            };
            'loop26: loop {
                let mut j: i32;
                let mut w: i32 = 5i32;
                let mut leaf: i32 = *codebook.offset((bs_cache >> 32i32 - w) as (isize)) as (i32);
                'loop27: loop {
                    if !(leaf < 0i32) {
                        break;
                    }
                    bs_cache = bs_cache << w;
                    bs_sh = bs_sh + w;
                    w = leaf & 7i32;
                    leaf = *codebook.offset(
                        (bs_cache >> 32i32 - w).wrapping_sub((leaf >> 3i32) as (u32)) as (isize),
                    ) as (i32);
                }
                bs_cache = bs_cache << (leaf >> 8i32);
                bs_sh = bs_sh + (leaf >> 8i32);
                j = 0i32;
                'loop29: loop {
                    if !(j < 2i32) {
                        break;
                    }
                    let mut lsb: i32 = leaf & 0xfi32;
                    if lsb == 15i32 && (linbits != 0) {
                        lsb = (lsb as (u32)).wrapping_add(bs_cache >> 32i32 - linbits) as (i32);
                        bs_cache = bs_cache << linbits;
                        bs_sh = bs_sh + linbits;
                        'loop37: loop {
                            if !(bs_sh >= 0i32) {
                                break;
                            }
                            bs_cache = bs_cache | *{
                                let _old = bs_next_ptr;
                                bs_next_ptr = bs_next_ptr.offset(1isize);
                                _old
                            } as (u32) << bs_sh;
                            bs_sh = bs_sh - 8i32;
                        }
                        *dst = one * L3_pow_43(lsb) * if bs_cache as (i32) < 0i32 {
                            -1i32
                        } else {
                            1i32
                        } as (f32);
                    } else {
                        *dst = g_pow43[((16i32 + lsb) as (u32))
                                           .wrapping_sub(16u32.wrapping_mul(bs_cache >> 31i32))
                                           as (usize)] * one;
                    }
                    bs_cache = bs_cache << if lsb != 0 { 1i32 } else { 0i32 };
                    bs_sh = bs_sh + if lsb != 0 { 1i32 } else { 0i32 };
                    j = j + 1;
                    dst = dst.offset(1isize);
                    leaf = leaf >> 4i32;
                }
                'loop30: loop {
                    if !(bs_sh >= 0i32) {
                        break;
                    }
                    bs_cache = bs_cache | *{
                        let _old = bs_next_ptr;
                        bs_next_ptr = bs_next_ptr.offset(1isize);
                        _old
                    } as (u32) << bs_sh;
                    bs_sh = bs_sh - 8i32;
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
            } > 0i32 && ({
                sfb_cnt = sfb_cnt - 1;
                sfb_cnt
            } >= 0i32))
            {
                break;
            }
        }
    }
    np = 1i32 - big_val_cnt;
    'loop3: loop {
        let mut codebook_count1: *const u8 = if (*gr_info).count1_table != 0 {
            tab33.as_ptr()
        } else {
            tab32.as_ptr()
        };
        let mut leaf: i32 = *codebook_count1.offset((bs_cache >> 32i32 - 4i32) as (isize)) as (i32);
        if leaf & 8i32 == 0 {
            leaf = *codebook_count1.offset(
                ((leaf >> 3i32) as (u32)).wrapping_add(bs_cache << 4i32 >> 32i32 - (leaf & 3i32))
                    as (isize),
            ) as (i32);
        }
        bs_cache = bs_cache << (leaf & 7i32);
        bs_sh = bs_sh + (leaf & 7i32);
        if (bs_next_ptr as (isize)).wrapping_sub((*bs).buf as (isize))
            / ::std::mem::size_of::<u8>() as (isize) * 8isize - 24isize
            + bs_sh as (isize) > layer3gr_limit as (isize)
        {
            break;
        }
        if {
            np = np - 1;
            np
        } == 0
        {
            np = *{
                let _old = sfb;
                sfb = sfb.offset(1isize);
                _old
            } as (i32) / 2i32;
            if np == 0 {
                break;
            }
            one = *{
                let _old = scf;
                scf = scf.offset(1isize);
                _old
            };
        }
        if leaf & 128i32 >> 0i32 != 0 {
            *dst.offset(0isize) = if bs_cache as (i32) < 0i32 { -one } else { one };
            bs_cache = bs_cache << 1i32;
            bs_sh = bs_sh + 1i32;
        }
        if leaf & 128i32 >> 1i32 != 0 {
            *dst.offset(1isize) = if bs_cache as (i32) < 0i32 { -one } else { one };
            bs_cache = bs_cache << 1i32;
            bs_sh = bs_sh + 1i32;
        }
        if {
            np = np - 1;
            np
        } == 0
        {
            np = *{
                let _old = sfb;
                sfb = sfb.offset(1isize);
                _old
            } as (i32) / 2i32;
            if np == 0 {
                break;
            }
            one = *{
                let _old = scf;
                scf = scf.offset(1isize);
                _old
            };
        }
        if leaf & 128i32 >> 2i32 != 0 {
            *dst.offset(2isize) = if bs_cache as (i32) < 0i32 { -one } else { one };
            bs_cache = bs_cache << 1i32;
            bs_sh = bs_sh + 1i32;
        }
        if leaf & 128i32 >> 3i32 != 0 {
            *dst.offset(3isize) = if bs_cache as (i32) < 0i32 { -one } else { one };
            bs_cache = bs_cache << 1i32;
            bs_sh = bs_sh + 1i32;
        }
        'loop20: loop {
            if !(bs_sh >= 0i32) {
                break;
            }
            bs_cache = bs_cache | *{
                let _old = bs_next_ptr;
                bs_next_ptr = bs_next_ptr.offset(1isize);
                _old
            } as (u32) << bs_sh;
            bs_sh = bs_sh - 8i32;
        }
        dst = dst.offset(4isize);
    }
    (*bs).pos = layer3gr_limit;
}

unsafe extern "C" fn L3_midside_stereo(mut left: *mut f32, mut n: i32) {
    let mut i: i32 = 0i32;
    let mut right: *mut f32 = left.offset(576isize);
    'loop1: loop {
        if !(i < n) {
            break;
        }
        let mut a: f32 = *left.offset(i as (isize));
        let mut b: f32 = *right.offset(i as (isize));
        *left.offset(i as (isize)) = a + b;
        *right.offset(i as (isize)) = a - b;
        i = i + 1;
    }
}

unsafe extern "C" fn L3_stereo_top_band(
    mut right: *const f32,
    mut sfb: *const u8,
    mut nbands: i32,
    mut max_band: *mut i32,
) {
    let mut _currentBlock;
    let mut i: i32;
    let mut k: i32;
    *max_band.offset(0isize) = {
        let _rhs = {
            let _rhs = -1i32;
            let _lhs = &mut *max_band.offset(2isize);
            *_lhs = _rhs;
            *_lhs
        };
        let _lhs = &mut *max_band.offset(1isize);
        *_lhs = _rhs;
        *_lhs
    };
    i = 0i32;
    'loop1: loop {
        if !(i < nbands) {
            break;
        }
        k = 0i32;
        'loop4: loop {
            if !(k < *sfb.offset(i as (isize)) as (i32)) {
                _currentBlock = 8;
                break;
            }
            if *right.offset(k as (isize)) != 0i32 as (f32)
                || *right.offset((k + 1i32) as (isize)) != 0i32 as (f32)
            {
                _currentBlock = 7;
                break;
            }
            k = k + 2i32;
        }
        if _currentBlock == 7 {
            *max_band.offset((i % 3i32) as (isize)) = i;
        }
        right = right.offset(*sfb.offset(i as (isize)) as (isize));
        i = i + 1;
    }
}

unsafe extern "C" fn L3_intensity_stereo_band(
    mut left: *mut f32,
    mut n: i32,
    mut kl: f32,
    mut kr: f32,
) {
    let mut i: i32;
    i = 0i32;
    'loop1: loop {
        if !(i < n) {
            break;
        }
        *left.offset((i + 576i32) as (isize)) = *left.offset(i as (isize)) * kr;
        *left.offset(i as (isize)) = *left.offset(i as (isize)) * kl;
        i = i + 1;
    }
}

unsafe extern "C" fn L3_stereo_process(
    mut left: *mut f32,
    mut ist_pos: *const u8,
    mut sfb: *const u8,
    mut hdr: *const u8,
    mut max_band: *mut i32,
    mut mpeg2_sh: i32,
) {
    static mut g_pan: [f32; 14] = [
        0i32 as (f32),
        1i32 as (f32),
        0.21132487f32,
        0.78867513f32,
        0.36602540f32,
        0.63397460f32,
        0.5f32,
        0.5f32,
        0.63397460f32,
        0.36602540f32,
        0.78867513f32,
        0.21132487f32,
        1i32 as (f32),
        0i32 as (f32),
    ];
    let mut i: u32;
    let mut max_pos: u32 = (if *hdr.offset(1isize) as (i32) & 0x8i32 != 0 {
        7i32
    } else {
        64i32
    }) as (u32);
    i = 0u32;
    'loop1: loop {
        if *sfb.offset(i as (isize)) == 0 {
            break;
        }
        let mut ipos: u32 = *ist_pos.offset(i as (isize)) as (u32);
        if i as (i32) > *max_band.offset(i.wrapping_rem(3u32) as (isize)) && (ipos < max_pos) {
            let mut kl: f32;
            let mut kr: f32;
            let mut s: f32 = if *hdr.offset(3isize) as (i32) & 0x20i32 != 0 {
                1.41421356f32
            } else {
                1i32 as (f32)
            };
            if *hdr.offset(1isize) as (i32) & 0x8i32 != 0 {
                kl = g_pan[2u32.wrapping_mul(ipos) as (usize)];
                kr = g_pan[2u32.wrapping_mul(ipos).wrapping_add(1u32) as (usize)];
            } else {
                kl = 1i32 as (f32);
                kr = L3_ldexp_q2(
                    1i32 as (f32),
                    (ipos.wrapping_add(1u32) >> 1i32 << mpeg2_sh) as (i32),
                );
                if ipos & 1u32 != 0 {
                    kl = kr;
                    kr = 1i32 as (f32);
                }
            }
            L3_intensity_stereo_band(left, *sfb.offset(i as (isize)) as (i32), kl * s, kr * s);
        } else if *hdr.offset(3isize) as (i32) & 0x20i32 != 0 {
            L3_midside_stereo(left, *sfb.offset(i as (isize)) as (i32));
        }
        left = left.offset(*sfb.offset(i as (isize)) as (isize));
        i = i.wrapping_add(1u32);
    }
}

unsafe extern "C" fn L3_intensity_stereo(
    mut left: *mut f32,
    mut ist_pos: *mut u8,
    mut gr: *const Struct5,
    mut hdr: *const u8,
) {
    let mut max_band: [i32; 3] = [0; 3];
    let mut n_sfb: i32 = (*gr).n_long_sfb as (i32) + (*gr).n_short_sfb as (i32);
    let mut i: i32;
    let mut max_blocks: i32 = if (*gr).n_short_sfb != 0 { 3i32 } else { 1i32 };
    L3_stereo_top_band(
        left.offset(576isize) as (*const f32),
        (*gr).sfbtab,
        n_sfb,
        max_band.as_mut_ptr(),
    );
    if (*gr).n_long_sfb != 0 {
        max_band[0usize] = {
            let _rhs = {
                let _rhs = if if max_band[0usize] < max_band[1usize] {
                    max_band[1usize]
                } else {
                    max_band[0usize]
                } < max_band[2usize]
                {
                    max_band[2usize]
                } else if max_band[0usize] < max_band[1usize] {
                    max_band[1usize]
                } else {
                    max_band[0usize]
                };
                let _lhs = &mut max_band[2usize];
                *_lhs = _rhs;
                *_lhs
            };
            let _lhs = &mut max_band[1usize];
            *_lhs = _rhs;
            *_lhs
        };
    }
    i = 0i32;
    'loop3: loop {
        if !(i < max_blocks) {
            break;
        }
        let mut default_pos: i32 = if *hdr.offset(1isize) as (i32) & 0x8i32 != 0 {
            3i32
        } else {
            0i32
        };
        let mut itop: i32 = n_sfb - max_blocks + i;
        let mut prev: i32 = itop - max_blocks;
        *ist_pos.offset(itop as (isize)) = if max_band[i as (usize)] >= prev {
            default_pos
        } else {
            *ist_pos.offset(prev as (isize)) as (i32)
        } as (u8);
        i = i + 1;
    }
    L3_stereo_process(
        left,
        ist_pos as (*const u8),
        (*gr).sfbtab,
        hdr,
        max_band.as_mut_ptr(),
        (*gr.offset(1isize)).scalefac_compress as (i32) & 1i32,
    );
}

unsafe extern "C" fn L3_reorder(mut grbuf: *mut f32, mut scratch: *mut f32, mut sfb: *const u8) {
    let mut i: i32;
    let mut len: i32;
    let mut src: *mut f32 = grbuf;
    let mut dst: *mut f32 = scratch;
    'loop1: loop {
        if !(0i32 != {
            len = *sfb as (i32);
            len
        }) {
            break;
        }
        i = 0i32;
        'loop4: loop {
            if !(i < len) {
                break;
            }
            *{
                let _old = dst;
                dst = dst.offset(1isize);
                _old
            } = *src.offset((0i32 * len) as (isize));
            *{
                let _old = dst;
                dst = dst.offset(1isize);
                _old
            } = *src.offset((1i32 * len) as (isize));
            *{
                let _old = dst;
                dst = dst.offset(1isize);
                _old
            } = *src.offset((2i32 * len) as (isize));
            i = i + 1;
            src = src.offset(1isize);
        }
        sfb = sfb.offset(3isize);
        src = src.offset((2i32 * len) as (isize));
    }
    memcpy(
        grbuf as (*mut ::std::os::raw::c_void),
        scratch as (*const ::std::os::raw::c_void),
        (((dst as (isize)).wrapping_sub(scratch as (isize))
            / ::std::mem::size_of::<f32>() as (isize)) as (usize))
            .wrapping_mul(::std::mem::size_of::<f32>()),
    );
}

unsafe extern "C" fn L3_antialias(mut grbuf: *mut f32, mut nbands: i32) {
    static mut g_aa: [[f32; 8]; 2] = [
        [
            0.85749293f32,
            0.88174200f32,
            0.94962865f32,
            0.98331459f32,
            0.99551782f32,
            0.99916056f32,
            0.99989920f32,
            0.99999316f32,
        ],
        [
            0.51449576f32,
            0.47173197f32,
            0.31337745f32,
            0.18191320f32,
            0.09457419f32,
            0.04096558f32,
            0.01419856f32,
            0.00369997f32,
        ],
    ];
    'loop1: loop {
        if !(nbands > 0i32) {
            break;
        }
        let mut i: i32 = 0i32;
        'loop4: loop {
            if !(i < 8i32) {
                break;
            }
            let mut u: f32 = *grbuf.offset((18i32 + i) as (isize));
            let mut d: f32 = *grbuf.offset((17i32 - i) as (isize));
            *grbuf.offset((18i32 + i) as (isize)) =
                u * g_aa[0usize][i as (usize)] - d * g_aa[1usize][i as (usize)];
            *grbuf.offset((17i32 - i) as (isize)) =
                u * g_aa[1usize][i as (usize)] + d * g_aa[0usize][i as (usize)];
            i = i + 1;
        }
        nbands = nbands - 1;
        grbuf = grbuf.offset(18isize);
    }
}

unsafe extern "C" fn L3_dct3_9(mut y: *mut f32) {
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
    s0 = *y.offset(0isize);
    s2 = *y.offset(2isize);
    s4 = *y.offset(4isize);
    s6 = *y.offset(6isize);
    s8 = *y.offset(8isize);
    t0 = s0 + s6 * 0.5f32;
    s0 = s0 - s6;
    t4 = (s4 + s2) * 0.93969262f32;
    t2 = (s8 + s2) * 0.76604444f32;
    s6 = (s4 - s8) * 0.17364818f32;
    s4 = s4 + (s8 - s2);
    s2 = s0 - s4 * 0.5f32;
    *y.offset(4isize) = s4 + s0;
    s8 = t0 - t2 + s6;
    s0 = t0 - t4 + t2;
    s4 = t0 + t4 - s6;
    s1 = *y.offset(1isize);
    s3 = *y.offset(3isize);
    s5 = *y.offset(5isize);
    s7 = *y.offset(7isize);
    s3 = s3 * 0.86602540f32;
    t0 = (s5 + s1) * 0.98480775f32;
    t4 = (s5 - s7) * 0.34202014f32;
    t2 = (s1 + s7) * 0.64278761f32;
    s1 = (s1 - s5 - s7) * 0.86602540f32;
    s5 = t0 - s3 - t2;
    s7 = t4 - s3 - t0;
    s3 = t4 + s3 - t2;
    *y.offset(0isize) = s4 - s7;
    *y.offset(1isize) = s2 + s1;
    *y.offset(2isize) = s0 - s3;
    *y.offset(3isize) = s8 + s5;
    *y.offset(5isize) = s8 - s5;
    *y.offset(6isize) = s0 + s3;
    *y.offset(7isize) = s2 - s1;
    *y.offset(8isize) = s4 + s7;
}

unsafe extern "C" fn L3_imdct36(
    mut grbuf: *mut f32,
    mut overlap: *mut f32,
    mut window: *const f32,
    mut nbands: i32,
) {
    let mut i: i32;
    let mut j: i32;
    static mut g_twid9: [f32; 18] = [
        0.73727734f32,
        0.79335334f32,
        0.84339145f32,
        0.88701083f32,
        0.92387953f32,
        0.95371695f32,
        0.97629601f32,
        0.99144486f32,
        0.99904822f32,
        0.67559021f32,
        0.60876143f32,
        0.53729961f32,
        0.46174861f32,
        0.38268343f32,
        0.30070580f32,
        0.21643961f32,
        0.13052619f32,
        0.04361938f32,
    ];
    j = 0i32;
    'loop1: loop {
        if !(j < nbands) {
            break;
        }
        let mut co: [f32; 9] = [0.0; 9];
        let mut si: [f32; 9] = [0.0; 9];
        co[0usize] = -*grbuf.offset(0isize);
        si[0usize] = *grbuf.offset(17isize);
        i = 0i32;
        'loop4: loop {
            if !(i < 4i32) {
                break;
            }
            si[(8i32 - 2i32 * i) as (usize)] = *grbuf.offset((4i32 * i + 1i32) as (isize))
                - *grbuf.offset((4i32 * i + 2i32) as (isize));
            co[(1i32 + 2i32 * i) as (usize)] = *grbuf.offset((4i32 * i + 1i32) as (isize))
                + *grbuf.offset((4i32 * i + 2i32) as (isize));
            si[(7i32 - 2i32 * i) as (usize)] = *grbuf.offset((4i32 * i + 4i32) as (isize))
                - *grbuf.offset((4i32 * i + 3i32) as (isize));
            co[(2i32 + 2i32 * i) as (usize)] = -(*grbuf.offset((4i32 * i + 3i32) as (isize))
                + *grbuf.offset((4i32 * i + 4i32) as (isize)));
            i = i + 1;
        }
        L3_dct3_9(co.as_mut_ptr());
        L3_dct3_9(si.as_mut_ptr());
        si[1usize] = -si[1usize];
        si[3usize] = -si[3usize];
        si[5usize] = -si[5usize];
        si[7usize] = -si[7usize];
        i = 0i32;
        'loop6: loop {
            if !(i < 9i32) {
                break;
            }
            let mut ovl: f32 = *overlap.offset(i as (isize));
            let mut sum: f32 = co[i as (usize)] * g_twid9[(9i32 + i) as (usize)]
                + si[i as (usize)] * g_twid9[(0i32 + i) as (usize)];
            *overlap.offset(i as (isize)) = co[i as (usize)] * g_twid9[(0i32 + i) as (usize)]
                - si[i as (usize)] * g_twid9[(9i32 + i) as (usize)];
            *grbuf.offset(i as (isize)) = ovl * *window.offset((0i32 + i) as (isize))
                - sum * *window.offset((9i32 + i) as (isize));
            *grbuf.offset((17i32 - i) as (isize)) = ovl * *window.offset((9i32 + i) as (isize))
                + sum * *window.offset((0i32 + i) as (isize));
            i = i + 1;
        }
        j = j + 1;
        grbuf = grbuf.offset(18isize);
        overlap = overlap.offset(9isize);
    }
}

unsafe extern "C" fn L3_idct3(mut x0: f32, mut x1: f32, mut x2: f32, mut dst: *mut f32) {
    let mut m1: f32 = x1 * 0.86602540f32;
    let mut a1: f32 = x0 - x2 * 0.5f32;
    *dst.offset(1isize) = x0 + x2;
    *dst.offset(0isize) = a1 + m1;
    *dst.offset(2isize) = a1 - m1;
}

unsafe extern "C" fn L3_imdct12(mut x: *mut f32, mut dst: *mut f32, mut overlap: *mut f32) {
    static mut g_twid3: [f32; 6] = [
        0.79335334f32,
        0.92387953f32,
        0.99144486f32,
        0.60876143f32,
        0.38268343f32,
        0.13052619f32,
    ];
    let mut co: [f32; 3] = [0.0; 3];
    let mut si: [f32; 3] = [0.0; 3];
    let mut i: i32;
    L3_idct3(
        -*x.offset(0isize),
        *x.offset(6isize) + *x.offset(3isize),
        *x.offset(12isize) + *x.offset(9isize),
        co.as_mut_ptr(),
    );
    L3_idct3(
        *x.offset(15isize),
        *x.offset(12isize) - *x.offset(9isize),
        *x.offset(6isize) - *x.offset(3isize),
        si.as_mut_ptr(),
    );
    si[1usize] = -si[1usize];
    i = 0i32;
    'loop1: loop {
        if !(i < 3i32) {
            break;
        }
        let mut ovl: f32 = *overlap.offset(i as (isize));
        let mut sum: f32 = co[i as (usize)] * g_twid3[(3i32 + i) as (usize)]
            + si[i as (usize)] * g_twid3[(0i32 + i) as (usize)];
        *overlap.offset(i as (isize)) = co[i as (usize)] * g_twid3[(0i32 + i) as (usize)]
            - si[i as (usize)] * g_twid3[(3i32 + i) as (usize)];
        *dst.offset(i as (isize)) =
            ovl * g_twid3[(2i32 - i) as (usize)] - sum * g_twid3[(5i32 - i) as (usize)];
        *dst.offset((5i32 - i) as (isize)) =
            ovl * g_twid3[(5i32 - i) as (usize)] + sum * g_twid3[(2i32 - i) as (usize)];
        i = i + 1;
    }
}

unsafe extern "C" fn L3_imdct_short(mut grbuf: *mut f32, mut overlap: *mut f32, mut nbands: i32) {
    'loop0: loop {
        if !(nbands > 0i32) {
            break;
        }
        let mut tmp: [f32; 18] = [0.0; 18];
        memcpy(
            tmp.as_mut_ptr() as (*mut ::std::os::raw::c_void),
            grbuf as (*const ::std::os::raw::c_void),
            ::std::mem::size_of::<[f32; 18]>(),
        );
        memcpy(
            grbuf as (*mut ::std::os::raw::c_void),
            overlap as (*const ::std::os::raw::c_void),
            6usize.wrapping_mul(::std::mem::size_of::<f32>()),
        );
        L3_imdct12(
            tmp.as_mut_ptr(),
            grbuf.offset(6isize),
            overlap.offset(6isize),
        );
        L3_imdct12(
            tmp.as_mut_ptr().offset(1isize),
            grbuf.offset(12isize),
            overlap.offset(6isize),
        );
        L3_imdct12(
            tmp.as_mut_ptr().offset(2isize),
            overlap,
            overlap.offset(6isize),
        );
        nbands = nbands - 1;
        overlap = overlap.offset(9isize);
        grbuf = grbuf.offset(18isize);
    }
}

unsafe extern "C" fn L3_imdct_gr(
    mut grbuf: *mut f32,
    mut overlap: *mut f32,
    mut block_type: u32,
    mut n_long_bands: u32,
) {
    static mut g_mdct_window: [[f32; 18]; 2] = [
        [
            0.99904822f32,
            0.99144486f32,
            0.97629601f32,
            0.95371695f32,
            0.92387953f32,
            0.88701083f32,
            0.84339145f32,
            0.79335334f32,
            0.73727734f32,
            0.04361938f32,
            0.13052619f32,
            0.21643961f32,
            0.30070580f32,
            0.38268343f32,
            0.46174861f32,
            0.53729961f32,
            0.60876143f32,
            0.67559021f32,
        ],
        [
            1i32 as (f32),
            1i32 as (f32),
            1i32 as (f32),
            1i32 as (f32),
            1i32 as (f32),
            1i32 as (f32),
            0.99144486f32,
            0.92387953f32,
            0.79335334f32,
            0i32 as (f32),
            0i32 as (f32),
            0i32 as (f32),
            0i32 as (f32),
            0i32 as (f32),
            0i32 as (f32),
            0.13052619f32,
            0.38268343f32,
            0.60876143f32,
        ],
    ];
    if n_long_bands != 0 {
        L3_imdct36(
            grbuf,
            overlap,
            g_mdct_window[0usize].as_ptr(),
            n_long_bands as (i32),
        );
        grbuf = grbuf.offset(18u32.wrapping_mul(n_long_bands) as (isize));
        overlap = overlap.offset(9u32.wrapping_mul(n_long_bands) as (isize));
    }
    if block_type == 2u32 {
        L3_imdct_short(grbuf, overlap, 32u32.wrapping_sub(n_long_bands) as (i32));
    } else {
        L3_imdct36(
            grbuf,
            overlap,
            g_mdct_window[(block_type == 3u32) as (usize)].as_ptr(),
            32u32.wrapping_sub(n_long_bands) as (i32),
        );
    }
}

unsafe extern "C" fn L3_change_sign(mut grbuf: *mut f32) {
    let mut b: i32;
    let mut i: i32;
    b = 0i32;
    grbuf = grbuf.offset(18isize);
    'loop1: loop {
        if !(b < 32i32) {
            break;
        }
        i = 1i32;
        'loop4: loop {
            if !(i < 18i32) {
                break;
            }
            *grbuf.offset(i as (isize)) = -*grbuf.offset(i as (isize));
            i = i + 2i32;
        }
        b = b + 2i32;
        grbuf = grbuf.offset(36isize);
    }
}

unsafe extern "C" fn L3_decode(
    mut h: *mut Struct1,
    mut s: *mut Struct4,
    mut gr_info: *mut Struct5,
    mut nch: i32,
) {
    let mut ch: i32;
    ch = 0i32;
    'loop1: loop {
        if !(ch < nch) {
            break;
        }
        let mut layer3gr_limit: i32 =
            (*s).bs.pos + (*gr_info.offset(ch as (isize))).part_23_length as (i32);
        L3_decode_scalefactors(
            (*h).header.as_mut_ptr() as (*const u8),
            (*s).ist_pos[ch as (usize)].as_mut_ptr(),
            &mut (*s).bs as (*mut Struct3),
            gr_info.offset(ch as (isize)) as (*const Struct5),
            (*s).scf.as_mut_ptr(),
            ch,
        );
        L3_huffman(
            (*s).grbuf[ch as (usize)].as_mut_ptr(),
            &mut (*s).bs as (*mut Struct3),
            gr_info.offset(ch as (isize)) as (*const Struct5),
            (*s).scf.as_mut_ptr() as (*const f32),
            layer3gr_limit,
        );
        ch = ch + 1;
    }
    if (*h).header[3usize] as (i32) & 0x10i32 != 0 {
        L3_intensity_stereo(
            (*s).grbuf[0usize].as_mut_ptr(),
            (*s).ist_pos[1usize].as_mut_ptr(),
            gr_info as (*const Struct5),
            (*h).header.as_mut_ptr() as (*const u8),
        );
    } else if (*h).header[3usize] as (i32) & 0xe0i32 == 0x60i32 {
        L3_midside_stereo((*s).grbuf[0usize].as_mut_ptr(), 576i32);
    }
    ch = 0i32;
    'loop7: loop {
        if !(ch < nch) {
            break;
        }
        let mut aa_bands: i32 = 31i32;
        let mut n_long_bands: i32 = (if (*gr_info).mixed_block_flag != 0 {
            2i32
        } else {
            0i32
        })
            << (((*h).header[2usize] as (i32) >> 2i32 & 3i32)
                + (((*h).header[1usize] as (i32) >> 3i32 & 1i32)
                    + ((*h).header[1usize] as (i32) >> 4i32 & 1i32)) * 3i32 == 2i32)
                as (i32);
        if (*gr_info).n_short_sfb != 0 {
            aa_bands = n_long_bands - 1i32;
            L3_reorder(
                (*s).grbuf[ch as (usize)]
                    .as_mut_ptr()
                    .offset((n_long_bands * 18i32) as (isize)),
                (*s).syn[0usize].as_mut_ptr(),
                (*gr_info).sfbtab.offset((*gr_info).n_long_sfb as (isize)),
            );
        }
        L3_antialias((*s).grbuf[ch as (usize)].as_mut_ptr(), aa_bands);
        L3_imdct_gr(
            (*s).grbuf[ch as (usize)].as_mut_ptr(),
            (*h).mdct_overlap[ch as (usize)].as_mut_ptr(),
            (*gr_info).block_type as (u32),
            n_long_bands as (u32),
        );
        L3_change_sign((*s).grbuf[ch as (usize)].as_mut_ptr());
        ch = ch + 1;
        gr_info = gr_info.offset(1isize);
    }
}

unsafe extern "C" fn L3_save_reservoir(mut h: *mut Struct1, mut s: *mut Struct4) {
    let mut pos: i32 = (((*s).bs.pos + 7i32) as (u32)).wrapping_div(8u32) as (i32);
    let mut remains: i32 = ((*s).bs.limit as (u32))
        .wrapping_div(8u32)
        .wrapping_sub(pos as (u32)) as (i32);
    if remains > 511i32 {
        pos = pos + (remains - 511i32);
        remains = 511i32;
    }
    if remains > 0i32 {
        memmove(
            (*h).reserv_buf.as_mut_ptr() as (*mut ::std::os::raw::c_void),
            (*s).maindata.as_mut_ptr().offset(pos as (isize)) as (*const ::std::os::raw::c_void),
            remains as (usize),
        );
    }
    (*h).reserv = remains;
}

#[no_mangle]
pub unsafe extern "C" fn mp3dec_decode_frame(
    mut dec: *mut Struct1,
    mut mp3: *const u8,
    mut mp3_bytes: i32,
    mut pcm: *mut i16,
    mut info: *mut Struct2,
) -> i32 {
    let mut _currentBlock;
    let mut i: i32 = 0i32;
    let mut igr: i32;
    let mut frame_size: i32 = 0i32;
    let mut success: i32 = 1i32;
    let mut hdr: *const u8;
    let mut bs_frame: [Struct3; 1] = [Struct3 {
        buf: ::std::ptr::null(),
        pos: 0,
        limit: 0,
    }];
    let mut scratch: Struct4 = Struct4 {
        bs: bs_frame[0],
        maindata: [0; 2815],
        gr_info: ::std::mem::uninitialized(),
        grbuf: [[0.0; 576]; 2],
        scf: [0.0; 40],
        syn: [[0.0; 64]; 33],
        ist_pos: [[0; 39]; 2],
    };
    if mp3_bytes > 4i32
        && ((*dec).header[0usize] as (i32) == 0xffi32)
        && (hdr_compare((*dec).header.as_mut_ptr() as (*const u8), mp3) != 0)
    {
        frame_size = hdr_frame_bytes(mp3, (*dec).free_format_bytes) + hdr_padding(mp3);
        if frame_size != mp3_bytes
            && (frame_size + 4i32 > mp3_bytes
                || hdr_compare(mp3, mp3.offset(frame_size as (isize))) == 0)
        {
            frame_size = 0i32;
        }
    }
    if frame_size == 0 {
        memset(
            dec as (*mut ::std::os::raw::c_void),
            0i32,
            ::std::mem::size_of::<Struct1>(),
        );
        i = mp3d_find_frame(
            mp3,
            mp3_bytes,
            &mut (*dec).free_format_bytes as (*mut i32),
            &mut frame_size as (*mut i32),
        );
        if frame_size == 0 || i + frame_size > mp3_bytes {
            (*info).frame_bytes = i;
            return 0i32;
        }
    }
    hdr = mp3.offset(i as (isize));
    memcpy(
        (*dec).header.as_mut_ptr() as (*mut ::std::os::raw::c_void),
        hdr as (*const ::std::os::raw::c_void),
        4usize,
    );
    (*info).frame_bytes = i + frame_size;
    (*info).channels = if *hdr.offset(3isize) as (i32) & 0xc0i32 == 0xc0i32 {
        1i32
    } else {
        2i32
    };
    (*info).hz = hdr_sample_rate_hz(hdr) as (i32);
    (*info).layer = 4i32 - (*hdr.offset(1isize) as (i32) >> 1i32 & 3i32);
    (*info).bitrate_kbps = hdr_bitrate_kbps(hdr) as (i32);
    if pcm.is_null() {
        hdr_frame_samples(hdr) as (i32)
    } else {
        bs_init(bs_frame.as_mut_ptr(), hdr.offset(4isize), frame_size - 4i32);
        if *hdr.offset(1isize) as (i32) & 1i32 == 0 {
            get_bits(bs_frame.as_mut_ptr(), 16i32);
        }
        if (*info).layer == 3i32 {
            let mut main_data_begin: i32 =
                L3_read_side_info(bs_frame.as_mut_ptr(), scratch.gr_info.as_mut_ptr(), hdr);
            if main_data_begin < 0i32 || bs_frame[0usize].pos > bs_frame[0usize].limit {
                mp3dec_init(dec);
                return 0i32;
            } else {
                success = L3_restore_reservoir(
                    dec,
                    bs_frame.as_mut_ptr(),
                    &mut scratch as (*mut Struct4),
                    main_data_begin,
                );
                if success != 0 {
                    igr = 0i32;
                    'loop19: loop {
                        if !(igr < if *hdr.offset(1isize) as (i32) & 0x8i32 != 0 {
                            2i32
                        } else {
                            1i32
                        }) {
                            break;
                        }
                        memset(
                            scratch.grbuf[0usize].as_mut_ptr() as (*mut ::std::os::raw::c_void),
                            0i32,
                            ((576i32 * 2i32) as (usize)).wrapping_mul(::std::mem::size_of::<f32>()),
                        );
                        L3_decode(
                            dec,
                            &mut scratch as (*mut Struct4),
                            scratch
                                .gr_info
                                .as_mut_ptr()
                                .offset((igr * (*info).channels) as (isize)),
                            (*info).channels,
                        );
                        mp3d_synth_granule(
                            (*dec).qmf_state.as_mut_ptr(),
                            scratch.grbuf[0usize].as_mut_ptr(),
                            18i32,
                            (*info).channels,
                            pcm,
                            scratch.syn[0usize].as_mut_ptr(),
                        );
                        igr = igr + 1;
                        pcm = pcm.offset((576i32 * (*info).channels) as (isize));
                    }
                }
                L3_save_reservoir(dec, &mut scratch as (*mut Struct4));
            }
        } else {
            let mut sci: [Struct6; 1] = ::std::mem::uninitialized();
            L12_read_scale_info(hdr, bs_frame.as_mut_ptr(), sci.as_mut_ptr());
            memset(
                scratch.grbuf[0usize].as_mut_ptr() as (*mut ::std::os::raw::c_void),
                0i32,
                ((576i32 * 2i32) as (usize)).wrapping_mul(::std::mem::size_of::<f32>()),
            );
            i = 0i32;
            igr = 0i32;
            'loop10: loop {
                if !(igr < 3i32) {
                    _currentBlock = 21;
                    break;
                }
                if 12i32 == {
                    i = i + L12_dequantize_granule(
                        scratch.grbuf[0usize].as_mut_ptr().offset(i as (isize)),
                        bs_frame.as_mut_ptr(),
                        sci.as_mut_ptr(),
                        (*info).layer | 1i32,
                    );
                    i
                } {
                    i = 0i32;
                    L12_apply_scf_384(
                        sci.as_mut_ptr(),
                        sci[0usize].scf.as_mut_ptr().offset(igr as (isize)) as (*const f32),
                        scratch.grbuf[0usize].as_mut_ptr(),
                    );
                    mp3d_synth_granule(
                        (*dec).qmf_state.as_mut_ptr(),
                        scratch.grbuf[0usize].as_mut_ptr(),
                        12i32,
                        (*info).channels,
                        pcm,
                        scratch.syn[0usize].as_mut_ptr(),
                    );
                    memset(
                        scratch.grbuf[0usize].as_mut_ptr() as (*mut ::std::os::raw::c_void),
                        0i32,
                        ((576i32 * 2i32) as (usize)).wrapping_mul(::std::mem::size_of::<f32>()),
                    );
                    pcm = pcm.offset((384i32 * (*info).channels) as (isize));
                }
                if bs_frame[0usize].pos > bs_frame[0usize].limit {
                    _currentBlock = 15;
                    break;
                }
                igr = igr + 1;
            }
            if _currentBlock == 21 {
            } else {
                mp3dec_init(dec);
                return 0i32;
            }
        }
        (success as (u32))
            .wrapping_mul(hdr_frame_samples((*dec).header.as_mut_ptr() as (*const u8)))
            as (i32)
    }
}

fn main() {
    let ret = unsafe { _c_main() };
    ::std::process::exit(ret);
}

#[no_mangle]
pub unsafe extern "C" fn _c_main() -> i32 {
    static mut mp3d: Struct1 = Struct1 {
        mdct_overlap: [[0f32; 288]; 2],
        qmf_state: [0f32; 960],
        reserv: 0i32,
        free_format_bytes: 0i32,
        header: [0u8; 4],
        reserv_buf: [0u8; 511],
    };
    mp3dec_init(&mut mp3d as (*mut Struct1));
    let mut info: Struct2;
    let mut pcm: [i16; 2304];
    0i32
}
