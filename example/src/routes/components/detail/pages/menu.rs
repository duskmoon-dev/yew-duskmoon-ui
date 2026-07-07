use crate::routes::components::catalog::ComponentSpec;
use crate::routes::components::detail::page::{primary_variant, ApiRow, ComponentPage};
use crate::routes::components::palette::{variant, PaletteColor};
use yew::prelude::*;
use yew_duskmoon::Menu;

const API: &[ApiRow] = &[
    ApiRow {
        prop: "class",
        ty: "Classes",
        default: "empty",
        docs: "Extra classes appended to the menu root; use it for visibility and placement classes such as menu-show or menu-top.",
    },
    ApiRow {
        prop: "children",
        ty: "Children",
        default: "empty",
        docs: "Menu labels, menu-item buttons or links, dividers, trailing content, and nested submenu markup.",
    },
    ApiRow {
        prop: "variant",
        ty: "Option<String>",
        default: "None",
        docs: "Appends a color class such as menu-primary to the root.",
    },
];

pub fn page(spec: &'static ComponentSpec) -> ComponentPage {
    ComponentPage::new(spec, usage, API, demo, color_variant)
}

fn usage(_: &ComponentSpec) -> String {
    r##"use yew_duskmoon::Menu;

html! {
    <Menu variant={Some("primary".to_owned())} class="menu-show">
        <div class="menu-label">{ "Workspace" }</div>
        <button class="menu-item menu-item-active">{ "Overview" }</button>
        <button class="menu-item">{ "Invite members" }</button>
        <div class="menu-divider"></div>
        <a href="#settings" class="menu-item">{ "Settings" }</a>
    </Menu>
}"##
    .to_owned()
}

fn demo(_: &ComponentSpec) -> Html {
    html! {
        <Menu variant={primary_variant()} class="component-detail-menu-demo menu-show">
            <div class="menu-label">{ "Actions" }</div>
            <button class="menu-item menu-item-active">
                <span class="menu-item-icon">{ "01" }</span>
                { "Open docs" }
                <span class="menu-item-trailing">{ "D" }</span>
            </button>
            <button class="menu-item">
                <span class="menu-item-icon">{ "02" }</span>
                { "View API" }
            </button>
            <div class="menu-divider"></div>
            <a href="#colors" class="menu-item">
                <span class="menu-item-icon">{ "03" }</span>
                { "Color matrix" }
            </a>
        </Menu>
    }
}

fn color_variant(color: PaletteColor) -> Html {
    html! {
        <Menu variant={variant(color)} class="component-detail-color-menu menu-show">
            <button class="menu-item menu-item-active">{ color.label }</button>
            <button class="menu-item">{ color.key }</button>
        </Menu>
    }
}
