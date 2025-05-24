use crate::schema::tech::{Area, Item};
use maud::{Markup, html};

pub fn socials() -> Markup {
    html! {
        div class="row float" {
            @for tech in &area().items {
                a href=(tech.link) target="_blank" rel="noopener noreferrer" {
                    img class=(tech.classes.join(" ") + " almond_size") src=(tech.icon) alt=(tech.name) {}
                }
            }
        }
    }
}

pub fn area() -> Area {
    let github = Item {
        name: "GitHub",
        icon: "./assets/social/github.svg".into(),
        link: "https://github.com/xaviduds",
        classes: vec!["github", "socials"],
    };

    let linkedin = Item {
        name: "Linkedin",
        icon: "./assets/social/linkedin.svg".into(),
        link: "https://www.linkedin.com/in/xaviduds/",
        classes: vec!["linkedin", "socials"],
    };

    let email = Item {
        name: "Email",
        icon: "./assets/social/email.svg".into(),
        link: "mailto:xaviduds@gmail.com",
        classes: vec!["email", "socials"],
    };

    Area {
        name: "Socials",
        items: vec![github, linkedin, email],
    }
}
