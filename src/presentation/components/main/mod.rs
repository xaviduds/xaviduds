pub mod experience;
pub mod projects;
pub mod stack;

use crate::presentation::components::main::{experience::experience, projects::projects};
use maud::html;
use stack::stack;

pub fn comp_main() -> String {
    html!(
    main class="row" {
        div class="column" {
            (experience())
            (projects())
        }
        (stack())
    }
    )
    .0
}
