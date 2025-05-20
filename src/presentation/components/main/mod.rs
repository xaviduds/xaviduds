pub mod projects;
pub mod stack;

use crate::presentation::components::main::projects::projects;
use maud::html;
use stack::stack;

pub fn comp_main() -> String {
    html!(
        main class="row" {
            (projects())
            (stack())
        }
    )
    .0
}
