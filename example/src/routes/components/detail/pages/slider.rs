use crate::routes::components::catalog::ComponentSpec;
use crate::routes::components::detail::page::{ApiRow, ComponentPage, primary_variant};
use crate::routes::components::palette::{PaletteColor, variant};
use yew::prelude::*;
use yew_duskmoon::Slider;

const SLIDER_API: &[ApiRow] = &[
    ApiRow {
        prop: "class",
        ty: "Classes",
        default: "empty",
        docs: "Extra classes appended to the slider root.",
    },
    ApiRow {
        prop: "children",
        ty: "Children",
        default: "empty",
        docs: "Track, thumb, mark, and label markup rendered inside the root.",
    },
    ApiRow {
        prop: "variant",
        ty: "Option<String>",
        default: "None",
        docs: "Appends a slider color class such as slider-secondary.",
    },
];

pub fn page(spec: &'static ComponentSpec) -> ComponentPage {
    ComponentPage::new(spec, usage, SLIDER_API, demo, color_variant)
}

fn usage(_: &ComponentSpec) -> String {
    "use yew_duskmoon::Slider;\n\nhtml! {\n    <Slider variant={Some(\"secondary\".to_owned())} class=\"slider-labels-always\">\n        <div class=\"slider-track\">\n            <div class=\"slider-track-filled\" style=\"width: 70%;\"></div>\n        </div>\n        <div class=\"slider-thumb\" style=\"left: 70%;\">\n            <div class=\"slider-thumb-label\">{ \"70%\" }</div>\n        </div>\n    </Slider>\n}".to_owned()
}

fn demo(_: &ComponentSpec) -> Html {
    html! {
        <div style="width: min(100%, 420px);">
            <Slider variant={primary_variant()} class="slider-range slider-labels-always">
                <div class="slider-track">
                    <div class="slider-track-filled" style="left: 20%; width: 50%;"></div>
                </div>
                <div class="slider-marks">
                    <div class="slider-mark"></div>
                    <div class="slider-mark slider-mark-active"></div>
                    <div class="slider-mark slider-mark-active"></div>
                    <div class="slider-mark"></div>
                </div>
                <div class="slider-thumb" style="left: 20%;">
                    <div class="slider-thumb-label">{ "18" }</div>
                </div>
                <div class="slider-thumb" style="left: 70%;">
                    <div class="slider-thumb-label">{ "22" }</div>
                </div>
            </Slider>
            <div class="slider-labels">
                <span>{ "16C" }</span>
                <span>{ "24C" }</span>
            </div>
        </div>
    }
}

fn color_variant(color: PaletteColor) -> Html {
    html! {
        <Slider variant={variant(color)} class="slider-labels-always">
            <div class="slider-track">
                <div class="slider-track-filled" style="width: 64%; background-color: var(--component-color);"></div>
            </div>
            <div class="slider-thumb" style="left: 64%; background-color: var(--component-color);">
                <div class="slider-thumb-label" style="background-color: var(--component-color); color: var(--component-content);">
                    { color.label }
                </div>
            </div>
        </Slider>
    }
}
