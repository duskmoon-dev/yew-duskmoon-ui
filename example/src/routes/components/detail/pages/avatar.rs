use crate::routes::components::catalog::ComponentSpec;
use crate::routes::components::detail::page::{primary_variant, ApiRow, ComponentPage};
use crate::routes::components::palette::{variant, PaletteColor};
use yew::prelude::*;
use yew_duskmoon::Avatar;

const AVATAR_API: &[ApiRow] = &[
    ApiRow {
        prop: "class",
        ty: "Classes",
        default: "empty",
        docs: "Extra CSS classes appended to the avatar root.",
    },
    ApiRow {
        prop: "children",
        ty: "Children",
        default: "empty",
        docs: "Initials, image, icon, or status content rendered inside the avatar.",
    },
    ApiRow {
        prop: "variant",
        ty: "Option<String>",
        default: "None",
        docs: "Appends a color modifier class such as avatar-primary.",
    },
];

pub fn page(spec: &'static ComponentSpec) -> ComponentPage {
    ComponentPage::new(spec, usage, AVATAR_API, demo, color_variant)
}

fn usage(_: &ComponentSpec) -> String {
    "use yew_duskmoon::Avatar;\n\nhtml! {\n    <Avatar\n        variant={Some(\"primary\".to_owned())}\n        class=\"avatar-lg avatar-status avatar-status-online\"\n    >\n        { \"GD\" }\n    </Avatar>\n}".to_owned()
}

fn demo(_: &ComponentSpec) -> Html {
    html! {
        <div class="detail-demo-stack">
            <Avatar variant={primary_variant()} class="avatar-lg avatar-status avatar-status-online">
                { "GD" }
            </Avatar>
            <Avatar class="avatar-md avatar-rounded avatar-secondary avatar-status avatar-status-away">
                { "UI" }
            </Avatar>
            <Avatar class="avatar-sm avatar-square avatar-tertiary avatar-bordered">
                { "A" }
            </Avatar>
            <div class="avatar-group avatar-group-sm" aria-label="Example team avatars">
                <Avatar variant={primary_variant()} class="avatar-sm">{ "DS" }</Avatar>
                <Avatar variant={Some("secondary".to_owned())} class="avatar-sm">{ "BE" }</Avatar>
                <Avatar variant={Some("tertiary".to_owned())} class="avatar-sm avatar-overflow">{ "+4" }</Avatar>
            </div>
        </div>
    }
}

fn color_variant(color: PaletteColor) -> Html {
    html! {
        <div class="component-detail-color-demo">
            <Avatar variant={variant(color)} class="avatar-md">
                <span>{ color.label.chars().next().unwrap_or('A').to_string() }</span>
            </Avatar>
            <code>{ format!("avatar-{}", color.key) }</code>
        </div>
    }
}
