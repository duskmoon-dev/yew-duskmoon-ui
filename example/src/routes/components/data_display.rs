use yew::prelude::*;
use yew_duskmoon::typography::TypographyLevel;
use yew_duskmoon::{Badge, Card, Chip, List, Table, Typography};

use super::palette::{variant, PALETTE};

#[function_component(DataDisplayComponent)]
pub fn data_display_component() -> Html {
    html! {
        <div class="app">
            <div class="app-main component-main">
                <Typography level={TypographyLevel::H2}>{"Data Display Components"}</Typography>

                <Card title={html! { <Typography level={TypographyLevel::H4}>{"Badge Component"}</Typography> }} classes="component-card">
                    <div class="color-grid compact-color-grid">
                        { for PALETTE.into_iter().map(|color| html! {
                            <div class="color-cell">
                                <Badge variant={variant(color)}>{ html! { color.label } }</Badge>
                                <Badge variant={variant(color)} class="badge-soft">{ "Soft" }</Badge>
                                <code>{ format!("badge-{}", color.key) }</code>
                            </div>
                        }) }
                    </div>
                </Card>

                <Card title={html! { <Typography level={TypographyLevel::H4}>{"Chip Component"}</Typography> }} classes="component-card">
                    <div class="color-grid compact-color-grid">
                        { for PALETTE.into_iter().map(|color| html! {
                            <div class="color-cell">
                                <Chip variant={variant(color)}>{ html! { color.label } }</Chip>
                                <Chip variant={variant(color)} class="chip-tonal">{ "Tonal" }</Chip>
                                <code>{ format!("chip-{}", color.key) }</code>
                            </div>
                        }) }
                    </div>
                </Card>

                <Card title={html! { <Typography level={TypographyLevel::H4}>{"List Component"}</Typography> }} classes="component-card">
                    <List class="color-list">
                        { for PALETTE.into_iter().map(|color| html! {
                            <div class={classes!("color-list-row", format!("color-list-row-{}", color.key))}>
                                <span>{ color.label }</span>
                                <Badge variant={variant(color)}>{ html! { color.key } }</Badge>
                            </div>
                        }) }
                    </List>
                </Card>

                <Card title={html! { <Typography level={TypographyLevel::H4}>{"Table Component"}</Typography> }} classes="component-card">
                    <Table class="color-table">
                        <thead>
                            <tr>
                                <th>{"Token"}</th>
                                <th>{"Class"}</th>
                                <th>{"Status"}</th>
                            </tr>
                        </thead>
                        <tbody>
                            { for PALETTE.into_iter().map(|color| html! {
                                <tr class={format!("table-color-row table-color-row-{}", color.key)}>
                                    <td>{ color.label }</td>
                                    <td><code>{ format!("{} / {}-content", color.key, color.key) }</code></td>
                                    <td><Badge variant={variant(color)}>{ "Available" }</Badge></td>
                                </tr>
                            }) }
                        </tbody>
                    </Table>
                </Card>
            </div>
        </div>
    }
}
