pub mod stack;
pub mod stack_wasm;

use maud::html;
use stack::stack;

pub fn comp_main() -> String {
    html!(
        main class="filler column framed middle_x middle_y" {
            (stack())
        }
    )
    .0 + r#"
    <script type="module">
        import init from "./stack_wasm/wasm_example.js";
        init();
    </script>

    "#
}
