use crate::routes::components::catalog::ComponentSpec;
use crate::routes::components::detail::page::{primary_variant, ApiRow, ComponentPage};
use crate::routes::components::palette::{variant, PaletteColor};
use yew::prelude::*;
use yew_duskmoon::NestedMenu;

const API: &[ApiRow] = &[
    ApiRow {
        prop: "class",
        ty: "Classes",
        default: "empty",
        docs: "Extra classes appended to the nested-menu root; use it for bordered, compact, and size variants.",
    },
    ApiRow {
        prop: "children",
        ty: "Children",
        default: "empty",
        docs: "Sidebar navigation content built from section titles, links, disabled items, and details/summary groups.",
    },
    ApiRow {
        prop: "variant",
        ty: "Option<String>",
        default: "None",
        docs: "Appends a color class such as nested-menu-primary to the root.",
    },
];

pub fn page(spec: &'static ComponentSpec) -> ComponentPage {
    ComponentPage::new(spec, usage, API, demo, color_variant)
}

fn usage(_: &ComponentSpec) -> String {
    r##"use yew_duskmoon::NestedMenu;

html! {
    <NestedMenu variant={Some("primary".to_owned())} class="nested-menu-bordered">
        <li class="nested-menu-title">{ "Components" }</li>
        <li><a href="#button" class="active">{ "Button" }</a></li>
        <li>
            <details open={true}>
                <summary>{ "Navigation" }</summary>
                <ul>
                    <li><a href="#menu">{ "Menu" }</a></li>
                    <li><a href="#tabs">{ "Tabs" }</a></li>
                </ul>
            </details>
        </li>
    </NestedMenu>
}"##
    .to_owned()
}

fn demo(_: &ComponentSpec) -> Html {
    html! {
        <NestedMenu variant={primary_variant()} class="component-detail-nested-menu-demo nested-menu-bordered">
            <li class="nested-menu-title">{ "Documentation" }</li>
            <li><a href="#docs" class="active" aria-current="page">{ "Usage" }</a></li>
            <li>
                <details open={true}>
                    <summary>{ "Navigation" }</summary>
                    <ul>
                        <li><a href="#menu">{ "Menu" }</a></li>
                        <li><a href="#navbar">{ "Navbar" }</a></li>
                        <li>
                            <details open={true}>
                                <summary>{ "Progress" }</summary>
                                <ul>
                                    <li><a href="#pagination">{ "Pagination" }</a></li>
                                    <li><a href="#stepper">{ "Stepper" }</a></li>
                                </ul>
                            </details>
                        </li>
                    </ul>
                </details>
            </li>
            <li class="disabled"><a href="#disabled" aria-disabled="true">{ "Archived" }</a></li>
        </NestedMenu>
    }
}

fn color_variant(color: PaletteColor) -> Html {
    html! {
        <NestedMenu variant={variant(color)} class="component-detail-color-demo nested-menu-compact">
            <li class="nested-menu-title">{ color.label }</li>
            <li><a href="#colors" class="active">{ color.key }</a></li>
            <li>
                <details open={true}>
                    <summary>{ "Group" }</summary>
                    <ul>
                        <li><a href="#demo">{ "Preview" }</a></li>
                    </ul>
                </details>
            </li>
        </NestedMenu>
    }
}
