install:
    cargo install bacon mprocs --locked
    yay -S --needed --noconfirm nodejs npm

run: install
    mprocs "bacon clippy-all" "bacon . --job run" "npx vite"
    # "dev-serve . --reload --port 8080"
