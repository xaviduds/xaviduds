use crate::schema::tech::{Area, Item};
use maud::{Markup, html};

pub fn stack() -> Markup {
    html! {
    #stack.title.mini_eca {"Tech Stack"}
    #sortContainer.column.start
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
                                img class=(tech.classes.join(" ") + " logo") src=(tech.icon) alt=(tech.name) {}
                            }
                        }
                    }
                }
            }
        script type="module" src="./js/sortable.js" {}
        }
    }
}

pub fn enchantment() -> Area {
    let rust = Item {
        name: "Rust",
        icon: "./assets/tech/rust.svg".into(),
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
        icon: "./assets/tech/typescript.svg".into(),
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
        icon: "./assets/tech/javascript.svg".into(),
        link: "https://www.javascriptlang.org/",
        classes: vec![
            "javascript",
            "enchantment",
            "personal_project",
            "professional_experience",
        ],
    };

    let python = Item {
        name: "Python",
        icon: "./assets/tech/python.svg".into(),
        link: "https://www.python.org/downloads/",
        classes: vec![
            "python",
            "enchantment",
            "personal_project",
            "professional_experience",
        ],
    };

    Area {
        name: "Enchantment",
        items: vec![rust, typescript, javascript, python],
        class: "enchantment",
    }
}

pub fn illusion() -> Area {
    let react = Item {
        name: "React",
        icon: "./assets/tech/react.svg".into(),
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
        icon: "./assets/tech/htmx.svg".into(),
        link: "https://htmx.org/",
        classes: vec!["htmx", "illusion", "personal_project"],
    };
    let html = Item {
        name: "HTML",
        icon: "./assets/tech/html.svg".into(),
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
        icon: "./assets/tech/css.svg".into(),
        link: "https://developer.mozilla.org/en-US/docs/Web/CSS",
        classes: vec![
            "css",
            "illusion",
            "personal_project",
            "professional_experience",
        ],
    };
    Area {
        name: "Illusion",
        items: vec![html, css, react, htmx],
        class: "illusion",
    }
}

pub fn dual_casting() -> Area {
    let nextjs = Item {
        name: "Next.js",
        icon: "./assets/tech/nextjs.svg".into(),
        link: "https://nextjs.org/",
        classes: vec!["nextjs", "dual_casting", "personal_project"],
    };

    let datastar = Item {
        name: "Datastar",
        icon: "./assets/tech/datastar.svg".into(),
        link: "https://data-star.dev/",
        classes: vec!["datastar", "dual_casting", "personal_project"],
    };

    Area {
        name: "Dual Casting",
        items: vec![nextjs, datastar],
        class: "dual_casting",
    }
}

pub fn invocation() -> Area {
    let nest_js = Item {
        name: "NestJS",
        icon: "./assets/tech/nestjs.svg".into(),
        link: "https://nestjs.com/",
        classes: vec!["nestjs", "invocation", "professional_experience"],
    };
    let elysia_js = Item {
        name: "ElysiaJS",
        icon: "./assets/tech/elysiajs.svg".into(),
        link: "https://elysiajs.com/",
        classes: vec!["elysia", "invocation", "personal_project"],
    };
    let axum = Item {
        name: "Axum",
        icon: "./assets/tech/axum.svg".into(),
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
        icon: "./assets/tech/express.svg".into(),
        link: "https://expressjs.com/",
        classes: vec!["express", "invocation", "personal_project"],
    };
    let flask = Item {
        name: "Flask",
        icon: "./assets/tech/flask.svg".into(),
        link: "https://flask.palletsprojects.com/en/stable/",
        classes: vec!["flask", "invocation", "personal_project"],
    };
    Area {
        name: "Invocation",
        items: vec![nest_js, elysia_js, axum, express, flask],
        class: "invocation",
    }
}

pub fn alquery() -> Area {
    let sqlite = Item {
        name: "SQLite",
        icon: "./assets/tech/sqlite.svg".into(),
        link: "https://www.sqlite.org/",
        classes: vec!["sqlite", "alquery", "personal_project"],
    };
    let postgres = Item {
        name: "PostgreSQL",
        icon: "./assets/tech/postgresql.svg".into(),
        link: "https://www.postgresql.org/",
        classes: vec!["postgres", "alquery", "personal_project"],
    };
    let mongodb = Item {
        name: "MongoDB",
        icon: "./assets/tech/mongodb.svg".into(),
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
        icon: "./assets/tech/cassandra.svg".into(),
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
        icon: "./assets/tech/prisma.svg".into(),
        link: "https://cassandra.apache.org/_/index.html",
        classes: vec![
            "prisma",
            "alquery",
            "personal_project",
            "professional_experience",
        ],
    };

    Area {
        name: "Alquery",
        items: vec![sqlite, postgres, mongodb, cassandra, prisma],
        class: "alquery",
    }
}

pub fn nimbus_weaving() -> Area {
    let docker = Item {
        name: "Docker",
        icon: "./assets/tech/docker.svg".into(),
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
        icon: "./assets/tech/git.svg".into(),
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
        icon: "./assets/tech/github.svg".into(),
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
        icon: "./assets/tech/tux.svg".into(),
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
        icon: "./assets/tech/vercel.svg".into(),
        link: "https://vercel.com/",
        classes: vec!["vercel", "ops", "personal_project"],
    };

    Area {
        name: "Nimbus Weaving",
        items: vec![docker, git, github, linux, vercel],
        class: "nimbus_weaving",
    }
}

pub fn tech_areas() -> Vec<Area> {
    vec![
        dual_casting(),
        enchantment(),
        illusion(),
        invocation(),
        alquery(),
        nimbus_weaving(),
    ]
}
