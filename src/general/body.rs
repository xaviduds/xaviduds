use crate::components::{footer::footer, header::header, main::main::comp_main};

use super::datastar::general_datastar;

pub fn body() -> String {
    "<body>".to_string() + header() + comp_main() + footer() + general_datastar() + "</body>"
}
