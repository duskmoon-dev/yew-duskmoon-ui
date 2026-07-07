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
    "use yew_duskmoon::TreeSelect;\n\nhtml! {\n    <TreeSelect variant={Some(\"primary\".to_owned())}>\n        <div class=\"tree-select-trigger\" role=\"combobox\" tabindex=\"0\">\n            <span class=\"tree-select-value tree-select-value-selected\">\n                <span class=\"tree-select-path\">{ \"Engineering / Platform\" }</span>\n            </span>\n            <span class=\"tree-select-arrow\">{ \"v\" }</span>\n        </div>\n    </TreeSelect>\n}".to_owned()
}

fn demo(_: &ComponentSpec) -> Html {
    html! {
        <TreeSelect variant={primary_variant()} class="tree-select-open tree-select-outlined">
            <div class="tree-select-trigger" role="combobox" aria-expanded="true" tabindex="0">
                <span class="tree-select-value tree-select-value-selected">
                    <span class="tree-select-path">{ "Engineering / Platform" }</span>
                </span>
                <span class="tree-select-arrow">{ "v" }</span>
            </div>
            <div class="tree-select-dropdown">
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
    html! {
        <TreeSelect variant={variant(color)}>
            <div class="tree-select-trigger" style="color: var(--component-color);" role="combobox" tabindex="0">
                <span class="tree-select-value tree-select-value-selected">
                    <span class="tree-select-path">{ color.label }</span>
                </span>
                <span class="tree-select-arrow">{ "v" }</span>
            </div>
        </TreeSelect>
    }
}
