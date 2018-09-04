#!/bin/sh
for v in vectors/*; do
    cargo run --release --bin minimp3_test -- $v
done;
