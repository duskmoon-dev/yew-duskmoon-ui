use crate::routes::components::catalog::ComponentSpec;
use crate::routes::components::detail::page::{primary_variant, ApiRow, ComponentPage};
use crate::routes::components::palette::{variant, PaletteColor};
use yew::prelude::*;
use yew_duskmoon::Collapse;

const COLLAPSE_API: &[ApiRow] = &[
    ApiRow {
        prop: "class",
        ty: "Classes",
        default: "empty",
        docs: "Extra CSS classes appended to the collapse root.",
    },
    ApiRow {
        prop: "children",
        ty: "Children",
        default: "empty",
        docs: "Trigger and content markup rendered inside the collapse container.",
    },
    ApiRow {
        prop: "variant",
        ty: "Option<String>",
        default: "None",
        docs: "Appends a color modifier class such as collapse-primary.",
    },
];

pub fn page(spec: &'static ComponentSpec) -> ComponentPage {
    ComponentPage::new(spec, usage, COLLAPSE_API, demo, color_variant)
}

fn usage(_: &ComponentSpec) -> String {
    "use yew_duskmoon::Collapse;\n\nhtml! {\n    <Collapse variant={Some(\"primary\".to_owned())} class=\"collapse-open collapse-card\">\n        <button type=\"button\" class=\"collapse-trigger\" aria-expanded=\"true\">\n            <span>{ \"Deployment details\" }</span>\n            <span class=\"collapse-icon\" aria-hidden=\"true\">{ \"v\" }</span>\n        </button>\n        <div class=\"collapse-content\">\n            <p>{ \"Build, check, and publish steps are visible.\" }</p>\n        </div>\n    </Collapse>\n}".to_owned()
}

fn demo(_: &ComponentSpec) -> Html {
    html! {
        <Collapse variant={primary_variant()} class="collapse-open collapse-card component-detail-demo-control">
            <button type="button" class="collapse-trigger" aria-expanded="true">
                <span>{ "Deployment details" }</span>
                <span class="collapse-icon" aria-hidden="true">{ "v" }</span>
            </button>
            <div class="collapse-content">
                <p>{ "The visible panel demonstrates the trigger, content slot, and open-state class expected by the upstream CSS." }</p>
            </div>
        </Collapse>
    }
}

fn color_variant(color: PaletteColor) -> Html {
    html! {
        <Collapse variant={variant(color)} class="collapse-open component-detail-color-demo">
            <button type="button" class="collapse-trigger" aria-expanded="true">
                <span>{ color.label }</span>
                <span class="collapse-icon" aria-hidden="true">{ "v" }</span>
            </button>
            <div class="collapse-content">
                <code>{ format!("collapse-{}", color.key) }</code>
            </div>
        </Collapse>
    }
}
