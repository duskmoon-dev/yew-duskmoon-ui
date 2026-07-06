use yew::prelude::*;
use yew_duskmoon::typography::TypographyLevel;
use yew_duskmoon::{Card, Checkbox, Input, Radio, Switch, Textarea, Typography};

use super::palette::{variant, PALETTE};

#[function_component(FormComponent)]
pub fn form_component() -> Html {
    html! {
        <div class="app">
            <div class="app-main component-main">
                <Typography level={TypographyLevel::H2}>{"Form Components"}</Typography>

                <Card title={html! { <Typography level={TypographyLevel::H4}>{"Inputs and Textareas"}</Typography> }} classes="component-card">
                    <div class="color-grid form-color-grid">
                        { for PALETTE.into_iter().map(|color| html! {
                            <div class="form-color-cell">
                                <label>{ color.label }</label>
                                <Input variant={variant(color)} class="input-bordered color-input">
                                    { html! { format!("{} input", color.label) } }
                                </Input>
                                <Textarea variant={variant(color)} class="textarea-bordered color-textarea">
                                    { html! { format!("{} textarea", color.label) } }
                                </Textarea>
                            </div>
                        }) }
                    </div>
                </Card>

                <Card title={html! { <Typography level={TypographyLevel::H4}>{"Checkboxes, Radios, and Switches"}</Typography> }} classes="component-card">
                    <div class="color-grid control-color-grid">
                        { for PALETTE.into_iter().map(|color| html! {
                            <div class="control-color-cell">
                                <span>{ color.label }</span>
                                <div class="control-samples">
                                    <Checkbox variant={variant(color)} class="demo-check is-on" />
                                    <Radio variant={variant(color)} class="demo-radio is-on" />
                                    <Switch variant={variant(color)} class="demo-switch is-on" />
                                </div>
                                <code>{ format!("{}-*", color.key) }</code>
                            </div>
                        }) }
                    </div>
                </Card>
            </div>
        </div>
    }
}
