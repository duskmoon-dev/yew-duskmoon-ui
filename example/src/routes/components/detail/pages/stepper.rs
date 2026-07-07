use crate::routes::components::catalog::ComponentSpec;
use crate::routes::components::detail::page::{primary_variant, ApiRow, ComponentPage};
use crate::routes::components::palette::{variant, PaletteColor};
use yew::prelude::*;
use yew_duskmoon::Stepper;

const API: &[ApiRow] = &[
    ApiRow {
        prop: "class",
        ty: "Classes",
        default: "empty",
        docs: "Extra classes appended to the stepper root; use it for vertical, alternate label, and color classes.",
    },
    ApiRow {
        prop: "children",
        ty: "Children",
        default: "empty",
        docs: "Step markup built from stepper-step, stepper-step-button, icon, label, description, connector, and content nodes.",
    },
    ApiRow {
        prop: "variant",
        ty: "Option<String>",
        default: "None",
        docs: "Appends a color class such as stepper-primary to the root.",
    },
];

pub fn page(spec: &'static ComponentSpec) -> ComponentPage {
    ComponentPage::new(spec, usage, API, demo, color_variant)
}

fn usage(_: &ComponentSpec) -> String {
    r##"use yew_duskmoon::Stepper;

html! {
    <Stepper variant={Some("primary".to_owned())}>
        <div class="stepper-step stepper-step-completed">
            <button class="stepper-step-button">
                <div class="stepper-step-icon">{ "1" }</div>
                <span class="stepper-step-label">{ "Account" }</span>
            </button>
            <div class="stepper-step-connector"></div>
        </div>
        <div class="stepper-step stepper-step-active">
            <button class="stepper-step-button">
                <div class="stepper-step-icon">{ "2" }</div>
                <span class="stepper-step-label">{ "Profile" }</span>
            </button>
            <div class="stepper-step-connector"></div>
        </div>
    </Stepper>
}"##
    .to_owned()
}

fn demo(_: &ComponentSpec) -> Html {
    html! {
        <Stepper variant={primary_variant()} class="component-detail-stepper-demo stepper-alt-labels">
            <div class="stepper-step stepper-step-completed">
                <button class="stepper-step-button">
                    <div class="stepper-step-icon">{ "1" }</div>
                    <span class="stepper-step-label">{ "Account" }</span>
                    <span class="stepper-step-description">{ "Owner details" }</span>
                </button>
                <div class="stepper-step-connector"></div>
            </div>
            <div class="stepper-step stepper-step-active">
                <button class="stepper-step-button">
                    <div class="stepper-step-icon">{ "2" }</div>
                    <span class="stepper-step-label">{ "Profile" }</span>
                    <span class="stepper-step-description">{ "Theme choices" }</span>
                </button>
                <div class="stepper-step-connector"></div>
            </div>
            <div class="stepper-step stepper-step-optional">
                <button class="stepper-step-button">
                    <div class="stepper-step-icon">{ "3" }</div>
                    <span class="stepper-step-label">{ "Confirm" }</span>
                    <span class="stepper-step-description">{ "Optional" }</span>
                </button>
            </div>
        </Stepper>
    }
}

fn color_variant(color: PaletteColor) -> Html {
    html! {
        <Stepper variant={variant(color)} class="component-detail-color-stepper">
            <span>{ color.label }</span>
            <div class="stepper-track">
                <i>{ "1" }</i>
                <b></b>
                <i>{ "2" }</i>
            </div>
        </Stepper>
    }
}
