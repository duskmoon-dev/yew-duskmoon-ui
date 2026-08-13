use crate::routes::components::catalog::ComponentSpec;
use crate::routes::components::detail::page::{primary_variant, ApiRow, ComponentPage};
use crate::routes::components::palette::{variant, PaletteColor};
use yew::prelude::*;
use yew_duskmoon::MarkdownInput;

const MARKDOWN_INPUT_API: &[ApiRow] = &[
    ApiRow {
        prop: "class",
        ty: "Classes",
        default: "empty",
        docs: "Extra classes appended to the MarkdownInput root.",
    },
    ApiRow {
        prop: "value",
        ty: "Option<AttrValue>",
        default: "None",
        docs: "Controlled markdown source. When set, the host owns updates.",
    },
    ApiRow {
        prop: "default_value",
        ty: "AttrValue",
        default: "empty",
        docs: "Initial markdown source for uncontrolled use.",
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
        docs: "Emitted with the next markdown source after input.",
    },
    ApiRow {
        prop: "readonly",
        ty: "bool",
        default: "false",
        docs: "Prevents source edits while preserving preview access.",
    },
    ApiRow {
        prop: "auto_size",
        ty: "bool",
        default: "false",
        docs: "Applies field-sizing: content to the source textarea through the auto-resize class.",
    },
    ApiRow {
        prop: "preview",
        ty: "bool",
        default: "true",
        docs: "Shows Write and Preview tabs when enabled.",
    },
    ApiRow {
        prop: "variant",
        ty: "Option<String>",
        default: "None",
        docs: "Appends a markdown input color class such as markdown-input-primary.",
    },
];

const SAMPLE_MARKDOWN: &str = r##"# Release note

Ship the markdown input with:

- a real textarea source editor
- Write and Preview tabs
- sanitized markdown rendering via `DmMarkdown`

```rust
html! {
    <MarkdownInput default_value={"# Draft"} />
}
```
"##;

pub fn page(spec: &'static ComponentSpec) -> ComponentPage {
    ComponentPage::new(spec, usage, MARKDOWN_INPUT_API, demo, color_variant)
}

fn usage(_: &ComponentSpec) -> String {
    "use yew_duskmoon::MarkdownInput;\n\nhtml! {\n    <MarkdownInput\n        auto_size={true}\n        variant={Some(\"primary\".to_owned())}\n        default_value={\"# Release note\\n\\n- Write markdown\\n- Preview rendered output\"}\n        placeholder={\"Write markdown...\"}\n    />\n}".to_owned()
}

fn demo(_: &ComponentSpec) -> Html {
    html! {
        <MarkdownInput
            auto_size={true}
            variant={primary_variant()}
            default_value={SAMPLE_MARKDOWN}
            placeholder="Write markdown..."
        />
    }
}

fn color_variant(color: PaletteColor) -> Html {
    html! {
        <MarkdownInput
            variant={variant(color)}
            class="component-detail-color-markdown-input"
            default_value={format!("## {} input\n\nPreview markdown before submit.", color.label)}
            preview={false}
        />
    }
}
