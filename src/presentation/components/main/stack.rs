use crate::schema::tech::{Area, Item};
use maud::{Markup, html};

pub fn stack() -> Markup {
    html! {
        section class="column middle_y middle_x"{
            @for area in tech_areas() {
                div class="row" {
                    @for tech in &area.items {
                        a href=(tech.link) class=(area.class.to_owned() + " middle_y column") target="_blank" rel="noopener noreferrer"
                            data-show=("$".to_owned() + area.class)
                            {
                            div class="logo" {
                                img class=(tech.classes.join(" ")) src=(tech.icon) alt=(tech.name) {}
                            }
                            div class="middle_x shy tabula_rasa" { (tech.name) }
                        }
                    }
                }
            }
        }
    }
}

pub fn tech_areas() -> Vec<Area> {
    let react = Item {
        name: "React",
        icon: "./assets/tech/react.svg".into(),
        link: "https://react.dev/",
        classes: vec![
            "react",
            "frontend",
            "personal_project",
            "professional_experience",
        ],
    };
    let htmx = Item {
        name: "HTMX",
        icon: "./assets/tech/htmx.svg".into(),
        link: "https://htmx.org/",
        classes: vec!["htmx", "frontend", "personal_project"],
    };
    let frontend = Area {
        name: "Frontend",
        items: vec![react, htmx],
        class: "frontend",
    };

    let nest_js = Item {
        name: "NestJS",
        icon: "./assets/tech/nestjs.svg".into(),
        link: "https://nestjs.com/",
        classes: vec!["nestjs", "backend", "professional_experience"],
    };
    let elysia_js = Item {
        name: "ElysiaJS",
        icon: "./assets/tech/elysiajs.svg".into(),
        link: "https://elysiajs.com/",
        classes: vec!["elysia", "backend", "personal_project"],
    };
    let axum = Item {
        name: "Axum",
        icon: "./assets/tech/axum.svg".into(),
        link: "https://github.com/tokio-rs/axum",
        classes: vec![
            "axum",
            "backend",
            "personal_project",
            "professional_experience",
        ],
    };
    let express = Item {
        name: "Express",
        icon: "./assets/tech/express.svg".into(),
        link: "https://expressjs.com/",
        classes: vec!["express", "backend", "personal_project"],
    };
    let flask = Item {
        name: "Flask",
        icon: "./assets/tech/flask.svg".into(),
        link: "https://flask.palletsprojects.com/en/stable/",
        classes: vec!["flask", "backend", "personal_project"],
    };
    let backend = Area {
        name: "Backend",
        items: vec![nest_js, elysia_js, axum, express, flask],
        class: "backend",
    };

    let nextjs = Item {
        name: "Next.js",
        icon: "./assets/tech/nextjs.svg".into(),
        link: "https://nextjs.org/",
        classes: vec!["nextjs", "fullstack", "personal_project"],
    };
    let datastar = Item {
        name: "Datastar",
        icon: "./assets/tech/datastar.webp".into(),
        link: "https://data-star.dev/",
        classes: vec!["datastar", "fullstack", "personal_project"],
    };
    let fullstack = Area {
        name: "Fullstack",
        items: vec![nextjs, datastar],
        class: "fullstack",
    };

    let rust = Item {
        name: "Rust",
        icon: "./assets/tech/rust.svg".into(),
        link: "https://www.rust-lang.org/",
        classes: vec![
            "rust",
            "languages",
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
            "languages",
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
            "languages",
            "personal_project",
            "professional_experience",
        ],
    };
    let languages = Area {
        name: "Languages",
        items: vec![rust, typescript, python],
        class: "languages",
    };

    let sqlite = Item {
        name: "SQLite",
        icon: "./assets/tech/sqlite.svg".into(),
        link: "https://www.sqlite.org/",
        classes: vec!["sqlite", "databases", "personal_project"],
    };
    let postgres = Item {
        name: "PostgreSQL",
        icon: "./assets/tech/postgresql.svg".into(),
        link: "https://www.postgresql.org/",
        classes: vec!["postgres", "databases", "personal_project"],
    };
    let mongodb = Item {
        name: "MongoDB",
        icon: "./assets/tech/mongodb.svg".into(),
        link: "https://www.mongodb.com/",
        classes: vec![
            "mongodb",
            "databases",
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
            "databases",
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
            "databases",
            "personal_project",
            "professional_experience",
        ],
    };
    let databases = Area {
        name: "Databases",
        items: vec![sqlite, postgres, mongodb, cassandra, prisma],
        class: "databases",
    };

    let docker = Item {
        name: "Docker",
        icon: "./assets/tech/docker.svg".into(),
        link: "https://www.docker.com/",
        classes: vec![
            "docker",
            "operations",
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
            "operations",
            "personal_project",
            "professional_experience",
        ],
    };
    let github = Item {
        name: "GitHub",
        icon: "./assets/tech/github.svg".into(),
        link: "https://github.com/",
        classes: vec![
            "github",
            "operations",
            "personal_project",
            "professional_experience",
        ],
    };
    let linux = Item {
        name: "Linux",
        icon: "./assets/tech/tux_w.svg".into(),
        link: "https://kernel.org/",
        classes: vec![
            "linux",
            "operations",
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
    let operations = Area {
        name: "Operations",
        items: vec![docker, git, github, linux, vercel],
        class: "operations",
    };

    vec![
        languages, frontend, fullstack, backend, databases, operations,
    ]
}
