use crate::schema::tech::{Area, Item};
use maud::{Markup, html};

pub fn socials() -> Markup {
    let area = area();

    html! {
        div class="row"{
            @for tech in &area.items {
                img href=(tech.link) target="_blank" rel="noopener noreferrer"
                    class=("almond_size ".to_owned() + tech.class)
                    src=(tech.icon) alt=(tech.name) {}
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
