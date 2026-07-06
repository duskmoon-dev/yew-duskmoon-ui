use yew::prelude::*;
use yew_duskmoon::typography::TypographyLevel;
use yew_duskmoon::{Card, Divider, Grid, GridColumns, GridGap, Typography};

use super::palette::{variant, PALETTE};

#[function_component(LayoutComponent)]
pub fn layout_component() -> Html {
    html! {
        <div class="app">
            <div class="app-main component-main">
                <Typography level={TypographyLevel::H2}>{"Layout Components"}</Typography>

                <Card title={html! { <Typography level={TypographyLevel::H4}>{"Divider Component"}</Typography> }} classes="component-card">
                    <div class="divider-palette">
                        { for PALETTE.into_iter().map(|color| html! {
                            <div class="divider-row">
                                <span>{ color.label }</span>
                                <Divider variant={variant(color)} class="color-divider" />
                                <code>{ format!("divider-{}", color.key) }</code>
                            </div>
                        }) }
                    </div>
                </Card>

                <Card title={html! { <Typography level={TypographyLevel::H4}>{"Grid Component"}</Typography> }} classes="component-card">
                    <div class="component-stack">
                        <div class="color-grid layout-color-grid">
                            { for PALETTE.into_iter().map(|color| html! {
                                <Grid variant={variant(color)} columns={Some(GridColumns::One)} gap={Some(GridGap::Sm)} class="layout-demo-grid grid-color-sample">
                                    <div class="grid-demo-tile">
                                        <span>{ color.label }</span>
                                        <code>{ format!("grid-{}", color.key) }</code>
                                    </div>
                                </Grid>
                            }) }
                        </div>

                        <Grid variant={Some("neutral".to_owned())} columns={Some(GridColumns::Three)} gap={Some(GridGap::Sm)} class="layout-demo-grid layout-demo-grid-fixed">
                            { for ["grid-cols-3", "gap-2", "fixed tracks"].into_iter().map(|label| html! {
                                <div class="grid-demo-tile">
                                    <span>{ label }</span>
                                </div>
                            }) }
                        </Grid>
                    </div>
                </Card>

                <Card title={html! { <Typography level={TypographyLevel::H4}>{"Base Surfaces"}</Typography> }} classes="component-card">
                    <div class="surface-scale">
                        { for ["base-100", "base-200", "base-300", "base-400", "base-500"].into_iter().map(|token| html! {
                            <div class={classes!("surface-token", format!("surface-token-{}", token))}>
                                <span>{ token }</span>
                            </div>
                        }) }
                    </div>
                </Card>
            </div>
        </div>
    }
}
