use yew::prelude::*;
use yew_duskmoon::typography::TypographyLevel;
use yew_duskmoon::{Badge, Card, Chip, DmMarkdown, List, Table, Typography};

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
                    <div class="color-grid compact-color-grid">
                        { for PALETTE.into_iter().map(|color| html! {
                            <List variant={variant(color)} class="color-list">
                                <div class="color-list-row">
                                    <span>{ color.label }</span>
                                    <Badge variant={variant(color)}>{ html! { color.key } }</Badge>
                                </div>
                            </List>
                        }) }
                    </div>
                </Card>

                <Card title={html! { <Typography level={TypographyLevel::H4}>{"DmMarkdown Component"}</Typography> }} classes="component-card">
                    <div class="color-grid markdown-color-grid">
                        { for PALETTE.into_iter().map(|color| html! {
                            <DmMarkdown
                                variant={variant(color)}
                                class="markdown-token-card"
                                markdown={format!("**{}** markdown\n\n- [x] Rendered from source\n- Uses `markdown-body-{}`", color.label, color.key)}
                            />
                        }) }
                    </div>
                </Card>

                <Card title={html! { <Typography level={TypographyLevel::H4}>{"Table Component"}</Typography> }} classes="component-card">
                    <div class="color-grid compact-color-grid">
                        { for PALETTE.into_iter().map(|color| html! {
                            <Table variant={variant(color)} class="color-table">
                                <tbody>
                                    <tr class="table-color-row">
                                        <td>{ color.label }</td>
                                        <td><code>{ format!("table-{}", color.key) }</code></td>
                                        <td><Badge variant={variant(color)}>{ "Available" }</Badge></td>
                                    </tr>
                                </tbody>
                            </Table>
                        }) }
                    </div>
                </Card>
            </div>
        </div>
    }
}
