pub mod stack;
pub mod test;

use maud::html;
use stack::stack;
use test::test;

pub fn comp_main() -> String {
    html!(main.filler.column.framed.middle_x.middle_y {
        (stack())

    })
    .0 + &test()
}
