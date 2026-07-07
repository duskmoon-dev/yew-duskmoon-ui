use crate::routes::components::catalog::ComponentSpec;
use crate::routes::components::detail::page::{ApiRow, ComponentPage, primary_variant};
use crate::routes::components::palette::{PaletteColor, variant};
use yew::prelude::*;
use yew_duskmoon::Select;

const SELECT_API: &[ApiRow] = &[
    ApiRow {
        prop: "class",
        ty: "Classes",
        default: "empty",
        docs: "Extra classes appended to the Select wrapper.",
    },
    ApiRow {
        prop: "children",
        ty: "Children",
        default: "empty",
        docs: "Selected-value display or custom select-like content.",
    },
    ApiRow {
        prop: "variant",
        ty: "Option<String>",
        default: "None",
        docs: "Appends a select color class such as select-primary.",
    },
];

pub fn page(spec: &'static ComponentSpec) -> ComponentPage {
    ComponentPage::new(spec, usage, SELECT_API, demo, color_variant)
}

fn usage(_: &ComponentSpec) -> String {
    "use yew_duskmoon::Select;\n\nhtml! {\n    <div class=\"select-container\">\n        <label class=\"select-label\">{ \"Environment\" }</label>\n        <Select variant={Some(\"primary\".to_owned())} class=\"select-outlined\">\n            { \"Production\" }\n        </Select>\n        <span class=\"select-helper\">{ \"Choose where this release will deploy.\" }</span>\n    </div>\n}".to_owned()
}

fn demo(_: &ComponentSpec) -> Html {
    html! {
        <div class="select-container">
            <label class="select-label">{ "Environment" }</label>
            <Select variant={primary_variant()} class="select-outlined">
                { "Production" }
            </Select>
            <span class="select-helper">{ "Choose where this release will deploy." }</span>
        </div>
    }
}

fn color_variant(color: PaletteColor) -> Html {
    html! {
        <Select variant={variant(color)} class="color-input">
            <span>{ color.label }</span>
        </Select>
    }
}
