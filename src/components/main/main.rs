use super::stack::stack;
use crate::components::main::projects::projects;
use maud::html;

pub fn comp_main() -> String {
    html!(
    main {
    (stack())(projects())
    }
    )
    .0
}
