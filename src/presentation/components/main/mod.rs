pub mod stack;

use maud::html;
use stack::stack;

pub fn comp_main() -> String {
    html!(
        main class="filler column framed middle_x middle_y" {
            (stack())
        }
    )
    .0
}
