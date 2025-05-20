pub mod card;
pub mod social;

use card::card;
use maud::html;

pub fn header() -> String {
    html!(
        header class="row" {
            (card())
        }
    )
    .0
}
