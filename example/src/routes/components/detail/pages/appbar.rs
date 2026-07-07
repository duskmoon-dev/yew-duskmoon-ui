use crate::routes::components::catalog::ComponentSpec;
use crate::routes::components::detail::page::{primary_variant, ComponentPage, STANDARD_API};
use crate::routes::components::palette::{variant, PaletteColor};
use yew::prelude::*;
use yew_duskmoon::AppBar;

pub fn page(spec: &'static ComponentSpec) -> ComponentPage {
    ComponentPage::new(spec, usage, STANDARD_API, demo, color_variant)
}

fn usage(_: &ComponentSpec) -> String {
    "use yew_duskmoon::AppBar;\n\nhtml! {\n    <AppBar variant={Some(\"primary\".to_owned())}>\n        <div class=\"appbar-leading\">\n            <button class=\"appbar-action\" aria-label=\"Menu\">{ \"Menu\" }</button>\n        </div>\n        <div class=\"appbar-title\">\n            <h1 class=\"appbar-heading\">{ \"Dashboard\" }</h1>\n        </div>\n        <div class=\"appbar-trailing\">\n            <button class=\"appbar-action\" aria-label=\"Search\">{ \"Search\" }</button>\n        </div>\n    </AppBar>\n}".to_owned()
}

fn demo(_: &ComponentSpec) -> Html {
    html! {
        <AppBar variant={primary_variant()} class="appbar-static appbar-bordered">
            <div class="appbar-leading">
                <button class="appbar-action" aria-label="Open menu">{ "Menu" }</button>
            </div>
            <div class="appbar-title-group">
                <h1 class="appbar-heading">{ "Component detail" }</h1>
                <p class="appbar-subtitle">{ "Navigation, title, and page actions" }</p>
            </div>
            <div class="appbar-trailing">
                <button class="appbar-action" aria-label="Search">{ "Search" }</button>
                <button class="appbar-action" aria-label="More actions">{ "More" }</button>
            </div>
        </AppBar>
    }
}

fn color_variant(color: PaletteColor) -> Html {
    html! {
        <AppBar variant={variant(color)} class="appbar-static appbar-bordered">
            <div class="appbar-title">
                <h1 class="appbar-heading">{ color.label }</h1>
            </div>
            <div class="appbar-trailing">
                <button class="appbar-action" aria-label="Favorite">{ "Star" }</button>
            </div>
        </AppBar>
    }
}
