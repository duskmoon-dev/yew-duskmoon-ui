use crate::routes::components::catalog::ComponentSpec;
use crate::routes::components::detail::page::{primary_variant, ApiRow, ComponentPage};
use crate::routes::components::palette::{variant, PaletteColor};
use yew::prelude::*;
use yew_duskmoon::Pagination;

const API: &[ApiRow] = &[
    ApiRow {
        prop: "class",
        ty: "Classes",
        default: "empty",
        docs: "Extra classes appended to the pagination root; use it for outlined, tonal, compact, size, or responsive classes.",
    },
    ApiRow {
        prop: "children",
        ty: "Children",
        default: "empty",
        docs: "Page controls, usually pagination-prev, pagination-item, pagination-item-active, and pagination-next links or buttons.",
    },
    ApiRow {
        prop: "variant",
        ty: "Option<String>",
        default: "None",
        docs: "Appends a color class such as pagination-primary to the root.",
    },
];

pub fn page(spec: &'static ComponentSpec) -> ComponentPage {
    ComponentPage::new(spec, usage, API, demo, color_variant)
}

fn usage(_: &ComponentSpec) -> String {
    r##"use yew_duskmoon::Pagination;

html! {
    <Pagination variant={Some("primary".to_owned())} class="pagination-outlined">
        <button class="pagination-prev" disabled={true}>{ "Previous" }</button>
        <a href="#page-1" class="pagination-item">{ "1" }</a>
        <a href="#page-2" class="pagination-item pagination-item-active">{ "2" }</a>
        <a href="#page-3" class="pagination-item">{ "3" }</a>
        <button class="pagination-next">{ "Next" }</button>
    </Pagination>
}"##
    .to_owned()
}

fn demo(_: &ComponentSpec) -> Html {
    html! {
        <Pagination variant={primary_variant()} class="component-detail-pagination-demo pagination-outlined pagination-responsive">
            <button class="pagination-prev" disabled={true}>{ "Previous" }</button>
            <a href="#page-1" class="pagination-item">{ "1" }</a>
            <a href="#page-2" class="pagination-item pagination-item-active is-active">{ "2" }</a>
            <a href="#page-3" class="pagination-item">{ "3" }</a>
            <a href="#page-4" class="pagination-item">{ "4" }</a>
            <button class="pagination-next">{ "Next" }</button>
        </Pagination>
    }
}

fn color_variant(color: PaletteColor) -> Html {
    html! {
        <Pagination variant={variant(color)} class="component-detail-color-pagination">
            <button class="pagination-prev">{ "<" }</button>
            <a href="#colors" class="pagination-item">{ "1" }</a>
            <a href="#demo" class="pagination-item pagination-item-active is-active">{ color.label }</a>
            <button class="pagination-next">{ ">" }</button>
        </Pagination>
    }
}
