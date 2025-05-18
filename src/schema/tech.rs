#[derive(Clone)]
pub struct Item {
    pub class: &'static str,
    pub name: &'static str,
    pub icon: String,
    pub link: &'static str,
}

impl Item {
    pub fn new(
        class: &'static str,
        name: &'static str,
        icon: &'static str,
        link: &'static str,
    ) -> Self {
        let icon = format!("./assets/{}", icon);
        Self {
            class,
            name,
            icon,
            link,
        }
    }
}

pub struct Area {
    pub name: &'static str,
    pub items: Vec<Item>,
}

impl Area {
    pub fn new(name: &'static str, items: Vec<Item>) -> Self {
        Self { name, items }
    }
}
