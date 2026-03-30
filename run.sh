#!/usr/bin/env bash
set -e

clear
echo "[MSG] Prefer using cargo instead of scripts."

mkdir out/ || true
cd src/
rustc main.rs

mv main ../out/
cd ..
./out/main

rm -rf out/
