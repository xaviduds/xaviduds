use crate::presentation::components::header::{about::about, card::card};
use maud::html;

pub fn header() -> String {
    html!(
        header class="row" {
            (card())
        }
    )
    .0
}
