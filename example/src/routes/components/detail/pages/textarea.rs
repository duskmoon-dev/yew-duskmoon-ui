use crate::routes::components::catalog::ComponentSpec;
use crate::routes::components::detail::page::{ApiRow, ComponentPage, primary_variant};
use crate::routes::components::palette::{PaletteColor, variant};
use yew::prelude::*;
use yew_duskmoon::Textarea;

const TEXTAREA_API: &[ApiRow] = &[
    ApiRow {
        prop: "class",
        ty: "Classes",
        default: "empty",
        docs: "Extra classes appended to the Textarea wrapper.",
    },
    ApiRow {
        prop: "children",
        ty: "Children",
        default: "empty",
        docs: "Textarea-like content rendered inside the wrapper.",
    },
    ApiRow {
        prop: "variant",
        ty: "Option<String>",
        default: "None",
        docs: "Appends a textarea color class such as textarea-primary.",
    },
];

pub fn page(spec: &'static ComponentSpec) -> ComponentPage {
    ComponentPage::new(spec, usage, TEXTAREA_API, demo, color_variant)
}

fn usage(_: &ComponentSpec) -> String {
    "use yew_duskmoon::Textarea;\n\nhtml! {\n    <div class=\"textarea-container\">\n        <label class=\"textarea-label\">{ \"Release notes\" }</label>\n        <Textarea variant={Some(\"primary\".to_owned())} class=\"textarea-outlined textarea-resize-none\">\n            { \"Summarize the user-visible changes...\" }\n        </Textarea>\n        <span class=\"textarea-helper\">{ \"Keep the note short and specific.\" }</span>\n    </div>\n}".to_owned()
}

fn demo(_: &ComponentSpec) -> Html {
    html! {
        <div class="textarea-container">
            <label class="textarea-label">{ "Release notes" }</label>
            <Textarea variant={primary_variant()} class="textarea-outlined textarea-resize-none">
                { "Improved deployment status messages and added audit links." }
            </Textarea>
            <span class="textarea-helper">{ "Keep the note short and specific." }</span>
        </div>
    }
}

fn color_variant(color: PaletteColor) -> Html {
    html! {
        <Textarea variant={variant(color)} class="color-textarea textarea-resize-none">
            <span>{ format!("{} textarea", color.label) }</span>
        </Textarea>
    }
}
