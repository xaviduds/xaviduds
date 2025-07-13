use maud::{Markup, html};

use crate::components::{
    education::education, experience::experience, projects::projects, stack::stack,
};

pub fn html_main() -> Markup {
    html!(
        main.row.middle_x.start_y.g_gap.mini_eca {
            .column.blue.hafuish {
               (experience())
               (education())
            }
            .column.green.hafuish {
                (projects())
                (stack())
            }
        }
        img src="assets/Untitled.jpg" {}
    )
}
