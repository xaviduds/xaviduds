use maud::html;

pub fn comp_main() -> String {
    html!(
        input data-bind="input" {}
        div data-text="$input" {}
    )
    .0
}
