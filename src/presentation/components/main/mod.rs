pub mod education;
pub mod experience;
pub mod projects;
pub mod stack;

use crate::presentation::components::main::{
    education::education, experience::experience, projects::projects, stack::stack,
};
use maud::html;

pub fn comp_main() -> String {
    html!(
        main.row.middle_x.start_y.g_gap.mini_eca {
            .column {
               (experience())
               (education())
            }
            .column {
                (projects())
                (stack())
            }
        }
    )
    .0
}
