use crate::routes::components::catalog::ComponentSpec;
use crate::routes::components::detail::page::{ApiRow, ComponentPage, primary_variant};
use crate::routes::components::palette::{PaletteColor, variant};
use yew::prelude::*;
use yew_duskmoon::Radio;

const RADIO_API: &[ApiRow] = &[
    ApiRow {
        prop: "class",
        ty: "Classes",
        default: "empty",
        docs: "Extra classes appended to the Radio wrapper.",
    },
    ApiRow {
        prop: "children",
        ty: "Children",
        default: "empty",
        docs: "Optional content rendered inside the wrapper.",
    },
    ApiRow {
        prop: "variant",
        ty: "Option<String>",
        default: "None",
        docs: "Appends a radio color class such as radio-primary.",
    },
];

pub fn page(spec: &'static ComponentSpec) -> ComponentPage {
    ComponentPage::new(spec, usage, RADIO_API, demo, color_variant)
}

fn usage(_: &ComponentSpec) -> String {
    "use yew_duskmoon::Radio;\n\nhtml! {\n    <label class=\"radio-label\">\n        <Radio variant={Some(\"primary\".to_owned())} class=\"demo-radio\" />\n        <span>{ \"Standard shipping\" }</span>\n    </label>\n}".to_owned()
}

fn demo(_: &ComponentSpec) -> Html {
    html! {
        <div class="radio-group">
            <span class="radio-group-label">{ "Delivery speed" }</span>
            <label class="radio-label">
                <Radio variant={primary_variant()} class="demo-radio" />
                <span>{ "Standard" }</span>
            </label>
            <label class="radio-label">
                <Radio class="demo-radio" />
                <span>{ "Express" }</span>
            </label>
            <label class="radio-label">
                <Radio class="demo-radio" />
                <span>{ "Overnight" }</span>
            </label>
        </div>
    }
}

fn color_variant(color: PaletteColor) -> Html {
    html! {
        <label class="radio-label">
            <Radio variant={variant(color)} class="demo-radio" />
            <span>{ color.label }</span>
        </label>
    }
}
