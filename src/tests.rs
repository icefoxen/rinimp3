use super::*;

extern crate minimp3;

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

    let data = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let bs = &mut Bs::new(&data, 0);
    // I'm going to crudely assume 0x8FFF is
    // Big Enough, I suppose.
    for i in 0..0x8FFF {
        let orig = unsafe { get_bits_corroded(bs, i as i32) };
        let rewritten = get_bits(bs, i);
        assert_eq!(orig, rewritten);
    }
}

use std::io::Cursor;

fn test_vs_minimp3(test_vector: &[u8]) {
    let reader = Cursor::new(&test_vector[..]);
    let reference_decoder = &mut minimp3::Decoder::new(reader);

    // Meh, allocation junk could be better
    let buf = &mut Vec::with_capacity(1152);
    let mp3d = &mut Mp3Dec::new();
    let mut data_start = mp3dec_skip_id3v2_slice(&test_vector[..]);
    let output_buf = &mut Vec::new();
    let mut total_frames = 1;
    let mut total_samples = 0;

    loop {
        let ref_frame = reference_decoder.next_frame();

        let rini_frame_info = &mut FrameInfo::default();
        buf.clear();
        buf.resize(1152, 0);
        let samples = mp3dec_decode_frame(mp3d, &test_vector[data_start..], buf, rini_frame_info);
        output_buf.extend_from_slice(buf);
        total_samples += samples;
        total_frames += 1;
        println!("Frame info: {:#?}", rini_frame_info);
        println!("Samples: {}, frames: {}", total_samples, total_frames);
        data_start += rini_frame_info.frame_bytes as usize;

        if let Ok(ref frame) = ref_frame {
            assert_eq!(frame.sample_rate, rini_frame_info.hz);
            assert_eq!(frame.channels, rini_frame_info.channels as usize);
            assert_eq!(frame.layer, rini_frame_info.layer as usize);
            assert_eq!(frame.bitrate, rini_frame_info.bitrate_kbps);
            assert_eq!(frame.data.len() * 2, rini_frame_info.frame_bytes as usize);
            // TODO: Actually check data :|
            // minimp3 has a Vec<i16> in the frame,
            // we have buf which is a Vec<u8>
        }

        // Both our decoders agree we are done.
        if rini_frame_info.frame_bytes == 0 && ref_frame.is_err() {
            break;
        }
    }
}

#[test]
fn test_reference_vectors() {
    let test_vector = include_bytes!("../vectors/ILL2_center2.bit");
    test_vs_minimp3(test_vector);
}
