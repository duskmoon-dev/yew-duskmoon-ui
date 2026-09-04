use crate::routes::components::catalog::ComponentSpec;
use crate::routes::components::detail::page::{ApiRow, ComponentPage};
use crate::routes::components::palette::PaletteColor;
use yew::prelude::*;
use yew_duskmoon::{
    Button, ButtonAppearance, ButtonSize, Color, IconButton, Tooltip, TooltipContent,
    TooltipPlacement, TooltipTone,
};

const TOOLTIP_API: &[ApiRow] = &[
    ApiRow {
        prop: "id",
        ty: "AttrValue",
        default: "required",
        docs: "Stable DOM id shared with the trigger's tooltip_id or interestfor relationship.",
    },
    ApiRow {
        prop: "class",
        ty: "Classes",
        default: "empty",
        docs: "Extra CSS classes appended to the tooltip surface.",
    },
    ApiRow {
        prop: "children",
        ty: "Children",
        default: "empty",
        docs: "Plain, multiline, or rich tooltip surface content.",
    },
    ApiRow {
        prop: "color",
        ty: "Option<Color>",
        default: "None",
        docs: "Typed DuskMoon palette modifier. It takes precedence over tone.",
    },
    ApiRow {
        prop: "tone",
        ty: "TooltipTone",
        default: "Dark",
        docs: "Dark or Light base treatment used when no typed or legacy color is supplied.",
    },
    ApiRow {
        prop: "placement",
        ty: "TooltipPlacement",
        default: "Top",
        docs: "Preferred Top, Bottom, Left, or Right anchor position; core CSS supplies flip fallbacks.",
    },
    ApiRow {
        prop: "content",
        ty: "TooltipContent",
        default: "Plain",
        docs: "Plain, Multiline, or Rich content layout.",
    },
    ApiRow {
        prop: "interactive",
        ty: "bool",
        default: "false",
        docs: "Allows pointer interaction with the native popover surface.",
    },
    ApiRow {
        prop: "variant",
        ty: "Option<String>",
        default: "None",
        docs: "Legacy escape hatch that appends tooltip-{variant}. Precedence is variant, then color, then tone.",
    },
];

pub fn page(spec: &'static ComponentSpec) -> ComponentPage {
    ComponentPage::new(spec, usage, TOOLTIP_API, demo, color_variant)
}

fn usage(_: &ComponentSpec) -> String {
    "use yew_duskmoon::{Color, IconButton, Tooltip, TooltipPlacement};\n\nhtml! {\n    <>\n        <IconButton\n            label=\"Inspect item\"\n            color={Color::Primary}\n            title=\"Inspect item\"\n            tooltip_id=\"inspect-item-tooltip\"\n        >\n            <span aria-hidden=\"true\">{ \"⌕\" }</span>\n        </IconButton>\n        <Tooltip\n            id=\"inspect-item-tooltip\"\n            color={Color::Primary}\n            placement={TooltipPlacement::Top}\n        >\n            { \"Inspect item details\" }\n        </Tooltip>\n    </>\n}\n\n// Native display relies on Popover API, interestfor, and CSS Anchor Positioning.".to_owned()
}

fn demo(_: &ComponentSpec) -> Html {
    html! {
        <div class="detail-demo-stack">
            <div class="component-detail-tooltip-anchor">
                <Button
                    color={Color::Primary}
                    tooltip_id="tooltip-demo-top"
                    title="Tooltip above"
                >
                    { "Top" }
                </Button>
                <Tooltip
                    id="tooltip-demo-top"
                    color={Color::Primary}
                    placement={TooltipPlacement::Top}
                >
                    { "Tooltip above" }
                </Tooltip>
            </div>

            <div class="component-detail-tooltip-anchor">
                <Button
                    appearance={Some(ButtonAppearance::Outlined)}
                    tooltip_id="tooltip-demo-bottom"
                    title="Tooltip below"
                >
                    { "Bottom" }
                </Button>
                <Tooltip
                    id="tooltip-demo-bottom"
                    tone={TooltipTone::Light}
                    placement={TooltipPlacement::Bottom}
                >
                    { "Tooltip below" }
                </Tooltip>
            </div>

            <div class="component-detail-tooltip-anchor">
                <Button
                    appearance={Some(ButtonAppearance::Tonal)}
                    color={Color::Secondary}
                    tooltip_id="tooltip-demo-left"
                    title="Multiline tooltip"
                >
                    { "Multiline" }
                </Button>
                <Tooltip
                    id="tooltip-demo-left"
                    color={Color::Secondary}
                    placement={TooltipPlacement::Left}
                    content={TooltipContent::Multiline}
                >
                    { "Longer guidance wraps within the core tooltip width." }
                </Tooltip>
            </div>

            <div class="component-detail-tooltip-anchor">
                <IconButton
                    label="Show schedule details"
                    appearance={ButtonAppearance::Text}
                    color={Color::Info}
                    size={ButtonSize::Small}
                    title="Show schedule details"
                    tooltip_id="tooltip-demo-rich"
                >
                    { info_icon() }
                </IconButton>
                <Tooltip
                    id="tooltip-demo-rich"
                    tone={TooltipTone::Light}
                    placement={TooltipPlacement::Right}
                    content={TooltipContent::Rich}
                >
                    <strong class="tooltip-rich-title">{ "Schedule" }</strong>
                    <span class="tooltip-rich-description">{ "The next window starts at 22:00 UTC." }</span>
                </Tooltip>
            </div>
        </div>
    }
}

fn color_variant(color: PaletteColor) -> Html {
    let tooltip_id: AttrValue = format!("tooltip-color-{}", color.key).into();

    html! {
        <div class="component-detail-color-demo component-detail-tooltip-color">
            <Button
                color={color.color}
                appearance={Some(ButtonAppearance::Tonal)}
                tooltip_id={tooltip_id.clone()}
                title={format!("{} tooltip", color.label)}
            >
                { html! { color.label } }
            </Button>
            <Tooltip id={tooltip_id} color={color.color} placement={TooltipPlacement::Top}>
                { html! { format!("{} tooltip", color.label) } }
            </Tooltip>
            <code>{ format!("Color::{}", color.label) }</code>
        </div>
    }
}

fn info_icon() -> Html {
    html! {
        <svg
            aria-hidden="true"
            viewBox="0 0 24 24"
            width="16"
            height="16"
            fill="none"
            stroke="currentColor"
            stroke-width="2"
        >
            <circle cx="12" cy="12" r="9" />
            <path d="M12 11v5" />
            <path d="M12 8h.01" />
        </svg>
    }
}
