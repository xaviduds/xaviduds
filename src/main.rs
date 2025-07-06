use presentation::general::html::final_html;
use std::fs;

mod presentation;
mod schema;

fn main() {
    fs::write("index.html", final_html()).unwrap();
}
