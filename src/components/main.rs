use maud::{Markup, html};

use crate::components::{
    education::education, profession::profession, projects::projects, stack::stack,
};

pub fn html_main() -> Markup {
    html!(
        main.row.middle_x.start_y.s_gap {
            .column.big_spoon.s_gap.fence-col {
                (profession())
                (education())
            }
            .column.little_spoon.s_gap.fence-col {
                (stack())
                (projects())
            }
        }
    )
}
