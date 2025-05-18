use maud::html;

use crate::schema::tech::{Area, Item};

pub fn footer() -> String {
    let area = area();

    html! {
        footer class="row middle_y middle_x"{
            @for tech in &area.items {
                a href=(tech.link) class="middle_x column" target="_blank" rel="noopener noreferrer" {
                    img class=("logo ".to_owned() + tech.class) src=(tech.icon) alt=(tech.name) {}
                }
            }
        }
    }
    .0
}

fn area() -> Area {
    let github = Item::new(
        "github",
        "GitHub",
        "social/github.svg",
        "https://github.com/xaviduds",
    );
    let linkedin = Item::new(
        "linkedin",
        "Linkedin",
        "social/linkedin.svg",
        "https://www.linkedin.com/in/xaviduds/",
    );
    let area = Area::new("Socials", vec![github, linkedin]);
    area
}
