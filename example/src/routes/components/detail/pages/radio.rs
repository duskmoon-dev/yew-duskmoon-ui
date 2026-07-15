use crate::routes::components::catalog::ComponentSpec;
use crate::routes::components::detail::page::{primary_variant, ApiRow, ComponentPage};
use crate::routes::components::palette::{variant, PaletteColor};
use yew::prelude::*;
use yew_duskmoon::Radio;

const RADIO_API: &[ApiRow] = &[
    ApiRow {
        prop: "aria_label",
        ty: "AttrValue",
        default: "Radio options",
        docs: "Accessible name applied when Radio wraps a group of options.",
    },
    ApiRow {
        prop: "class",
        ty: "Classes",
        default: "empty",
        docs: "Extra classes appended to the Radio wrapper.",
    },
    ApiRow {
        prop: "children",
        ty: "Children",
        default: "empty",
        docs: "Optional content rendered inside the wrapper.",
    },
    ApiRow {
        prop: "variant",
        ty: "Option<String>",
        default: "None",
        docs: "Appends a radio color class such as radio-primary.",
    },
];

pub fn page(spec: &'static ComponentSpec) -> ComponentPage {
    ComponentPage::new(spec, usage, RADIO_API, demo, color_variant)
}

fn usage(_: &ComponentSpec) -> String {
    r#"use yew_duskmoon::Radio;

html! {
    <Radio variant={Some("primary".to_owned())} class="radio-group" aria_label="Shipping method">
        <label class="radio-label">
            <input class="radio radio-primary" type="radio" name="shipping" value="standard" checked={true} />
            <span>{ "Standard shipping" }</span>
        </label>
        <label class="radio-label">
            <input class="radio radio-primary" type="radio" name="shipping" value="express" />
            <span>{ "Express shipping" }</span>
        </label>
    </Radio>
}"#
    .to_owned()
}

fn demo(_: &ComponentSpec) -> Html {
    html! {
        <Radio variant={primary_variant()} class="component-detail-demo-control radio-group" aria_label="Delivery speed">
            <span class="radio-group-label">{ "Delivery speed" }</span>
            <label class="radio-label">
                <input class="radio radio-primary" type="radio" name="delivery-speed" value="standard" checked={true} />
                <span>{ "Standard" }</span>
            </label>
            <label class="radio-label">
                <input class="radio radio-primary" type="radio" name="delivery-speed" value="express" />
                <span>{ "Express" }</span>
            </label>
            <label class="radio-label">
                <input class="radio radio-primary" type="radio" name="delivery-speed" value="overnight" />
                <span>{ "Overnight" }</span>
            </label>
        </Radio>
    }
}

fn color_variant(color: PaletteColor) -> Html {
    html! {
        <Radio variant={variant(color)} class="component-detail-color-demo" aria_label={format!("{} radio", color.label)}>
            <label class="radio-label">
                <input class={classes!("radio", "component-detail-color-radio", format!("radio-{}", color.key))} type="radio" checked={true} />
                <span>{ color.label }</span>
            </label>
        </Radio>
    }
}
