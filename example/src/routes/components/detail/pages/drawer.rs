use crate::routes::components::catalog::ComponentSpec;
use crate::routes::components::detail::page::{primary_variant, ApiRow, ComponentPage};
use crate::routes::components::palette::{variant, PaletteColor};
use yew::prelude::*;
use yew_duskmoon::Drawer;

const API: &[ApiRow] = &[
    ApiRow {
        prop: "class",
        ty: "Classes",
        default: "empty",
        docs: "Extra classes appended to the drawer root; use it for position, open state, size, and surface classes.",
    },
    ApiRow {
        prop: "children",
        ty: "Children",
        default: "empty",
        docs: "Drawer sections such as drawer-header, drawer-body, drawer-item, dividers, labels, and footer actions.",
    },
    ApiRow {
        prop: "variant",
        ty: "Option<String>",
        default: "None",
        docs: "Appends a color class such as drawer-primary to the root.",
    },
];

pub fn page(spec: &'static ComponentSpec) -> ComponentPage {
    ComponentPage::new(spec, usage, API, demo, color_variant)
}

fn usage(_: &ComponentSpec) -> String {
    r##"use yew_duskmoon::Drawer;

html! {
    <Drawer variant={Some("primary".to_owned())} class="drawer-left drawer-open drawer-md">
        <div class="drawer-header">
            <h2 class="drawer-title">{ "Menu" }</h2>
            <button class="drawer-close" aria-label="Close drawer">{ "x" }</button>
        </div>
        <div class="drawer-body">
            <a href="#dashboard" class="drawer-item drawer-item-active">{ "Dashboard" }</a>
            <a href="#settings" class="drawer-item">{ "Settings" }</a>
        </div>
    </Drawer>
}"##
    .to_owned()
}

fn demo(_: &ComponentSpec) -> Html {
    html! {
        <Drawer variant={primary_variant()} class="component-detail-drawer-demo drawer-left drawer-open drawer-md">
            <div class="drawer-header">
                <h2 class="drawer-title">{ "Workspace" }</h2>
                <button class="drawer-close" aria-label="Close drawer">{ "x" }</button>
            </div>
            <div class="drawer-body">
                <a href="#docs" class="drawer-item drawer-item-active">
                    <span class="drawer-item-icon">{ "01" }</span>
                    { "Docs" }
                </a>
                <a href="#api" class="drawer-item">
                    <span class="drawer-item-icon">{ "02" }</span>
                    { "API" }
                </a>
                <div class="drawer-divider"></div>
                <span class="drawer-label">{ "Component" }</span>
                <a href="#demo" class="drawer-item">{ "Rendered demo" }</a>
                <a href="#colors" class="drawer-item">{ "Color variants" }</a>
            </div>
            <div class="drawer-footer">
                <button class="btn btn-primary">{ "Open" }</button>
            </div>
        </Drawer>
    }
}

fn color_variant(color: PaletteColor) -> Html {
    html! {
        <Drawer variant={variant(color)} class="component-detail-color-demo drawer-left drawer-open drawer-sm">
            <div class="drawer-header">
                <h2 class="drawer-title">{ color.label }</h2>
            </div>
            <div class="drawer-body">
                <a href="#colors" class="drawer-item drawer-item-active">{ color.key }</a>
                <a href="#demo" class="drawer-item">{ "Preview" }</a>
            </div>
        </Drawer>
    }
}
