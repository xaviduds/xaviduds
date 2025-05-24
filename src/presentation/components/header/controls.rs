use maud::{Markup, html};

use crate::presentation::components::main::stack::tech_areas;

pub fn controls() -> Markup {
    html!(
        section class="filler framed row" {
            @for area in tech_areas() {
                div data-signals=(format!("{{{}: true}}", area.class)) {
                    button data-on-click=(format!("${}=!${}", area.class, area.class)) { (area.name) }
                }
            }
        }
    )
}
