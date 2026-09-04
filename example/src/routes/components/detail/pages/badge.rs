use crate::routes::components::catalog::ComponentSpec;
use crate::routes::components::detail::page::{ApiRow, ComponentPage};
use crate::routes::components::palette::PaletteColor;
use yew::prelude::*;
use yew_duskmoon::{Badge, BadgeAppearance, BadgeSize, Color};

const BADGE_API: &[ApiRow] = &[
    ApiRow {
        prop: "class",
        ty: "Classes",
        default: "empty",
        docs: "Extra CSS classes appended to the badge root.",
    },
    ApiRow {
        prop: "children",
        ty: "Children",
        default: "empty",
        docs: "Short label, count, or status text rendered inside the badge.",
    },
    ApiRow {
        prop: "color",
        ty: "Option<Color>",
        default: "None",
        docs: "Typed DuskMoon palette modifier. With no color, the core neutral surface treatment is used.",
    },
    ApiRow {
        prop: "appearance",
        ty: "BadgeAppearance",
        default: "Filled",
        docs: "Filled, Tonal, or Outlined treatment using the exact core class contract.",
    },
    ApiRow {
        prop: "size",
        ty: "BadgeSize",
        default: "Medium",
        docs: "Core-supported Small, Medium, or Large size.",
    },
    ApiRow {
        prop: "variant",
        ty: "Option<String>",
        default: "None",
        docs: "Legacy escape hatch that appends badge-{variant}. It takes precedence over color; typed appearance and size still apply.",
    },
    ApiRow {
        prop: "role",
        ty: "Option<AttrValue>",
        default: "None",
        docs: "Optional semantic role. Ordinary metadata badges intentionally receive no status role by default.",
    },
    ApiRow {
        prop: "aria_label",
        ty: "Option<AttrValue>",
        default: "None",
        docs: "Optional accessible label for status or count badges.",
    },
    ApiRow {
        prop: "title",
        ty: "Option<AttrValue>",
        default: "None",
        docs: "Optional native title text.",
    },
];

pub fn page(spec: &'static ComponentSpec) -> ComponentPage {
    ComponentPage::new(spec, usage, BADGE_API, demo, color_variant)
}

fn usage(_: &ComponentSpec) -> String {
    "use yew_duskmoon::{Badge, BadgeAppearance, BadgeSize, Color};\n\nhtml! {\n    <Badge\n        color={Color::Secondary}\n        appearance={BadgeAppearance::Tonal}\n        size={BadgeSize::Small}\n    >\n        { \"Metadata\" }\n    </Badge>\n}".to_owned()
}

fn demo(_: &ComponentSpec) -> Html {
    html! {
        <div class="detail-demo-stack">
            <Badge>{ "Metadata" }</Badge>
            <Badge
                color={Color::Secondary}
                appearance={BadgeAppearance::Tonal}
                size={BadgeSize::Small}
                title="Secondary metadata"
            >
                { "Metadata" }
            </Badge>
            <Badge color={Color::Warning} appearance={BadgeAppearance::Outlined}>
                { "Pending" }
            </Badge>
            <Badge
                color={Color::Success}
                appearance={BadgeAppearance::Tonal}
                role="status"
                aria_label="Service status: ready"
            >
                { "Ready" }
            </Badge>
        </div>
    }
}

fn color_variant(color: PaletteColor) -> Html {
    html! {
        <div class="component-detail-color-demo">
            <Badge color={color.color} appearance={BadgeAppearance::Tonal}>
                <span>{ color.label }</span>
            </Badge>
            <code>{ format!("Color::{}", color.label) }</code>
        </div>
    }
}
