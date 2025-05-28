use crate::schema::experience::Experience;
use maud::{Markup, html};

pub fn experience() -> Markup {
    let experiences = items();

    html!(
       #experiences.m_gap.column.mini_eca.middle_y {
           p.title {"Professional Experiences"}
           @for experience in experiences {
               .column.framed.mini_eca.xs_gap {
                    .row{
                        .row.s_gap {
                            p.experience_title { (experience.title) }
                            p { "@" }
                            p.experience_company { (experience.company) }
                        }
                        .row.end.filler{
                            p.experience_start { (experience.start) }
                            p { "-" }
                            p.experience_end { (experience.end) }
                        }
                    }
                    p.experience_details { (experience.details) }
                    .row {
                        @for tag in experience.tags {
                            p.tag {(tag)}
                        }
                    }
               }
           }
       }
    )
}

fn items() -> Vec<Experience> {
    let lsb = Experience {
        company: "Lean Scheduling Brazil",
        start: "2022/02",
        end: "2022/05",
        title: "APS Consultant",
        tags: vec!["PPCP", "Opcenter APS"],
        details: "Learned about Advanced Planning and Scheduling of manufacturing processes and customer support.",
    };
    let sullab = Experience {
        company: "Grupo Sullab",
        start: "2022/08",
        end: "2023/04",
        title: "Production Engineering Intern",
        tags: vec![
            "Data Analysis",
            "Data Visualization",
            "Automation",
            "Python",
        ],
        details: "Increased business intelligence in the domain of
        biomedicine machines distribution.",
    };
    let evcomx = Experience {
        company: "EVCOMX",
        start: "2023/08",
        end: "2023/10",
        title: "Product Owner Intern",
        tags: vec!["Scrum", "Agile", "Product Ownership"],
        details: "Implemented Scrum rituals and
        worked alongside Data Scientists to
        develop a ML and Operations Research product.",
    };
    let preto_no_branco = Experience {
        company: "Preto no Branco",
        start: "2025/02",
        end: "Present",
        title: "Software Engineer",
        tags: vec![
            "Rust",
            "TypeScript",
            "NestJS",
            "React",
            "Cassandra",
            "MongoDB",
        ],
        details: "Developed from static webpages to the backend of enterprise software. ",
    };

    vec![preto_no_branco, evcomx, sullab, lsb]
}
