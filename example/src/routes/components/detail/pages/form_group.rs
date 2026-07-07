use crate::routes::components::catalog::ComponentSpec;
use crate::routes::components::detail::page::{ComponentPage, STANDARD_API};
use crate::routes::components::palette::{variant, PaletteColor};
use yew::prelude::*;
use yew_duskmoon::FormGroup;

pub fn page(spec: &'static ComponentSpec) -> ComponentPage {
    ComponentPage::new(spec, usage, STANDARD_API, demo, color_variant)
}

fn usage(_: &ComponentSpec) -> String {
    r#"use yew_duskmoon::FormGroup;

html! {
    <FormGroup variant={Some("primary".to_owned())}>
        <label class="form-label form-label-required">{ "Email address" }</label>
        <input class="input" type="email" placeholder="you@example.com" />
        <span class="form-hint">{ "We will use this for account notifications." }</span>
    </FormGroup>
}"#
    .to_owned()
}

fn demo(_: &ComponentSpec) -> Html {
    html! {
        <div class="detail-demo-stack">
            <FormGroup variant={Some("primary".to_owned())} class="component-detail-demo-control">
                <label class="form-label form-label-required">{ "Workspace name" }</label>
                <input class="input input-primary" type="text" value="Duskmoon" readonly={true} />
                <span class="form-hint">{ "Visible to every team member." }</span>
            </FormGroup>
            <FormGroup variant={Some("error".to_owned())} class="component-detail-demo-control form-group-error">
                <label class="form-label">{ "Slug" }</label>
                <input class="input input-error" type="text" value="dusk moon" readonly={true} />
                <span class="form-hint form-counter-error">{ "Use lowercase letters, numbers, or hyphens." }</span>
            </FormGroup>
        </div>
    }
}

fn color_variant(color: PaletteColor) -> Html {
    html! {
        <FormGroup variant={variant(color)} class="component-detail-color-demo">
            <label class="form-label">{ color.label }</label>
            <input class={classes!("input", format!("input-{}", color.key))} type="text" placeholder={format!("{} field", color.label)} />
            <span class="form-hint">{ format!("form-group-{}", color.key) }</span>
        </FormGroup>
    }
}
