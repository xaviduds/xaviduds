use super::{body::body, datastar::general_datastar};

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
        + general_datastar()
        + "</head>"
        + &body()
}
