use crate::routes::components::catalog::ComponentSpec;
use crate::routes::components::detail::page::{primary_variant, ApiRow, ComponentPage};
use crate::routes::components::palette::{variant, PaletteColor};
use yew::prelude::*;
use yew_duskmoon::BottomSheet;

const API: &[ApiRow] = &[
    ApiRow {
        prop: "class",
        ty: "Classes",
        default: "empty",
        docs: "Extra classes appended to the local bottomsheet root; add upstream-style classes such as bottom-sheet-show or bottom-sheet-modal here.",
    },
    ApiRow {
        prop: "children",
        ty: "Children",
        default: "empty",
        docs: "Panel internals such as bottom-sheet-handle, header, title, close button, body, and footer actions.",
    },
    ApiRow {
        prop: "variant",
        ty: "Option<String>",
        default: "None",
        docs: "Appends a local color class such as bottomsheet-primary to the root.",
    },
];

pub fn page(spec: &'static ComponentSpec) -> ComponentPage {
    ComponentPage::new(spec, usage, API, demo, color_variant)
}

fn usage(_: &ComponentSpec) -> String {
    r##"use yew_duskmoon::BottomSheet;

html! {
    <BottomSheet variant={Some("primary".to_owned())} class="bottom-sheet bottom-sheet-show bottom-sheet-modal">
        <div class="bottom-sheet-handle"></div>
        <div class="bottom-sheet-header">
            <h2 class="bottom-sheet-title">{ "Actions" }</h2>
            <button class="bottom-sheet-close" aria-label="Close">{ "x" }</button>
        </div>
        <div class="bottom-sheet-body">
            <p>{ "Supplemental task content goes here." }</p>
        </div>
        <div class="bottom-sheet-footer">
            <button class="btn btn-text">{ "Cancel" }</button>
            <button class="btn btn-primary">{ "Confirm" }</button>
        </div>
    </BottomSheet>
}"##
    .to_owned()
}

fn demo(_: &ComponentSpec) -> Html {
    html! {
        <BottomSheet variant={primary_variant()} class="component-detail-bottomsheet-demo bottom-sheet bottom-sheet-show bottom-sheet-modal">
            <div class="bottom-sheet-handle"></div>
            <div class="bottom-sheet-header">
                <h2 class="bottom-sheet-title">{ "Quick actions" }</h2>
                <button class="bottom-sheet-close" aria-label="Close">{ "x" }</button>
            </div>
            <div class="bottom-sheet-body">
                <p>{ "Bottom sheets keep focused mobile tasks close to the thumb zone." }</p>
            </div>
            <div class="bottom-sheet-footer">
                <button class="btn btn-text">{ "Cancel" }</button>
                <button class="btn btn-primary">{ "Apply" }</button>
            </div>
        </BottomSheet>
    }
}

fn color_variant(color: PaletteColor) -> Html {
    html! {
        <BottomSheet variant={variant(color)} class="component-detail-color-demo bottom-sheet bottom-sheet-show">
            <div class="bottom-sheet-handle"></div>
            <div class="bottom-sheet-header">
                <h2 class="bottom-sheet-title">{ color.label }</h2>
            </div>
            <div class="bottom-sheet-body">
                <p>{ format!("bottomsheet-{}", color.key) }</p>
            </div>
        </BottomSheet>
    }
}
