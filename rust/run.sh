#!/bin/sh

cargo build --release && time ./target/release/imgresize /home/hacklog/Videos/test/thumb/163P 128

