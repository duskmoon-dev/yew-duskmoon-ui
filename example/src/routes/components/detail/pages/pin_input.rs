use crate::routes::components::catalog::ComponentSpec;
use crate::routes::components::detail::page::{ComponentPage, STANDARD_API};
use crate::routes::components::palette::{variant, PaletteColor};
use yew::prelude::*;
use yew_duskmoon::PinInput;

pub fn page(spec: &'static ComponentSpec) -> ComponentPage {
    ComponentPage::new(spec, usage, STANDARD_API, demo, color_variant)
}

fn usage(_: &ComponentSpec) -> String {
    r#"use yew_duskmoon::PinInput;

html! {
    <PinInput variant={Some("primary".to_owned())} class="pin-input-4">
        <input class="pin-input-field" type="password" maxlength="1" inputmode="numeric" />
        <input class="pin-input-field" type="password" maxlength="1" inputmode="numeric" />
        <input class="pin-input-field" type="password" maxlength="1" inputmode="numeric" />
        <input class="pin-input-field" type="password" maxlength="1" inputmode="numeric" />
    </PinInput>
}"#
    .to_owned()
}

fn demo(_: &ComponentSpec) -> Html {
    html! {
        <div class="pin-group">
            <label class="pin-label">{ "Security PIN" }</label>
            <PinInput variant={Some("primary".to_owned())} class="pin-input-4 pin-input-circle">
                { for ["1", "7", "", ""].into_iter().map(|value| html! {
                    <input class="pin-input-field" type="password" maxlength="1" inputmode="numeric" value={value} readonly={true} />
                }) }
            </PinInput>
            <span class="pin-helper">{ "Two digits entered. PIN fields remain masked." }</span>
        </div>
    }
}

fn color_variant(color: PaletteColor) -> Html {
    html! {
        <PinInput variant={variant(color)} class="component-detail-color-demo pin-input-4">
            <span class="pin-label">{ color.label }</span>
            { for ["", "", "", ""].into_iter().map(|_| html! {
                <input class="pin-input-field" type="password" maxlength="1" inputmode="numeric" placeholder="0" />
            }) }
            <code>{ format!("pin-input-{}", color.key) }</code>
        </PinInput>
    }
}
