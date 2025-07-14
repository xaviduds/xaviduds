use maud::{Markup, html};

pub struct Area {
    pub name: &'static str,
    pub items: Vec<&'static str>,
}

pub fn stack() -> Markup {
    html! {
    .column.x_start {
        .title.s_space {"Tools"}
        .column.start {
            @for area in tech_areas() {
                .column.s_space {
                   p.area_title { (area.name) }
                    .row.s_gap {
                        @for tech in &area.items {
                                p.tech_name {(tech)}
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn human_languages() -> Area {
    Area {
        name: "Human Languages",
        items: vec!["English", "Portuguese"],
    }
}

pub fn programming_languages() -> Area {
    Area {
        name: "Programming Languages",
        items: vec!["Rust", "Typescript", "Javascript", "Python"],
    }
}

pub fn illusion() -> Area {
    Area {
        name: "Frontend",
        items: vec!["HTML", "CSS", "React", "HTMX"],
    }
}

pub fn dual_casting() -> Area {
    Area {
        name: "Fullstack",
        items: vec!["Next.js", "Datastar"],
    }
}

pub fn invocation() -> Area {
    Area {
        name: "Backend",
        items: vec!["NestJS", "ElysiaJS", "Axum", "Express", "Flask"],
    }
}

pub fn alquery() -> Area {
    Area {
        name: "Databases",
        items: vec!["SQLite", "PostgreSQL", "MongoDB", "Cassandra"],
    }
}

pub fn nimbus_weaving() -> Area {
    Area {
        name: "DevOps",
        items: vec!["Docker", "Git", "GitHub", "Linux", "Vercel"],
    }
}

pub fn machine_learning() -> Area {
    Area {
        name: "Machine Learning",
        items: vec![
            "scikit-learn",
            "Fine Tuning (Ultralytics [YOLO])",
            "Deep Learning (Rust's Burn)",
        ],
    }
}

pub fn tech_areas() -> Vec<Area> {
    vec![
        human_languages(),
        programming_languages(),
        machine_learning(),
        illusion(),
        dual_casting(),
        invocation(),
        alquery(),
        nimbus_weaving(),
    ]
}
