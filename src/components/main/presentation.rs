use maud::{Markup, html};

pub fn presentation() -> Markup {
    html!(
        img class="selfie" src="assets/social/selfie.jpg"
        h1 { "Eduardo de Melo Xavier" }
        p { "Software Engineer" }
        p { "Based in Porto Alegre, Brazil."}

    )
}
