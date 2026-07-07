use crate::routes::components::catalog::ComponentSpec;
use crate::routes::components::detail::page::{ApiRow, ComponentPage, primary_variant};
use crate::routes::components::palette::{PaletteColor, variant};
use yew::prelude::*;
use yew_duskmoon::Alert;

const ALERT_API: &[ApiRow] = &[
    ApiRow {
        prop: "class",
        ty: "Classes",
        default: "empty",
        docs: "Extra classes appended to the alert root.",
    },
    ApiRow {
        prop: "children",
        ty: "Children",
        default: "empty",
        docs: "Icon, content, action, and dismiss markup rendered inside the alert.",
    },
    ApiRow {
        prop: "variant",
        ty: "Option<String>",
        default: "None",
        docs: "Appends an alert color class such as alert-success.",
    },
];

pub fn page(spec: &'static ComponentSpec) -> ComponentPage {
    ComponentPage::new(spec, usage, ALERT_API, demo, color_variant)
}

fn usage(_: &ComponentSpec) -> String {
    "use yew_duskmoon::Alert;\n\nhtml! {\n    <Alert variant={Some(\"success\".to_owned())}>\n        <span class=\"alert-icon\">{ \"i\" }</span>\n        <span class=\"alert-content\">\n            <strong class=\"alert-title\">{ \"Deploy complete\" }</strong>\n            <span class=\"alert-description\">{ \"Production is running the latest build.\" }</span>\n        </span>\n    </Alert>\n}".to_owned()
}

fn demo(_: &ComponentSpec) -> Html {
    html! {
        <Alert variant={primary_variant()} class="alert-dismissible">
            <span class="alert-icon">{ "i" }</span>
            <span class="alert-content">
                <strong class="alert-title">{ "Deploy complete" }</strong>
                <span class="alert-description">{ "Production is running the latest build." }</span>
                <span class="alert-actions">
                    <button class="btn btn-sm btn-text" type="button">{ "View logs" }</button>
                </span>
            </span>
            <button class="alert-close" type="button" aria-label="Close">{ "x" }</button>
        </Alert>
    }
}

fn color_variant(color: PaletteColor) -> Html {
    html! {
        <Alert variant={variant(color)} class="alert-compact">
            <span class="alert-icon">{ "i" }</span>
            <span class="alert-content">
                <strong class="alert-title">{ color.label }</strong>
                <span class="alert-description">{ format!("alert-{}", color.key) }</span>
            </span>
        </Alert>
    }
}
