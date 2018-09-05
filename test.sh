#!/bin/sh
for v in vectors/*.pcm vectors/*.bit; do
    cargo run --bin minimp3_test -- $v
done;
