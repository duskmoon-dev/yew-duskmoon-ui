use strum::IntoEnumIterator;
use yew::prelude::*;
use yew_duskmoon::button::ButtonType;
use yew_duskmoon::Card;
use yew_duskmoon::Button;

use super::palette::{variant, PALETTE};

/// Components page
#[function_component(ButtonComponent)]
pub fn component() -> Html {
    html! {
        <div class="app">
            <div class="app-main component-main">
                <Card title={ html!{ "Duskmoon Components - Button" } } classes="component-card">
                    <div class="component-stack">
                        <div class="code">
                            <pre>
                                {"use yew_duskmoon::Button;\n"}
                                {"use yew_duskmoon::button::ButtonType;\n"}
                            </pre>
                        </div>

                        <section class="component-section">
                            <h3>{ "Color variants" }</h3>
                            <div class="color-grid button-color-grid">
                                { for PALETTE.into_iter().map(|color| html! {
                                    <div class="color-cell">
                                        <Button variant={variant(color)} classes="color-button">
                                            { html! { color.label } }
                                        </Button>
                                        <code>{ format!("btn-{}", color.key) }</code>
                                    </div>
                                }) }
                            </div>
                        </section>

                        <section class="component-section">
                            <h3>{ "Type variants" }</h3>
                            <ul class="component-list">
                                {ButtonType::iter().into_iter().map(|t| {
                                    html!{
                                        <li class="component-example-row">
                                            <label>{ format!("ButtonType::{:?}", t) }</label>
                                            <div class="example-actions">
                                                <Button r#type={t.clone()}>{html! { format!("{:?}", t) }}</Button>
                                                <Button r#type={t.clone()} disabled={true}>{"Disabled"}</Button>
                                                <Button r#type={t.clone()} loading={true}>{"Loading"}</Button>
                                            </div>
                                            <code>{format!("html! {{ <Button r#type={{ButtonType::{}}}>Button</Button> }}", t.clone())}</code>
                                        </li>
                                    }
                                }).collect::<Html>()}
                            </ul>
                        </section>
                    </div>
                </Card>
            </div>
        </div>
    }
}
