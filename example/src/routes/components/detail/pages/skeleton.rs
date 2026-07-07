use crate::routes::components::catalog::ComponentSpec;
use crate::routes::components::detail::page::{ComponentPage, STANDARD_API};
use crate::routes::components::palette::{variant, PaletteColor};
use yew::prelude::*;
use yew_duskmoon::Skeleton;

pub fn page(spec: &'static ComponentSpec) -> ComponentPage {
    ComponentPage::new(spec, usage, STANDARD_API, demo, color_variant)
}

fn usage(_: &ComponentSpec) -> String {
    "use yew_duskmoon::Skeleton;\n\nhtml! {\n    <div class=\"skeleton-card-content\">\n        <Skeleton class=\"skeleton-avatar\" />\n        <Skeleton class=\"skeleton-line\" />\n        <Skeleton class=\"skeleton-line skeleton-line-short\" />\n    </div>\n}".to_owned()
}

fn demo(_: &ComponentSpec) -> Html {
    html! {
        <div class="skeleton-card skeleton-wave" style="width: min(100%, 28rem);">
            <div class="skeleton-card-header">
                <Skeleton class="skeleton-avatar"></Skeleton>
                <div class="skeleton-card-body" style="flex: 1;">
                    <Skeleton class="skeleton-line"></Skeleton>
                    <Skeleton class="skeleton-line skeleton-line-medium"></Skeleton>
                </div>
            </div>
            <Skeleton class="skeleton-image"></Skeleton>
            <div class="skeleton-card-body">
                <Skeleton class="skeleton-line"></Skeleton>
                <Skeleton class="skeleton-line"></Skeleton>
                <Skeleton class="skeleton-line skeleton-line-short"></Skeleton>
            </div>
        </div>
    }
}

fn color_variant(color: PaletteColor) -> Html {
    html! {
        <Skeleton variant={variant(color)} class="skeleton-card component-detail-color-demo">
            <Skeleton class="skeleton-line"></Skeleton>
            <Skeleton class="skeleton-line skeleton-line-medium"></Skeleton>
            <code>{ format!("skeleton-{}", color.key) }</code>
        </Skeleton>
    }
}
