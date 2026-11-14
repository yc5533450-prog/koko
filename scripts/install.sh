#!/usr/bin/env bash
set -euo pipefail
echo '[1/3] Orijinal CHIMERA indiriliyor'
if [ ! -f chimera-vocal-synth/Cargo.toml ]; then
git clone --depth 1 https://github.com/jamesondh/chimera-vocal-synth.git chimera-vocal-synth
fi
echo '[2/3] Rust + WebAssembly kuruluyor'
if ! command -v rustup >/dev/null 2>&1; then
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --profile minimal --target wasm32-unknown-unknown
else
rustup target add wasm32-unknown-unknown
fi
echo '[3/3] Web uygulamasi bagimliliklari kuruluyor'
cd chimera-vocal-synth
bunx bun@1.4.4 install --frozen-lockfile
