use crate::routes::components::catalog::ComponentSpec;
use crate::routes::components::detail::page::{primary_variant, ApiRow, ComponentPage};
use crate::routes::components::palette::{variant, PaletteColor};
use yew::prelude::*;
use yew_duskmoon::Chip;

const CHIP_API: &[ApiRow] = &[
    ApiRow {
        prop: "class",
        ty: "Classes",
        default: "empty",
        docs: "Extra CSS classes appended to the chip root.",
    },
    ApiRow {
        prop: "children",
        ty: "Children",
        default: "empty",
        docs: "Chip label plus optional icon, avatar, or remove action markup.",
    },
    ApiRow {
        prop: "variant",
        ty: "Option<String>",
        default: "None",
        docs: "Appends a color modifier class such as chip-primary.",
    },
];

pub fn page(spec: &'static ComponentSpec) -> ComponentPage {
    ComponentPage::new(spec, usage, CHIP_API, demo, color_variant)
}

fn usage(_: &ComponentSpec) -> String {
    "use yew_duskmoon::Chip;\n\nhtml! {\n    <Chip variant={Some(\"primary\".to_owned())} class=\"chip-selectable chip-selected\">\n        { \"Selected filter\" }\n    </Chip>\n}".to_owned()
}

fn demo(_: &ComponentSpec) -> Html {
    html! {
        <div class="detail-demo-stack">
            <Chip variant={primary_variant()} class="chip-selectable chip-selected">
                { "Selected filter" }
            </Chip>
            <Chip class="chip-clickable chip-outlined">{ "Assist action" }</Chip>
            <Chip variant={Some("info".to_owned())}>
                <span class="chip-avatar" aria-hidden="true">{ "G" }</span>
                { "Gao" }
                <button type="button" class="chip-remove" aria-label="Remove Gao">{ "x" }</button>
            </Chip>
            <Chip variant={Some("warning".to_owned())} class="chip-sm">{ "Compact" }</Chip>
        </div>
    }
}

fn color_variant(color: PaletteColor) -> Html {
    html! {
        <div class="component-detail-color-demo">
            <Chip variant={variant(color)} class="chip-tonal">
                <span>{ color.label }</span>
            </Chip>
            <code>{ format!("chip-{}", color.key) }</code>
        </div>
    }
}
