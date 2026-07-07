use crate::routes::components::catalog::ComponentSpec;
use crate::routes::components::detail::page::{primary_variant, ApiRow, ComponentPage};
use crate::routes::components::palette::{variant, PaletteColor};
use yew::prelude::*;
use yew_duskmoon::CircleMenu;

const API: &[ApiRow] = &[
    ApiRow {
        prop: "class",
        ty: "Classes",
        default: "empty",
        docs: "Extra classes appended to the circle-menu root; use it for size modifiers such as circle-menu-sm or circle-menu-lg.",
    },
    ApiRow {
        prop: "children",
        ty: "Children",
        default: "empty",
        docs: "Radial menu structure, typically a checkbox toggler, matching label, and circle-menu-list items.",
    },
    ApiRow {
        prop: "variant",
        ty: "Option<String>",
        default: "None",
        docs: "Appends a color class such as circle-menu-primary to the root.",
    },
];

pub fn page(spec: &'static ComponentSpec) -> ComponentPage {
    ComponentPage::new(spec, usage, API, demo, color_variant)
}

fn usage(_: &ComponentSpec) -> String {
    r##"use yew_duskmoon::CircleMenu;

html! {
    <CircleMenu variant={Some("primary".to_owned())} class="circle-menu-lg">
        <input type="checkbox" class="circle-menu-toggler" id="actions-menu" />
        <label class="circle-menu-label" for="actions-menu"></label>
        <ul class="circle-menu-list">
            <li class="circle-menu-item"><a href="#edit">{ "Edit" }</a></li>
            <li class="circle-menu-item"><a href="#copy">{ "Copy" }</a></li>
            <li class="circle-menu-item"><a href="#share">{ "Share" }</a></li>
        </ul>
    </CircleMenu>
}"##
    .to_owned()
}

fn demo(_: &ComponentSpec) -> Html {
    html! {
        <CircleMenu variant={primary_variant()} class="component-detail-circle-menu-demo circle-menu-lg">
            <input type="checkbox" class="circle-menu-toggler" id="detail-circle-menu" />
            <label class="circle-menu-label" for="detail-circle-menu" aria-label="Toggle quick actions"></label>
            <ul class="circle-menu-list">
                <li class="circle-menu-item"><a href="#docs">{ "Docs" }</a></li>
                <li class="circle-menu-item"><a href="#api">{ "API" }</a></li>
                <li class="circle-menu-item"><a href="#demo">{ "Demo" }</a></li>
                <li class="circle-menu-item"><a href="#colors">{ "Color" }</a></li>
                <li class="circle-menu-item"><button type="button">{ "Run" }</button></li>
                <li class="circle-menu-item"><button type="button">{ "Save" }</button></li>
            </ul>
        </CircleMenu>
    }
}

fn color_variant(color: PaletteColor) -> Html {
    let id = format!("detail-circle-menu-{}", color.key);

    html! {
        <CircleMenu variant={variant(color)} class="component-detail-color-demo circle-menu-sm">
            <input type="checkbox" class="circle-menu-toggler" id={id.clone()} />
            <label class="circle-menu-label" for={id} aria-label="Toggle color menu"></label>
            <ul class="circle-menu-list">
                <li class="circle-menu-item"><a href="#colors">{ color.label }</a></li>
                <li class="circle-menu-item"><a href="#api">{ color.key }</a></li>
                <li class="circle-menu-item"><button type="button">{ "More" }</button></li>
            </ul>
        </CircleMenu>
    }
}
