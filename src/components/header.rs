use crate::components::main::presentation::presentation;
use maud::html;

pub fn header() -> String {
    html!((presentation())).0
}
