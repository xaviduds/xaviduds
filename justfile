install:
    cargo install bacon mprocs dioxus-cli --locked

run: install
    mprocs "bacon clippy-all" "dx serve --platform desktop"
