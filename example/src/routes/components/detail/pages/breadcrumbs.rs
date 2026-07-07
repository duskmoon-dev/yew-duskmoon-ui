use crate::routes::components::catalog::ComponentSpec;
use crate::routes::components::detail::page::{primary_variant, ComponentPage, STANDARD_API};
use crate::routes::components::palette::{variant, PaletteColor};
use yew::prelude::*;
use yew_duskmoon::Breadcrumbs;

pub fn page(spec: &'static ComponentSpec) -> ComponentPage {
    ComponentPage::new(spec, usage, STANDARD_API, demo, color_variant)
}

fn usage(_: &ComponentSpec) -> String {
    "use yew_duskmoon::Breadcrumbs;\n\nhtml! {\n    <Breadcrumbs variant={Some(\"primary\".to_owned())} class=\"breadcrumbs-chevron\">\n        <a href=\"/\" class=\"breadcrumb-link\">{ \"Home\" }</a>\n        <span class=\"breadcrumb-separator\"></span>\n        <a href=\"/docs\" class=\"breadcrumb-link\">{ \"Docs\" }</a>\n        <span class=\"breadcrumb-separator\"></span>\n        <span class=\"breadcrumb-item breadcrumb-item-active\">{ \"Breadcrumbs\" }</span>\n    </Breadcrumbs>\n}".to_owned()
}

fn demo(spec: &ComponentSpec) -> Html {
    html! {
        <Breadcrumbs variant={primary_variant()} class="breadcrumbs-chevron breadcrumbs-contained component-detail-breadcrumbs-demo">
            <a href="#docs" class="breadcrumb-link">{ "Docs" }</a>
            <span class="breadcrumb-separator"></span>
            <a href="#components" class="breadcrumb-link">{ "Components" }</a>
            <span class="breadcrumb-separator"></span>
            <button class="breadcrumb-ellipsis" aria-label="Show hidden breadcrumbs"></button>
            <span class="breadcrumb-separator"></span>
            <span class="breadcrumb-item breadcrumb-item-active" aria-current="page">{ spec.name }</span>
        </Breadcrumbs>
    }
}

fn color_variant(color: PaletteColor) -> Html {
    html! {
        <Breadcrumbs variant={variant(color)} class="breadcrumbs-contained component-detail-color-breadcrumbs">
            <a href="#catalog" class="breadcrumb-link">{ "Catalog" }</a>
            <span class="breadcrumb-separator"></span>
            <span class="breadcrumb-item breadcrumb-item-active">{ color.label }</span>
        </Breadcrumbs>
    }
}
