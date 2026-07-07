use crate::routes::components::catalog::ComponentSpec;
use crate::routes::components::detail::page::{primary_variant, ApiRow, ComponentPage};
use crate::routes::components::palette::{variant, PaletteColor};
use yew::prelude::*;
use yew_duskmoon::Toggle;

const TOGGLE_API: &[ApiRow] = &[
    ApiRow {
        prop: "class",
        ty: "Classes",
        default: "empty",
        docs: "Extra CSS classes appended to the toggle group root.",
    },
    ApiRow {
        prop: "children",
        ty: "Children",
        default: "empty",
        docs: "Fallback content rendered inside a single toggle button.",
    },
    ApiRow {
        prop: "variant",
        ty: "Option<String>",
        default: "None",
        docs: "Applies the toggle color variant.",
    },
    ApiRow {
        prop: "options",
        ty: "Vec<AttrValue>",
        default: "empty",
        docs: "Labels for the segmented toggle buttons.",
    },
    ApiRow {
        prop: "active",
        ty: "usize",
        default: "0",
        docs: "Initially active option index.",
    },
    ApiRow {
        prop: "disabled",
        ty: "bool",
        default: "false",
        docs: "Disables every toggle option.",
    },
    ApiRow {
        prop: "aria_label",
        ty: "AttrValue",
        default: "Toggle options",
        docs: "Accessible label for the toggle group.",
    },
    ApiRow {
        prop: "onclick",
        ty: "Callback<usize>",
        default: "noop",
        docs: "Emits the selected option index when a button is pressed.",
    },
];

pub fn page(spec: &'static ComponentSpec) -> ComponentPage {
    ComponentPage::new(spec, usage, TOGGLE_API, demo, color_variant)
}

fn usage(_: &ComponentSpec) -> String {
    "use yew_duskmoon::Toggle;\n\nhtml! {\n    <Toggle\n        variant={Some(\"primary\".to_owned())}\n        options={vec![\"List\".into(), \"Grid\".into(), \"Details\".into()]}\n        active={1}\n        aria_label=\"View mode\"\n    />\n}".to_owned()
}

fn demo(_: &ComponentSpec) -> Html {
    html! {
        <div class="detail-demo-stack">
            <Toggle
                variant={primary_variant()}
                class="component-detail-toggle-demo"
                options={vec!["List".into(), "Grid".into(), "Details".into()]}
                active={1}
                aria_label="View mode"
            />
            <Toggle
                variant={primary_variant()}
                class="component-detail-toggle-demo"
                options={vec!["Compact".into(), "Comfortable".into()]}
                active={0}
                disabled={true}
                aria_label="Disabled density choice"
            />
        </div>
    }
}

fn color_variant(color: PaletteColor) -> Html {
    html! {
        <Toggle
            variant={variant(color)}
            class="component-detail-color-toggle"
            options={vec![color.label.into(), "Code".into(), "Preview".into()]}
        />
    }
}
