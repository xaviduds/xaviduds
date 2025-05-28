pub mod card;

use card::card;
use maud::html;

pub fn header() -> String {
    html!(
        header.column.m_gap {
            (card())
        }
    )
    .0
}
