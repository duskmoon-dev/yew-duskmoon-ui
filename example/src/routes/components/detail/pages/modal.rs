use crate::routes::components::catalog::ComponentSpec;
use crate::routes::components::detail::page::{primary_variant, ComponentPage, STANDARD_API};
use crate::routes::components::palette::{variant, PaletteColor};
use yew::prelude::*;
use yew_duskmoon::Modal;

pub fn page(spec: &'static ComponentSpec) -> ComponentPage {
    ComponentPage::new(spec, usage, STANDARD_API, demo, color_variant)
}

fn usage(_: &ComponentSpec) -> String {
    "use yew_duskmoon::Modal;\n\nhtml! {\n    <Modal class=\"modal-open modal-middle\" variant={Some(\"primary\".to_owned())}>\n        <div class=\"modal-box\">\n            <div class=\"modal-header\">\n                <h2 class=\"modal-title\">{ \"Confirm deployment\" }</h2>\n            </div>\n            <div class=\"modal-body\">{ \"Review the release notes before continuing.\" }</div>\n            <div class=\"modal-action\">\n                <button class=\"btn btn-text\">{ \"Cancel\" }</button>\n                <button class=\"btn btn-primary\">{ \"Deploy\" }</button>\n            </div>\n        </div>\n    </Modal>\n}".to_owned()
}

fn demo(_: &ComponentSpec) -> Html {
    html! {
        <>
            { preview_style() }
            <Modal variant={primary_variant()} class="modal-open modal-middle component-detail-modal-preview">
                <div class="modal-box">
                    <button class="modal-close" aria-label="Close modal">{ "x" }</button>
                    <div class="modal-header">
                        <h2 class="modal-title">{ "Confirm deployment" }</h2>
                    </div>
                    <div class="modal-body">
                        <p>{ "Deploy the selected build to production after the final health check." }</p>
                    </div>
                    <div class="modal-action">
                        <button class="btn btn-text">{ "Cancel" }</button>
                        <button class="btn btn-primary">{ "Deploy" }</button>
                    </div>
                </div>
            </Modal>
        </>
    }
}

fn color_variant(color: PaletteColor) -> Html {
    html! {
        <Modal variant={variant(color)} class="modal-open modal-middle component-detail-modal-preview">
            <div class="modal-box">
                <div class="modal-header">
                    <h2 class="modal-title">{ color.label }</h2>
                </div>
                <div class="modal-body">
                    <code>{ format!("modal-{}", color.key) }</code>
                </div>
            </div>
        </Modal>
    }
}

fn preview_style() -> Html {
    html! {
        <style>
            { ".component-detail-modal-preview.modal{position:relative;inset:auto;z-index:auto;visibility:visible;opacity:1;width:min(100%,34rem);min-height:18rem;padding:1rem;border:1px solid var(--dm-line);border-radius:8px;background:color-mix(in oklch,var(--dm-ink) 34%,transparent)}.component-detail-modal-preview.modal .modal-backdrop{display:none}.component-detail-modal-preview.modal .modal-box{width:100%;max-height:none;border:1px solid color-mix(in oklch,var(--component-color,var(--dm-line-strong)) 34%,var(--dm-line))}" }
        </style>
    }
}
