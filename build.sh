#!/usr/bin/env bash
# 构建 GitHub Pages 静态产物到 dist/
set -euo pipefail
cd "$(dirname "$0")"

cargo build --release --target wasm32-unknown-unknown

rm -rf dist && mkdir -p dist
wasm-bindgen target/wasm32-unknown-unknown/release/pvz-rs.wasm \
  --target web --out-dir dist --out-name pvz_rs --no-typescript

cp web/index.html dist/index.html
tar --format=ustar -cf dist/assets.tar -C assets .
# 每次构建写入独立版本号，避免浏览器/CDN 继续使用旧植物与僵尸贴图
ASSET_VERSION="$(git rev-parse --short HEAD 2>/dev/null || date +%Y%m%d%H%M%S)"
sed -i "s/__ASSET_VERSION__/${ASSET_VERSION}/g" dist/index.html
cp -r assets dist/assets
touch dist/.nojekyll
echo "built -> dist/  (本地预览: python3 -m http.server 8765 --directory dist)"
