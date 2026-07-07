use crate::routes::components::catalog::ComponentSpec;
use crate::routes::components::detail::page::{ComponentPage, STANDARD_API};
use crate::routes::components::palette::{variant, PaletteColor};
use yew::prelude::*;
use yew_duskmoon::Toast;

pub fn page(spec: &'static ComponentSpec) -> ComponentPage {
    ComponentPage::new(spec, usage, STANDARD_API, demo, color_variant)
}

fn usage(_: &ComponentSpec) -> String {
    "use yew_duskmoon::Toast;\n\nhtml! {\n    <Toast variant={Some(\"info\".to_owned())} class=\"toast-show\">\n        <span class=\"toast-icon\">{ \"i\" }</span>\n        <span class=\"toast-content\">\n            <strong class=\"toast-title\">{ \"Build finished\" }</strong>\n            <span class=\"toast-message\">{ \"The preview is ready.\" }</span>\n        </span>\n        <button class=\"toast-close\" aria-label=\"Dismiss\">{ \"x\" }</button>\n    </Toast>\n}".to_owned()
}

fn demo(_: &ComponentSpec) -> Html {
    html! {
        <div class="detail-demo-stack">
            <Toast variant={Some("success".to_owned())} class="toast-show">
                <span class="toast-icon">{ "OK" }</span>
                <span class="toast-content">
                    <strong class="toast-title">{ "Settings saved" }</strong>
                    <span class="toast-message">{ "Changes were applied to the workspace." }</span>
                    <span class="toast-actions">
                        <button class="btn btn-text">{ "View" }</button>
                    </span>
                </span>
                <button class="toast-close" aria-label="Dismiss toast">{ "x" }</button>
            </Toast>
            <Toast variant={Some("warning".to_owned())} class="toast-show toast-compact">
                <span class="toast-icon">{ "!" }</span>
                <span class="toast-content">
                    <strong class="toast-title">{ "Session expiring" }</strong>
                    <span class="toast-message">{ "Refresh credentials soon." }</span>
                </span>
            </Toast>
        </div>
    }
}

fn color_variant(color: PaletteColor) -> Html {
    html! {
        <Toast variant={variant(color)} class="toast-show toast-filled">
            <span class="toast-icon">{ "i" }</span>
            <span class="toast-content">
                <strong class="toast-title">{ color.label }</strong>
                <span class="toast-message">{ format!("toast-{}", color.key) }</span>
            </span>
        </Toast>
    }
}
