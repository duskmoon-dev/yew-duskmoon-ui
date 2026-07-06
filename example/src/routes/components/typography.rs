use strum::IntoEnumIterator;
use yew::prelude::*;
use yew_duskmoon::typography::TypographyLevel;
use yew_duskmoon::Card;
use yew_duskmoon::Typography;

use super::palette::PALETTE;

/// Components page
#[function_component(TypographyComponent)]
pub fn component() -> Html {
    html! {
        <div class="app">
            <div class="app-main component-main">
                <Card title={ html!{ "Duskmoon Components - Typography" } } classes="component-card">
                    <div class="component-stack">
                        <section class="component-section">
                            <h3>{ "Color variants" }</h3>
                            <div class="color-grid typography-color-grid">
                                { for PALETTE.into_iter().map(|color| html! {
                                    <div class={format!("typography-token typography-token-{}", color.key)}>
                                        <Typography level={TypographyLevel::H4}>{ html! { color.label } }</Typography>
                                        <Typography>{ html! { format!("text-{} / bg-{}", color.key, color.key) } }</Typography>
                                    </div>
                                }) }
                            </div>
                        </section>

                        <section class="component-section">
                            <h3>{ "Levels" }</h3>
                            <div class="component-list">
                                {TypographyLevel::iter().into_iter().map(|l| {
                                    html!{
                                        <div class="component-example-row">
                                            <label>{ format!("TypographyLevel::{:?}", l) }</label>
                                            <Typography level={l.clone()}>{html! { format!("Typography Level {:?}", l) }}</Typography>
                                            <code>{format!("html! {{ <Typography level={{TypographyLevel::{}}}>Typography</Typography> }}", l)}</code>
                                        </div>
                                    }
                                }).collect::<Html>()}
                            </div>
                        </section>
                    </div>
                </Card>
            </div>
        </div>
    }
}
