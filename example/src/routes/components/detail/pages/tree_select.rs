use crate::routes::components::catalog::ComponentSpec;
use crate::routes::components::detail::page::{ApiRow, ComponentPage, primary_variant};
use crate::routes::components::palette::{PaletteColor, variant};
use yew::prelude::*;
use yew_duskmoon::TreeSelect;

const TREE_SELECT_API: &[ApiRow] = &[
    ApiRow {
        prop: "class",
        ty: "Classes",
        default: "empty",
        docs: "Extra classes appended to the tree-select root.",
    },
    ApiRow {
        prop: "children",
        ty: "Children",
        default: "empty",
        docs: "Trigger, dropdown, search, and tree node markup.",
    },
    ApiRow {
        prop: "variant",
        ty: "Option<String>",
        default: "None",
        docs: "Appends a tree-select color class such as tree-select-primary.",
    },
];

pub fn page(spec: &'static ComponentSpec) -> ComponentPage {
    ComponentPage::new(spec, usage, TREE_SELECT_API, demo, color_variant)
}

fn usage(_: &ComponentSpec) -> String {
    r#"use yew_duskmoon::TreeSelect;

html! {
    <TreeSelect variant={Some("primary".to_owned())}>
        <button class="tree-select-trigger" type="button" command="toggle-popover" commandfor="team-tree">
            <span class="tree-select-value tree-select-value-selected">
                <span class="tree-select-path">{ "Engineering / Platform" }</span>
            </span>
            <span class="tree-select-arrow">{ "v" }</span>
        </button>
        <div class="tree-select-dropdown" id="team-tree" popover="auto">
            <div class="tree-select-options">{ "Tree nodes..." }</div>
        </div>
    </TreeSelect>
}"#
        .to_owned()
}

fn demo(_: &ComponentSpec) -> Html {
    html! {
        <TreeSelect variant={primary_variant()} class="tree-select-outlined">
            <button class="tree-select-trigger" type="button" command="toggle-popover" commandfor="demo-tree-select">
                <span class="tree-select-value tree-select-value-selected">
                    <span class="tree-select-path">{ "Engineering / Platform" }</span>
                </span>
                <span class="tree-select-arrow">{ "v" }</span>
            </button>
            <div class="tree-select-dropdown" id="demo-tree-select" popover="auto">
                <div class="tree-select-search">
                    <input type="text" class="tree-select-search-input" placeholder="Search..." />
                </div>
                <div class="tree-select-options">
                    <div class="tree-select-node tree-select-node-expanded">
                        <button class="tree-select-node-toggle" type="button">{ ">" }</button>
                        <span class="tree-select-node-label">{ "Engineering" }</span>
                    </div>
                    <div class="tree-select-children">
                        <div class="tree-select-node tree-select-node-leaf tree-select-node-selected">
                            <span class="tree-select-node-label">{ "Platform" }</span>
                        </div>
                        <div class="tree-select-node tree-select-node-leaf">
                            <span class="tree-select-node-label">{ "Design Systems" }</span>
                        </div>
                    </div>
                </div>
            </div>
        </TreeSelect>
    }
}

fn color_variant(color: PaletteColor) -> Html {
    let id: AttrValue = format!("tree-select-color-{}", color.key).into();

    html! {
        <TreeSelect variant={variant(color)}>
            <button class="tree-select-trigger" style="color: var(--component-color);" type="button" command="toggle-popover" commandfor={id.clone()}>
                <span class="tree-select-value tree-select-value-selected">
                    <span class="tree-select-path">{ color.label }</span>
                </span>
                <span class="tree-select-arrow">{ "v" }</span>
            </button>
            <div class="tree-select-dropdown" id={id} popover="auto">
                <div class="tree-select-options">
                    <div class="tree-select-node tree-select-node-leaf">
                        <span class="tree-select-node-label">{ color.label }</span>
                    </div>
                </div>
            </div>
        </TreeSelect>
    }
}
