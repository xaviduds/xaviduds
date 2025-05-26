use maud::PreEscaped;

use super::tech::Item;

pub struct Project {
    pub class: &'static str,
    pub name: &'static str,
    pub icon: &'static str,
    pub description: PreEscaped<&'static str>,
    pub links: Vec<Item>,
}
