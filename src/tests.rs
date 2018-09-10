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
#[allow(non_snake_case)]
fn test_reference_vector_ILL2_center() {
    let test_vector = include_bytes!("../vectors/ILL2_center2.bit");
    test_vs_minimp3(test_vector);
}
#[test]
#[allow(non_snake_case)]
fn test_reference_vector_ILL2_dual() {
    let test_vector = include_bytes!("../vectors/ILL2_dual.bit");
    test_vs_minimp3(test_vector);
}
#[test]
#[allow(non_snake_case)]
fn test_reference_vector_ILL2_dynx22() {
    let test_vector = include_bytes!("../vectors/ILL2_dynx22.bit");
    test_vs_minimp3(test_vector);
}
#[test]
#[allow(non_snake_case)]
fn test_reference_vector_ILL2_dynx31() {
    let test_vector = include_bytes!("../vectors/ILL2_dynx31.bit");
    test_vs_minimp3(test_vector);
}
#[test]
#[allow(non_snake_case)]
fn test_reference_vector_ILL2_dynx32() {
    let test_vector = include_bytes!("../vectors/ILL2_dynx32.bit");
    test_vs_minimp3(test_vector);
}
#[test]
#[allow(non_snake_case)]
fn test_reference_vector_ILL2_ext_switching() {
    let test_vector = include_bytes!("../vectors/ILL2_ext_switching.bit");
    test_vs_minimp3(test_vector);
}
#[test]
#[allow(non_snake_case)]
fn test_reference_vector_ILL2_layer1() {
    let test_vector = include_bytes!("../vectors/ILL2_layer1.bit");
    test_vs_minimp3(test_vector);
}
#[test]
#[allow(non_snake_case)]
fn test_reference_vector_ILL2_layer3() {
    let test_vector = include_bytes!("../vectors/ILL2_layer3.bit");
    test_vs_minimp3(test_vector);
}
#[test]
#[allow(non_snake_case)]
fn test_reference_vector_ILL2_mono() {
    let test_vector = include_bytes!("../vectors/ILL2_mono.bit");
    test_vs_minimp3(test_vector);
}
#[test]
#[allow(non_snake_case)]
fn test_reference_vector_ILL2_multilingual() {
    let test_vector = include_bytes!("../vectors/ILL2_multilingual.bit");
    test_vs_minimp3(test_vector);
}
#[test]
#[allow(non_snake_case)]
fn test_reference_vector_ILL2_overalloc1() {
    let test_vector = include_bytes!("../vectors/ILL2_overalloc1.bit");
    test_vs_minimp3(test_vector);
}
#[test]
#[allow(non_snake_case)]
fn test_reference_vector_ILL2_overalloc2() {
    let test_vector = include_bytes!("../vectors/ILL2_overalloc2.bit");
    test_vs_minimp3(test_vector);
}
#[test]
#[allow(non_snake_case)]
fn test_reference_vector_ILL2_prediction() {
    let test_vector = include_bytes!("../vectors/ILL2_prediction.bit");
    test_vs_minimp3(test_vector);
}
#[test]
#[allow(non_snake_case)]
fn test_reference_vector_ILL2_samples() {
    let test_vector = include_bytes!("../vectors/ILL2_samples.bit");
    test_vs_minimp3(test_vector);
}
#[test]
#[allow(non_snake_case)]
fn test_reference_vector_ILL2_scf63() {
    let test_vector = include_bytes!("../vectors/ILL2_scf63.bit");
    test_vs_minimp3(test_vector);
}
#[test]
#[allow(non_snake_case)]
fn test_reference_vector_ILL2_tca21() {
    let test_vector = include_bytes!("../vectors/ILL2_tca21.bit");
    test_vs_minimp3(test_vector);
}
#[test]
#[allow(non_snake_case)]
fn test_reference_vector_ILL2_tca30() {
    let test_vector = include_bytes!("../vectors/ILL2_tca30.bit");
    test_vs_minimp3(test_vector);
}
#[test]
#[allow(non_snake_case)]
fn test_reference_vector_ILL2_tca30_PC() {
    let test_vector = include_bytes!("../vectors/ILL2_tca30_PC.bit");
    test_vs_minimp3(test_vector);
}
#[test]
#[allow(non_snake_case)]
fn test_reference_vector_ILL2_tca31_mtx0() {
    let test_vector = include_bytes!("../vectors/ILL2_tca31_mtx0.bit");
    test_vs_minimp3(test_vector);
}
#[test]
#[allow(non_snake_case)]
fn test_reference_vector_ILL2_tca31_mtx2() {
    let test_vector = include_bytes!("../vectors/ILL2_tca31_mtx2.bit");
    test_vs_minimp3(test_vector);
}
#[test]
#[allow(non_snake_case)]
fn test_reference_vector_ILL2_tca31_PC() {
    let test_vector = include_bytes!("../vectors/ILL2_tca31_PC.bit");
    test_vs_minimp3(test_vector);
}
#[test]
#[allow(non_snake_case)]
fn test_reference_vector_ILL2_tca32_PC() {
    let test_vector = include_bytes!("../vectors/ILL2_tca32_PC.bit");
    test_vs_minimp3(test_vector);
}
#[test]
#[allow(non_snake_case)]
fn test_reference_vector_ILL2_wrongcrc() {
    let test_vector = include_bytes!("../vectors/ILL2_wrongcrc.bit");
    test_vs_minimp3(test_vector);
}
#[test]
#[allow(non_snake_case)]
fn test_reference_vector_ILL4_ext_id1() {
    let test_vector = include_bytes!("../vectors/ILL4_ext_id1.bit");
    test_vs_minimp3(test_vector);
}
#[test]
#[allow(non_snake_case)]
fn test_reference_vector_ILL4_sync() {
    let test_vector = include_bytes!("../vectors/ILL4_sync.bit");
    test_vs_minimp3(test_vector);
}
#[test]
#[allow(non_snake_case)]
fn test_reference_vector_ILL4_wrongcrc() {
    let test_vector = include_bytes!("../vectors/ILL4_wrongcrc.bit");
    test_vs_minimp3(test_vector);
}
#[test]
#[allow(non_snake_case)]
fn test_reference_vector_ILL4_wrong_length1() {
    let test_vector = include_bytes!("../vectors/ILL4_wrong_length1.bit");
    test_vs_minimp3(test_vector);
}
#[test]
#[allow(non_snake_case)]
fn test_reference_vector_ILL4_wrong_length2() {
    let test_vector = include_bytes!("../vectors/ILL4_wrong_length2.bit");
    test_vs_minimp3(test_vector);
}
#[test]
fn test_reference_vector_l1_fl1() {
    let test_vector = include_bytes!("../vectors/l1-fl1.bit");
    test_vs_minimp3(test_vector);
}
#[test]
fn test_reference_vector_l1_fl2() {
    let test_vector = include_bytes!("../vectors/l1-fl2.bit");
    test_vs_minimp3(test_vector);
}
#[test]
fn test_reference_vector_l1_fl3() {
    let test_vector = include_bytes!("../vectors/l1-fl3.bit");
    test_vs_minimp3(test_vector);
}
#[test]
fn test_reference_vector_l1_fl4() {
    let test_vector = include_bytes!("../vectors/l1-fl4.bit");
    test_vs_minimp3(test_vector);
}
#[test]
fn test_reference_vector_l1_fl5() {
    let test_vector = include_bytes!("../vectors/l1-fl5.bit");
    test_vs_minimp3(test_vector);
}
#[test]
fn test_reference_vector_l1_fl6() {
    let test_vector = include_bytes!("../vectors/l1-fl6.bit");
    test_vs_minimp3(test_vector);
}
#[test]
fn test_reference_vector_l1_fl7() {
    let test_vector = include_bytes!("../vectors/l1-fl7.bit");
    test_vs_minimp3(test_vector);
}
#[test]
fn test_reference_vector_l1_fl8() {
    let test_vector = include_bytes!("../vectors/l1-fl8.bit");
    test_vs_minimp3(test_vector);
}
#[test]
fn test_reference_vector_l1_fl10() {
    let test_vector = include_bytes!("../vectors/l2-fl10.bit");
    test_vs_minimp3(test_vector);
}
#[test]
fn test_reference_vector_l1_fl11() {
    let test_vector = include_bytes!("../vectors/l2-fl11.bit");
    test_vs_minimp3(test_vector);
}
#[test]
fn test_reference_vector_l1_fl12() {
    let test_vector = include_bytes!("../vectors/l2-fl12.bit");
    test_vs_minimp3(test_vector);
}
#[test]
fn test_reference_vector_l1_fl13() {
    let test_vector = include_bytes!("../vectors/l2-fl13.bit");
    test_vs_minimp3(test_vector);
}
#[test]
fn test_reference_vector_l1_fl14() {
    let test_vector = include_bytes!("../vectors/l2-fl14.bit");
    test_vs_minimp3(test_vector);
}
#[test]
fn test_reference_vector_l1_fl15() {
    let test_vector = include_bytes!("../vectors/l2-fl15.bit");
    test_vs_minimp3(test_vector);
}
#[test]
fn test_reference_vector_l1_fl16() {
    let test_vector = include_bytes!("../vectors/l2-fl16.bit");
    test_vs_minimp3(test_vector);
}
#[test]
fn test_reference_vector_l2_nonstandard_fl1_fl2_ff() {
    let test_vector = include_bytes!("../vectors/l2-nonstandard-fl1_fl2_ff.bit");
    test_vs_minimp3(test_vector);
}
#[test]
fn test_reference_vector_l2_nonstandard_free_format() {
    let test_vector = include_bytes!("../vectors/l2-nonstandard-free_format.bit");
    test_vs_minimp3(test_vector);
}
#[test]
fn test_reference_vector_l2_nonstandard_test32_size() {
    let test_vector = include_bytes!("../vectors/l2-nonstandard-test32-size.bit");
    test_vs_minimp3(test_vector);
}
#[test]
fn test_reference_vector_l2_test32() {
    let test_vector = include_bytes!("../vectors/l2-test32.bit");
    test_vs_minimp3(test_vector);
}
#[test]
fn test_reference_vector_l3_compl() {
    let test_vector = include_bytes!("../vectors/l3-compl.bit");
    test_vs_minimp3(test_vector);
}
#[test]
fn test_reference_vector_l3_he_32khz() {
    let test_vector = include_bytes!("../vectors/l3-he_32khz.bit");
    test_vs_minimp3(test_vector);
}
#[test]
fn test_reference_vector_l3_he_44khz() {
    let test_vector = include_bytes!("../vectors/l3-he_44khz.bit");
    test_vs_minimp3(test_vector);
}
#[test]
fn test_reference_vector_l3_he_48khz() {
    let test_vector = include_bytes!("../vectors/l3-he_48khz.bit");
    test_vs_minimp3(test_vector);
}
#[test]
fn test_reference_vector_l3_hecommon() {
    let test_vector = include_bytes!("../vectors/l3-hecommon.bit");
    test_vs_minimp3(test_vector);
}
#[test]
fn test_reference_vector_l3_he_free() {
    let test_vector = include_bytes!("../vectors/l3-he_free.bit");
    test_vs_minimp3(test_vector);
}
#[test]
fn test_reference_vector_l3_he_mode() {
    let test_vector = include_bytes!("../vectors/l3-he_mode.bit");
    test_vs_minimp3(test_vector);
}
#[test]
fn test_reference_vector_l3_id3v2() {
    let test_vector = include_bytes!("../vectors/l3-id3v2.bit");
    test_vs_minimp3(test_vector);
}
#[test]
fn test_reference_vector_l3_nonstandard_big_iscf() {
    let test_vector = include_bytes!("../vectors/l3-nonstandard-big-iscf.bit");
    test_vs_minimp3(test_vector);
}
#[test]
fn test_reference_vector_l3_nonstandard_compl_sideinfo_bigvalues() {
    let test_vector = include_bytes!("../vectors/l3-nonstandard-compl-sideinfo-bigvalues.bit");
    test_vs_minimp3(test_vector);
}
#[test]
fn test_reference_vector_l3_nonstandard_compl_sideinfo_blocktype() {
    let test_vector = include_bytes!("../vectors/l3-nonstandard-compl-sideinfo-blocktype.bit");
    test_vs_minimp3(test_vector);
}
#[test]
fn test_reference_vector_l3_nonstandard_compl_sideinfo_size() {
    let test_vector = include_bytes!("../vectors/l3-nonstandard-compl-sideinfo-size.bit");
    test_vs_minimp3(test_vector);
}
#[test]
fn test_reference_vector_l3_nonstandard_sideinfo_size() {
    let test_vector = include_bytes!("../vectors/l3-nonstandard-sideinfo-size.bit");
    test_vs_minimp3(test_vector);
}
#[test]
fn test_reference_vector_l3_si() {
    let test_vector = include_bytes!("../vectors/l3-si.bit");
    test_vs_minimp3(test_vector);
}
#[test]
fn test_reference_vector_l3_si_block() {
    let test_vector = include_bytes!("../vectors/l3-si_block.bit");
    test_vs_minimp3(test_vector);
}
#[test]
fn test_reference_vector_l3_si_huff() {
    let test_vector = include_bytes!("../vectors/l3-si_huff.bit");
    test_vs_minimp3(test_vector);
}
#[test]
fn test_reference_vector_l3_sin1k0db() {
    let test_vector = include_bytes!("../vectors/l3-sin1k0db.bit");
    test_vs_minimp3(test_vector);
}
#[test]
fn test_reference_vector_l3_test45() {
    let test_vector = include_bytes!("../vectors/l3-test45.bit");
    test_vs_minimp3(test_vector);
}
#[test]
fn test_reference_vector_l3_test46() {
    let test_vector = include_bytes!("../vectors/l3-test46.bit");
    test_vs_minimp3(test_vector);
}
#[test]
#[allow(non_snake_case)]
fn test_reference_vector_M2L3_bitrate_16_all() {
    let test_vector = include_bytes!("../vectors/M2L3_bitrate_16_all.bit");
    test_vs_minimp3(test_vector);
}
#[test]
#[allow(non_snake_case)]
fn test_reference_vector_M2L3_bitrate_22_all() {
    let test_vector = include_bytes!("../vectors/M2L3_bitrate_22_all.bit");
    test_vs_minimp3(test_vector);
}
#[test]
#[allow(non_snake_case)]
fn test_reference_vector_M2L3_bitrate_24_all() {
    let test_vector = include_bytes!("../vectors/M2L3_bitrate_24_all.bit");
    test_vs_minimp3(test_vector);
}
#[test]
#[allow(non_snake_case)]
fn test_reference_vector_M2L3_compl24() {
    let test_vector = include_bytes!("../vectors/M2L3_compl24.bit");
    test_vs_minimp3(test_vector);
}
#[test]
#[allow(non_snake_case)]
fn test_reference_vector_M2L3_noise() {
    let test_vector = include_bytes!("../vectors/M2L3_noise.bit");
    test_vs_minimp3(test_vector);
}
