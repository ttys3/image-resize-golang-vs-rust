#!/bin/sh

cargo build --release && time target/release/imgresize /home/hacklog/Videos/壁纸wallpaper/ 128 128

