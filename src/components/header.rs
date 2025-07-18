use maud::{Markup, PreEscaped, html};

pub fn html_header() -> Markup {
    html! {
        header.row.s_space {
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

pub fn info() -> Markup {
    html! {
        .column.s_gap {
            @for tech in &social_area() {
                .row.end_x.middle_y.s_gap {
                    @if tech.name == "age" {
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
                    } @else {
                        p {(tech.name)}
                    }
                    img.s_size.info src=(tech.icon) {}
                }
            }
        }
    }
}

fn description() -> Markup {
    let starting_date = "2025-02-03";
    html!(
        p {
            "Developer with "
            span id="exp" { "some experience" }
            " of experience. In that time I helped build a safety-critical system
            with Backend Rust code and a Machine Learning solution."
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

pub struct Item {
    pub name: &'static str,
    pub icon: &'static str,
}

pub fn social_area() -> Vec<Item> {
    let age = Item {
        name: "age",
        icon: "./assets/cake.svg",
    };
    let location = Item {
        name: "Porto Alegre, Brazil",
        icon: "./assets/location.svg",
    };
    let github = Item {
        name: "xaviduds",
        icon: "./assets/github.svg",
    };

    let linkedin = Item {
        name: "in/xaviduds",
        icon: "./assets/linkedin.svg",
    };

    let email = Item {
        name: "xaviduds@gmail.com",
        icon: "./assets/email.svg",
    };

    vec![age, location, github, linkedin, email]
}
