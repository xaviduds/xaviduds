use crate::components::{footer::footer, header::header, main::main::comp_main};

pub fn body() -> String {
    "<body>".to_string() + &header() + &comp_main() + &footer() + "</body>"
}
