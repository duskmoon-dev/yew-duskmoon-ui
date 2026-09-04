use crate::routes::components::catalog::ComponentSpec;
use crate::routes::components::detail::page::{ApiRow, ComponentPage};
use yew::prelude::*;
use yew_duskmoon::{Button, Modal};

const API: &[ApiRow] = &[
    ApiRow {
        prop: "id",
        ty: "AttrValue",
        default: "required",
        docs: "Stable DOM id targeted by commandfor. Modal is a legacy name implementing the native Dialog contract.",
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
        docs: "Native dialog content. Prefer Dialog in new code.",
    },
    ApiRow {
        prop: "variant",
        ty: "Option<String>",
        default: "None",
        docs: "Legacy escape hatch forwarded to the Dialog contract.",
    },
];

pub fn page(spec: &'static ComponentSpec) -> ComponentPage {
    ComponentPage::without_color_variants(spec, usage, API, demo)
}

fn usage(_: &ComponentSpec) -> String {
    r#"// Legacy name with the new Dialog contract. Prefer `Dialog` in new code.
use yew_duskmoon::{Button, Modal};

html! {
    <>
        <Button command="show-modal" command_for="legacy-modal">{ "Open" }</Button>
        <Modal id="legacy-modal">
            <div class="dialog-box">
                <div class="dialog-header">
                    <h2 class="dialog-title">{ "Native modal dialog" }</h2>
                </div>
                <div class="dialog-body">{ "Modal now renders the same native dialog contract." }</div>
                <div class="dialog-footer">
                    <Button command="close" command_for="legacy-modal">{ "Close" }</Button>
                </div>
            </div>
        </Modal>
    </>
}"#
        .to_owned()
}

fn demo(_: &ComponentSpec) -> Html {
    html! {
        <>
            <p>{ "Modal is retained as a legacy name and now requires the native Dialog contract. Prefer Dialog in new code." }</p>
            <Button command="show-modal" command_for="demo-legacy-modal" variant={Some("primary".to_owned())}>
                { "Open compatibility modal" }
            </Button>
            <Modal id="demo-legacy-modal">
                <div class="dialog-box">
                    <div class="dialog-header">
                        <h2 class="dialog-title">{ "Native modal dialog" }</h2>
                    </div>
                    <div class="dialog-body">
                        <p>{ "This surface uses the browser's dialog top layer instead of modal-open classes or Yew visibility state." }</p>
                    </div>
                    <div class="dialog-footer">
                        <Button command="close" command_for="demo-legacy-modal">{ "Close" }</Button>
                    </div>
                </div>
            </Modal>
        </>
    }
}
