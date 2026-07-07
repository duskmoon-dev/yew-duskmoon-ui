use crate::routes::components::catalog::ComponentSpec;
use crate::routes::components::detail::page::{primary_variant, ComponentPage, STANDARD_API};
use crate::routes::components::palette::{variant, PaletteColor};
use yew::prelude::*;
use yew_duskmoon::Divider;

pub fn page(spec: &'static ComponentSpec) -> ComponentPage {
    ComponentPage::new(spec, usage, STANDARD_API, demo, color_variant)
}

fn usage(_: &ComponentSpec) -> String {
    "use yew_duskmoon::Divider;\n\nhtml! {\n    <>\n        <p>{ \"Content above\" }</p>\n        <Divider variant={Some(\"primary\".to_owned())}>{ \"OR\" }</Divider>\n        <p>{ \"Content below\" }</p>\n    </>\n}".to_owned()
}

fn demo(_: &ComponentSpec) -> Html {
    html! {
        <div style="width: min(100%, 34rem);">
            <p>{ "Content above" }</p>
            <Divider variant={primary_variant()}>{ "OR" }</Divider>
            <p>{ "Content below" }</p>
            <Divider class="divider-start divider-dashed">{ "Section title" }</Divider>
            <div style="display: flex; align-items: center; min-height: 4rem;">
                <span>{ "Left" }</span>
                <Divider variant={primary_variant()} class="divider-vertical"></Divider>
                <span>{ "Right" }</span>
            </div>
        </div>
    }
}

fn color_variant(color: PaletteColor) -> Html {
    html! {
        <Divider variant={variant(color)} class="divider-thick">
            <span>{ color.label }</span>
        </Divider>
    }
}
