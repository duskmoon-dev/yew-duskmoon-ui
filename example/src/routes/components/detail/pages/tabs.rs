use crate::routes::components::catalog::ComponentSpec;
use crate::routes::components::detail::page::{primary_variant, ApiRow, ComponentPage};
use crate::routes::components::palette::{variant, PaletteColor};
use yew::prelude::*;
use yew_duskmoon::Tabs;

const API: &[ApiRow] = &[
    ApiRow {
        prop: "class",
        ty: "Classes",
        default: "empty",
        docs: "Extra classes appended to the tabs root; use it for pill, tonal, boxed, vertical, size, and alignment classes.",
    },
    ApiRow {
        prop: "children",
        ty: "Children",
        default: "empty",
        docs: "Tab buttons or links, usually using tab and tab-active classes, paired with tab-panel content outside or nearby.",
    },
    ApiRow {
        prop: "variant",
        ty: "Option<String>",
        default: "None",
        docs: "Appends a color class such as tabs-primary to the root.",
    },
];

pub fn page(spec: &'static ComponentSpec) -> ComponentPage {
    ComponentPage::new(spec, usage, API, demo, color_variant)
}

fn usage(_: &ComponentSpec) -> String {
    r##"use yew_duskmoon::Tabs;

html! {
    <>
        <Tabs variant={Some("primary".to_owned())} class="tabs-pill">
            <button class="tab tab-active" data-tab="overview">{ "Overview" }</button>
            <button class="tab" data-tab="activity">{ "Activity" }</button>
            <button class="tab" data-tab="settings">{ "Settings" }</button>
        </Tabs>
        <div class="tab-panel tab-panel-show" id="overview">
            <p>{ "Overview content" }</p>
        </div>
    </>
}"##
    .to_owned()
}

fn demo(_: &ComponentSpec) -> Html {
    html! {
        <div class="detail-demo-stack">
            <Tabs variant={primary_variant()} class="component-detail-tabs-demo tabs-pill">
                <button class="tab tab-active is-active" data-tab="tab-docs">{ "Docs" }</button>
                <button class="tab" data-tab="tab-api">{ "API" }</button>
                <button class="tab" data-tab="tab-demo">{ "Demo" }</button>
            </Tabs>
            <div class="tab-panel tab-panel-show" id="tab-docs">
                <p>{ "Usage, props, and variant examples stay grouped as sibling views." }</p>
            </div>
        </div>
    }
}

fn color_variant(color: PaletteColor) -> Html {
    html! {
        <Tabs variant={variant(color)} class="component-detail-color-tabs">
            <button class="tab tab-active is-active">{ color.label }</button>
            <button class="tab">{ color.key }</button>
        </Tabs>
    }
}
