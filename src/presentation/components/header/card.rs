use crate::presentation::components::header::social::socials;
use maud::{Markup, PreEscaped, html};

pub fn card() -> Markup {
    html!(
        div class="row middle_x middle_y censa" {
            (selfie())
            div class="column" {
                div class="row censa bottom" {
                    (title())
                    div class="right" {
                        (socials())
                    }
                }
                (speech())
            }
        }
    )
}

fn selfie() -> Markup {
    html!(
        img class="selfie" src="assets/social/selfie.jpg" {}
    )
}

fn title() -> Markup {
    html!(
        div class="column" {
            div class="row middle_y censa" {
                p class="title" { "Eduardo de Melo Xavier" }
                (age())
            }
            div class="row middle_y censa" {
                p class="stripped" { "Software Engineer @ Preto no Branco" }
                (info())
            }
        }
    )
}

fn info() -> Markup {
    html!(
        div class="column" {
            div class="row middle_y" {
                div class="pea_size" {
                    svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="white" width="100%" height="100%" {
                        path d="M12 2C8.13 2 5 5.13 5 9c0 5.25 7 13 7 13s7-7.75 7-13c0-3.87-3.13-7-7-7zm0 9.5a2.5 2.5 0 1 1 0-5 2.5 2.5 0 0 1 0 5z" {}
                    }
                }
                p class="stripped" { "Porto Alegre, Brazil."}
            }
        }
    )
}

fn age() -> Markup {
    html! {
        div id="age" {
            "Loading age..."
        }
        script {
            (PreEscaped(r#"
                   (function() {
                       const birth = new Date("2000-03-20");
                       const now = new Date();
                       let age = now.getFullYear() - birth.getFullYear();
                       const m = now.getMonth() - birth.getMonth();
                       if (m < 0 || (m === 0 && now.getDate() < birth.getDate())) {
                           age--;
                       }
                       document.getElementById("age").textContent = age + " years old";
                   })();
               "#))
        }
    }
}

fn speech() -> Markup {
    html!(
        p { "I have professional experience using Rust and NestJS on the backend and
            React on the frontend. I also did data analysis/visualization and automation with Python.
          I studied Machine Learning and Operations Research in my Production Engineering degree;
          having worked as a Product Owner in a project that used them."
        }
    )
}
