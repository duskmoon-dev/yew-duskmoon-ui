use crate::routes::components::catalog::ComponentSpec;
use crate::routes::components::detail::page::{primary_variant, ApiRow, ComponentPage};
use crate::routes::components::palette::{variant, PaletteColor};
use yew::prelude::*;
use yew_duskmoon::Navbar;

const API: &[ApiRow] = &[
    ApiRow {
        prop: "class",
        ty: "Classes",
        default: "empty",
        docs: "Extra classes appended to the navbar root; use it for surface, color, position, and density classes.",
    },
    ApiRow {
        prop: "children",
        ty: "Children",
        default: "empty",
        docs: "Navbar regions and links, typically navbar-start, navbar-center, navbar-end, navbar-brand, and navbar-item.",
    },
    ApiRow {
        prop: "variant",
        ty: "Option<String>",
        default: "None",
        docs: "Appends a color class such as navbar-primary to the root.",
    },
];

pub fn page(spec: &'static ComponentSpec) -> ComponentPage {
    ComponentPage::new(spec, usage, API, demo, color_variant)
}

fn usage(_: &ComponentSpec) -> String {
    r##"use yew_duskmoon::Navbar;

html! {
    <Navbar variant={Some("primary".to_owned())} class="navbar-sticky">
        <div class="navbar-start">
            <a href="/" class="navbar-brand">{ "Duskmoon" }</a>
        </div>
        <div class="navbar-center">
            <a href="#docs" class="navbar-item">{ "Docs" }</a>
            <a href="#api" class="navbar-item">{ "API" }</a>
        </div>
        <div class="navbar-end">
            <a href="#demo" class="navbar-item">{ "Demo" }</a>
        </div>
    </Navbar>
}"##
    .to_owned()
}

fn demo(_: &ComponentSpec) -> Html {
    html! {
        <Navbar variant={primary_variant()} class="component-detail-navbar-demo navbar-surface-container-high">
            <div class="navbar-start">
                <a href="#docs" class="navbar-brand">{ "Duskmoon" }</a>
            </div>
            <div class="navbar-center">
                <a href="#docs" class="navbar-item">{ "Docs" }</a>
                <a href="#api" class="navbar-item">{ "API" }</a>
                <a href="#demo" class="navbar-item">{ "Demo" }</a>
            </div>
            <div class="navbar-end">
                <a href="#colors" class="navbar-item">{ "Colors" }</a>
            </div>
        </Navbar>
    }
}

fn color_variant(color: PaletteColor) -> Html {
    html! {
        <Navbar variant={variant(color)} class="component-detail-color-navbar">
            <div class="navbar-start">
                <a href="#colors" class="navbar-brand">{ color.label }</a>
            </div>
            <div class="navbar-end">
                <a href="#demo" class="navbar-item">{ color.key }</a>
            </div>
        </Navbar>
    }
}
