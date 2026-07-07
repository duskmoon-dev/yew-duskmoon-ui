use crate::routes::components::catalog::ComponentSpec;
use crate::routes::components::detail::page::{ApiRow, ComponentPage, primary_variant};
use crate::routes::components::palette::{PaletteColor, variant};
use yew::prelude::*;
use yew_duskmoon::Dialog;

const DIALOG_API: &[ApiRow] = &[
    ApiRow {
        prop: "class",
        ty: "Classes",
        default: "empty",
        docs: "Extra classes appended to the Dialog wrapper.",
    },
    ApiRow {
        prop: "children",
        ty: "Children",
        default: "empty",
        docs: "Dialog box, header, body, and footer markup rendered inside the wrapper.",
    },
    ApiRow {
        prop: "variant",
        ty: "Option<String>",
        default: "None",
        docs: "Appends a dialog color class such as dialog-primary.",
    },
];

pub fn page(spec: &'static ComponentSpec) -> ComponentPage {
    ComponentPage::new(spec, usage, DIALOG_API, demo, color_variant)
}

fn usage(_: &ComponentSpec) -> String {
    "use yew_duskmoon::Dialog;\n\nhtml! {\n    <Dialog variant={Some(\"primary\".to_owned())} class=\"demo-modal dialog-divider\">\n        <div class=\"dialog-box\">\n            <div class=\"dialog-header\">\n                <h2 class=\"dialog-title\">{ \"Confirm deployment\" }</h2>\n                <button class=\"dialog-close\" type=\"button\" aria-label=\"Close\">{ \"x\" }</button>\n            </div>\n            <div class=\"dialog-body\">\n                <p>{ \"Deploy the selected revision to production?\" }</p>\n            </div>\n            <div class=\"dialog-footer\">\n                <button class=\"btn btn-ghost\" type=\"button\">{ \"Cancel\" }</button>\n                <button class=\"btn btn-primary\" type=\"button\">{ \"Deploy\" }</button>\n            </div>\n        </div>\n    </Dialog>\n}".to_owned()
}

fn demo(_: &ComponentSpec) -> Html {
    html! {
        <Dialog variant={primary_variant()} class="demo-modal dialog-divider">
            <div class="dialog-box">
                <div class="dialog-header">
                    <h2 class="dialog-title">{ "Confirm deployment" }</h2>
                    <button class="dialog-close" type="button" aria-label="Close">{ "x" }</button>
                </div>
                <div class="dialog-body">
                    <p>{ "Deploy the selected revision to production?" }</p>
                </div>
                <div class="dialog-footer">
                    <button class="btn btn-ghost" type="button">{ "Cancel" }</button>
                    <button class="btn btn-primary" type="button">{ "Deploy" }</button>
                </div>
            </div>
        </Dialog>
    }
}

fn color_variant(color: PaletteColor) -> Html {
    html! {
        <Dialog variant={variant(color)} class="demo-modal">
            <div class="dialog-box">
                <div class="dialog-header">
                    <h3 class="dialog-title" style="color: var(--component-color);">{ color.label }</h3>
                </div>
                <div class="dialog-body">
                    <p>{ format!("dialog-{}", color.key) }</p>
                </div>
            </div>
        </Dialog>
    }
}
