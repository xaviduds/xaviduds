use maud::{Markup, html};

pub fn about() -> Markup {
    html!(
        div class="column red" {
            h1 { "About me:" }
                p { "I'm passionate about performatic and simple software.
                    As a Software Engineer, I have used Rust and NestJS on the backend, and React on the frontend.
                    I also like Machine Learning and Operations Research.
                    Having worked on the business side, as a Product Owner, in a project that used them.
                    In one of my experiences I delivered value with data
                    analysis/visualization and automated manual processes with Python."}
            p { "I like Linux, all you can eat buffets and anime/manga."}
        }
    )
}
