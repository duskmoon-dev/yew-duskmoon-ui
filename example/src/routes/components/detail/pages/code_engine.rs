use crate::routes::components::catalog::ComponentSpec;
use crate::routes::components::detail::page::{primary_variant, ApiRow, ComponentPage};
use crate::routes::components::palette::{variant, PaletteColor};
use yew::prelude::*;
use yew_duskmoon::{CodeEditor, CodeLanguage};

const CODE_ENGINE_API: &[ApiRow] = &[
    ApiRow {
        prop: "class",
        ty: "Classes",
        default: "empty",
        docs: "Extra CSS classes appended to the editor root.",
    },
    ApiRow {
        prop: "value",
        ty: "Option<AttrValue>",
        default: "None",
        docs: "Controlled source text. When set, the host owns updates.",
    },
    ApiRow {
        prop: "default_value",
        ty: "AttrValue",
        default: "empty",
        docs: "Initial source text for uncontrolled use.",
    },
    ApiRow {
        prop: "placeholder",
        ty: "AttrValue",
        default: "empty",
        docs: "Placeholder shown while the editor is empty.",
    },
    ApiRow {
        prop: "on_change",
        ty: "Callback<AttrValue>",
        default: "noop",
        docs: "Emitted with the next source text after input.",
    },
    ApiRow {
        prop: "readonly",
        ty: "bool",
        default: "false",
        docs: "Prevents edits while preserving source selection.",
    },
    ApiRow {
        prop: "show_line_numbers",
        ty: "bool",
        default: "true",
        docs: "Shows the line gutter beside the source field.",
    },
    ApiRow {
        prop: "show_status_bar",
        ty: "bool",
        default: "true",
        docs: "Shows cursor position, selection count, language, and line count below the editor.",
    },
    ApiRow {
        prop: "syntax_highlight",
        ty: "bool",
        default: "true",
        docs: "Renders a Rust-powered syntax highlight layer behind the editable textarea.",
    },
    ApiRow {
        prop: "language",
        ty: "CodeLanguage",
        default: "PlainText",
        docs: "Language metadata used for class names and the language badge.",
    },
    ApiRow {
        prop: "rows",
        ty: "usize",
        default: "12",
        docs: "Initial visible row count for the source field.",
    },
    ApiRow {
        prop: "variant",
        ty: "Option<String>",
        default: "None",
        docs: "Appends a color modifier class such as code-engine-primary.",
    },
    ApiRow {
        prop: "aria_label",
        ty: "AttrValue",
        default: "Code editor",
        docs: "Accessible label for the underlying source field.",
    },
];

const SAMPLE_RUST: &str = r#"use yew::prelude::*;
use code_engine::{CodeEditor, CodeLanguage};

const MAX_LINES: usize = 240;

// Clamp oversized buffers before rendering.
#[function_component(App)]
pub fn app() -> Html {
    let title: String = "Editor ready".to_owned();
    let limit = MAX_LINES.min(42);

    html! {
        <CodeEditor
            language={CodeLanguage::Rust}
            default_value={format!("fn main() {{ println!(\"{title}: {limit}\"); }}")}
        />
    }
}
"#;

pub fn page(spec: &'static ComponentSpec) -> ComponentPage {
    ComponentPage::new(spec, usage, CODE_ENGINE_API, demo, color_variant)
}

fn usage(_: &ComponentSpec) -> String {
    "use yew_duskmoon::{CodeEditor, CodeLanguage};\n\nhtml! {\n    <CodeEditor\n        variant={Some(\"primary\".to_owned())}\n        language={CodeLanguage::Rust}\n        default_value={\"fn main() {\\n    println!(\\\"hello\\\");\\n}\"}\n    />\n}".to_owned()
}

fn demo(_: &ComponentSpec) -> Html {
    html! {
        <CodeEditor
            variant={primary_variant()}
            language={CodeLanguage::Rust}
            default_value={SAMPLE_RUST}
            aria_label="Rust source"
        />
    }
}

fn color_variant(color: PaletteColor) -> Html {
    html! {
        <CodeEditor
            variant={variant(color)}
            class="component-detail-color-code-engine"
            language={CodeLanguage::Nix}
            default_value={format!("services.{} = {{\n  enable = true;\n}};", color.key)}
            rows={4}
            aria_label={format!("{} source", color.label)}
        />
    }
}
