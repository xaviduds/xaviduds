build:
	cargo build \
	--target wasm32-unknown-unknown

	wasm-bindgen \
	--out-name wasm_example \
    --out-dir stack_wasm \
    --target web target/wasm32-unknown-unknown/debug/portfolio.wasm

on:
	mprocs "bacon . --job run" "npm run dev"

install:
	npm install vite
	yay -S mprocs --no-confirm --needed
