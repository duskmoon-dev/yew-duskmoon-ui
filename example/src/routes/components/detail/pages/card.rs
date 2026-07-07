use crate::routes::components::catalog::ComponentSpec;
use crate::routes::components::detail::page::{primary_variant, ComponentPage, CARD_API};
use crate::routes::components::palette::{variant, PaletteColor};
use yew::prelude::*;
use yew_duskmoon::{button::ButtonType, Button, Card};

pub fn page(spec: &'static ComponentSpec) -> ComponentPage {
    ComponentPage::new(spec, usage, CARD_API, demo, color_variant)
}

fn usage(_: &ComponentSpec) -> String {
    "use yew_duskmoon::{Button, Card};\n\nhtml! {\n    <Card variant={Some(\"primary\".to_owned())} title={html! { \"Release summary\" }}>\n        <p>{ \"Cards group related content and actions in one surface.\" }</p>\n        <div class=\"card-actions\">\n            <Button variant={Some(\"primary\".to_owned())}>{ \"Open\" }</Button>\n        </div>\n    </Card>\n}".to_owned()
}

fn demo(_: &ComponentSpec) -> Html {
    html! {
        <Card variant={primary_variant()} title={html! { <span>{ "Release summary" }</span> }} classes="component-detail-card-demo card-interactive">
            <p>{ "Cards keep related content, status, and actions in one readable surface." }</p>
            <div class="card-actions">
                <Button variant={primary_variant()}>{ "Open report" }</Button>
                <Button r#type={ButtonType::Text}>{ "Dismiss" }</Button>
            </div>
        </Card>
    }
}

fn color_variant(color: PaletteColor) -> Html {
    html! {
        <Card variant={variant(color)} title={html! { <span>{ color.label }</span> }} classes="component-detail-color-card">
            <span>{ format!("card-{}", color.key) }</span>
        </Card>
    }
}
