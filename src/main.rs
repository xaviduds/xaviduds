mod components;

use crate::components::{header::html_header, main::html_main};
use maud::{DOCTYPE, html};
use std::fs::write;

fn main() {
    write(
        "index.html",
        html! {
            (DOCTYPE)
            html lang="en" {
               head {
                    meta charset="UTF-8";
                    meta name="viewport" content="width=device-width, initial-scale=1.0";
                    meta http-equiv="X-UA-Compatible" content="ie=edge";
                    link rel="icon" r#type="image/x-icon" href="assets/favicon.ico";
                    title { "Eduardo Xavier"}
                    link rel="stylesheet" href="style.css";
               }
                body.column m_gap middle_y {
                    (html_header())
                    (html_main())
                }
            }
        }
        .0,
    )
    .unwrap();
}
