use general::html::final_html;
use std::fs;

mod components;
mod general;
mod schema;

fn main() {
    fs::write("index.html", final_html()).unwrap();
}
