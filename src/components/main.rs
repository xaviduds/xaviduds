use maud::{Markup, html};

use crate::components::{
    education::education, experience::profession, projects::projects, stack::stack,
};

pub fn html_main() -> Markup {
    html!(
        main.row.middle_x.start_y{
            .column.hafuish {
               (profession())
               (education())
            }
            .column.hafuish {
                (projects())
                (stack())
            }
        }
    )
}
