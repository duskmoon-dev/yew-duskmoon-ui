use crate::routes::components::catalog::ComponentSpec;
use crate::routes::components::detail::page::{primary_variant, ComponentPage, STANDARD_API};
use crate::routes::components::palette::{variant, PaletteColor};
use yew::prelude::*;
use yew_duskmoon::BottomNavigation;

pub fn page(spec: &'static ComponentSpec) -> ComponentPage {
    ComponentPage::new(spec, usage, STANDARD_API, demo, color_variant)
}

fn usage(_: &ComponentSpec) -> String {
    "use yew_duskmoon::BottomNavigation;\n\nhtml! {\n    <BottomNavigation\n        variant={Some(\"primary\".to_owned())}\n        class=\"bottom-nav bottom-nav-responsive\"\n    >\n        <a href=\"#home\" class=\"bottom-nav-item active\">\n            <span class=\"bottom-nav-indicator\"><span class=\"bottom-nav-icon\">{ \"H\" }</span></span>\n            <span class=\"bottom-nav-label\">{ \"Home\" }</span>\n        </a>\n        <a href=\"#search\" class=\"bottom-nav-item\">\n            <span class=\"bottom-nav-indicator\"><span class=\"bottom-nav-icon\">{ \"S\" }</span></span>\n            <span class=\"bottom-nav-label\">{ \"Search\" }</span>\n        </a>\n    </BottomNavigation>\n}".to_owned()
}

fn demo(_: &ComponentSpec) -> Html {
    html! {
        <>
            { preview_style() }
            <BottomNavigation variant={primary_variant()} class="bottom-nav bottom-nav-responsive bottom-nav-bordered component-detail-bottom-navigation-preview">
                { nav_item("#home", "H", "Home", true, None) }
                { nav_item("#search", "S", "Search", false, None) }
                { nav_item("#inbox", "I", "Inbox", false, Some("3")) }
                { nav_item("#profile", "P", "Profile", false, None) }
            </BottomNavigation>
        </>
    }
}

fn color_variant(color: PaletteColor) -> Html {
    html! {
        <BottomNavigation
            variant={variant(color)}
            class={classes!(
                "bottom-nav",
                "bottom-nav-responsive",
                "component-detail-bottom-navigation-preview",
                format!("bottom-nav-{}", color.key),
            )}
        >
            { nav_item("#catalog", "C", color.label, true, None) }
            { nav_item("#docs", "D", "Docs", false, None) }
        </BottomNavigation>
    }
}

fn nav_item(href: &'static str, icon: &'static str, label: &'static str, active: bool, badge: Option<&'static str>) -> Html {
    html! {
        <a href={href} class={classes!("bottom-nav-item", active.then_some("active"))}>
            <span class="bottom-nav-indicator">
                <span class="bottom-nav-icon">{ icon }</span>
                {
                    badge.map(|value| html! { <span class="bottom-nav-badge">{ value }</span> })
                        .unwrap_or_default()
                }
            </span>
            <span class="bottom-nav-label">{ label }</span>
        </a>
    }
}

fn preview_style() -> Html {
    html! {
        <style>
            { ".component-detail-bottom-navigation-preview.bottom-nav{position:relative;left:auto;right:auto;bottom:auto;z-index:auto;width:min(100%,34rem);min-height:4.75rem;padding-bottom:0;border:1px solid var(--dm-line);border-radius:8px;overflow:hidden}.component-detail-bottom-navigation-preview.bottom-nav .bottom-nav-item{min-width:0}" }
        </style>
    }
}
