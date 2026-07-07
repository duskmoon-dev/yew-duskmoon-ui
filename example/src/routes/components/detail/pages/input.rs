use crate::routes::components::catalog::ComponentSpec;
use crate::routes::components::detail::page::{ComponentPage, STANDARD_API};
use crate::routes::components::palette::{variant, PaletteColor};
use yew::prelude::*;
use yew_duskmoon::Input;

pub fn page(spec: &'static ComponentSpec) -> ComponentPage {
    ComponentPage::new(spec, usage, STANDARD_API, demo, color_variant)
}

fn usage(_: &ComponentSpec) -> String {
    r#"use yew_duskmoon::Input;

html! {
    <Input variant={Some("primary".to_owned())} class="input-bordered">
        <input class="input input-primary" type="text" placeholder="Enter text" />
    </Input>
}"#
    .to_owned()
}

fn demo(_: &ComponentSpec) -> Html {
    html! {
        <div class="detail-demo-stack">
            <Input variant={Some("primary".to_owned())} class="component-detail-demo-control">
                <label class="form-label">{ "Outlined input" }</label>
                <input class="input input-primary" type="text" value="Repository name" readonly={true} />
            </Input>
            <Input variant={Some("success".to_owned())} class="component-detail-demo-control">
                <label class="form-label">{ "Success state" }</label>
                <input class="input input-success" type="text" value="yew-duskmoon-ui" readonly={true} />
                <span class="form-hint">{ "Name is available." }</span>
            </Input>
        </div>
    }
}

fn color_variant(color: PaletteColor) -> Html {
    html! {
        <Input variant={variant(color)} class="component-detail-color-demo">
            <label class="form-label">{ color.label }</label>
            <input class={classes!("input", format!("input-{}", color.key))} type="text" placeholder={color.label} />
            <code>{ format!("input-{}", color.key) }</code>
        </Input>
    }
}
