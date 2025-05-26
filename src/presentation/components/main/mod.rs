pub mod stack;
pub mod test;

use maud::html;
use stack::stack;
use test::test;

pub fn comp_main() -> String {
    html!(main.column{
        (stack())

    })
    .0 + &test()
}
