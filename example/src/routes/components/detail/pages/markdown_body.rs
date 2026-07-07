use crate::routes::components::catalog::ComponentSpec;
use crate::routes::components::detail::page::{primary_variant, ApiRow, ComponentPage};
use crate::routes::components::palette::{variant, PaletteColor};
use yew::prelude::*;
use yew_duskmoon::MarkdownBody;

const MARKDOWN_BODY_API: &[ApiRow] = &[
    ApiRow {
        prop: "class",
        ty: "Classes",
        default: "empty",
        docs: "Extra CSS classes appended to the markdown body root.",
    },
    ApiRow {
        prop: "children",
        ty: "Children",
        default: "empty",
        docs: "Rendered markdown HTML such as headings, paragraphs, lists, tables, and code.",
    },
    ApiRow {
        prop: "variant",
        ty: "Option<String>",
        default: "None",
        docs: "Appends a color modifier class such as markdown-body-primary.",
    },
];

pub fn page(spec: &'static ComponentSpec) -> ComponentPage {
    ComponentPage::new(spec, usage, MARKDOWN_BODY_API, demo, color_variant)
}

fn usage(_: &ComponentSpec) -> String {
    "use yew_duskmoon::MarkdownBody;\n\nhtml! {\n    <MarkdownBody>\n        <h2>{ \"Release notes\" }</h2>\n        <p>{ \"Render parsed markdown inside this typography scope.\" }</p>\n        <pre><code>{ \"cargo check --target wasm32-unknown-unknown\" }</code></pre>\n    </MarkdownBody>\n}".to_owned()
}

fn demo(_: &ComponentSpec) -> Html {
    html! {
        <MarkdownBody variant={primary_variant()} class="component-detail-demo-control">
            <h2>{ "Release notes" }</h2>
            <p>{ "Markdown Body scopes generated prose so headings, links, lists, tables, blockquotes, and code share one readable rhythm." }</p>
            <ul>
                <li><strong>{ "Typography" }</strong>{ ": headings, paragraphs, links, and emphasis." }</li>
                <li><strong>{ "Code" }</strong>{ ": inline " }<code>{ "props.variant" }</code>{ " and fenced blocks." }</li>
            </ul>
            <blockquote>
                <p>{ "Use this wrapper after a markdown parser has produced sanitized HTML." }</p>
            </blockquote>
            <pre><code>{ "cargo check --manifest-path example/Cargo.toml --target wasm32-unknown-unknown" }</code></pre>
        </MarkdownBody>
    }
}

fn color_variant(color: PaletteColor) -> Html {
    html! {
        <MarkdownBody variant={variant(color)} class="component-detail-color-demo">
            <p>
                <strong>{ color.label }</strong>
                { " markdown scope" }
            </p>
            <code>{ format!("markdown-body-{}", color.key) }</code>
        </MarkdownBody>
    }
}
