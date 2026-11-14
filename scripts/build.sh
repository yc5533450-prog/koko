#!/usr/bin/env bash
set -euo pipefail
HERE="$(cd "$(dirname "$0")/.." && pwd)"
cd "$HERE"
if [ ! -f chimera-vocal-synth/Cargo.toml ]; then
echo 'Orijinal kaynak bulunamadi. Once scripts/install.sh calistirilmali.' >&2
exit 1
fi
echo 'Turkce yamasi orijinal Rust/Wasm projeye uygulanıyor...'
python3 turkce_yama.py "$HERE/chimera-vocal-synth"
echo 'Gercek CHIMERA Rust/WebAssembly + Vite web sitesi derleniyor...'
export PATH="$HOME/.cargo/bin:$PATH"
cd "$HERE/chimera-vocal-synth"
bun run build
test -f dist/index.html
echo "Hazir: $HERE/chimera-vocal-synth/dist"
