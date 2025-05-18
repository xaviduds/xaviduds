use super::framework::datastar;
use crate::presentation::components::{header::header::header, main::main::comp_main};

pub fn final_html() -> String {
    r#"<!DOCTYPE html>
    <html lang="en">
      <head>
        <meta charset="UTF-8">
        <meta name="viewport" content="width=device-width, initial-scale=1.0">
        <meta http-equiv="X-UA-Compatible" content="ie=edge">
        <title>HTML 5 Boilerplate</title>
        <link rel="stylesheet" href="style.css">"#
        .to_string()
        + datastar()
        + "</head>"
        + &body()
}

fn body() -> String {
    "<body>".to_string() + &header() + &comp_main() + "</body>"
}
