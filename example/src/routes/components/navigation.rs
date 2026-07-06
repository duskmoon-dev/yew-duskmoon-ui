use yew::prelude::*;
use yew_duskmoon::typography::TypographyLevel;
use yew_duskmoon::{Breadcrumbs, Card, Menu, Pagination, Stepper, Typography};

use super::palette::{variant, PALETTE};

#[function_component(NavigationComponent)]
pub fn navigation_component() -> Html {
    let current_page = use_state(|| 1);
    let select_page = {
        let current_page = current_page.clone();
        Callback::from(move |p: usize| current_page.set(p))
    };

    html! {
        <div class="app">
            <div class="app-main component-main">
                <Typography level={TypographyLevel::H2}>{"Navigation Components"}</Typography>

                <Card title={html! { <Typography level={TypographyLevel::H4}>{"Breadcrumbs"}</Typography> }} classes="component-card">
                    <div class="flex flex-col gap-4 w-full">
                        <Breadcrumbs class="color-breadcrumbs">
                            { for PALETTE.into_iter().map(|color| html! {
                                <>
                                    <span class={format!("breadcrumb-token breadcrumb-token-{}", color.key)}>{ color.label }</span>
                                    <span class="breadcrumb-divider">{"/"}</span>
                                </>
                            }) }
                            <span class="breadcrumb-current">{"Navigation"}</span>
                        </Breadcrumbs>
                    </div>
                </Card>

                <Card title={html! { <Typography level={TypographyLevel::H4}>{"Menu"}</Typography> }} classes="component-card">
                    <div class="color-grid menu-color-grid">
                        <Menu class="color-menu">
                            { for PALETTE.into_iter().map(|color| html! {
                                <a href="#menu" class={format!("menu-color-item menu-color-item-{}", color.key)}>
                                    <span>{ color.label }</span>
                                    <code>{ format!("menu-{}", color.key) }</code>
                                </a>
                            }) }
                        </Menu>
                    </div>
                </Card>

                <Card title={html! { <Typography level={TypographyLevel::H4}>{"Pagination"}</Typography> }} classes="component-card">
                    <div class="color-grid pagination-color-grid">
                        { for PALETTE.into_iter().map(|color| html! {
                            <div class="pagination-color-cell">
                                <span>{ color.label }</span>
                                <Pagination class={classes!("color-pagination", format!("pagination-{}", color.key))}>
                                    {
                                        for (1..=3).map(|p| {
                                            let select = select_page.clone();
                                            let active = p == *current_page;
                                            let btn_class = classes!("pagination-item", active.then_some("is-active"));
                                            html! {
                                                <button onclick={move |_| select.emit(p)} class={btn_class}>
                                                    { p }
                                                </button>
                                            }
                                        })
                                    }
                                </Pagination>
                            </div>
                        }) }
                    </div>
                </Card>

                <Card title={html! { <Typography level={TypographyLevel::H4}>{"Stepper"}</Typography> }} classes="component-card">
                    <div class="stepper-palette">
                        { for PALETTE.into_iter().map(|color| html! {
                            <Stepper variant={variant(color)} class="color-stepper">
                                <span>{ color.label }</span>
                                <div class="stepper-track">
                                    <i>{"1"}</i>
                                    <b></b>
                                    <i>{"2"}</i>
                                    <b class="is-muted"></b>
                                    <i class="is-muted">{"3"}</i>
                                </div>
                            </Stepper>
                        }) }
                    </div>
                </Card>
            </div>
        </div>
    }
}
