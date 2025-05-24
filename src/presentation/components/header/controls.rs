use maud::{Markup, html};

use crate::presentation::components::main::stack::tech_areas;

pub fn controls() -> Markup {
    html!(
        section class="filler framed" {
            @for area in tech_areas() {
                button class=(area.classes.join(" ")) { (area.name) }
            }
        }
    )
}
