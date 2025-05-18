use maud::{Markup, html};

pub fn presentation() -> Markup {
    html!(
       h1 { "Eduardo de Melo Xavier" }
       p { "Backend Engineer" }
    )
}
