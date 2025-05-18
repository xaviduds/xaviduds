use crate::schema::tech::{Area, Item};
use maud::{Markup, html};

pub fn socials() -> Markup {
    let area = area();

    html! {
        div class="row middle_y middle_x"{
            @for tech in &area.items {
                a href=(tech.link) class="middle_x column" target="_blank" rel="noopener noreferrer" {
                    img class=("almond_size ".to_owned() + tech.class) src=(tech.icon) alt=(tech.name) {}
                }
            }
        }
    }
}

pub fn area() -> Area {
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
    let email = Item::new(
        "email",
        "Email",
        "social/email.svg",
        "mailto:xaviduds@gmail.com",
    );
    let area = Area::new("Socials", vec![github, linkedin, email]);
    area
}
