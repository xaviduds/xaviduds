use general::html::html;
use std::fs;

mod components;
mod general;

fn main() {
    fs::write("index.html", html()).unwrap();
}
