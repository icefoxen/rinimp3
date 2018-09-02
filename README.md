# rinimp3
An attempt to make a port of [lieff's minimp3](https://github.com/lieff/minimp3) to Rust.  The goal is basically to make the minimp3-rs crate, bindings to the C library, unnecessary.

First step is to just duplicate it as closely as possible, including passing its tests and fuzzing.  Next step would be to smooth its API out and make it more Rusty; https://github.com/germangb/minimp3-rs might be useful for inspiration.

Differences:

 * i16 output only
 * No SIMD
 * No conditional compilation -- basically should operate as if the following flags are defined:
  * `#define MINIMP3_NO_SIMD`
  * `#define MINIMP3_NONSTANDARD_BUT_LOGICAL`???