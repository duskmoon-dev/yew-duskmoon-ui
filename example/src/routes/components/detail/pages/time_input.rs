use crate::routes::components::catalog::ComponentSpec;
use crate::routes::components::detail::page::{ApiRow, ComponentPage, primary_variant};
use crate::routes::components::palette::{PaletteColor, variant};
use yew::prelude::*;
use yew_duskmoon::TimeInput;

const TIME_INPUT_API: &[ApiRow] = &[
    ApiRow {
        prop: "class",
        ty: "Classes",
        default: "empty",
        docs: "Extra classes appended to the time-input root.",
    },
    ApiRow {
        prop: "children",
        ty: "Children",
        default: "empty",
        docs: "Time segment fields, separators, period controls, or picker markup.",
    },
    ApiRow {
        prop: "variant",
        ty: "Option<String>",
        default: "None",
        docs: "Appends a time-input color class such as time-input-primary.",
    },
];

pub fn page(spec: &'static ComponentSpec) -> ComponentPage {
    ComponentPage::new(spec, usage, TIME_INPUT_API, demo, color_variant)
}

fn usage(_: &ComponentSpec) -> String {
    "use yew_duskmoon::TimeInput;\n\nhtml! {\n    <TimeInput variant={Some(\"primary\".to_owned())}>\n        <input type=\"text\" class=\"time-input-segment\" value=\"09\" maxlength=\"2\" />\n        <span class=\"time-input-separator\">{ \":\" }</span>\n        <input type=\"text\" class=\"time-input-segment\" value=\"30\" maxlength=\"2\" />\n        <div class=\"time-input-period\">\n            <button class=\"time-input-period-btn time-input-period-btn-active\">{ \"AM\" }</button>\n            <button class=\"time-input-period-btn\">{ \"PM\" }</button>\n        </div>\n    </TimeInput>\n}".to_owned()
}

fn demo(_: &ComponentSpec) -> Html {
    html! {
        <div class="form-group">
            <label class="form-label">{ "Meeting time" }</label>
            <TimeInput variant={primary_variant()}>
                <div class="time-input-segments">
                    <input type="text" class="time-input-segment" value="09" maxlength="2" />
                    <span class="time-input-separator">{ ":" }</span>
                    <input type="text" class="time-input-segment" value="30" maxlength="2" />
                    <div class="time-input-period">
                        <button class="time-input-period-btn time-input-period-btn-active">{ "AM" }</button>
                        <button class="time-input-period-btn">{ "PM" }</button>
                    </div>
                </div>
            </TimeInput>
            <span class="helper-text">{ "Select the time for your meeting." }</span>
        </div>
    }
}

fn color_variant(color: PaletteColor) -> Html {
    html! {
        <TimeInput variant={variant(color)}>
            <div class="time-input-segments" style="color: var(--component-color);">
                <input type="text" class="time-input-segment" value="12" maxlength="2" />
                <span class="time-input-separator">{ ":" }</span>
                <input type="text" class="time-input-segment" value="45" maxlength="2" />
            </div>
        </TimeInput>
    }
}
