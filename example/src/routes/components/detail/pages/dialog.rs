use crate::routes::components::catalog::ComponentSpec;
use crate::routes::components::detail::page::{ApiRow, ComponentPage};
use yew::prelude::*;
use yew_duskmoon::{Button, Dialog, IconButton};

const DIALOG_API: &[ApiRow] = &[
    ApiRow {
        prop: "id",
        ty: "AttrValue",
        default: "required",
        docs: "Stable DOM id targeted by commandfor on show-modal and close buttons.",
    },
    ApiRow {
        prop: "class",
        ty: "Classes",
        default: "empty",
        docs: "Extra classes appended to the native dialog element.",
    },
    ApiRow {
        prop: "children",
        ty: "Children",
        default: "empty",
        docs: "Dialog box, header, body, and footer markup rendered inside the native dialog.",
    },
    ApiRow {
        prop: "variant",
        ty: "Option<String>",
        default: "None",
        docs: "Legacy escape hatch that appends a dialog-{variant} class.",
    },
];

pub fn page(spec: &'static ComponentSpec) -> ComponentPage {
    ComponentPage::without_color_variants(spec, usage, DIALOG_API, demo)
}

fn usage(_: &ComponentSpec) -> String {
    r#"use yew_duskmoon::{Button, Dialog, IconButton};

html! {
    <>
        <Button command="show-modal" command_for="confirm-deployment">
            { "Open dialog" }
        </Button>
        <Dialog id="confirm-deployment" class="dialog-divider">
            <div class="dialog-box">
                <div class="dialog-header">
                    <h2 class="dialog-title">{ "Confirm deployment" }</h2>
                    <IconButton
                        label="Close dialog"
                        class="dialog-close"
                        command="close"
                        command_for="confirm-deployment"
                    >
                        { "x" }
                    </IconButton>
                </div>
                <div class="dialog-body">
                    <p>{ "Deploy the selected revision to production?" }</p>
                </div>
                <div class="dialog-footer">
                    <Button command="close" command_for="confirm-deployment">{ "Cancel" }</Button>
                    <Button command="close" command_for="confirm-deployment">{ "Deploy" }</Button>
                </div>
            </div>
        </Dialog>
    </>
}"#
        .to_owned()
}

fn demo(_: &ComponentSpec) -> Html {
    html! {
        <>
            <Button command="show-modal" command_for="demo-dialog" variant={Some("primary".to_owned())}>
                { "Open native dialog" }
            </Button>
            <Dialog id="demo-dialog" class="dialog-divider">
                <div class="dialog-box">
                    <div class="dialog-header">
                        <h2 class="dialog-title">{ "Confirm deployment" }</h2>
                        <IconButton
                            label="Close dialog"
                            class="dialog-close"
                            command="close"
                            command_for="demo-dialog"
                        >
                            { "x" }
                        </IconButton>
                    </div>
                    <div class="dialog-body">
                        <p>{ "The native dialog provides the modal top layer, backdrop, Escape handling, and focus restoration." }</p>
                    </div>
                    <div class="dialog-footer">
                        <Button
                            appearance={Some(yew_duskmoon::ButtonAppearance::Text)}
                            command="close"
                            command_for="demo-dialog"
                        >
                            { "Cancel" }
                        </Button>
                        <Button
                            variant={Some("primary".to_owned())}
                            command="close"
                            command_for="demo-dialog"
                        >
                            { "Deploy" }
                        </Button>
                    </div>
                </div>
            </Dialog>
        </>
    }
}
