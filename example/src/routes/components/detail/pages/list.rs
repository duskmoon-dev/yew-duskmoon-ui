use crate::routes::components::catalog::ComponentSpec;
use crate::routes::components::detail::page::{primary_variant, ApiRow, ComponentPage};
use crate::routes::components::palette::{variant, PaletteColor};
use yew::prelude::*;
use yew_duskmoon::{Badge, BadgeSize, List};

const LIST_API: &[ApiRow] = &[
    ApiRow {
        prop: "class",
        ty: "Classes",
        default: "empty",
        docs: "Extra CSS classes appended to the list root.",
    },
    ApiRow {
        prop: "children",
        ty: "Children",
        default: "empty",
        docs: "List item rows, leading/trailing slots, dividers, and subheaders.",
    },
    ApiRow {
        prop: "variant",
        ty: "Option<String>",
        default: "None",
        docs: "Appends a color modifier class such as list-primary.",
    },
];

pub fn page(spec: &'static ComponentSpec) -> ComponentPage {
    ComponentPage::new(spec, usage, LIST_API, demo, color_variant)
}

fn usage(_: &ComponentSpec) -> String {
    "use yew_duskmoon::List;\n\nhtml! {\n    <List variant={Some(\"primary\".to_owned())} class=\"list-bordered\">\n        <div class=\"list-item list-item-two-line list-item-active\">\n            <div class=\"list-item-content\">\n                <span class=\"list-item-text\">{ \"Inbox\" }</span>\n                <span class=\"list-item-secondary\">{ \"12 unread messages\" }</span>\n            </div>\n        </div>\n    </List>\n}".to_owned()
}

fn demo(_: &ComponentSpec) -> Html {
    html! {
        <List variant={primary_variant()} class="component-detail-list-demo list-bordered">
            <div class="list-item list-item-two-line list-item-interactive list-item-active">
                <div class="list-item-content">
                    <span class="list-item-text">{ "Inbox" }</span>
                    <span class="list-item-secondary">{ "12 unread messages" }</span>
                </div>
                <div class="list-item-trailing">
                    <Badge variant={primary_variant()} size={BadgeSize::Small}>{ "12" }</Badge>
                </div>
            </div>
            <div class="list-item list-item-two-line list-item-interactive">
                <div class="list-item-content">
                    <span class="list-item-text">{ "Build queue" }</span>
                    <span class="list-item-secondary">{ "3 jobs waiting for review" }</span>
                </div>
            </div>
            <div class="list-item list-item-disabled">
                <div class="list-item-content">
                    <span class="list-item-text">{ "Archived" }</span>
                </div>
            </div>
        </List>
    }
}

fn color_variant(color: PaletteColor) -> Html {
    html! {
        <List variant={variant(color)} class="component-detail-color-list">
            <div class="list-item list-item-interactive">
                <div class="list-item-content">
                    <span class="list-item-text">{ color.label }</span>
                    <code>{ format!("list-{}", color.key) }</code>
                </div>
            </div>
        </List>
    }
}
