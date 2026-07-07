use yew::prelude::*;
use yew_duskmoon::button::ButtonType;
use yew_duskmoon::typography::TypographyLevel;
use yew_duskmoon::{
    Accordion, Alert, Appbar, Autocomplete, Avatar, Badge, BottomNavigation, Bottomsheet,
    Breadcrumbs, Button, Card, Cascader, Checkbox, Chip, CircleMenu, CodeBlock, Collapse,
    Datepicker, Dialog, Divider, Drawer, FileUpload, Form, FormGroup, Grid, GridColumns, GridGap,
    Input, Link, List, MarkdownBody, Menu, Modal, MultiSelect, Navbar, NestedMenu, OtpInput,
    Pagination, PinInput, Popover, Progress, Radio, Rating, SegmentControl, Select, Skeleton,
    Slider, Snackbar, Stepper, Switch, Table, Tabs, Textarea, TimeInput, Timeline, Toast, Toggle,
    Tooltip, TreeSelect, Typography,
};

use super::catalog::{component_by_slug, ApiKind, ComponentSpec};
use super::palette::{variant, PaletteColor, PALETTE};
use super::ComponentsRoute;

#[derive(Clone, Copy, PartialEq, Eq)]
struct ApiRow {
    prop: &'static str,
    ty: &'static str,
    default: &'static str,
    docs: &'static str,
}

const STANDARD_API: &[ApiRow] = &[
    ApiRow {
        prop: "class",
        ty: "Classes",
        default: "empty",
        docs: "Extra CSS classes appended to the component root.",
    },
    ApiRow {
        prop: "children",
        ty: "Children",
        default: "empty",
        docs: "Content rendered inside the component root.",
    },
    ApiRow {
        prop: "variant",
        ty: "Option<String>",
        default: "None",
        docs: "Appends a component-specific modifier class such as component-primary.",
    },
];

const BUTTON_API: &[ApiRow] = &[
    ApiRow {
        prop: "classes",
        ty: "Classes",
        default: "empty",
        docs: "Extra CSS classes appended to the button or anchor.",
    },
    ApiRow {
        prop: "type",
        ty: "ButtonType",
        default: "Default",
        docs: "Visual and semantic button mode.",
    },
    ApiRow {
        prop: "variant",
        ty: "Option<String>",
        default: "None",
        docs: "Appends a color modifier class such as btn-primary.",
    },
    ApiRow {
        prop: "href",
        ty: "AttrValue",
        default: "empty",
        docs: "Anchor href used when type is Link.",
    },
    ApiRow {
        prop: "target",
        ty: "AttrValue",
        default: "empty",
        docs: "Anchor target used when type is Link.",
    },
    ApiRow {
        prop: "rel",
        ty: "AttrValue",
        default: "empty",
        docs: "Anchor rel used when type is Link.",
    },
    ApiRow {
        prop: "disabled",
        ty: "bool",
        default: "false",
        docs: "Prevents interaction and marks the button disabled.",
    },
    ApiRow {
        prop: "loading",
        ty: "bool",
        default: "false",
        docs: "Applies the loading state and prevents interaction.",
    },
    ApiRow {
        prop: "children",
        ty: "Children",
        default: "empty",
        docs: "Button label or icon content.",
    },
    ApiRow {
        prop: "onclick",
        ty: "Callback<MouseEvent>",
        default: "noop",
        docs: "Click handler emitted when the button is interactive.",
    },
];

const CARD_API: &[ApiRow] = &[
    ApiRow {
        prop: "classes",
        ty: "Classes",
        default: "empty",
        docs: "Extra CSS classes appended to the card root.",
    },
    ApiRow {
        prop: "title",
        ty: "Option<Html>",
        default: "None",
        docs: "Optional title rendered in the card header.",
    },
    ApiRow {
        prop: "variant",
        ty: "Option<String>",
        default: "None",
        docs: "Appends a color modifier class such as card-primary.",
    },
    ApiRow {
        prop: "children",
        ty: "Children",
        default: "empty",
        docs: "Card body content.",
    },
];

const GRID_API: &[ApiRow] = &[
    ApiRow {
        prop: "class",
        ty: "Classes",
        default: "empty",
        docs: "Extra CSS classes appended to the grid root.",
    },
    ApiRow {
        prop: "children",
        ty: "Children",
        default: "empty",
        docs: "Grid item content.",
    },
    ApiRow {
        prop: "variant",
        ty: "Option<String>",
        default: "None",
        docs: "Appends a color modifier class such as grid-primary.",
    },
    ApiRow {
        prop: "columns",
        ty: "Option<GridColumns>",
        default: "None",
        docs: "Column preset for fixed, auto-fill, or auto-fit tracks.",
    },
    ApiRow {
        prop: "gap",
        ty: "Option<GridGap>",
        default: "None",
        docs: "Gap preset between grid items.",
    },
    ApiRow {
        prop: "inline",
        ty: "bool",
        default: "false",
        docs: "Renders inline-grid instead of grid.",
    },
    ApiRow {
        prop: "style",
        ty: "AttrValue",
        default: "empty",
        docs: "Additional inline style appended after preset grid styles.",
    },
];

#[derive(Properties, Clone, PartialEq)]
pub struct ComponentDetailProps {
    pub slug: String,
}

#[function_component(ComponentDetail)]
pub fn component_detail(props: &ComponentDetailProps) -> Html {
    match component_by_slug(&props.slug) {
        Some(spec) => render_component_detail(spec),
        None => html! {
            <div class="app example-page component-detail-page">
                <main class="app-main component-detail-main">
                    <section class="detail-hero">
                        <span class="eyebrow">{ "Component catalog" }</span>
                        <Typography level={TypographyLevel::H1} classes="page-title">
                            { "Component not found." }
                        </Typography>
                        <Link<ComponentsRoute> to={ComponentsRoute::ComponentsRoot} classes="detail-back-link">
                            { "Back to catalog" }
                        </Link<ComponentsRoute>>
                    </section>
                </main>
            </div>
        },
    }
}

fn render_component_detail(spec: &'static ComponentSpec) -> Html {
    html! {
        <div class="app example-page component-detail-page">
            <section class="detail-hero component-detail-hero">
                <div>
                    <span class="eyebrow">{ spec.category }</span>
                    <h1 class="page-title">{ spec.name }</h1>
                </div>
                <div class="detail-hero-copy">
                    <p>{ spec.description }</p>
                    <Link<ComponentsRoute> to={ComponentsRoute::ComponentsRoot} classes="detail-back-link">
                        { "Back to catalog" }
                    </Link<ComponentsRoute>>
                </div>
            </section>

            <main class="app-main component-detail-main">
                <section id="docs" class="detail-section">
                    <div class="detail-section-head">
                        <span class="section-kicker">{ "Docs" }</span>
                        <h2>{ "Usage guidance" }</h2>
                    </div>
                    <div class="detail-copy">
                        <p>{ spec.docs }</p>
                        <p>
                            { "The Yew component is " }
                            <code>{ spec.component_name }</code>
                            { " and its base style class is " }
                            <code>{ spec.class_name }</code>
                            { "." }
                        </p>
                    </div>
                    <pre class="detail-code"><code>{ usage_example(spec) }</code></pre>
                </section>

                <section id="api" class="detail-section">
                    <div class="detail-section-head">
                        <span class="section-kicker">{ "API" }</span>
                        <h2>{ "Props" }</h2>
                    </div>
                    <div class="api-table-wrap">
                        <table class="api-table">
                            <thead>
                                <tr>
                                    <th>{ "Prop" }</th>
                                    <th>{ "Type" }</th>
                                    <th>{ "Default" }</th>
                                    <th>{ "Description" }</th>
                                </tr>
                            </thead>
                            <tbody>
                                { for api_rows(spec.api_kind).iter().map(|row| html! {
                                    <tr>
                                        <td><code>{ row.prop }</code></td>
                                        <td><code>{ row.ty }</code></td>
                                        <td>{ row.default }</td>
                                        <td>{ row.docs }</td>
                                    </tr>
                                }) }
                            </tbody>
                        </table>
                    </div>
                </section>

                <section id="demo" class="detail-section detail-demo-section">
                    <div class="detail-section-head">
                        <span class="section-kicker">{ "Demo" }</span>
                        <h2>{ "Rendered component" }</h2>
                    </div>
                    <div class="demo-stage">
                        { render_demo(spec) }
                    </div>
                </section>

                <section id="colors" class="detail-section detail-colors-section">
                    <div class="detail-section-head">
                        <span class="section-kicker">{ "Colors" }</span>
                        <h2>{ "Variant matrix" }</h2>
                    </div>
                    <div class="detail-color-grid">
                        { for PALETTE.into_iter().map(|color| render_color_variant(spec, color)) }
                    </div>
                </section>
            </main>
        </div>
    }
}

fn api_rows(kind: ApiKind) -> &'static [ApiRow] {
    match kind {
        ApiKind::Standard => STANDARD_API,
        ApiKind::Button => BUTTON_API,
        ApiKind::Card => CARD_API,
        ApiKind::Grid => GRID_API,
    }
}

fn usage_example(spec: &ComponentSpec) -> String {
    match spec.api_kind {
        ApiKind::Button => {
            "use yew_duskmoon::Button;\n\nhtml! {\n    <Button variant={Some(\"primary\".to_owned())}>{ \"Save\" }</Button>\n}".to_owned()
        },
        ApiKind::Card => {
            "use yew_duskmoon::Card;\n\nhtml! {\n    <Card variant={Some(\"primary\".to_owned())} title={html! { \"Card title\" }}>\n        { \"Card content\" }\n    </Card>\n}".to_owned()
        },
        ApiKind::Grid => {
            "use yew_duskmoon::{Grid, GridColumns, GridGap};\n\nhtml! {\n    <Grid variant={Some(\"primary\".to_owned())} columns={Some(GridColumns::AutoFit48)} gap={Some(GridGap::Md)}>\n        <div>{ \"Grid item\" }</div>\n    </Grid>\n}".to_owned()
        },
        ApiKind::Standard => format!(
            "use yew_duskmoon::{};\n\nhtml! {{\n    <{} variant={{Some(\"primary\".to_owned())}} class=\"{}-demo\">\n        {{ \"{} content\" }}\n    </{}>\n}}",
            spec.component_name, spec.component_name, spec.class_name, spec.name, spec.component_name
        ),
    }
}

fn primary_variant() -> Option<String> {
    Some("primary".to_owned())
}

fn secondary_variant() -> Option<String> {
    Some("secondary".to_owned())
}

fn tertiary_variant() -> Option<String> {
    Some("tertiary".to_owned())
}

fn demo_body(spec: &ComponentSpec) -> Html {
    html! {
        <>
            <strong>{ spec.name }</strong>
            <span>{ spec.description }</span>
        </>
    }
}

fn color_label(color: PaletteColor) -> Html {
    html! {
        <>
            <strong>{ color.label }</strong>
            <code>{ color.key }</code>
        </>
    }
}

macro_rules! standard_demo {
    ($component:ident, $spec:expr) => {
        html! {
            <$component variant={primary_variant()} class="component-detail-demo-control">
                { demo_body($spec) }
            </$component>
        }
    };
}

macro_rules! color_component {
    ($component:ident, $color:expr) => {
        html! {
            <$component variant={variant($color)} class="component-detail-color-demo">
                { color_label($color) }
            </$component>
        }
    };
}

fn render_color_variant(spec: &ComponentSpec, color: PaletteColor) -> Html {
    match spec.slug {
        "button" => html! {
            <Button variant={variant(color)} classes="component-detail-color-button">
                { html! { color.label } }
            </Button>
        },
        "card" => html! {
            <Card variant={variant(color)} title={html! { <span>{ color.label }</span> }} classes="component-detail-color-card">
                <span>{ format!("card-{}", color.key) }</span>
            </Card>
        },
        "grid" => html! {
            <Grid variant={variant(color)} columns={Some(GridColumns::Two)} gap={Some(GridGap::Xs)} class="component-detail-color-grid-demo">
                <span>{ color.label }</span>
                <code>{ format!("grid-{}", color.key) }</code>
            </Grid>
        },
        "list" => html! {
            <List variant={variant(color)} class="component-detail-color-list">
                <span>{ color.label }</span>
                <code>{ format!("list-{}", color.key) }</code>
            </List>
        },
        "table" => html! {
            <Table variant={variant(color)} class="component-detail-color-table">
                <div>{ color.label }</div>
                <Badge variant={variant(color)}>{ html! { color.key } }</Badge>
            </Table>
        },
        "breadcrumbs" => html! {
            <Breadcrumbs variant={variant(color)} class="component-detail-color-breadcrumbs">
                <span>{ "Catalog" }</span>
                <span>{ "/" }</span>
                <strong>{ color.label }</strong>
            </Breadcrumbs>
        },
        "pagination" => html! {
            <Pagination variant={variant(color)} class="component-detail-color-pagination">
                <span>{ color.label }</span>
                <button>{ "1" }</button>
                <button class="is-active">{ "2" }</button>
            </Pagination>
        },
        "stepper" => html! {
            <Stepper variant={variant(color)} class="component-detail-color-stepper">
                <span>{ color.label }</span>
                <div class="stepper-track">
                    <i>{ "1" }</i>
                    <b></b>
                    <i>{ "2" }</i>
                </div>
            </Stepper>
        },
        "tabs" => html! {
            <Tabs variant={variant(color)} class="component-detail-color-tabs">
                <button class="is-active">{ color.label }</button>
                <button>{ "API" }</button>
            </Tabs>
        },
        "menu" => html! {
            <Menu variant={variant(color)} class="component-detail-color-menu">
                <a href="#colors">{ color.label }</a>
                <a href="#demo">{ color.key }</a>
            </Menu>
        },
        "navbar" => html! {
            <Navbar variant={variant(color)} class="component-detail-color-navbar">
                <strong>{ color.label }</strong>
                <span>{ color.key }</span>
            </Navbar>
        },
        "accordion" => html! {
            <Accordion variant={variant(color)} class="component-detail-color-demo">
                <strong>{ color.label }</strong>
                <span>{ "Accordion" }</span>
            </Accordion>
        },
        "bottom-sheet" => html! {
            <Bottomsheet variant={variant(color)} class="component-detail-color-demo">
                <strong>{ color.label }</strong>
                <span>{ "Bottom sheet" }</span>
            </Bottomsheet>
        },
        "popover" => html! {
            <Popover variant={variant(color)} class="component-detail-color-demo">
                <strong>{ color.label }</strong>
                <span>{ "Popover" }</span>
            </Popover>
        },
        "avatar" => color_component!(Avatar, color),
        "badge" => color_component!(Badge, color),
        "chip" => color_component!(Chip, color),
        "code-block" => color_component!(CodeBlock, color),
        "collapse" => color_component!(Collapse, color),
        "markdown-body" => color_component!(MarkdownBody, color),
        "timeline" => color_component!(Timeline, color),
        "autocomplete" => color_component!(Autocomplete, color),
        "cascader" => color_component!(Cascader, color),
        "checkbox" => color_component!(Checkbox, color),
        "datepicker" => color_component!(Datepicker, color),
        "file-upload" => color_component!(FileUpload, color),
        "form" => color_component!(Form, color),
        "form-group" => color_component!(FormGroup, color),
        "input" => color_component!(Input, color),
        "multi-select" => color_component!(MultiSelect, color),
        "otp-input" => color_component!(OtpInput, color),
        "pin-input" => color_component!(PinInput, color),
        "radio" => color_component!(Radio, color),
        "rating" => color_component!(Rating, color),
        "segment-control" => color_component!(SegmentControl, color),
        "select" => color_component!(Select, color),
        "slider" => color_component!(Slider, color),
        "switch" => color_component!(Switch, color),
        "textarea" => color_component!(Textarea, color),
        "time-input" => color_component!(TimeInput, color),
        "tree-select" => color_component!(TreeSelect, color),
        "alert" => color_component!(Alert, color),
        "dialog" => color_component!(Dialog, color),
        "modal" => color_component!(Modal, color),
        "progress" => color_component!(Progress, color),
        "skeleton" => color_component!(Skeleton, color),
        "snackbar" => color_component!(Snackbar, color),
        "toast" => color_component!(Toast, color),
        "tooltip" => color_component!(Tooltip, color),
        "appbar" => color_component!(Appbar, color),
        "divider" => color_component!(Divider, color),
        "bottom-navigation" => color_component!(BottomNavigation, color),
        "circle-menu" => color_component!(CircleMenu, color),
        "drawer" => color_component!(Drawer, color),
        "nested-menu" => color_component!(NestedMenu, color),
        "toggle" => color_component!(Toggle, color),
        _ => html! {
            <div class="component-detail-color-demo">
                { color_label(color) }
            </div>
        },
    }
}

fn render_demo(spec: &ComponentSpec) -> Html {
    match spec.slug {
        "button" => html! {
            <div class="detail-demo-stack">
                <Button variant={primary_variant()} classes="component-detail-action">{ "Primary action" }</Button>
                <Button variant={secondary_variant()} classes="component-detail-action">{ "Secondary action" }</Button>
                <Button r#type={ButtonType::Link} variant={tertiary_variant()} href={"#api"} classes="component-detail-action">{ "API link" }</Button>
            </div>
        },
        "card" => html! {
            <Card variant={primary_variant()} title={html! { <span>{ "Card title" }</span> }} classes="component-detail-card-demo">
                <p>{ "Cards keep related content and actions in one readable surface." }</p>
            </Card>
        },
        "grid" => html! {
            <Grid variant={primary_variant()} columns={Some(GridColumns::AutoFit48)} gap={Some(GridGap::Md)} class="component-detail-grid-demo">
                { for ["Auto", "Fit", "Grid"].into_iter().map(|label| html! {
                    <div class="component-detail-grid-item">{ label }</div>
                }) }
            </Grid>
        },
        "list" => html! {
            <List variant={primary_variant()} class="component-detail-list-demo">
                <div>{ "Planning" }</div>
                <div>{ "Implementation" }</div>
                <div>{ "Verification" }</div>
            </List>
        },
        "table" => html! {
            <Table variant={primary_variant()} class="component-detail-table-demo">
                <div class="component-detail-table-row is-head">
                    <span>{ "Token" }</span>
                    <span>{ "State" }</span>
                </div>
                <div class="component-detail-table-row">
                    <span>{ "primary" }</span>
                    <Badge variant={primary_variant()}>{ "Ready" }</Badge>
                </div>
            </Table>
        },
        "breadcrumbs" => html! {
            <Breadcrumbs variant={primary_variant()} class="component-detail-breadcrumbs-demo">
                <span>{ "Components" }</span>
                <span>{ "/" }</span>
                <strong>{ spec.name }</strong>
            </Breadcrumbs>
        },
        "pagination" => html! {
            <Pagination variant={primary_variant()} class="component-detail-pagination-demo">
                <button>{ "1" }</button>
                <button class="is-active">{ "2" }</button>
                <button>{ "3" }</button>
            </Pagination>
        },
        "stepper" => html! {
            <Stepper variant={primary_variant()} class="component-detail-stepper-demo">
                <span>{ "Configure" }</span>
                <div class="stepper-track">
                    <i>{ "1" }</i>
                    <b></b>
                    <i>{ "2" }</i>
                    <b class="is-muted"></b>
                    <i class="is-muted">{ "3" }</i>
                </div>
            </Stepper>
        },
        "tabs" => html! {
            <Tabs variant={primary_variant()} class="component-detail-tabs-demo">
                <button class="is-active">{ "Docs" }</button>
                <button>{ "API" }</button>
                <button>{ "Demo" }</button>
            </Tabs>
        },
        "menu" => html! {
            <Menu variant={primary_variant()} class="component-detail-menu-demo">
                <a href="#docs">{ "Docs" }</a>
                <a href="#api">{ "API" }</a>
                <a href="#demo">{ "Demo" }</a>
            </Menu>
        },
        "navbar" => html! {
            <Navbar variant={primary_variant()} class="component-detail-navbar-demo">
                <strong>{ "Duskmoon" }</strong>
                <span>{ "Components" }</span>
                <span>{ "Themes" }</span>
            </Navbar>
        },
        "accordion" => html! {
            <Accordion variant={primary_variant()} class="component-detail-demo-control">
                <strong>{ "Accordion section" }</strong>
                <span>{ "Expandable surface content" }</span>
            </Accordion>
        },
        "bottom-sheet" => html! {
            <Bottomsheet variant={primary_variant()} class="component-detail-bottomsheet-demo">
                <strong>{ "Bottom sheet" }</strong>
                <span>{ "Mobile panel content" }</span>
            </Bottomsheet>
        },
        "popover" => html! {
            <Popover variant={primary_variant()} class="component-detail-popover-demo">
                <strong>{ "Popover" }</strong>
                <span>{ "Anchored contextual content" }</span>
            </Popover>
        },
        "avatar" => standard_demo!(Avatar, spec),
        "badge" => standard_demo!(Badge, spec),
        "chip" => standard_demo!(Chip, spec),
        "code-block" => standard_demo!(CodeBlock, spec),
        "collapse" => standard_demo!(Collapse, spec),
        "markdown-body" => standard_demo!(MarkdownBody, spec),
        "timeline" => standard_demo!(Timeline, spec),
        "autocomplete" => standard_demo!(Autocomplete, spec),
        "cascader" => standard_demo!(Cascader, spec),
        "checkbox" => standard_demo!(Checkbox, spec),
        "datepicker" => standard_demo!(Datepicker, spec),
        "file-upload" => standard_demo!(FileUpload, spec),
        "form" => standard_demo!(Form, spec),
        "form-group" => standard_demo!(FormGroup, spec),
        "input" => standard_demo!(Input, spec),
        "multi-select" => standard_demo!(MultiSelect, spec),
        "otp-input" => standard_demo!(OtpInput, spec),
        "pin-input" => standard_demo!(PinInput, spec),
        "radio" => standard_demo!(Radio, spec),
        "rating" => standard_demo!(Rating, spec),
        "segment-control" => standard_demo!(SegmentControl, spec),
        "select" => standard_demo!(Select, spec),
        "slider" => standard_demo!(Slider, spec),
        "switch" => standard_demo!(Switch, spec),
        "textarea" => standard_demo!(Textarea, spec),
        "time-input" => standard_demo!(TimeInput, spec),
        "tree-select" => standard_demo!(TreeSelect, spec),
        "alert" => standard_demo!(Alert, spec),
        "dialog" => standard_demo!(Dialog, spec),
        "modal" => standard_demo!(Modal, spec),
        "progress" => standard_demo!(Progress, spec),
        "skeleton" => standard_demo!(Skeleton, spec),
        "snackbar" => standard_demo!(Snackbar, spec),
        "toast" => standard_demo!(Toast, spec),
        "tooltip" => standard_demo!(Tooltip, spec),
        "appbar" => standard_demo!(Appbar, spec),
        "divider" => standard_demo!(Divider, spec),
        "bottom-navigation" => standard_demo!(BottomNavigation, spec),
        "circle-menu" => standard_demo!(CircleMenu, spec),
        "drawer" => standard_demo!(Drawer, spec),
        "nested-menu" => standard_demo!(NestedMenu, spec),
        "toggle" => standard_demo!(Toggle, spec),
        _ => html! {
            <div class="component-detail-demo-control">
                { demo_body(spec) }
            </div>
        },
    }
}
