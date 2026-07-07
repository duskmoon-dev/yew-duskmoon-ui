use crate::routes::components::catalog::ComponentSpec;
use crate::routes::components::detail::page::{primary_variant, ComponentPage, STANDARD_API};
use crate::routes::components::palette::{variant, PaletteColor};
use yew::prelude::*;
use yew_duskmoon::Progress;

pub fn page(spec: &'static ComponentSpec) -> ComponentPage {
    ComponentPage::new(spec, usage, STANDARD_API, demo, color_variant)
}

fn usage(_: &ComponentSpec) -> String {
    "use yew_duskmoon::Progress;\n\nhtml! {\n    <Progress variant={Some(\"primary\".to_owned())} class=\"progress-labeled\">\n        <div class=\"progress-bar\" style=\"width: 68%;\"></div>\n        <span class=\"progress-label\">{ \"68%\" }</span>\n    </Progress>\n}".to_owned()
}

fn demo(_: &ComponentSpec) -> Html {
    html! {
        <div class="detail-demo-stack">
            <Progress variant={primary_variant()} class="progress-labeled">
                <div class="progress-bar" style="width: 68%;"></div>
                <span class="progress-label">{ "68%" }</span>
            </Progress>
            <Progress variant={primary_variant()} class="progress-indeterminate">
                <div class="progress-bar"></div>
            </Progress>
            <div class="progress-circular">
                <svg class="progress-circular-svg" viewBox="0 0 48 48">
                    <circle class="progress-circular-track" cx="24" cy="24" r="20"></circle>
                    <circle
                        class="progress-circular-bar"
                        cx="24"
                        cy="24"
                        r="20"
                        stroke-dasharray="125.6"
                        stroke-dashoffset="37.68"
                    ></circle>
                </svg>
                <span class="progress-circular-label">{ "70%" }</span>
            </div>
        </div>
    }
}

fn color_variant(color: PaletteColor) -> Html {
    html! {
        <div class="component-detail-color-demo">
            <span>{ color.label }</span>
            <Progress variant={variant(color)} class="progress-labeled">
                <div class="progress-bar" style="width: 64%;"></div>
                <span class="progress-label">{ "64%" }</span>
            </Progress>
        </div>
    }
}
