use crate::schema::{project::Project, tech::Item};
use maud::{Markup, PreEscaped, html};

pub fn projects() -> Markup {
    let projects = items();

    html!(
        #projects.column.middle_y{
            p.title {"Familiars"}
            @for project in projects {
                .row.middle_y.separa.project {
                   .start_x.middle_y {

                    img class=(project.class.to_owned() + " m_logo start_y" ) src=(project.icon) alt=(project.name) {}
                    .column{
                        p.title {(project.name)}
                        p {(project.description)}
                    }
                   }
                    .column.middle_y {
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

    // let kamalion = Project {
    //     class: "kamalie",
    //     name: "Kamalie",
    //     icon: "./assets/project/kamalie.svg",
    //     description: PreEscaped("An rpg."),
    //     links: vec![Item {
    //         name: "Code",
    //         link: "https://github.com/xaviduds/kamalie",
    //         icon: "./assets/tech/github.svg",
    //         classes: vec!["github", "social"],
    //     }],
    // };

    let mosca = Project {
        class: "mosca",
        name: "M.O.S.C.A",
        icon: "./assets/project/mosca.svg",
        description: PreEscaped("M.O.S.C.A"),
        links: vec![Item {
            name: "Code",
            link: "https://github.com/lince-social/mosca",
            icon: "./assets/tech/github.svg",
            classes: vec!["github", "social"],
        }],
    };

    vec![lince, mosca]
}
