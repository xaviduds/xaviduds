use maud::{Markup, PreEscaped, html};

use crate::components::stack::{Area, Item};

pub fn html_header() -> Markup {
    html! {
        header.green.row {
            .row.s_gap.middle_y {
                (selfie())
                .column.m_gap {
                    (title())
                    (description())
                }
            }
            (info())
        }
    }
}

fn selfie() -> Markup {
    html!(
        img.selfie src="assets/selfie.jpg" {}
    )
}

fn title() -> Markup {
    html!(
        .column{
            p.title { "Eduardo de Melo Xavier" }
            p.stripped { "Software Engineer @ Preto no Branco" }
        }
    )
}

fn info() -> Markup {
    html!(
        .column.showoff {
            (age())
            (location())
            (socials())
        }
    )
}

fn location() -> Markup {
    html!(
        .row.middle_y.s_gap {
            p.stripped { "Porto Alegre, Brazil." }
            .s_size.end_y {
                svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="black" width="100%" height="100%" {
                    path d="M12 2C8.13 2 5 5.13 5 9c0 5.25 7 13 7 13s7-7.75 7-13c0-3.87-3.13-7-7-7zm0 9.5a2.5 2.5 0 1 1 0-5 2.5 2.5 0 0 1 0 5z" {}
                }
            }
        }
    )
}

fn age() -> Markup {
    html! {
        #age {
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

pub fn socials() -> Markup {
    html! {
        .column {
            @for tech in &social_area().items {
                .row.middle_y {
                    p {(tech.name)}
                    a href=(tech.link) target="_blank" rel="noopener noreferrer" {
                        img.s_size src=(tech.icon) alt=(tech.name) {}
                    }
                }
            }
        }
    }
}

fn description() -> Markup {
    let starting_date = "2025-02-03";
    html!(
        p {
            "Software Engineer with "
            span id="exp" { "some experience" }
            " of professional experience focused on Backend challenges.
            I feel most motivated when working with necessarily complex systems, Rust codebases and robust desktop applications."
        }
        script {
            (PreEscaped(format!(r#"
                (function() {{
                    const start = new Date("{}");
                    const now = new Date();

                    let years = now.getFullYear() - start.getFullYear();
                    let months = now.getMonth() - start.getMonth();
                    if (months < 0) {{
                        years--;
                        months += 12;
                    }}

                    const parts = [];
                    if (years === 1) {{
                        parts.push("1 year");
                    }} else if (years > 1) {{
                        parts.push(years + " years");
                    }}

                    if (months === 1) {{
                        parts.push("1 month");
                    }} else if (months > 1 || (years === 0 && months >= 0)) {{
                        parts.push(months + " months");
                    }}

                    document.getElementById("exp").textContent = parts.join(" and ");
                }})();
            "#, starting_date)))
        }
    )
}

pub fn social_area() -> Area {
    let github = Item {
        name: "github.com/xaviduds",
        icon: "./assets/github.svg",
        link: "https://github.com/xaviduds",
    };

    let linkedin = Item {
        name: "linkedin.com/in/xaviduds",
        icon: "./assets/linkedin.svg",
        link: "https://linkedin.com/in/xaviduds",
    };

    let email = Item {
        name: "xaviduds@gmail.com",
        icon: "./assets/email.svg",
        link: "mailto:xaviduds@gmail.com",
    };

    Area {
        name: "Socials",
        items: vec![github, linkedin, email],
    }
}
