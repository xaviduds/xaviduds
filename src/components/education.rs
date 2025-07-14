use maud::{Markup, html};

use crate::components::profession::{Experience, experiences};

pub fn education() -> Markup {
    html!((experiences("Education", education_experiences_provider())))
}

fn education_experiences_provider() -> Vec<Experience> {
    let pucrs = Experience {
        organization: "PUC-RS",
        title: "Production Engineering Student",
        start: "2019/03",
        end: "2024/06",
        details: "Took half the degree, having pursued classes in the Data Science certification. Pivoting to the self study of technology.",
        presence: "Onsite",
    };

    vec![pucrs]
}
