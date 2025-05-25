pub mod card;

use card::card;
use maud::html;

pub fn header() -> String {
    html!(
        header class="row s_gap" {
            (card())
        }
    )
    .0
}
