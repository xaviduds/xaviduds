use crate::schema::project::Project;
use maud::{Markup, PreEscaped, html};

pub fn projects() -> Markup {
    let projects = items();

    html!(
        h1 { "Projects" }
        @for project in projects {
            div class="project row middle_x" {
                img class=(project.class) src=(project.icon) alt=(project.name) {}
                div class="column" {
                    p class="title" {(project.name)}
                    p {(project.description)}
                }
            }
        }
    )
}

fn items() -> Vec<Project> {
    let lince = Project::new(
        "lince",
        "Lince",
        "project/lince_preto_no_branco.svg",
        PreEscaped(
            r#"A tool for everything.
            Documentation can be found
            <a href="https://lince-social.github.io/book">here</a>."#,
        ),
    );
    vec![lince]
}
