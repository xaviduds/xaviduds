pub mod card;
pub mod projects;

use card::card;
use maud::html;
use projects::projects;

pub fn header() -> String {
    html!(
        header class="row s_gap" {
            (card())
            (projects())
        }
    )
    .0
}
