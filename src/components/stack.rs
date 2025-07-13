use maud::{Markup, html};

#[derive(Clone)]
pub struct Item {
    pub name: &'static str,
    pub icon: &'static str,
    pub link: &'static str,
    pub classes: Vec<&'static str>,
}

pub struct Area {
    pub name: &'static str,
    pub items: Vec<Item>,
    pub class: &'static str,
}

pub fn stack() -> Markup {
    html! {
    #sort_container.column {
        .title.mini_eca.middle_x {"Stack"}
        .column.start
            data-signals="{orderInfo: ''}"
            data-on-reordered="$orderInfo = event.detail.orderInfo"
            {
            @for area in tech_areas() {
                .row.middle_y.mini_eca {
                    button
                        data-signals=(format!("{{{}: true}}", area.class))
                        data-on-click=(format!("${}=!${}", area.class, area.class))
                        data-class=(format!("{{'active_{}_area active_jumpy_button': ${}, 'inactive_{}_area inactive_jumpy_button': !${}}}",
                            area.class,area.class, area.class, area.class)) {
                            (area.name)
                    }
                    .row {
                        @for tech in &area.items {
                            a
                                href=(tech.link) target="_blank" rel="noopener noreferrer"
                                class=(tech.classes.iter().map(|class| class.to_string() + "_card ").collect::<String>()
                                    + area.class + " jumpy_button middle_y middle_x column tech_item")
                                data-show=("$".to_owned() + area.class) {
                                    p {(tech.name)}
                                    // img class=(tech.classes.join(" ") + " logo") src=(tech.icon) alt=(tech.name) {}
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn enchantment() -> Area {
    let rust = Item {
        name: "Rust",
        icon: "./assets/rust.svg",
        link: "https://www.rust-lang.org/",
        classes: vec![
            "rust",
            "enchantment",
            "personal_project",
            "professional_experience",
        ],
    };

    let typescript = Item {
        name: "Typescript",
        icon: "./assets/typescript.svg",
        link: "https://www.typescriptlang.org/",
        classes: vec![
            "typescript",
            "enchantment",
            "personal_project",
            "professional_experience",
        ],
    };

    let javascript = Item {
        name: "Javascript",
        icon: "./assets/javascript.svg",
        link: "https://developer.mozilla.org/docs/Web/JavaScript",
        classes: vec![
            "javascript",
            "enchantment",
            "personal_project",
            "professional_experience",
        ],
    };

    let python = Item {
        name: "Python",
        icon: "./assets/python.svg",
        link: "https://www.python.org/downloads/",
        classes: vec![
            "python",
            "enchantment",
            "personal_project",
            "professional_experience",
        ],
    };

    Area {
        name: "Languages",
        items: vec![rust, typescript, javascript, python],
        class: "enchantment",
    }
}

pub fn illusion() -> Area {
    let react = Item {
        name: "React",
        icon: "./assets/react.svg",
        link: "https://react.dev/",
        classes: vec![
            "react",
            "illusion",
            "personal_project",
            "professional_experience",
        ],
    };
    let htmx = Item {
        name: "HTMX",
        icon: "./assets/htmx.svg",
        link: "https://htmx.org/",
        classes: vec!["htmx", "illusion", "personal_project"],
    };
    let html = Item {
        name: "HTML",
        icon: "./assets/html.svg",
        link: "https://developer.mozilla.org/en-US/docs/Web/HTML",
        classes: vec![
            "html",
            "illusion",
            "personal_project",
            "professional_experience",
        ],
    };
    let css = Item {
        name: "CSS",
        icon: "./assets/css.svg",
        link: "https://developer.mozilla.org/en-US/docs/Web/CSS",
        classes: vec![
            "css",
            "illusion",
            "personal_project",
            "professional_experience",
        ],
    };
    Area {
        name: "Frontend",
        items: vec![html, css, react, htmx],
        class: "illusion",
    }
}

pub fn dual_casting() -> Area {
    let nextjs = Item {
        name: "Next.js",
        icon: "./assets/nextjs.svg",
        link: "https://nextjs.org/",
        classes: vec!["nextjs", "dual_casting", "personal_project"],
    };

    let datastar = Item {
        name: "Datastar",
        icon: "./assets/datastar.svg",
        link: "https://data-star.dev/",
        classes: vec!["datastar", "dual_casting", "personal_project"],
    };

    Area {
        name: "Fullstack",
        items: vec![nextjs, datastar],
        class: "dual_casting",
    }
}

pub fn invocation() -> Area {
    let nest_js = Item {
        name: "NestJS",
        icon: "./assets/nestjs.svg",
        link: "https://nestjs.com/",
        classes: vec!["nestjs", "invocation", "professional_experience"],
    };
    let elysia_js = Item {
        name: "ElysiaJS",
        icon: "./assets/elysiajs.svg",
        link: "https://elysiajs.com/",
        classes: vec!["elysia", "invocation", "personal_project"],
    };
    let axum = Item {
        name: "Axum",
        icon: "./assets/axum.svg",
        link: "https://github.com/tokio-rs/axum",
        classes: vec![
            "axum",
            "invocation",
            "personal_project",
            "professional_experience",
        ],
    };
    let express = Item {
        name: "Express",
        icon: "./assets/express.svg",
        link: "https://expressjs.com/",
        classes: vec!["express", "invocation", "personal_project"],
    };
    let flask = Item {
        name: "Flask",
        icon: "./assets/flask.svg",
        link: "https://flask.palletsprojects.com/en/stable/",
        classes: vec!["flask", "invocation", "personal_project"],
    };
    Area {
        name: "Backend",
        items: vec![nest_js, elysia_js, axum, express, flask],
        class: "invocation",
    }
}

pub fn alquery() -> Area {
    let sqlite = Item {
        name: "SQLite",
        icon: "./assets/sqlite.svg",
        link: "https://www.sqlite.org/",
        classes: vec!["sqlite", "alquery", "personal_project"],
    };
    let postgres = Item {
        name: "PostgreSQL",
        icon: "./assets/postgresql.svg",
        link: "https://www.postgresql.org/",
        classes: vec!["postgres", "alquery", "personal_project"],
    };
    let mongodb = Item {
        name: "MongoDB",
        icon: "./assets/mongodb.svg",
        link: "https://www.mongodb.com/",
        classes: vec![
            "mongodb",
            "alquery",
            "personal_project",
            "professional_experience",
        ],
    };
    let cassandra = Item {
        name: "Cassandra",
        icon: "./assets/cassandra.svg",
        link: "https://cassandra.apache.org/_/index.html",
        classes: vec![
            "cassandra",
            "alquery",
            "personal_project",
            "professional_experience",
        ],
    };
    let prisma = Item {
        name: "Prisma",
        icon: "./assets/prisma.svg",
        link: "https://cassandra.apache.org/_/index.html",
        classes: vec![
            "prisma",
            "alquery",
            "personal_project",
            "professional_experience",
        ],
    };

    Area {
        name: "Databases",
        items: vec![sqlite, postgres, mongodb, cassandra, prisma],
        class: "alquery",
    }
}

pub fn nimbus_weaving() -> Area {
    let docker = Item {
        name: "Docker",
        icon: "./assets/docker.svg",
        link: "https://www.docker.com/",
        classes: vec![
            "docker",
            "nimbus_weaving",
            "personal_project",
            "professional_experience",
        ],
    };
    let git = Item {
        name: "Git",
        icon: "./assets/git.svg",
        link: "https://git-scm.com/",
        classes: vec![
            "git",
            "nimbus_weaving",
            "personal_project",
            "professional_experience",
        ],
    };
    let github = Item {
        name: "GitHub",
        icon: "./assets/github.svg",
        link: "https://github.com/xaviduds",
        classes: vec![
            "github",
            "nimbus_weaving",
            "personal_project",
            "professional_experience",
        ],
    };
    let linux = Item {
        name: "Linux",
        icon: "./assets/tux.svg",
        link: "https://kernel.org/",
        classes: vec![
            "linux",
            "nimbus_weaving",
            "personal_project",
            "professional_experience",
        ],
    };
    let vercel = Item {
        name: "Vercel",
        icon: "./assets/vercel.svg",
        link: "https://vercel.com/",
        classes: vec!["vercel", "ops", "personal_project"],
    };

    Area {
        name: "DevOps",
        items: vec![docker, git, github, linux, vercel],
        class: "nimbus_weaving",
    }
}

pub fn machine_learning() -> Area {
    let scikit_learn = Item {
        name: "scikit-learn",
        icon: "./assets/tux.svg",
        link: "rada",
        classes: vec!["scikit-learn", "ml"],
    };
    Area {
        name: "Machine Learning",
        items: vec![scikit_learn],
        class: "machine_learning",
    }
}

pub fn tech_areas() -> Vec<Area> {
    vec![
        enchantment(),
        machine_learning(),
        illusion(),
        dual_casting(),
        invocation(),
        alquery(),
        nimbus_weaving(),
    ]
}
