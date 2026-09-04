use crate::routes::components::catalog::ComponentSpec;
use crate::routes::components::detail::page::{ApiRow, ComponentPage};
use crate::routes::components::palette::{variant, PaletteColor};
use yew::prelude::*;
use yew_duskmoon::{Button, Popover, PopoverCommand, PopoverMode};

const API: &[ApiRow] = &[
    ApiRow {
        prop: "id",
        ty: "AttrValue",
        default: "required",
        docs: "Stable unique DOM id shared by the command trigger and native popover surface.",
    },
    ApiRow {
        prop: "class",
        ty: "Classes",
        default: "empty",
        docs: "Extra classes appended to the native popover surface; use placement and size modifiers here.",
    },
    ApiRow {
        prop: "children",
        ty: "Children",
        default: "empty",
        docs: "Content rendered directly inside the native popover surface.",
    },
    ApiRow {
        prop: "variant",
        ty: "Option<String>",
        default: "None",
        docs: "Appends a color class such as popover-primary to the surface.",
    },
    ApiRow {
        prop: "mode",
        ty: "PopoverMode",
        default: "Auto",
        docs: "Auto enables browser light-dismiss; Manual requires an explicit hide-popover command.",
    },
    ApiRow {
        prop: "command",
        ty: "PopoverCommand",
        default: "Toggle",
        docs: "Toggle, Show, or Hide command emitted by the generated trigger.",
    },
    ApiRow {
        prop: "trigger_label",
        ty: "AttrValue",
        default: "Show popover",
        docs: "Button label rendered by the command trigger.",
    },
    ApiRow {
        prop: "trigger_class",
        ty: "Classes",
        default: "btn btn-primary",
        docs: "CSS classes applied to the generated command trigger button.",
    },
];

pub fn page(spec: &'static ComponentSpec) -> ComponentPage {
    ComponentPage::new(spec, usage, API, demo, color_variant)
}

fn usage(_: &ComponentSpec) -> String {
    r##"use yew_duskmoon::{Button, Popover, PopoverCommand, PopoverMode};

html! {
    <Popover
        id="deployment-options"
        variant={Some("primary".to_owned())}
        class="popover-bottom"
        mode={PopoverMode::Manual}
        command={PopoverCommand::Show}
        trigger_label="Show popover"
    >
        <div class="popover-body">{ "Contextual content tied to the trigger." }</div>
        <div class="popover-footer">
            <Button command="hide-popover" command_for="deployment-options">
                { "Close" }
            </Button>
        </div>
    </Popover>
}"##
        .to_owned()
}

fn demo(_: &ComponentSpec) -> Html {
    html! {
        <div class="detail-demo-stack" style="min-height: 15rem; align-content: flex-start; align-items: flex-start;">
            <Popover
                id="demo-popover-auto"
                variant={Some("primary".to_owned())}
                class="popover-bottom popover-lg"
                trigger_label="Toggle auto popover"
            >
                <div class="popover-header">
                    <strong class="popover-title">{ "Deployment options" }</strong>
                </div>
                <div class="popover-body">
                    <p>{ "The browser owns top-layer placement, light-dismiss, and Escape handling." }</p>
                </div>
            </Popover>
            <Popover
                id="demo-popover-manual"
                variant={Some("secondary".to_owned())}
                class="popover-bottom popover-lg"
                mode={PopoverMode::Manual}
                command={PopoverCommand::Show}
                trigger_label="Open manual popover"
                trigger_class={classes!("btn", "btn-secondary")}
            >
                <div class="popover-header">
                    <strong class="popover-title">{ "Explicit dismissal" }</strong>
                </div>
                <div class="popover-body">
                    <p>{ "Manual mode stays open until a hide-popover command is invoked." }</p>
                </div>
                <div class="popover-footer">
                    <Button
                        appearance={Some(yew_duskmoon::ButtonAppearance::Text)}
                        command="hide-popover"
                        command_for="demo-popover-manual"
                    >
                        { "Close" }
                    </Button>
                </div>
            </Popover>
        </div>
    }
}

fn color_variant(color: PaletteColor) -> Html {
    let id: AttrValue = format!("popover-color-{}", color.key).into();

    html! {
        <div class="component-detail-color-demo">
            <Popover
                id={id}
                variant={variant(color)}
                class="popover-bottom"
                trigger_label={color.label}
                trigger_class={classes!("btn", format!("btn-{}", color.key))}
            >
                <div class="popover-body">
                    <div class="popover-title">{ color.label }</div>
                    <p>{ format!("popover-{}", color.key) }</p>
                </div>
            </Popover>
        </div>
    }
}
