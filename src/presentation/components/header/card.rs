use crate::presentation::components::header::social::socials;
use maud::{Markup, html};

pub fn card() -> Markup {
    // let birthday = "2000/03/20";
    // let professional_developer = "2025/02/03";

    html!(
        div class="row middle_x middle_y" {
            (moh_media_ele())
            div class="column" {
                div class="row" {
                    (title())
                    (info())
                }
                (speech())
            }
        }
    )
}

fn moh_media_ele() -> Markup {
    html!(
        div class="column" {
            img class="selfie" src="assets/social/selfie.jpg" {}
            (socials())
        }
    )
}

fn title() -> Markup {
    html!(
        div class="column" {
            p class="title" { "Eduardo de Melo Xavier" }
            p { "Software Engineer @ Preto no Branco" }
        }
    )
}

fn info() -> Markup {
    html!(
        div class="column red" {
            div class="row middle_y" {
                div class="pea_size" {
                    svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="white" width="100%" height="100%" {
                        path d="M12 2C8.13 2 5 5.13 5 9c0 5.25 7 13 7 13s7-7.75 7-13c0-3.87-3.13-7-7-7zm0 9.5a2.5 2.5 0 1 1 0-5 2.5 2.5 0 0 1 0 5z" {}
                    }
                }
                p { "Porto Alegre, Brazil."}
            }
        }
    )
}

fn speech() -> Markup {
    html!(
        p class="blue"{ "I'm passionate about performatic and simple software.
          As a Software Engineer, I've used Rust and NestJS on the backend, and React on the frontend.
          I studied Machine Learning and Operations Research. Having worked as a Product Owner in a project that used them.
          In one of my experiences I delivered value with data
          analysis/visualization and automated manual processes with Python."
        }
    )
}
