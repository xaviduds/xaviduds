use maud::{Markup, html};

use crate::schema::experience::Experience;

pub fn education() -> Markup {
    html! {
        #educations {
            p.title { "Education" }
            @for education in (education_items()) {
                .row.xs_gap {
                    p { (education.title) }
                    p { "@"}
                    p { (education.organization) }
                }
                .row.xs_gap {
                    p {(education.start)}
                    p {"-"}
                    p {(education.end)}
                }
                p {(education.details)}

            }

        }
    }
}

fn education_items() -> Vec<Experience> {
    let pucrs = Experience {
        organization: "PUC-RS",
        title: "Production Engineering Student",
        start: "2019/03",
        end: "2024/06",
        details: "Took half the degree, having pursued classes in the Data Science certification. Pivoting to the self study of technology.",
    };

    vec![pucrs]
}
