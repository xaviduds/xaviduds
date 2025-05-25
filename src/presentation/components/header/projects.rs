use crate::schema::project::Project;

use maud::{Markup, PreEscaped, html};

pub fn projects() -> Markup {
    let projects = items();

    html!(
        @for project in projects {
            .project.row.middle_y {
                img class=(project.class) src=(project.icon) alt=(project.name) {}
                .column {
                    p.title {(project.name)}
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
            r#"A tool for <i>everything</i>.
            <a href="https://github.com/lince-social/lince">Code</a>
            | <a href="https://lince-social.github.io/book">Docs</a>."#,
        ),
    );

    vec![lince]
}
