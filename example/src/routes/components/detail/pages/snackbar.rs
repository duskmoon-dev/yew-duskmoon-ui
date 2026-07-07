use crate::routes::components::catalog::ComponentSpec;
use crate::routes::components::detail::page::{ComponentPage, STANDARD_API};
use crate::routes::components::palette::{variant, PaletteColor};
use yew::prelude::*;
use yew_duskmoon::Snackbar;

pub fn page(spec: &'static ComponentSpec) -> ComponentPage {
    ComponentPage::new(spec, usage, STANDARD_API, demo, color_variant)
}

fn usage(_: &ComponentSpec) -> String {
    "use yew_duskmoon::Snackbar;\n\nhtml! {\n    <Snackbar variant={Some(\"success\".to_owned())} class=\"snackbar-show\">\n        <span class=\"snackbar-message\">{ \"Message sent\" }</span>\n        <button class=\"snackbar-action\">{ \"Undo\" }</button>\n        <button class=\"snackbar-close\" aria-label=\"Dismiss\">{ \"x\" }</button>\n    </Snackbar>\n}".to_owned()
}

fn demo(_: &ComponentSpec) -> Html {
    html! {
        <Snackbar variant={Some("success".to_owned())} class="snackbar-show snackbar-multiline">
            <span class="snackbar-icon">{ "OK" }</span>
            <span class="snackbar-message">{ "Message sent. The snackbar can include an action and a close affordance." }</span>
            <button class="snackbar-action">{ "Undo" }</button>
            <button class="snackbar-close" aria-label="Dismiss snackbar">{ "x" }</button>
        </Snackbar>
    }
}

fn color_variant(color: PaletteColor) -> Html {
    html! {
        <Snackbar variant={variant(color)} class="snackbar-show">
            <span class="snackbar-message">{ color.label }</span>
            <button class="snackbar-action">{ "Action" }</button>
        </Snackbar>
    }
}
