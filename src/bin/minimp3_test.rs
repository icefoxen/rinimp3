/// This is a translation of minimp3_test.c,
/// which also includes minimp3_ex.h.

extern "C" {
    fn __errno_location() -> *mut i32;
    fn abs(__x: i32) -> i32;
    fn close(__fd: i32) -> i32;
    fn exit(__status: i32);
    fn fclose(__stream: *mut IoFile) -> i32;
    fn fopen(__filename: *const u8, __modes: *const u8) -> *mut IoFile;
    fn fread(
        __ptr: *mut ::std::os::raw::c_void,
        __size: usize,
        __n: usize,
        __stream: *mut IoFile,
    ) -> usize;
    fn free(__ptr: *mut ::std::os::raw::c_void);
    fn fseek(__stream: *mut IoFile, __off: isize, __whence: i32) -> i32;
    fn fstat(__fd: i32, __buf: *mut stat) -> i32;
    fn ftell(__stream: *mut IoFile) -> isize;
    fn fwrite(
        __ptr: *const ::std::os::raw::c_void,
        __size: usize,
        __n: usize,
        __s: *mut IoFile,
    ) -> usize;
    fn malloc(__size: usize) -> *mut ::std::os::raw::c_void;
    fn memcpy(
        __dest: *mut ::std::os::raw::c_void,
        __src: *const ::std::os::raw::c_void,
        __n: usize,
    ) -> *mut ::std::os::raw::c_void;
    fn memset(
        __s: *mut ::std::os::raw::c_void,
        __c: i32,
        __n: usize,
    ) -> *mut ::std::os::raw::c_void;
    fn mmap(
        __addr: *mut ::std::os::raw::c_void,
        __len: usize,
        __prot: i32,
        __flags: i32,
        __fd: i32,
        __offset: isize,
    ) -> *mut ::std::os::raw::c_void;
    fn munmap(__addr: *mut ::std::os::raw::c_void, __len: usize) -> i32;
    fn open(__file: *const u8, __oflag: i32, ...) -> i32;
    fn printf(__format: *const u8, ...) -> i32;
    fn realloc(__ptr: *mut ::std::os::raw::c_void, __size: usize) -> *mut ::std::os::raw::c_void;
    fn rewind(__stream: *mut IoFile);
    fn strcasecmp(__s1: *const u8, __s2: *const u8) -> i32;
    fn strncmp(__s1: *const u8, __s2: *const u8, __n: usize) -> i32;
    fn strrchr(__s: *const u8, __c: i32) -> *mut u8;
}

enum IoFile {}

extern crate rinimp3;
#[allow(unused_imports)]
#[macro_use]
extern crate structopt;
use rinimp3::*;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct Mp3decFileInfo {
    pub buffer: *mut i16,
    pub samples: usize,
    pub channels: i32,
    pub hz: i32,
    pub layer: i32,
    pub avg_bitrate_kbps: i32,
}

unsafe fn mp3dec_skip_id3v2(buf: *const u8, buf_size: usize) -> usize {
    if buf_size > 10usize
        && (strncmp(
            buf as (*mut u8) as (*const u8),
            (*b"ID3\0").as_ptr(),
            3usize,
        ) == 0)
    {
        (((*buf.offset(6isize) as (i32) & 0x7fi32) << 21i32
            | (*buf.offset(7isize) as (i32) & 0x7fi32) << 14i32
            | (*buf.offset(8isize) as (i32) & 0x7fi32) << 7i32
            | *buf.offset(9isize) as (i32) & 0x7fi32)
            + 10i32) as (usize)
    } else {
        0usize
    }
}
pub unsafe fn mp3dec_load_buf(
    dec: *mut Mp3Dec,
    mut buf: *const u8,
    mut buf_size: usize,
    info: *mut Mp3decFileInfo,
    progress_cb: unsafe fn(*mut ::std::os::raw::c_void, usize, usize, *mut FrameInfo) -> i32,
    user_data: *mut ::std::os::raw::c_void,
) {
    let orig_buf_size: usize = buf_size;
    let mut pcm: [i16; 2304] = [0; 2304];
    let mut frame_info: FrameInfo = ::std::mem::zeroed();
    memset(
        info as (*mut ::std::os::raw::c_void),
        0i32,
        ::std::mem::size_of::<Mp3decFileInfo>(),
    );
    memset(
        &mut frame_info as (*mut FrameInfo) as (*mut ::std::os::raw::c_void),
        0i32,
        ::std::mem::size_of::<FrameInfo>(),
    );
    let id3v2size: usize = mp3dec_skip_id3v2(buf, buf_size);
    if id3v2size > buf_size {
    } else {
        buf = buf.offset(id3v2size as (isize));
        buf_size = buf_size.wrapping_sub(id3v2size);
        *dec = Mp3Dec::new();
        let mut samples: i32;
        'loop2: loop {
            samples = mp3dec_decode_frame(
                &mut *dec,
                ::std::slice::from_raw_parts(buf, buf_size),
                &mut pcm[..],
                &mut frame_info,
            );
            buf = buf.offset(frame_info.frame_bytes as (isize));
            buf_size = buf_size.wrapping_sub(frame_info.frame_bytes as (usize));
            if samples != 0 {
                break;
            }
            if frame_info.frame_bytes == 0 {
                break;
            }
        }
        (if samples == 0 {
        } else {
            samples = samples * frame_info.channels;
            let mut allocated: usize = buf_size
                .wrapping_div(frame_info.frame_bytes as (usize))
                .wrapping_mul(samples as (usize))
                .wrapping_mul(::std::mem::size_of::<i16>())
                .wrapping_add(
                    ((1152i32 * 2i32) as (usize)).wrapping_mul(::std::mem::size_of::<i16>()),
                );
            (*info).buffer = malloc(allocated) as (*mut i16);
            (if (*info).buffer.is_null() {
            } else {
                (*info).samples = samples as (usize);
                memcpy(
                    (*info).buffer as (*mut ::std::os::raw::c_void),
                    pcm.as_mut_ptr() as (*const ::std::os::raw::c_void),
                    (*info).samples.wrapping_mul(::std::mem::size_of::<i16>()),
                );
                (*info).channels = frame_info.channels;
                (*info).hz = frame_info.hz;
                (*info).layer = frame_info.layer;
                let mut avg_bitrate_kbps: usize = frame_info.bitrate_kbps as (usize);
                let mut frames: usize = 1usize;
                let mut frame_bytes: i32;
                'loop7: loop {
                    if allocated
                        .wrapping_sub((*info).samples.wrapping_mul(::std::mem::size_of::<i16>()))
                        < ((1152i32 * 2i32) as (usize)).wrapping_mul(::std::mem::size_of::<i16>())
                    {
                        allocated = allocated.wrapping_mul(2usize);
                        (*info).buffer =
                            realloc((*info).buffer as (*mut ::std::os::raw::c_void), allocated)
                                as (*mut i16);
                    }
                    // TODO: ooooh I hope the `allocated` value
                    // here is correct!
                    // mp3_decode_frame() doesn't actually need to
                    // know the length though, it assumes the
                    // buffer is a fixed size.
                    let buffer_slice = ::std::slice::from_raw_parts_mut(
                        (*info).buffer.offset((*info).samples as (isize)),
                        allocated,
                    );
                    samples = mp3dec_decode_frame(
                        &mut *dec,
                        ::std::slice::from_raw_parts(buf, buf_size),
                        buffer_slice,
                        // &*info.buffer[info.samples as usize..],
                        &mut frame_info,
                    );
                    frame_bytes = frame_info.frame_bytes;
                    buf = buf.offset(frame_bytes as (isize));
                    buf_size = buf_size.wrapping_sub(frame_bytes as (usize));
                    if samples != 0 {
                        if (*info).hz != frame_info.hz || (*info).layer != frame_info.layer {
                            break;
                        }
                        if (*info).channels != 0 && ((*info).channels != frame_info.channels) {
                            (*info).channels = 0i32;
                        }
                        (*info).samples = (*info)
                            .samples
                            .wrapping_add((samples * frame_info.channels) as (usize));
                        avg_bitrate_kbps =
                            avg_bitrate_kbps.wrapping_add(frame_info.bitrate_kbps as (usize));
                        frames = frames.wrapping_add(1usize);
                        progress_cb(
                            user_data,
                            orig_buf_size,
                            orig_buf_size.wrapping_sub(buf_size),
                            &mut frame_info as (*mut FrameInfo),
                        );
                    }
                    if frame_bytes == 0 {
                        break;
                    }
                }
                if allocated != (*info).samples.wrapping_mul(::std::mem::size_of::<i16>()) {
                    (*info).buffer = realloc(
                        (*info).buffer as (*mut ::std::os::raw::c_void),
                        (*info).samples.wrapping_mul(::std::mem::size_of::<i16>()),
                    ) as (*mut i16);
                }
                (*info).avg_bitrate_kbps = avg_bitrate_kbps.wrapping_div(frames) as (i32);
            })
        })
    }
}

pub unsafe fn mp3dec_iterate_buf(
    mut buf: *const u8,
    mut buf_size: usize,
    callback: unsafe fn(*mut ::std::os::raw::c_void, *const u8, i32, usize, *mut FrameInfo) -> i32,
    user_data: *mut ::std::os::raw::c_void,
) {
    let mut frame_info: FrameInfo = ::std::mem::zeroed();
    memset(
        &mut frame_info as (*mut FrameInfo) as (*mut ::std::os::raw::c_void),
        0i32,
        ::std::mem::size_of::<FrameInfo>(),
    );
    let id3v2size: usize = mp3dec_skip_id3v2(buf, buf_size);
    (if id3v2size > buf_size {
    } else {
        let orig_buf: *const u8 = buf;
        buf = buf.offset(id3v2size as (isize));
        buf_size = buf_size.wrapping_sub(id3v2size);
        'loop3: loop {
            let mut free_format_bytes: i32 = 0i32;
            let mut frame_size: i32 = 0i32;
            let buf_slice = ::std::slice::from_raw_parts(buf, buf_size);
            let i: i32 = mp3d_find_frame(
                buf_slice,
                buf_size as (i32),
                &mut free_format_bytes,
                &mut frame_size,
            );
            buf = buf.offset(i as (isize));
            buf_size = buf_size.wrapping_sub(i as (usize));
            if !(i != 0 && (frame_size == 0)) {
                if frame_size == 0 {
                    break;
                }
                let hdr: &[u8] = ::std::slice::from_raw_parts(buf, buf_size);
                frame_info.channels = if hdr[3] as (i32) & 0xc0i32 == 0xc0i32 {
                    1i32
                } else {
                    2i32
                };
                frame_info.hz = hdr_sample_rate_hz(hdr) as (i32);
                // TODO: Double-check precedence on this next line
                frame_info.layer = 4i32 - hdr[1] as (i32) >> 1i32 & 3i32;
                frame_info.bitrate_kbps = hdr_bitrate_kbps(hdr) as (i32);
                frame_info.frame_bytes = frame_size;
                if callback(
                    user_data,
                    hdr.as_ptr(),
                    frame_size,
                    ((hdr.as_ptr() as (isize)).wrapping_sub(orig_buf as (isize))
                        / ::std::mem::size_of::<u8>() as (isize)) as (usize),
                    &mut frame_info as (*mut FrameInfo),
                ) != 0
                {
                    break;
                }
                buf = buf.offset(frame_size as (isize));
                buf_size = buf_size.wrapping_sub(frame_size as (usize));
            }
            if false {
                break;
            }
        }
    })
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct Mp3decMapInfo {
    pub buffer: *const u8,
    pub size: usize,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct Mp3decEx {
    pub mp3d: Mp3Dec,
    pub file: Mp3decMapInfo,
    pub seek_method: i32,
    pub is_file: i32,
}

pub unsafe fn mp3dec_ex_open_buf(
    dec: *mut Mp3decEx,
    buf: *const u8,
    buf_size: usize,
    seek_method: i32,
) -> i32 {
    memset(
        dec as (*mut ::std::os::raw::c_void),
        0i32,
        ::std::mem::size_of::<Mp3decEx>(),
    );

    (*dec).mp3d = Mp3Dec::new();
    (*dec).file.buffer = buf;
    (*dec).file.size = buf_size;
    (*dec).seek_method = seek_method;
    0i32
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: isize,
    pub tv_nsec: isize,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct stat {
    pub st_dev: usize,
    pub st_ino: usize,
    pub st_nlink: usize,
    pub st_mode: u32,
    pub st_uid: u32,
    pub st_gid: u32,
    pub __pad0: i32,
    pub st_rdev: usize,
    pub st_size: isize,
    pub st_blksize: isize,
    pub st_blocks: isize,
    pub st_atim: timespec,
    pub st_mtim: timespec,
    pub st_ctim: timespec,
    pub __glibc_reserved: [isize; 3],
}

unsafe fn mp3dec_open_file(file_name: *const u8, map_info: *mut Mp3decMapInfo) -> i32 {
    let mut file: i32;
    let mut st: stat = ::std::mem::zeroed();
    memset(
        map_info as (*mut ::std::os::raw::c_void),
        0i32,
        ::std::mem::size_of::<Mp3decMapInfo>(),
    );
    'loop1: loop {
        file = open(file_name, 0o0i32);
        if !(file < 0i32 && (*__errno_location() == 11i32 || *__errno_location() == 4i32)) {
            break;
        }
    }
    if file < 0i32 || fstat(file, &mut st as (*mut stat)) < 0i32 {
        close(file);
        -1i32
    } else {
        (*map_info).size = st.st_size as (usize);
        'loop4: loop {
            (*map_info).buffer = mmap(
                0i32 as (*mut ::std::os::raw::c_void),
                st.st_size as (usize),
                0x1i32,
                0x2i32 | 0x8000i32,
                file,
                0isize,
            ) as (*const u8);
            if !(-1i32 as (*mut ::std::os::raw::c_void) as (*const u8) == (*map_info).buffer
                && (*__errno_location() == 11i32 || *__errno_location() == 4i32))
            {
                break;
            }
        }
        close(file);
        (if -1i32 as (*mut ::std::os::raw::c_void) as (*const u8) == (*map_info).buffer {
            -1i32
        } else {
            0i32
        })
    }
}

unsafe fn mp3dec_close_file(map_info: *mut Mp3decMapInfo) {
    if !(*map_info).buffer.is_null()
        && (-1i32 as (*mut ::std::os::raw::c_void) as (*const u8) != (*map_info).buffer)
    {
        munmap(
            (*map_info).buffer as (*mut ::std::os::raw::c_void),
            (*map_info).size,
        );
    }
    (*map_info).buffer = 0i32 as (*const u8);
    (*map_info).size = 0usize;
}

pub unsafe fn mp3dec_load(
    dec: *mut Mp3Dec,
    file_name: *const u8,
    info: *mut Mp3decFileInfo,
    progress_cb: unsafe fn(*mut ::std::os::raw::c_void, usize, usize, *mut FrameInfo) -> i32,
    user_data: *mut ::std::os::raw::c_void,
) -> i32 {
    let ret: i32;
    let mut map_info: Mp3decMapInfo = ::std::mem::zeroed();
    if {
        ret = mp3dec_open_file(file_name, &mut map_info as (*mut Mp3decMapInfo));
        ret
    } != 0
    {
        ret
    } else {
        mp3dec_load_buf(
            dec,
            map_info.buffer,
            map_info.size,
            info,
            progress_cb,
            user_data,
        );
        mp3dec_close_file(&mut map_info as (*mut Mp3decMapInfo));
        0i32
    }
}

pub unsafe fn mp3dec_iterate(
    file_name: *const u8,
    callback: unsafe fn(*mut ::std::os::raw::c_void, *const u8, i32, usize, *mut FrameInfo) -> i32,
    user_data: *mut ::std::os::raw::c_void,
) -> i32 {
    let ret: i32;
    let mut map_info: Mp3decMapInfo = ::std::mem::zeroed();
    if {
        ret = mp3dec_open_file(file_name, &mut map_info as (*mut Mp3decMapInfo));
        ret
    } != 0
    {
        ret
    } else {
        mp3dec_iterate_buf(map_info.buffer, map_info.size, callback, user_data);
        mp3dec_close_file(&mut map_info as (*mut Mp3decMapInfo));
        0i32
    }
}

pub unsafe fn mp3dec_ex_close(dec: *mut Mp3decEx) {
    if (*dec).is_file != 0 {
        mp3dec_close_file(&mut (*dec).file as (*mut Mp3decMapInfo));
    } else {
        free((*dec).file.buffer as (*mut ::std::os::raw::c_void));
    }
    memset(
        dec as (*mut ::std::os::raw::c_void),
        0i32,
        ::std::mem::size_of::<Mp3decEx>(),
    );
}

pub unsafe fn mp3dec_ex_open(dec: *mut Mp3decEx, file_name: *const u8, seek_method: i32) -> i32 {
    let ret: i32;
    memset(
        dec as (*mut ::std::os::raw::c_void),
        0i32,
        ::std::mem::size_of::<Mp3decEx>(),
    );
    if {
        ret = mp3dec_open_file(file_name, &mut (*dec).file as (*mut Mp3decMapInfo));
        ret
    } != 0
    {
        ret
    } else {
        (*dec).seek_method = seek_method;
        (*dec).is_file = 1i32;

        (*dec).mp3d = Mp3Dec::new();
        0i32
    }
}

use std::fs;
use std::io::Read;
use std::path::PathBuf;
use structopt::StructOpt;

/// Basic command line stuff...
#[derive(Debug, StructOpt)]
#[structopt(name = "minimp3_test")]
struct Opt {
    /// Input file.  TODO: Valid input types?
    #[structopt(parse(from_os_str))]
    input_file: PathBuf,
    /// Output file.  TODO: Valid output types?
    #[structopt(parse(from_os_str))]
    output_file: Option<PathBuf>,
}

fn main() {
    let opt = Opt::from_args();
    println!("{:?}", opt);

    let mp3d = &mut Mp3Dec::new();
    let input_buf = &mut Vec::new();
    {
        let mut f = fs::File::open(opt.input_file).expect("Input file not found");
        f.read_to_end(input_buf).expect("Could not read input file");
    }
    let mut data_start = mp3dec_skip_id3v2_slice(&input_buf);
    let output_buf = &mut Vec::new();
    let mut total_frames = 1;
    let mut total_samples = 0;
    loop {
        let frame_info = &mut FrameInfo::default();
        // Meh, allocation junk could be better
        let buf = &mut Vec::with_capacity(1152);
        buf.resize(1152, 0);
        let samples = mp3dec_decode_frame(mp3d, &mut input_buf[data_start..], buf, frame_info);
        output_buf.extend_from_slice(buf);
        total_samples += samples;
        total_frames += 1;
        println!("Frame info: {:#?}", frame_info);
        println!("Samples: {}, frames: {}", total_samples, total_frames);
        data_start += frame_info.frame_bytes as usize;
        if frame_info.frame_bytes == 0 {
            break;
        }
    }

    /*
    let mut i: i32;
    let data_bytes: i32;
    let mut total_samples: i32 = 0i32;
    let mut maxdiff: i32 = 0i32;
    let mut mse: f64 = 0.0f64;
    let psnr: f64;
    let mut info: Mp3decFileInfo = ::std::mem::zeroed();
    unsafe fn callback(
        _: *mut ::std::os::raw::c_void,
        _: usize,
        _: usize,
        _: *mut FrameInfo,
    ) -> i32 {
        0
    };
    if mp3dec_load(
        &mut mp3d as (*mut Mp3Dec),
        input_file_name,
        &mut info as (*mut Mp3decFileInfo),
        callback,
        0i32 as (*mut ::std::os::raw::c_void),
    ) != 0
    {
        printf((*b"error: file not found or read error\0").as_ptr());
        exit(1i32);
    }
    let buffer: *mut i16 = info.buffer;
    if wave_out != 0 && !file_out.is_null() {
        fwrite(
            wav_header(0i32, 0i32, 0i32, 0i32).as_ptr() as (*const ::std::os::raw::c_void),
            1usize,
            44usize,
            file_out,
        );
    }
    if info.samples != 0 {
        total_samples = (total_samples as (usize)).wrapping_add(info.samples) as (i32);
        if !buf_ref.is_null() {
            let max_samples: i32 = (if (ref_size as (usize)).wrapping_div(2usize) > info.samples {
                info.samples
            } else {
                (ref_size as (usize)).wrapping_div(2usize)
            }) as (i32);
            i = 0i32;
            'loop7: loop {
                if !(i < max_samples) {
                    break;
                }
                let mse_temp: i32 = abs(*buffer.offset(i as (isize)) as (i32)
                    - read16le(
                        &*buf_ref
                            .offset((i as (usize)).wrapping_mul(::std::mem::size_of::<i16>())
                                as (isize)) as (*const u8)
                            as (*const ::std::os::raw::c_void),
                    ) as (i32));
                if mse_temp > maxdiff {
                    maxdiff = mse_temp;
                }
                mse = mse + (mse_temp as (f32) * mse_temp as (f32)) as (f64);
                i = i + 1;
            }
        }
        if !file_out.is_null() {
            fwrite(
                buffer as (*const ::std::os::raw::c_void),
                info.samples,
                ::std::mem::size_of::<i16>(),
                file_out,
            );
        }
        free(buffer as (*mut ::std::os::raw::c_void));
    }
    mse = mse / if total_samples != 0 {
        total_samples
    } else {
        1i32
    } as (f64);
    if 0i32 as (f64) == mse {
        psnr = 99.0f64;
    } else {
        psnr = 10.0f64 * (0x7fffi32 as (f64) * 0x7fffi32 as (f64) / mse);
    }
    printf(
        (*b"rate=%d samples=%d max_diff=%d PSNR=%f\n\0").as_ptr(),
        info.hz,
        total_samples,
        maxdiff,
        psnr,
    );
    if psnr < 96i32 as (f64) {
        printf((*b"PSNR compliance failed\n\0").as_ptr());
        exit(1i32);
    }
    if wave_out != 0 && !file_out.is_null() {
        data_bytes = (ftell(file_out) - 44isize) as (i32);
        rewind(file_out);
        fwrite(
            wav_header(info.hz, info.channels, 16i32, data_bytes).as_ptr()
                as (*const ::std::os::raw::c_void),
            1usize,
            44usize,
            file_out,
        );
    }

*/

    return;
    /*
    use std::os::unix::ffi::OsStringExt;
    let mut argv_storage = ::std::env::args_os()
        .map(|str| {
            let mut vec = str.into_vec();
            vec.push(b'\0');
            vec
        })
        .collect::<Vec<_>>();
    let mut argv = argv_storage
        .iter_mut()
        .map(|vec| vec.as_mut_ptr())
        .chain(Some(::std::ptr::null_mut()))
        .collect::<Vec<_>>();
    let ret = unsafe { _c_main(argv_storage.len() as (i32), argv.as_mut_ptr()) };
    ::std::process::exit(ret);
    */
}

unsafe fn preload(file: *mut IoFile, data_size: *mut i32) -> *mut u8 {
    let data: *mut u8;
    *data_size = 0i32;
    if file.is_null() {
        0i32 as (*mut u8)
    } else if fseek(file, 0isize, 2i32) != 0 {
        0i32 as (*mut u8)
    } else {
        *data_size = ftell(file) as (i32);
        (if *data_size < 0i32 {
            0i32 as (*mut u8)
        } else if fseek(file, 0isize, 0i32) != 0 {
            0i32 as (*mut u8)
        } else {
            data = malloc(*data_size as (usize)) as (*mut u8);
            (if data.is_null() {
                0i32 as (*mut u8)
            } else {
                if fread(
                    data as (*mut ::std::os::raw::c_void),
                    1usize,
                    *data_size as (usize),
                    file,
                ) as (i32)
                    != *data_size
                {
                    exit(1i32);
                }
                data
            })
        })
    }
}

unsafe fn wav_header(hz: i32, ch: i32, bips: i32, data_bytes: i32) -> Vec<u8> {
    let hdr: &mut Vec<u8> =
        &mut Vec::from(&b"RIFFsizeWAVEfmt \x10\0\0\0\x01\0ch_hz_abpsbabsdatasize\0"[..]);
    let avg_bytes_per_sec: usize = (bips * ch * hz >> 3i32) as (usize);
    let block_align: u32 = (bips * ch >> 3i32) as (u32);
    *(hdr.as_mut_ptr().offset(0x4isize) as (*mut ::std::os::raw::c_void) as (*mut i32)) =
        44i32 + data_bytes - 8i32;
    *(hdr.as_mut_ptr().offset(0x14isize) as (*mut ::std::os::raw::c_void) as (*mut i16)) = 1i16;
    *(hdr.as_mut_ptr().offset(0x16isize) as (*mut ::std::os::raw::c_void) as (*mut i16)) =
        ch as (i16);
    *(hdr.as_mut_ptr().offset(0x18isize) as (*mut ::std::os::raw::c_void) as (*mut i32)) = hz;
    *(hdr.as_mut_ptr().offset(0x1cisize) as (*mut ::std::os::raw::c_void) as (*mut i32)) =
        avg_bytes_per_sec as (i32);
    *(hdr.as_mut_ptr().offset(0x20isize) as (*mut ::std::os::raw::c_void) as (*mut i16)) =
        block_align as (i16);
    *(hdr.as_mut_ptr().offset(0x22isize) as (*mut ::std::os::raw::c_void) as (*mut i16)) =
        bips as (i16);
    *(hdr.as_mut_ptr().offset(0x28isize) as (*mut ::std::os::raw::c_void) as (*mut i32)) =
        data_bytes;
    hdr.clone()
}

unsafe fn read16le(p: *const ::std::os::raw::c_void) -> i16 {
    let src: *const u8 = p as (*const u8);
    (*src.offset(0isize) as (i32) << 0i32 | *src.offset(1isize) as (i32) << 8i32) as (i16)
}

unsafe fn decode_file(
    input_file_name: *const u8,
    buf_ref: *const u8,
    ref_size: i32,
    file_out: *mut IoFile,
    wave_out: i32,
) {
    let mut mp3d: Mp3Dec = ::std::mem::zeroed();
    let mut i: i32;
    let data_bytes: i32;
    let mut total_samples: i32 = 0i32;
    let mut maxdiff: i32 = 0i32;
    let mut mse: f64 = 0.0f64;
    let psnr: f64;
    let mut info: Mp3decFileInfo = ::std::mem::zeroed();
    unsafe fn callback(
        _: *mut ::std::os::raw::c_void,
        _: usize,
        _: usize,
        _: *mut FrameInfo,
    ) -> i32 {
        0
    };
    if mp3dec_load(
        &mut mp3d as (*mut Mp3Dec),
        input_file_name,
        &mut info as (*mut Mp3decFileInfo),
        callback,
        0i32 as (*mut ::std::os::raw::c_void),
    ) != 0
    {
        printf((*b"error: file not found or read error\0").as_ptr());
        exit(1i32);
    }
    let buffer: *mut i16 = info.buffer;
    if wave_out != 0 && !file_out.is_null() {
        fwrite(
            wav_header(0i32, 0i32, 0i32, 0i32).as_ptr() as (*const ::std::os::raw::c_void),
            1usize,
            44usize,
            file_out,
        );
    }
    if info.samples != 0 {
        total_samples = (total_samples as (usize)).wrapping_add(info.samples) as (i32);
        if !buf_ref.is_null() {
            let max_samples: i32 = (if (ref_size as (usize)).wrapping_div(2usize) > info.samples {
                info.samples
            } else {
                (ref_size as (usize)).wrapping_div(2usize)
            }) as (i32);
            i = 0i32;
            'loop7: loop {
                if !(i < max_samples) {
                    break;
                }
                let mse_temp: i32 = abs(*buffer.offset(i as (isize)) as (i32)
                    - read16le(
                        &*buf_ref
                            .offset((i as (usize)).wrapping_mul(::std::mem::size_of::<i16>())
                                as (isize)) as (*const u8)
                            as (*const ::std::os::raw::c_void),
                    ) as (i32));
                if mse_temp > maxdiff {
                    maxdiff = mse_temp;
                }
                mse = mse + (mse_temp as (f32) * mse_temp as (f32)) as (f64);
                i = i + 1;
            }
        }
        if !file_out.is_null() {
            fwrite(
                buffer as (*const ::std::os::raw::c_void),
                info.samples,
                ::std::mem::size_of::<i16>(),
                file_out,
            );
        }
        free(buffer as (*mut ::std::os::raw::c_void));
    }
    mse = mse / if total_samples != 0 {
        total_samples
    } else {
        1i32
    } as (f64);
    if 0i32 as (f64) == mse {
        psnr = 99.0f64;
    } else {
        psnr = 10.0f64 * (0x7fffi32 as (f64) * 0x7fffi32 as (f64) / mse);
    }
    printf(
        (*b"rate=%d samples=%d max_diff=%d PSNR=%f\n\0").as_ptr(),
        info.hz,
        total_samples,
        maxdiff,
        psnr,
    );
    if psnr < 96i32 as (f64) {
        printf((*b"PSNR compliance failed\n\0").as_ptr());
        exit(1i32);
    }
    if wave_out != 0 && !file_out.is_null() {
        data_bytes = (ftell(file_out) - 44isize) as (i32);
        rewind(file_out);
        fwrite(
            wav_header(info.hz, info.channels, 16i32, data_bytes).as_ptr()
                as (*const ::std::os::raw::c_void),
            1usize,
            44usize,
            file_out,
        );
    }
}

pub unsafe fn _c_main(argc: i32, argv: *mut *mut u8) -> i32 {
    let mut wave_out: i32 = 0i32;
    let mut ref_size: i32 = 0;
    let ref_file_name: *mut u8 = if argc > 2i32 {
        *argv.offset(2isize)
    } else {
        0i32 as (*mut ::std::os::raw::c_void) as (*mut u8)
    };
    let output_file_name: *mut u8 = if argc > 3i32 {
        *argv.offset(3isize)
    } else {
        0i32 as (*mut ::std::os::raw::c_void) as (*mut u8)
    };
    let mut file_out: *mut IoFile = 0i32 as (*mut ::std::os::raw::c_void) as (*mut IoFile);
    if !output_file_name.is_null() {
        file_out = fopen(output_file_name as (*const u8), (*b"wb\0").as_ptr());
        let ext: *mut u8 = strrchr(output_file_name as (*const u8), b'.' as (i32));
        if !ext.is_null()
            && (strcasecmp(ext.offset(1isize) as (*const u8), (*b"wav\0").as_ptr()) == 0)
        {
            wave_out = 1i32;
        }
    }
    let file_ref: *mut IoFile = if !ref_file_name.is_null() {
        fopen(ref_file_name as (*const u8), (*b"rb\0").as_ptr())
    } else {
        0i32 as (*mut ::std::os::raw::c_void) as (*mut IoFile)
    };
    let buf_ref: *mut u8 = preload(file_ref, &mut ref_size as (*mut i32));
    if !file_ref.is_null() {
        fclose(file_ref);
    }
    let input_file_name: *mut u8 = if argc > 1i32 {
        *argv.offset(1isize)
    } else {
        0i32 as (*mut ::std::os::raw::c_void) as (*mut u8)
    };
    if input_file_name.is_null() {
        printf((*b"error: no file names given\n\0").as_ptr());
        1i32
    } else {
        decode_file(
            input_file_name as (*const u8),
            buf_ref as (*const u8),
            ref_size,
            file_out,
            wave_out,
        );
        if !buf_ref.is_null() {
            free(buf_ref as (*mut ::std::os::raw::c_void));
        }
        if !file_out.is_null() {
            fclose(file_out);
        }
        0i32
    }
}
