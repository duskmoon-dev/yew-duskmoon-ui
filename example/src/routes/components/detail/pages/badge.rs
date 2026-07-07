use crate::routes::components::catalog::ComponentSpec;
use crate::routes::components::detail::page::{ApiRow, ComponentPage, primary_variant};
use crate::routes::components::palette::{variant, PaletteColor};
use yew::prelude::*;
use yew_duskmoon::Badge;

const BADGE_API: &[ApiRow] = &[
    ApiRow {
        prop: "class",
        ty: "Classes",
        default: "empty",
        docs: "Extra CSS classes appended to the badge root.",
    },
    ApiRow {
        prop: "children",
        ty: "Children",
        default: "empty",
        docs: "Short label, count, or status text rendered inside the badge.",
    },
    ApiRow {
        prop: "variant",
        ty: "Option<String>",
        default: "None",
        docs: "Appends a color modifier class such as badge-success.",
    },
];

pub fn page(spec: &'static ComponentSpec) -> ComponentPage {
    ComponentPage::new(spec, usage, BADGE_API, demo, color_variant)
}

fn usage(_: &ComponentSpec) -> String {
    "use yew_duskmoon::Badge;\n\nhtml! {\n    <Badge variant={Some(\"success\".to_owned())} class=\"badge-tonal badge-sm\">\n        { \"Active\" }\n    </Badge>\n}".to_owned()
}

fn demo(_: &ComponentSpec) -> Html {
    html! {
        <div class="detail-demo-stack">
            <Badge variant={primary_variant()}>{ "New" }</Badge>
            <Badge variant={Some("success".to_owned())} class="badge-tonal">{ "Active" }</Badge>
            <Badge variant={Some("warning".to_owned())} class="badge-outlined">{ "Pending" }</Badge>
            <Badge variant={Some("error".to_owned())} class="badge-notification">{ "5" }</Badge>
            <Badge variant={Some("info".to_owned())} class="badge-dot" />
        </div>
    }
}

fn color_variant(color: PaletteColor) -> Html {
    html! {
        <div class="component-detail-color-demo">
            <Badge variant={variant(color)} class="badge-tonal">
                <span>{ color.label }</span>
            </Badge>
            <code>{ format!("badge-{}", color.key) }</code>
        </div>
    }
}
