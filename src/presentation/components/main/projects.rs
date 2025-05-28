use crate::schema::{project::Project, tech::Item};
use maud::{Markup, PreEscaped, html};

pub fn projects() -> Markup {
    let projects = items();

    html!(
        .row.middle_x.middle_y.filler {
            @for project in projects {
                .row.middle_y.middle_x.m_gap {
                    img class=(project.class) src=(project.icon) alt=(project.name) {}
                    .column {
                        p.title {(project.name)}
                        p {(project.description)}
                    }
                    .column {
                        @for link in &project.links {
                            a href=(link.link) target="_blank" rel="noopener noreferrer" {
                                img class=(link.classes.join(" ") + " s") src=(link.icon) alt=(link.name) {}
                            }
                        }
                    }
                }
            }
        }
    )
}

fn items() -> Vec<Project> {
    let lince = Project {
        class: "lince",
        name: "Lince",
        icon: "./assets/project/lince_preto_no_branco.svg",
        description: PreEscaped("A tool for <i>everything</i>."),
        links: vec![
            Item {
                name: "Code",
                link: "https://github.com/lince-social/lince",
                icon: "./assets/tech/github.svg",
                classes: vec!["github", "social"],
            },
            Item {
                name: "Documentation",
                link: "https://lince-social.github.io/book",
                icon: "./assets/social/documentation.svg",
                classes: vec!["documentation", "social"],
            },
        ],
    };

    vec![lince]
}
