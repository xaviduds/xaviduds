use maud::PreEscaped;

pub struct Project {
    pub class: &'static str,
    pub name: &'static str,
    pub icon: String,
    pub description: PreEscaped<&'static str>, // change here
}

impl Project {
    pub fn new(
        class: &'static str,
        name: &'static str,
        icon: &'static str,
        description: PreEscaped<&'static str>,
    ) -> Self {
        let icon = format!("./assets/{}", icon);

        Self {
            class,
            name,
            icon,
            description,
        }
    }
}
