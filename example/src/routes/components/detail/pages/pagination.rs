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
        docs: "Manual page controls. When total is set, Pagination renders automatic controls instead.",
    },
    ApiRow {
        prop: "variant",
        ty: "Option<String>",
        default: "None",
        docs: "Appends a color class such as pagination-primary to the root.",
    },
    ApiRow {
        prop: "total",
        ty: "Option<usize>",
        default: "None",
        docs: "Total item count. Set this to enable automatic range, page count, current page, page-size, and refresh controls.",
    },
    ApiRow {
        prop: "page_size",
        ty: "usize",
        default: "10",
        docs: "Items per page used by automatic pagination.",
    },
    ApiRow {
        prop: "current",
        ty: "usize",
        default: "1",
        docs: "Current page. Values are clamped to the calculated page range.",
    },
    ApiRow {
        prop: "page_size_options",
        ty: "Vec<usize>",
        default: "[10, 20, 30, 50, 100]",
        docs: "Selectable page sizes. The active page size is included automatically when missing.",
    },
    ApiRow {
        prop: "on_change",
        ty: "Callback<usize>",
        default: "noop",
        docs: "Emitted with the requested page when previous, next, or the current-page input changes.",
    },
    ApiRow {
        prop: "on_page_size_change",
        ty: "Callback<usize>",
        default: "noop",
        docs: "Emitted with the selected page size.",
    },
    ApiRow {
        prop: "on_refresh",
        ty: "Callback<MouseEvent>",
        default: "noop",
        docs: "Emitted when the refresh control is clicked.",
    },
    ApiRow {
        prop: "show_refresh",
        ty: "bool",
        default: "true",
        docs: "Shows or hides the refresh control in automatic mode.",
    },
    ApiRow {
        prop: "page_size_label",
        ty: "AttrValue",
        default: "items/page",
        docs: "Text displayed after the page-size value.",
    },
    ApiRow {
        prop: "refresh_label",
        ty: "AttrValue",
        default: "Refresh",
        docs: "Refresh control label.",
    },
    ApiRow {
        prop: "aria_label",
        ty: "AttrValue",
        default: "Pagination",
        docs: "Accessible label for the automatic pagination navigation region.",
    },
];

pub fn page(spec: &'static ComponentSpec) -> ComponentPage {
    ComponentPage::new(spec, usage, API, demo, color_variant)
}

fn usage(_: &ComponentSpec) -> String {
    r##"use yew::prelude::*;
use yew_duskmoon::Pagination;

html! {
    <Pagination
        variant={Some("primary".to_owned())}
        total={Some(8038)}
        page_size={30}
        current={1}
        page_size_label="条/页"
        refresh_label="刷新"
        on_change={Callback::from(|_page: usize| {})}
        on_page_size_change={Callback::from(|_size: usize| {})}
    />
}"##
    .to_owned()
}

fn demo(_: &ComponentSpec) -> Html {
    html! {
        <Pagination
            variant={primary_variant()}
            class="component-detail-pagination-demo"
            total={Some(8038)}
            page_size={30}
            current={1}
            page_size_label="条/页"
            refresh_label="刷新"
        />
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
