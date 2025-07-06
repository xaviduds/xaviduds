install:
    cargo install bacon mprocs --locked
    yay -S --needed --noconfirm nodejs npm
    npm init -y
    npm install vite

run: install
    mprocs "bacon clippy-all" "bacon . --job run" "npx vite"
