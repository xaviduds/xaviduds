use super::stack::stack;
use crate::presentation::components::main::{experience::experience, projects::projects};
use maud::html;

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
