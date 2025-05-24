pub mod stack;

use maud::html;
use stack::stack;

pub fn comp_main() -> String {
    html!(
        main class="row" {
            (stack())
        }
    )
    .0
}
