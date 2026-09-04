use yew::prelude::*;
use yew_duskmoon::typography::TypographyLevel;
use yew_duskmoon::{
    Badge, BadgeAppearance, BadgeSize, Card, Chip, Color, DmMarkdown, List, Table, TableBorders,
    TableDensity, Typography,
};

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
                                <Badge color={color.color}>{ html! { color.label } }</Badge>
                                <Badge
                                    color={color.color}
                                    appearance={BadgeAppearance::Tonal}
                                    size={BadgeSize::Small}
                                >
                                    { "Tonal" }
                                </Badge>
                                <code>{ format!("Color::{}", color.label) }</code>
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
                                    <Badge color={color.color} size={BadgeSize::Small}>{ html! { color.key } }</Badge>
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
                    <Table
                        responsive={true}
                        hoverable={true}
                        striped={true}
                        density={TableDensity::Compact}
                        borders={TableBorders::Bordered}
                    >
                        <caption>{ "Semantic table using typed density and border modifiers" }</caption>
                        <thead>
                            <tr>
                                <th scope="col">{ "Record" }</th>
                                <th scope="col">{ "Category" }</th>
                                <th scope="col">{ "Status" }</th>
                            </tr>
                        </thead>
                        <tbody>
                            <tr>
                                <th scope="row">{ "Record A-104" }</th>
                                <td>
                                    <Badge
                                        color={Color::Secondary}
                                        appearance={BadgeAppearance::Tonal}
                                        size={BadgeSize::Small}
                                    >
                                        { "Standard" }
                                    </Badge>
                                </td>
                                <td>
                                    <Badge
                                        color={Color::Success}
                                        appearance={BadgeAppearance::Outlined}
                                        size={BadgeSize::Small}
                                        aria_label="Status: ready"
                                    >
                                        { "Ready" }
                                    </Badge>
                                </td>
                            </tr>
                            <tr>
                                <th scope="row">{ "Record B-208" }</th>
                                <td>
                                    <Badge
                                        color={Color::Tertiary}
                                        appearance={BadgeAppearance::Tonal}
                                        size={BadgeSize::Small}
                                    >
                                        { "External" }
                                    </Badge>
                                </td>
                                <td>
                                    <Badge
                                        color={Color::Warning}
                                        appearance={BadgeAppearance::Outlined}
                                        size={BadgeSize::Small}
                                        aria_label="Status: pending"
                                    >
                                        { "Pending" }
                                    </Badge>
                                </td>
                            </tr>
                        </tbody>
                    </Table>
                </Card>
            </div>
        </div>
    }
}
