use crate::schema::tech::{Area, Item};
use maud::{Markup, PreEscaped, html};

pub fn card() -> Markup {
    html!(
        .row.middle_x.middle_y.m_gap {
            (selfie())
            div class="column" {
                .column.m_gap {
                    (title())
                }
            }
            (socials())
        }
    )
}

fn selfie() -> Markup {
    html!(
        img class="selfie" src="assets/social/selfie2.jpg" {}
    )
}

fn title() -> Markup {
    html!(
        div class="column middle_x s_gap" {
            div class="row middle_y m_gap" {
                p class="title" { "Eduardo de Melo Xavier" }
                (age())
            }
            div class="row middle_y m_gap" {
                p class="stripped" { "Software Engineer @ Preto no Branco" }
                (info())
            }
        }
    )
}

fn info() -> Markup {
    html!(
        div class="column" {
            div class="row middle_y" {
                .xs {
                    svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="white" width="100%" height="100%" {
                        path d="M12 2C8.13 2 5 5.13 5 9c0 5.25 7 13 7 13s7-7.75 7-13c0-3.87-3.13-7-7-7zm0 9.5a2.5 2.5 0 1 1 0-5 2.5 2.5 0 0 1 0 5z" {}
                    }
                }
                p class="stripped" { "Porto Alegre, Brazil."}
            }
        }
    )
}

fn age() -> Markup {
    html! {
        div id="age" {
            "Loading age..."
        }
        script {
            (PreEscaped(r#"
                   (function() {
                       const birth = new Date("2000-03-20");
                       const now = new Date();
                       let age = now.getFullYear() - birth.getFullYear();
                       const m = now.getMonth() - birth.getMonth();
                       if (m < 0 || (m === 0 && now.getDate() < birth.getDate())) {
                           age--;
                       }
                       document.getElementById("age").textContent = age + " years old";
                   })();
               "#))
        }
    }
}

pub fn socials() -> Markup {
    html! {
        .column{
            @for tech in &social_area().items {
                a href=(tech.link) target="_blank" rel="noopener noreferrer" {
                    img class=(tech.classes.join(" ") + " s") src=(tech.icon) alt=(tech.name) {}
                }
            }
        }
    }
}

pub fn social_area() -> Area {
    let github = Item {
        name: "GitHub",
        icon: "./assets/tech/github.svg",
        link: "https://github.com/xaviduds",
        classes: vec!["github", "socials"],
    };

    let linkedin = Item {
        name: "Linkedin",
        icon: "./assets/social/linkedin.svg",
        link: "https://www.linkedin.com/in/xaviduds/",
        classes: vec!["linkedin", "socials"],
    };

    let email = Item {
        name: "Email",
        icon: "./assets/social/email.svg",
        link: "mailto:xaviduds@gmail.com",
        classes: vec!["email", "socials"],
    };

    Area {
        name: "Socials",
        items: vec![github, linkedin, email],
        class: "socials",
    }
}
