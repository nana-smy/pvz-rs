#!/usr/bin/env bash
# 构建 GitHub Pages 静态产物到 dist/
set -euo pipefail
cd "$(dirname "$0")"

cargo build --release --target wasm32-unknown-unknown

rm -rf dist && mkdir -p dist
wasm-bindgen target/wasm32-unknown-unknown/release/pvz-rs.wasm \
  --target web --out-dir dist --out-name pvz_rs --no-typescript

cp web/index.html dist/index.html
touch dist/.nojekyll
echo "built -> dist/  (本地预览: python3 -m http.server 8765 --directory dist)"
