pub mod experience;
pub mod projects;
pub mod stack;
pub mod test;

use experience::experience;
use maud::html;
use projects::projects;
use stack::stack;
use test::test;

pub fn comp_main() -> String {
    html!(main.column.middle_y{
        (experience())
        (projects())
        (stack())

    })
    .0 + &test()
}
