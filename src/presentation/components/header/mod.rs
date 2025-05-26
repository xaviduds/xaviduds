pub mod card;
pub mod projects;

use card::card;
use maud::html;
use projects::projects;

pub fn header() -> String {
    html!(
        header.row.m_gap {
            (card())
            (projects())
        }
    )
    .0
}
