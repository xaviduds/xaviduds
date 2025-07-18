use maud::{Markup, html};

pub struct Experience {
    pub organization: &'static str,
    pub start: &'static str,
    pub end: &'static str,
    pub title: &'static str,
    pub details: &'static str,
    pub presence: &'static str,
}

pub fn profession() -> Markup {
    let professional_experiences = professional_experiences_provider();
    html!((experiences("Profession", professional_experiences)))
}

pub fn experiences(title: &'static str, experiences: Vec<Experience>) -> Markup {
    html!(
       section #experiences.s_gap.column.s_space {
           .title {(title)}
           @for experience in experiences {
               .experience.column {
                   .column.separa.nowrap {
                        .row.nowrap.xs_gap {
                            p.experience_title { (experience.title) }
                            p.experience_at { "@" }
                            p.experience_organization { (experience.organization) }
                        }
                        .row.separa {
                            p {(experience.presence)}
                            .row.s_gap {
                                p.experience_start { (experience.start) }
                                p.experience_separator { "-" }
                                p.experience_end { (experience.end) }
                            }
                        }
                   }
                    p.experience_details { (experience.details) }
            }
           }
       }
    )
}

fn professional_experiences_provider() -> Vec<Experience> {
    let lsb = Experience {
        organization: "LSB - Lean Scheduling Brazil",
        start: "2022/02",
        end: "2022/05",
        title: "APS Consultant",
        details: "Production Engineering Internship.
        Learned about manufacturing processes, PPCP, from S&OP to MRP
        and Opcenter Advanced Planning and Scheduling.",
        presence: "Hybrid", // details: "",
    };
    let sullab = Experience {
        organization: "Grupo Sullab",
        start: "2022/08",
        end: "2023/04",
        title: "Production Engineering Intern",
        details: "Developed BI with
        Data Analysis and Visualization in the domain of
        biomedicine machine distribution.",
        presence: "Hybrid", // details: "",
    };
    let evcomx = Experience {
        organization: "EVCOMX",
        start: "2023/08",
        end: "2023/10",
        title: "Product Owner Intern",
        details: "Implemented Scrum rituals and
        worked alongside Data Scientists to
        develop a ML and Operations Research product.",
        presence: "Hybrid", // details: "",
    };
    let preto_no_branco = Experience {
        organization: "Preto no Branco",
        start: "2025/02",
        end: "Present",
        title: "Software Engineer",
        details: "Developed from static webpages to the backend of enterprise software and Machine Learning.
        Worked with Rust, TypeScript, Python, NestJS, React, MongoDB and Cassandra.",
        presence: "On-site",
    };

    vec![preto_no_branco, evcomx, sullab, lsb]
}
