use crate::routes::components::catalog::ComponentSpec;
use crate::routes::components::detail::page::{ApiRow, ComponentPage, primary_variant};
use crate::routes::components::palette::{PaletteColor, variant};
use yew::prelude::*;
use yew_duskmoon::Switch;

const SWITCH_API: &[ApiRow] = &[
    ApiRow {
        prop: "class",
        ty: "Classes",
        default: "empty",
        docs: "Extra classes appended to the Switch wrapper.",
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
        docs: "Appends a switch color class such as switch-primary.",
    },
];

pub fn page(spec: &'static ComponentSpec) -> ComponentPage {
    ComponentPage::new(spec, usage, SWITCH_API, demo, color_variant)
}

fn usage(_: &ComponentSpec) -> String {
    "use yew_duskmoon::Switch;\n\nhtml! {\n    <label class=\"switch-label\">\n        <Switch variant={Some(\"success\".to_owned())} class=\"demo-switch\" />\n        <span>{ \"Enable notifications\" }</span>\n    </label>\n}".to_owned()
}

fn demo(_: &ComponentSpec) -> Html {
    html! {
        <div class="switch-group">
            <label class="switch-label">
                <Switch variant={primary_variant()} class="demo-switch" />
                <span>{ "Auto-save drafts" }</span>
            </label>
            <label class="switch-label">
                <Switch class="demo-switch" />
                <span>{ "Send weekly summary" }</span>
            </label>
        </div>
    }
}

fn color_variant(color: PaletteColor) -> Html {
    html! {
        <label class="switch-label">
            <Switch variant={variant(color)} class="demo-switch" />
            <span>{ color.label }</span>
        </label>
    }
}
