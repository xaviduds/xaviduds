use crate::schema::tech::{Area, Item};
use maud::{Markup, html};

pub fn stack() -> Markup {
    let areas = areas();

    html! {
        h1 { "Tech Stack" }

        @for area in &areas {
            h2 { (area.name) }

            div class="row" {
                @for tech in &area.items {
                    a href=(tech.link) class="middle_x column" target="_blank" rel="noopener noreferrer" {
                        img class=("logo ".to_owned() + tech.class) src=(tech.icon) alt=(tech.name) {}
                        div class="middle_x shy tabula_rasa" { (tech.name) }
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

    let frontend = Area::new("Frontend", vec![react, htmx, datastar.clone()]);

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

    let backend = Area::new(
        "Backend",
        vec![nest_js, elysia_js, axum, datastar, express, flask],
    );

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

    // let = Item::new("", "", "tech/.svg");
    let languages = Area::new("Languages", vec![rust, typescript, python]);

    vec![languages, frontend, backend]
}
