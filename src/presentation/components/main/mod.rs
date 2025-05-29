pub mod projects;
pub mod stack;
pub mod test;

use maud::html;
use projects::projects;
use stack::stack;
use test::test;

pub fn comp_main() -> String {
    html!(
        p.middle_y {"The magic below is alive and I cannot contain it. Maybe you can..."}
        main #main.row.middle_x.start_y.g_gap.mini_eca
            data-signals="{orderMain: ''}"
            data-on-reordered="$orderMain= event.detail.orderMain"
        {
            (projects())
            (stack())
            script type="module" src="./js/sortableMain.js" {}
        }
    )
    .0 + &test()
}
