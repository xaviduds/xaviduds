pub mod card;
pub mod controls;

use card::card;
use controls::controls;
use maud::html;

pub fn header() -> String {
    html!(
        header class="row mini_censa" {
            (card())
            (controls())
        }
    )
    .0
}
