use maud::{Markup, html};

use crate::presentation::components::main::stack::tech_areas;

pub fn controls() -> Markup {
    html!(
        section class="filler framed row middle_y middle_x dark_blue top mini_eca" {
            @for area in tech_areas() {
                div data-signals=(format!("{{{}: true}}", area.class)) {
                    button data-on-click=(format!("${}=!${}", area.class, area.class)) data-class=(format!("{{'active_jumpy_button': ${}, 'inactive_jumpy_button': !${}}}", area.class, area.class)) { (area.name) }
                }
            }
        }
    )
}
