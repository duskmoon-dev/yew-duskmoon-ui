use crate::routes::components::catalog::ComponentSpec;
use crate::routes::components::detail::page::{ComponentPage, BUTTON_API, ICON_BUTTON_API};
use crate::routes::components::palette::PaletteColor;
use yew::prelude::*;
use yew_duskmoon::{
    Button, ButtonAppearance, ButtonSize, ButtonType, Color, IconButton, NativeButtonType, Tooltip,
    TooltipPlacement,
};

pub fn page(spec: &'static ComponentSpec) -> ComponentPage {
    ComponentPage::new(spec, usage, BUTTON_API, demo, color_variant)
        .with_additional_api("IconButton props", ICON_BUTTON_API)
}

fn usage(_: &ComponentSpec) -> String {
    "use yew_duskmoon::{\n    Button, ButtonAppearance, ButtonSize, Color, IconButton, NativeButtonType,\n    Tooltip, TooltipPlacement,\n};\n\nhtml! {\n    <>\n        <Button\n            native_type={NativeButtonType::Submit}\n            appearance={Some(ButtonAppearance::Outlined)}\n            color={Color::Primary}\n            size={ButtonSize::Large}\n        >\n            { \"Publish\" }\n        </Button>\n        <IconButton\n            label=\"Open actions\"\n            color={Color::Primary}\n            tooltip_id=\"open-actions-tooltip\"\n            title=\"Open actions\"\n        >\n            <span aria-hidden=\"true\">{ \"⋯\" }</span>\n        </IconButton>\n        <Tooltip id=\"open-actions-tooltip\" placement={TooltipPlacement::Top}>\n            { \"Open actions\" }\n        </Tooltip>\n    </>\n}".to_owned()
}

fn demo(_: &ComponentSpec) -> Html {
    html! {
        <div class="component-detail-button-demo">
            <section class="component-detail-demo-group">
                <div>
                    <h3>{ "Button" }</h3>
                    <p>{ "Native types, typed appearances, and guarded disabled or loading states." }</p>
                </div>
                <div class="detail-demo-stack">
                    <Button
                        native_type={NativeButtonType::Submit}
                        color={Color::Primary}
                        classes="component-detail-action"
                    >
                        { "Publish" }
                    </Button>
                    <Button
                        appearance={Some(ButtonAppearance::Outlined)}
                        color={Color::Error}
                        classes="component-detail-action"
                        title="Delete the selected item"
                    >
                        { "Delete" }
                    </Button>
                    <Button
                        appearance={Some(ButtonAppearance::Text)}
                        aria_pressed={false}
                        classes="component-detail-action"
                    >
                        { "Toggle preview" }
                    </Button>
                    <Button
                        r#type={ButtonType::Link}
                        href="#api"
                        disabled={true}
                        aria_label="API link unavailable"
                        title="This link is currently unavailable"
                        classes="component-detail-action"
                    >
                        { "Disabled link" }
                    </Button>
                    <Button
                        appearance={Some(ButtonAppearance::Tonal)}
                        color={Color::Secondary}
                        loading={true}
                        classes="component-detail-action"
                    >
                        { "Saving" }
                    </Button>
                </div>
            </section>

            <section class="component-detail-demo-group">
                <div>
                    <h3>{ "IconButton" }</h3>
                    <p>
                        { "Every icon action requires a label. The first uses the safe default type=button; the reset action opts into type=reset." }
                    </p>
                </div>
                <div class="detail-demo-stack">
                    <div class="component-detail-tooltip-anchor">
                        <IconButton
                            label="Inspect item"
                            color={Color::Primary}
                            title="Inspect item"
                            tooltip_id="button-demo-inspect-tooltip"
                        >
                            { action_icon("M10.5 5a5.5 5.5 0 1 0 0 11 5.5 5.5 0 0 0 0-11Zm4 9.5 4.5 4.5") }
                        </IconButton>
                        <Tooltip id="button-demo-inspect-tooltip" placement={TooltipPlacement::Top}>
                            { "Inspect item" }
                        </Tooltip>
                    </div>
                    <IconButton
                        label="Add item"
                        appearance={ButtonAppearance::Tonal}
                        color={Color::Secondary}
                        size={ButtonSize::Small}
                        title="Add item"
                    >
                        { action_icon("M12 5v14M5 12h14") }
                    </IconButton>
                    <div class="component-detail-tooltip-anchor">
                        <IconButton
                            label="Delete item"
                            appearance={ButtonAppearance::Outlined}
                            color={Color::Error}
                            size={ButtonSize::Large}
                            title="Delete item"
                            tooltip_id="button-demo-delete-tooltip"
                        >
                            { action_icon("M4 7h16M9 7V4h6v3m-8 0 1 12h8l1-12M10 11v4m4-4v4") }
                        </IconButton>
                        <Tooltip
                            id="button-demo-delete-tooltip"
                            color={Color::Error}
                            placement={TooltipPlacement::Top}
                        >
                            { "Delete item" }
                        </Tooltip>
                    </div>
                    <IconButton
                        label="Refresh items"
                        appearance={ButtonAppearance::Filled}
                        color={Color::Info}
                        loading={true}
                        title="Refreshing items"
                    >
                        { action_icon("M20 7v5h-5M4 17v-5h5m9.4-3A7 7 0 0 0 6.7 6.7L4 9m16 6-2.7 2.3A7 7 0 0 1 5.6 15") }
                    </IconButton>
                    <IconButton
                        label="Previous item"
                        color={Color::Neutral}
                        size={ButtonSize::Small}
                        disabled={true}
                        title="Previous item unavailable"
                    >
                        { action_icon("m14 6-6 6 6 6") }
                    </IconButton>
                    <IconButton
                        label="Reset view"
                        native_type={NativeButtonType::Reset}
                        appearance={ButtonAppearance::Text}
                        color={Color::Primary}
                        title="Reset view"
                    >
                        { action_icon("M4 4v6h6M5.5 15a7 7 0 1 0 .5-7") }
                    </IconButton>
                </div>
            </section>
        </div>
    }
}

fn color_variant(color: PaletteColor) -> Html {
    html! {
        <Button
            color={color.color}
            appearance={Some(ButtonAppearance::Tonal)}
            classes="component-detail-color-button"
        >
            { html! { color.label } }
        </Button>
    }
}

fn action_icon(path: &'static str) -> Html {
    html! {
        <svg
            aria-hidden="true"
            viewBox="0 0 24 24"
            width="18"
            height="18"
            fill="none"
            stroke="currentColor"
            stroke-width="2"
            stroke-linecap="round"
            stroke-linejoin="round"
        >
            <path d={path} />
        </svg>
    }
}
