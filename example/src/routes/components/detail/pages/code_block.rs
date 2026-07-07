use crate::routes::components::catalog::ComponentSpec;
use crate::routes::components::detail::page::{primary_variant, ApiRow, ComponentPage};
use crate::routes::components::palette::{variant, PaletteColor};
use yew::prelude::*;
use yew_duskmoon::CodeBlock;

const CODE_BLOCK_API: &[ApiRow] = &[
    ApiRow {
        prop: "class",
        ty: "Classes",
        default: "empty",
        docs: "Extra CSS classes appended to the code block root.",
    },
    ApiRow {
        prop: "children",
        ty: "Children",
        default: "empty",
        docs: "Header, language badge, copy button, and pre/code content.",
    },
    ApiRow {
        prop: "variant",
        ty: "Option<String>",
        default: "None",
        docs: "Appends a color modifier class such as code-block-primary.",
    },
];

pub fn page(spec: &'static ComponentSpec) -> ComponentPage {
    ComponentPage::new(spec, usage, CODE_BLOCK_API, demo, color_variant)
}

fn usage(_: &ComponentSpec) -> String {
    "use yew_duskmoon::CodeBlock;\n\nhtml! {\n    <CodeBlock>\n        <div class=\"code-header\">\n            <span class=\"code-title\">{ \"example.rs\" }</span>\n            <span class=\"code-language\">{ \"rust\" }</span>\n        </div>\n        <div class=\"code-content\">\n            <pre><code>{ \"html! { <Button>{ \\\"Save\\\" }</Button> }\" }</code></pre>\n        </div>\n    </CodeBlock>\n}".to_owned()
}

fn demo(_: &ComponentSpec) -> Html {
    html! {
        <CodeBlock variant={primary_variant()} class="code-block-compact">
            <div class="code-header">
                <span class="code-title">{ "example.rs" }</span>
                <span class="code-language">{ "rust" }</span>
                <button type="button" class="copy-button">
                    <span class="copy-text">{ "Copy" }</span>
                </button>
            </div>
            <div class="code-content">
                <pre><code>{ "html! {\n    <Button variant={Some(\"primary\".to_owned())}>{ \"Save\" }</Button>\n}" }</code></pre>
            </div>
        </CodeBlock>
    }
}

fn color_variant(color: PaletteColor) -> Html {
    html! {
        <div class="component-detail-color-demo">
            <CodeBlock variant={variant(color)} class="code-block-compact">
                <div class="code-header">
                    <span class="code-language">{ color.key }</span>
                </div>
                <div class="code-content">
                    <pre><code>{ format!("code-block-{}", color.key) }</code></pre>
                </div>
            </CodeBlock>
        </div>
    }
}
