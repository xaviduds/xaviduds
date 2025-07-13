use maud::{Markup, html};

#[derive(Clone)]
pub struct Item {
    pub name: &'static str,
    pub icon: &'static str,
    pub link: &'static str,
}

pub struct Area {
    pub name: &'static str,
    pub items: Vec<Item>,
}

pub fn stack() -> Markup {
    html! {
    .column.x_start {
        .title.s_space {"Stack"}
        .column.start {
            @for area in tech_areas() {
                .column.s_space {
                   p.area_title { (area.name) }
                    .row.s_gap {
                        @for tech in &area.items {
                                p.tech_name {(tech.name)}
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
    };

    let typescript = Item {
        name: "Typescript",
        icon: "./assets/typescript.svg",
        link: "https://www.typescriptlang.org/",
    };

    let javascript = Item {
        name: "Javascript",
        icon: "./assets/javascript.svg",
        link: "https://developer.mozilla.org/docs/Web/JavaScript",
    };

    let python = Item {
        name: "Python",
        icon: "./assets/python.svg",
        link: "https://www.python.org/downloads/",
    };

    Area {
        name: "Languages",
        items: vec![rust, typescript, javascript, python],
    }
}

pub fn illusion() -> Area {
    let react = Item {
        name: "React",
        icon: "./assets/react.svg",
        link: "https://react.dev/",
    };
    let htmx = Item {
        name: "HTMX",
        icon: "./assets/htmx.svg",
        link: "https://htmx.org/",
    };
    let html = Item {
        name: "HTML",
        icon: "./assets/html.svg",
        link: "https://developer.mozilla.org/en-US/docs/Web/HTML",
    };
    let css = Item {
        name: "CSS",
        icon: "./assets/css.svg",
        link: "https://developer.mozilla.org/en-US/docs/Web/CSS",
    };
    Area {
        name: "Frontend",
        items: vec![html, css, react, htmx],
    }
}

pub fn dual_casting() -> Area {
    let nextjs = Item {
        name: "Next.js",
        icon: "./assets/nextjs.svg",
        link: "https://nextjs.org/",
    };

    let datastar = Item {
        name: "Datastar",
        icon: "./assets/datastar.svg",
        link: "https://data-star.dev/",
    };

    Area {
        name: "Fullstack",
        items: vec![nextjs, datastar],
    }
}

pub fn invocation() -> Area {
    let nest_js = Item {
        name: "NestJS",
        icon: "./assets/nestjs.svg",
        link: "https://nestjs.com/",
    };
    let elysia_js = Item {
        name: "ElysiaJS",
        icon: "./assets/elysiajs.svg",
        link: "https://elysiajs.com/",
    };
    let axum = Item {
        name: "Axum",
        icon: "./assets/axum.svg",
        link: "https://github.com/tokio-rs/axum",
    };
    let express = Item {
        name: "Express",
        icon: "./assets/express.svg",
        link: "https://expressjs.com/",
    };
    let flask = Item {
        name: "Flask",
        icon: "./assets/flask.svg",
        link: "https://flask.palletsprojects.com/en/stable/",
    };
    Area {
        name: "Backend",
        items: vec![nest_js, elysia_js, axum, express, flask],
    }
}

pub fn alquery() -> Area {
    let sqlite = Item {
        name: "SQLite",
        icon: "./assets/sqlite.svg",
        link: "https://www.sqlite.org/",
    };
    let postgres = Item {
        name: "PostgreSQL",
        icon: "./assets/postgresql.svg",
        link: "https://www.postgresql.org/",
    };
    let mongodb = Item {
        name: "MongoDB",
        icon: "./assets/mongodb.svg",
        link: "https://www.mongodb.com/",
    };
    let cassandra = Item {
        name: "Cassandra",
        icon: "./assets/cassandra.svg",
        link: "https://cassandra.apache.org/_/index.html",
    };

    Area {
        name: "Databases",
        items: vec![sqlite, postgres, mongodb, cassandra],
    }
}

pub fn nimbus_weaving() -> Area {
    let docker = Item {
        name: "Docker",
        icon: "./assets/docker.svg",
        link: "https://www.docker.com/",
    };
    let git = Item {
        name: "Git",
        icon: "./assets/git.svg",
        link: "https://git-scm.com/",
    };
    let github = Item {
        name: "GitHub",
        icon: "./assets/github.svg",
        link: "https://github.com/xaviduds",
    };
    let linux = Item {
        name: "Linux",
        icon: "./assets/tux.svg",
        link: "https://kernel.org/",
    };
    let vercel = Item {
        name: "Vercel",
        icon: "./assets/vercel.svg",
        link: "https://vercel.com/",
    };

    Area {
        name: "DevOps",
        items: vec![docker, git, github, linux, vercel],
    }
}

pub fn machine_learning() -> Area {
    let scikit_learn = Item {
        name: "scikit-learn",
        icon: "./assets/tux.svg",
        link: "rada",
    };
    Area {
        name: "Machine Learning",
        items: vec![scikit_learn],
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
