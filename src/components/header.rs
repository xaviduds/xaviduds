use maud::{Markup, PreEscaped, html};

use crate::components::stack::{Area, Item};

pub fn html_header() -> Markup {
    html!(
        header {
            .row.middle_x.middle_y.m_gap {
                (selfie())
                .column {
                    .column.m_gap {
                        (title())
                    }
                }
                (socials())
            }
            p {"Technology professional focused on the Backend challenges of Software Engineering.
                I feel most motivated when working with necessarily complex systems, Rust codebases and robust desktop applications.
            "}
        }
    )
}

fn selfie() -> Markup {
    html!(
        img.selfie src="assets/selfie.jpg" {}
    )
}

fn title() -> Markup {
    html!(
        .column.middle_x.s_gap {
            .row.middle_y.m_gap {
                p.title { "Eduardo de Melo Xavier" }
                (age())
            }
            .row.middle_y.m_gap {
                p.stripped { "Software Engineer @ Preto no Branco" }
                (info())
            }
        }
    )
}

fn info() -> Markup {
    html!(
        .column {
            .row.middle_y {
                .xs {
                    svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="white" width="100%" height="100%" {
                        path d="M12 2C8.13 2 5 5.13 5 9c0 5.25 7 13 7 13s7-7.75 7-13c0-3.87-3.13-7-7-7zm0 9.5a2.5 2.5 0 1 1 0-5 2.5 2.5 0 0 1 0 5z" {}
                    }
                }
                p.stripped { "Porto Alegre, Brazil." }
            }
        }
    )
}

fn age() -> Markup {
    html! {
        #age {
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
        .column {
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
        icon: "./assets/github.svg",
        link: "https://github.com/xaviduds",
        classes: vec!["github", "socials"],
    };

    let linkedin = Item {
        name: "Linkedin",
        icon: "./assets/linkedin.svg",
        link: "https://www.linkedin.com/in/xaviduds/",
        classes: vec!["linkedin", "socials"],
    };

    let email = Item {
        name: "Email",
        icon: "./assets/email.svg",
        link: "mailto:xaviduds@gmail.com",
        classes: vec!["email", "socials"],
    };

    Area {
        name: "Socials",
        items: vec![github, linkedin, email],
        class: "socials",
    }
}
