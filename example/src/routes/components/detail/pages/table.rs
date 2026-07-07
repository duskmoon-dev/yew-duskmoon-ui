use crate::routes::components::catalog::ComponentSpec;
use crate::routes::components::detail::page::{primary_variant, ApiRow, ComponentPage};
use crate::routes::components::palette::{variant, PaletteColor};
use yew::prelude::*;
use yew_duskmoon::{Badge, Table};

const TABLE_API: &[ApiRow] = &[
    ApiRow {
        prop: "class",
        ty: "Classes",
        default: "empty",
        docs: "Extra CSS classes appended to the table wrapper root.",
    },
    ApiRow {
        prop: "children",
        ty: "Children",
        default: "empty",
        docs: "Structured row and cell markup rendered inside the table wrapper.",
    },
    ApiRow {
        prop: "variant",
        ty: "Option<String>",
        default: "None",
        docs: "Appends a color modifier class such as table-primary.",
    },
];

pub fn page(spec: &'static ComponentSpec) -> ComponentPage {
    ComponentPage::new(spec, usage, TABLE_API, demo, color_variant)
}

fn usage(_: &ComponentSpec) -> String {
    "use yew_duskmoon::{Badge, Table};\n\nhtml! {\n    <Table variant={Some(\"primary\".to_owned())} class=\"table-striped table-hover\">\n        <div class=\"component-detail-table-row is-head\">\n            <span>{ \"Release\" }</span>\n            <span>{ \"State\" }</span>\n        </div>\n        <div class=\"component-detail-table-row\">\n            <span>{ \"v1.16.0\" }</span>\n            <Badge variant={Some(\"success\".to_owned())}>{ \"Ready\" }</Badge>\n        </div>\n    </Table>\n}".to_owned()
}

fn demo(_: &ComponentSpec) -> Html {
    html! {
        <Table variant={primary_variant()} class="component-detail-table-demo table-striped table-hover">
            <div class="component-detail-table-row is-head">
                <span>{ "Release" }</span>
                <span>{ "State" }</span>
            </div>
            <div class="component-detail-table-row">
                <span>{ "v1.16.0" }</span>
                <Badge variant={primary_variant()}>{ "Ready" }</Badge>
            </div>
            <div class="component-detail-table-row">
                <span>{ "docs refresh" }</span>
                <Badge variant={Some("warning".to_owned())}>{ "Review" }</Badge>
            </div>
            <div class="component-detail-table-row">
                <span>{ "component audit" }</span>
                <Badge variant={Some("success".to_owned())}>{ "Done" }</Badge>
            </div>
        </Table>
    }
}

fn color_variant(color: PaletteColor) -> Html {
    html! {
        <Table variant={variant(color)} class="component-detail-color-table">
            <div>{ color.label }</div>
            <Badge variant={variant(color)}>{ html! { color.key } }</Badge>
        </Table>
    }
}
