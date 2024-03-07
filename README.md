# 2048


wasm-bindgen
```bash
cargo build --release --target wasm32-unknown-unknown
wasm-bindgen --no-typescript --target web \
    --out-dir ./out/ \
    --out-name "bevy-2048" \
    ./target/wasm32-unknown-unknown/release/bevy-2048.wasm
```


Use the wasm-opt Tool
```bash
# Optimize for size.
wasm-opt -Os -o output.wasm input.wasm

# Optimize aggressively for size.
wasm-opt -Oz -o target/wasm32-unknown-unknown/release/bevy-2048-opt.wasm target/wasm32-unknown-unknown/release/bevy-2048.wasm

# Optimize for speed.
wasm-opt -O -o output.wasm input.wasm

# Optimize aggressively for speed.
wasm-opt -O3 -o output.wasm input.wasm
```