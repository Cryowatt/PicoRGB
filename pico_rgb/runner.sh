#!/bin/sh
mkdir -p ../target/publish/
elf2uf2-rs $1 ../target/publish/pico_rgb.uf2