# 2048


## WASM运行
```bash
rustup target install wasm32-unknown-unknown
cargo install wasm-server-runner
cargo run --target wasm32-unknown-unknown
```

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
wasm-opt -Os -o out/bevy-2048_bg.wasm out/bevy-2048_bg.wasm

# Optimize aggressively for size.
wasm-opt -Oz -o out/bevy-2048_bg.wasm out/bevy-2048_bg.wasm

# Optimize for speed.
wasm-opt -O -o out/bevy-2048_bg.wasm out/bevy-2048_bg.wasm

# Optimize aggressively for speed.
wasm-opt -O3 -o out/bevy-2048_bg.wasm out/bevy-2048_bg.wasm
```

Run on browser
```bash
npx serve .
```