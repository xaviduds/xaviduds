use maud::{Markup, html};

pub struct Experience {
    pub organization: &'static str,
    pub start: &'static str,
    pub end: &'static str,
    pub title: &'static str,
    pub details: &'static str,
}

pub fn experience() -> Markup {
    let experiences = items();

    html!(
       section #experiences {
           .title { "Experiences" }
           @for experience in experiences {
               .experience {
                   .row.nowrap {
                        p.experience_title { (experience.title) }
                        p { "@" }
                        p.experience_organization { (experience.organization) }
                   }
                    .row {
                        p.experience_start { (experience.start) }
                        p { "-" }
                        p.experience_end { (experience.end) }
                    }
                    p.experience_details { (experience.details) }
            }
           }
       }
    )
}

fn items() -> Vec<Experience> {
    let lsb = Experience {
        organization: "LSB - Lean Scheduling Brazil",
        start: "2022/02",
        end: "2022/05",
        title: "APS Consultant",
        details: "Production Engineering Internship.
        Learned about manufacturing processes, PPCP, from S&OP to MRP
        and Opcenter Advanced Planning and Scheduling.",
    };
    let sullab = Experience {
        organization: "Grupo Sullab",
        start: "2022/08",
        end: "2023/04",
        title: "Production Engineering Intern",
        details: "Developed BI with
        Data Analysis and Visualization in the domain of
        biomedicine machine distribution.",
    };
    let evcomx = Experience {
        organization: "EVCOMX",
        start: "2023/08",
        end: "2023/10",
        title: "Product Owner Intern",
        details: "Implemented Scrum rituals and
        worked alongside Data Scientists to
        develop a ML and Operations Research product.",
    };
    let preto_no_branco = Experience {
        organization: "Preto no Branco",
        start: "2025/02",
        end: "Present",
        title: "Software Engineer",
        details: "Developed from static webpages to the backend of enterprise software.
        Worked with Rust, TypeScript, NestJS, React, MongoDB and Cassandra.",
    };

    vec![preto_no_branco, evcomx, sullab, lsb]
}
