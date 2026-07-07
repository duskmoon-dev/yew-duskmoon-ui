use crate::routes::components::catalog::ComponentSpec;
use crate::routes::components::detail::page::{
    primary_variant, secondary_variant, tertiary_variant, ComponentPage, BUTTON_API,
};
use crate::routes::components::palette::{variant, PaletteColor};
use yew::prelude::*;
use yew_duskmoon::button::ButtonType;
use yew_duskmoon::Button;

pub fn page(spec: &'static ComponentSpec) -> ComponentPage {
    ComponentPage::new(spec, usage, BUTTON_API, demo, color_variant)
}

fn usage(_: &ComponentSpec) -> String {
    "use yew_duskmoon::button::ButtonType;\nuse yew_duskmoon::Button;\n\nhtml! {\n    <div class=\"detail-demo-stack\">\n        <Button r#type={ButtonType::Primary}>{ \"Publish\" }</Button>\n        <Button r#type={ButtonType::Text}>{ \"Cancel\" }</Button>\n        <Button r#type={ButtonType::Link} href=\"#api\">{ \"Read API\" }</Button>\n    </div>\n}".to_owned()
}

fn demo(_: &ComponentSpec) -> Html {
    html! {
        <div class="detail-demo-stack">
            <Button r#type={ButtonType::Primary} classes="component-detail-action">{ "Publish" }</Button>
            <Button variant={secondary_variant()} classes="component-detail-action">{ "Secondary action" }</Button>
            <Button r#type={ButtonType::Text} classes="component-detail-action">{ "Cancel" }</Button>
            <Button r#type={ButtonType::Link} variant={tertiary_variant()} href="#api" classes="component-detail-action">{ "API link" }</Button>
            <Button variant={primary_variant()} loading={true} classes="component-detail-action">{ "Saving" }</Button>
        </div>
    }
}

fn color_variant(color: PaletteColor) -> Html {
    html! {
        <Button variant={variant(color)} classes="component-detail-color-button">
            { html! { color.label } }
        </Button>
    }
}
