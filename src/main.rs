use std::fs;

use general::html::final_html;

mod components;
mod general;

fn main() {
    fs::write("index.html", final_html()).unwrap();
}
