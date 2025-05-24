#[derive(Clone)]
pub struct Item {
    pub name: &'static str,
    pub icon: String,
    pub link: &'static str,
    pub classes: Vec<&'static str>,
}

pub struct Area {
    pub name: &'static str,
    pub items: Vec<Item>,
    pub class: &'static str,
}
