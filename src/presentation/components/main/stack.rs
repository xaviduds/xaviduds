use crate::schema::tech::{Area, Item};
use maud::{Markup, html};

pub fn stack() -> Markup {
    let areas = areas();

    html! {
        section {
            h1 { "Tech Stack" }

            @for area in &areas {
                h2 { (area.name) }

                div class="row" {
                    @for tech in &area.items {
                        a href=(tech.link) class="middle_y column" target="_blank" rel="noopener noreferrer" {
                            div class="logo" {
                            img class=(tech.class) src=(tech.icon) alt=(tech.name) {}
                            }
                            div class="middle_x shy tabula_rasa" { (tech.name) }
                        }
                    }
                }
            }
        }
    }
}

fn areas() -> Vec<Area> {
    let react = Item::new("react", "React", "tech/react.svg", "https://react.dev/");
    let datastar = Item::new(
        "datastar",
        "Datastar",
        "tech/datastar.webp",
        "https://data-star.dev/",
    );
    let htmx = Item::new("htmx", "HTMX", "tech/htmx.svg", "https://htmx.org/");

    let frontend = Area::new("Frontend", vec![react, htmx]);

    let express = Item::new(
        "express",
        "Express",
        "tech/express.svg",
        "https://expressjs.com/",
    );
    let nest_js = Item::new("nestjs", "NestJS", "tech/nestjs.svg", "https://nestjs.com/");
    let axum = Item::new(
        "axum",
        "Axum",
        "tech/axum.svg",
        "https://github.com/tokio-rs/axum",
    );
    let elysia_js = Item::new(
        "elysia",
        "ElysiaJS",
        "tech/elysiajs.svg",
        "https://elysiajs.com/",
    );
    let flask = Item::new(
        "flask",
        "Flask",
        "tech/flask.svg",
        "https://flask.palletsprojects.com/en/stable/",
    );

    let backend = Area::new("Backend", vec![nest_js, elysia_js, axum, express, flask]);

    let nextjs = Item::new(
        "nextjs",
        "Next.js",
        "tech/nextjs.svg",
        "https://nextjs.org/",
    );

    let fullstack = Area::new("Fullstack", vec![nextjs, datastar]);

    // let node_js = Item::new("nodejs", "NodeJS", "tech/nodejs.svg");
    // let bun = Item::new("bun", "Bun", "tech/bun.svg");
    // let javascript = Item::new("javascript", "Javascript", "tech/javascript.svg");
    let typescript = Item::new(
        "typescript",
        "Typescript",
        "tech/typescript.svg",
        "https://www.typescriptlang.org/",
    );
    let rust = Item::new(
        "rust",
        "Rust",
        "tech/rust.svg",
        "https://www.rust-lang.org/",
    );
    let python = Item::new(
        "python",
        "Python",
        "tech/python.svg",
        "https://www.python.org/downloads/",
    );

    let languages = Area::new("Languages", vec![rust, typescript, python]);

    let sqlite = Item::new(
        "sqlite",
        "SQLite",
        "tech/sqlite.svg",
        "https://www.sqlite.org/",
    );

    let postgres = Item::new(
        "postgres",
        "PostgreSQL",
        "tech/postgresql.svg",
        "https://www.postgresql.org/",
    );

    let mongodb = Item::new(
        "mongodb",
        "MongoDB",
        "tech/mongodb.svg",
        "https://www.mongodb.com/",
    );

    let cassandra = Item::new(
        "cassandra",
        "Cassandra",
        "tech/cassandra.svg",
        "https://cassandra.apache.org/_/index.html",
    );

    let databases = Area::new("Databases", vec![sqlite, postgres, mongodb, cassandra]);

    vec![languages, frontend, fullstack, backend, databases]
}
