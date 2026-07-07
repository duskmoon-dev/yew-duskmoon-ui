use crate::routes::components::catalog::ComponentSpec;
use crate::routes::components::detail::page::{ComponentPage, STANDARD_API};
use crate::routes::components::palette::{variant, PaletteColor};
use yew::prelude::*;
use yew_duskmoon::OtpInput;

pub fn page(spec: &'static ComponentSpec) -> ComponentPage {
    ComponentPage::new(spec, usage, STANDARD_API, demo, color_variant)
}

fn usage(_: &ComponentSpec) -> String {
    r#"use yew_duskmoon::OtpInput;

html! {
    <OtpInput variant={Some("primary".to_owned())} class="otp-input-6">
        <input class="otp-input-field" type="text" maxlength="1" inputmode="numeric" />
        <input class="otp-input-field" type="text" maxlength="1" inputmode="numeric" />
        <input class="otp-input-field" type="text" maxlength="1" inputmode="numeric" />
        <input class="otp-input-field" type="text" maxlength="1" inputmode="numeric" />
        <input class="otp-input-field" type="text" maxlength="1" inputmode="numeric" />
        <input class="otp-input-field" type="text" maxlength="1" inputmode="numeric" />
    </OtpInput>
}"#
    .to_owned()
}

fn demo(_: &ComponentSpec) -> Html {
    html! {
        <div class="otp-group">
            <label class="otp-label">{ "Verification code" }</label>
            <OtpInput variant={Some("primary".to_owned())} class="otp-input-6 otp-input-wide">
                { for ["4", "8", "2", "", "", ""].into_iter().map(|value| html! {
                    <input class={classes!("otp-input-field", (!value.is_empty()).then_some("otp-input-field-filled"))} type="text" maxlength="1" inputmode="numeric" value={value} readonly={true} />
                }) }
            </OtpInput>
            <span class="otp-helper">{ "Enter the 6-digit code from your authenticator app." }</span>
        </div>
    }
}

fn color_variant(color: PaletteColor) -> Html {
    html! {
        <OtpInput variant={variant(color)} class="component-detail-color-demo otp-input-4">
            <span class="otp-label">{ color.label }</span>
            { for ["", "", "", ""].into_iter().map(|_| html! {
                <input class="otp-input-field" type="text" maxlength="1" inputmode="numeric" placeholder="0" />
            }) }
            <code>{ format!("otp-input-{}", color.key) }</code>
        </OtpInput>
    }
}
