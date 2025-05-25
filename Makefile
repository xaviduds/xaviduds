install:
	yay -S --needed --noconfirm \
	nodejs \
	npm

	npm install
	npm install vite
	cargo install bacon mprocs

run:
	mprocs "bacon clippy-all" "bacon . --job run" "npm run dev"

build:
	cargo build --release
