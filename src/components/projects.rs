use maud::{Markup, PreEscaped, html};

pub struct Project {
    pub class: &'static str,
    pub name: &'static str,
    pub icon: &'static str,
    pub description: PreEscaped<&'static str>,
}

pub fn projects() -> Markup {
    let projects = items();

    html!(
        #projects.column{
            #lince-title.row.middle_y.s_gap.separa {
                p.title {"Project: Lince" }
            }
            @for project in projects {
                .s_gap {

                .start_x.middle_y {
                    img class=(project.class.to_owned() + " m_logo start_y" ) src=(project.icon) alt=(project.name) {}
                    p {(project.description)}
                }
                .row.s_gap.middle_y.end_x {
                    p {"github.com/lince-social/lince"}
                    img.s_size src="./assets/github.svg" {}
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
        icon: "./assets/lince_preto.svg",
        description: PreEscaped(
            "An automation and life system in Rust. Has cron-jobs, tables, a DSL, Shell/SQL commands and more.",
        ),
    };

    vec![lince]
}
