use crate::routes::components::catalog::ComponentSpec;
use crate::routes::components::detail::page::{ComponentPage, STANDARD_API};
use crate::routes::components::palette::{variant, PaletteColor};
use yew::prelude::*;
use yew_duskmoon::{Checkbox, Form, FormGroup, Input};

pub fn page(spec: &'static ComponentSpec) -> ComponentPage {
    ComponentPage::new(spec, usage, STANDARD_API, demo, color_variant)
}

fn usage(_: &ComponentSpec) -> String {
    r#"use yew_duskmoon::{Form, FormGroup, Input};

html! {
    <Form variant={Some("primary".to_owned())}>
        <FormGroup>
            <label class="form-label form-label-required">{ "Email" }</label>
            <Input class="input-bordered">{ "you@example.com" }</Input>
            <span class="form-hint">{ "Use your work email." }</span>
        </FormGroup>
        <div class="form-actions">
            <button class="btn btn-primary" type="submit">{ "Save" }</button>
        </div>
    </Form>
}"#
    .to_owned()
}

fn demo(_: &ComponentSpec) -> Html {
    html! {
        <Form variant={Some("primary".to_owned())} class="component-detail-demo-control">
            <div class="form-section">
                <h3 class="form-section-title">{ "Profile" }</h3>
                <p class="form-section-description">{ "A compact form using labels, inputs, validation copy, and actions." }</p>
            </div>
            <div class="form-grid form-grid-2">
                <FormGroup>
                    <label class="form-label form-label-required">{ "Full name" }</label>
                    <Input class="input-bordered">{ "Ada Lovelace" }</Input>
                    <span class="form-hint">{ "Shown on invoices and activity." }</span>
                </FormGroup>
                <FormGroup variant={Some("success".to_owned())}>
                    <label class="form-label">{ "Email" }</label>
                    <Input variant={Some("success".to_owned())} class="input-bordered">{ "ada@example.com" }</Input>
                    <span class="form-hint">{ "Verified." }</span>
                </FormGroup>
            </div>
            <Checkbox variant={Some("primary".to_owned())} class="checkbox-group">
                <label class="checkbox-label">
                    <input class="checkbox checkbox-primary" type="checkbox" checked={true} />
                    <span>{ "Send product updates" }</span>
                </label>
            </Checkbox>
            <div class="form-actions form-actions-right">
                <button class="btn btn-secondary" type="button">{ "Cancel" }</button>
                <button class="btn btn-primary" type="button">{ "Save changes" }</button>
            </div>
        </Form>
    }
}

fn color_variant(color: PaletteColor) -> Html {
    html! {
        <Form variant={variant(color)} class="component-detail-color-demo">
            <FormGroup>
                <label class="form-label">{ color.label }</label>
                <Input variant={variant(color)} class="input-bordered">
                    { html! { format!("form-{}", color.key) } }
                </Input>
            </FormGroup>
        </Form>
    }
}
