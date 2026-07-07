use crate::routes::components::catalog::ComponentSpec;
use crate::routes::components::detail::page::{primary_variant, ApiRow, ComponentPage};
use crate::routes::components::palette::{variant, PaletteColor};
use yew::prelude::*;
use yew_duskmoon::{Popover, PopoverTrigger};

const API: &[ApiRow] = &[
    ApiRow {
        prop: "class",
        ty: "Classes",
        default: "empty",
        docs: "Extra classes appended to the popover root; use it for placement, size, arrow, and elevation classes.",
    },
    ApiRow {
        prop: "children",
        ty: "Children",
        default: "empty",
        docs: "Popover content, usually wrapped in popover-body and optionally richer interactive controls.",
    },
    ApiRow {
        prop: "variant",
        ty: "Option<String>",
        default: "None",
        docs: "Appends a color class such as popover-primary to the root.",
    },
    ApiRow {
        prop: "trigger",
        ty: "PopoverTrigger",
        default: "Click",
        docs: "Controls whether the trigger button opens the popover on click, hover, or focus.",
    },
    ApiRow {
        prop: "trigger_label",
        ty: "AttrValue",
        default: "Show popover",
        docs: "Button label rendered as the popover trigger.",
    },
    ApiRow {
        prop: "trigger_class",
        ty: "Classes",
        default: "btn btn-primary",
        docs: "CSS classes applied to the generated trigger button.",
    },
];

pub fn page(spec: &'static ComponentSpec) -> ComponentPage {
    ComponentPage::new(spec, usage, API, demo, color_variant)
}

fn usage(_: &ComponentSpec) -> String {
    r##"use yew_duskmoon::{Popover, PopoverTrigger};

html! {
    <Popover
        variant={Some("primary".to_owned())}
        class="popover-bottom"
        trigger={PopoverTrigger::Click}
        trigger_label="Show popover"
    >
        <div class="popover-body">
            { "Contextual content tied to the trigger." }
        </div>
    </Popover>
}"##
    .to_owned()
}

fn demo(_: &ComponentSpec) -> Html {
    html! {
        <div class="detail-demo-stack" style="min-height: 15rem; align-content: flex-start; align-items: flex-start;">
            <Popover
                variant={primary_variant()}
                class="popover-bottom popover-lg popover-interactive"
                trigger={PopoverTrigger::Click}
                trigger_label="Click popover"
            >
                <div class="popover-body">
                    <strong class="popover-title">{ "Deployment options" }</strong>
                    <p>{ "Use popovers for lightweight contextual choices anchored to a nearby trigger." }</p>
                    <div class="detail-demo-stack">
                        <button class="btn btn-text">{ "Dismiss" }</button>
                        <button class="btn btn-primary">{ "Apply" }</button>
                    </div>
                </div>
            </Popover>
            <Popover
                variant={Some("secondary".to_owned())}
                class="popover-bottom popover-lg"
                trigger={PopoverTrigger::Hover}
                trigger_label="Hover popover"
                trigger_class={classes!("btn", "btn-secondary")}
            >
                <div class="popover-body">
                    <strong class="popover-title">{ "Hover trigger" }</strong>
                    <p>{ "The panel stays open while the pointer is over the trigger or popover." }</p>
                </div>
            </Popover>
            <Popover
                variant={Some("tertiary".to_owned())}
                class="popover-bottom popover-lg"
                trigger={PopoverTrigger::Focus}
                trigger_label="Focus popover"
                trigger_class={classes!("btn", "btn-tertiary")}
            >
                <div class="popover-body">
                    <strong class="popover-title">{ "Focus trigger" }</strong>
                    <p>{ "Keyboard focus can open the popover without requiring a pointer." }</p>
                </div>
            </Popover>
        </div>
    }
}

fn color_variant(color: PaletteColor) -> Html {
    html! {
        <div class="component-detail-color-demo">
            <Popover
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
