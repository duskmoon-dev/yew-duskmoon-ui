use crate::routes::components::catalog::ComponentSpec;
use crate::routes::components::detail::page::{primary_variant, ComponentPage, STANDARD_API};
use crate::routes::components::palette::{variant, PaletteColor};
use yew::prelude::*;
use yew_duskmoon::Tooltip;

pub fn page(spec: &'static ComponentSpec) -> ComponentPage {
    ComponentPage::new(spec, usage, STANDARD_API, demo, color_variant)
}

fn usage(_: &ComponentSpec) -> String {
    "use yew_duskmoon::Tooltip;\n\nhtml! {\n    <Tooltip variant={Some(\"primary\".to_owned())} class=\"tooltip-top\">\n        <button class=\"btn btn-primary\">{ \"Hover me\" }</button>\n        <span class=\"tooltip-content\">{ \"This is a tooltip\" }</span>\n    </Tooltip>\n}".to_owned()
}

fn demo(_: &ComponentSpec) -> Html {
    html! {
        <div class="detail-demo-stack">
            <Tooltip variant={primary_variant()} class="tooltip-open tooltip-top">
                <button class="btn btn-primary">{ "Top" }</button>
                <span class="tooltip-content">{ "Tooltip on top" }</span>
            </Tooltip>
            <Tooltip variant={primary_variant()} class="tooltip-open tooltip-bottom">
                <button class="btn btn-primary">{ "Bottom" }</button>
                <span class="tooltip-content">{ "Tooltip below" }</span>
            </Tooltip>
            <Tooltip variant={Some("light".to_owned())} class="tooltip-open tooltip-right tooltip-rich">
                <button class="btn">{ "Rich" }</button>
                <span class="tooltip-content">
                    <strong class="tooltip-rich-title">{ "Deployment window" }</strong>
                    <span class="tooltip-rich-description">{ "Starts at 22:00 UTC." }</span>
                </span>
            </Tooltip>
        </div>
    }
}

fn color_variant(color: PaletteColor) -> Html {
    html! {
        <Tooltip variant={variant(color)} class="tooltip-open tooltip-top">
            <button class="btn">{ color.label }</button>
            <span class="tooltip-content">{ format!("tooltip-{}", color.key) }</span>
        </Tooltip>
    }
}
