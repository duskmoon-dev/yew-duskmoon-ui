use crate::routes::components::catalog::ComponentSpec;
use crate::routes::components::detail::page::{ApiRow, ComponentPage};
use crate::routes::components::palette::{PaletteColor, variant};
use yew::prelude::*;
use yew_duskmoon::Rating;

const RATING_API: &[ApiRow] = &[
    ApiRow {
        prop: "class",
        ty: "Classes",
        default: "empty",
        docs: "Extra classes appended to the rating root.",
    },
    ApiRow {
        prop: "children",
        ty: "Children",
        default: "empty",
        docs: "Rating items, value labels, or review counts rendered inside the root.",
    },
    ApiRow {
        prop: "variant",
        ty: "Option<String>",
        default: "None",
        docs: "Appends a rating color class such as rating-warning.",
    },
];

pub fn page(spec: &'static ComponentSpec) -> ComponentPage {
    ComponentPage::new(spec, usage, RATING_API, demo, color_variant)
}

fn usage(_: &ComponentSpec) -> String {
    "use yew_duskmoon::Rating;\n\nhtml! {\n    <Rating variant={Some(\"warning\".to_owned())} class=\"rating-readonly\">\n        <span class=\"rating-item filled\">{ \"\\\\u{2605}\" }</span>\n        <span class=\"rating-item filled\">{ \"\\\\u{2605}\" }</span>\n        <span class=\"rating-item filled\">{ \"\\\\u{2605}\" }</span>\n        <span class=\"rating-item filled\">{ \"\\\\u{2605}\" }</span>\n        <span class=\"rating-item\">{ \"\\\\u{2606}\" }</span>\n        <span class=\"rating-count\">{ \"4.0\" }</span>\n    </Rating>\n}".to_owned()
}

fn demo(_: &ComponentSpec) -> Html {
    html! {
        <div class="rating-labeled">
            <span class="rating-label">{ "Product quality" }</span>
            <Rating variant={Some("warning".to_owned())} class="rating-readonly rating-animated">
                <span class="rating-item filled">{ "\u{2605}" }</span>
                <span class="rating-item filled">{ "\u{2605}" }</span>
                <span class="rating-item filled">{ "\u{2605}" }</span>
                <span class="rating-item filled">{ "\u{2605}" }</span>
                <span class="rating-item">{ "\u{2606}" }</span>
                <span class="rating-count">{ "(128 reviews)" }</span>
            </Rating>
        </div>
    }
}

fn color_variant(color: PaletteColor) -> Html {
    html! {
        <Rating variant={variant(color)} class="rating-readonly rating-compact">
            <span class="rating-item filled" style="color: var(--component-color);">{ "\u{2605}" }</span>
            <span class="rating-item filled" style="color: var(--component-color);">{ "\u{2605}" }</span>
            <span class="rating-item filled" style="color: var(--component-color);">{ "\u{2605}" }</span>
            <span class="rating-count">{ color.label }</span>
        </Rating>
    }
}
