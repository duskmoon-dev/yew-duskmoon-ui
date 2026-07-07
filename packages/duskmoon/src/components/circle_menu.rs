use yew::prelude::*;
use yew::virtual_dom::AttrValue;

use super::variants;

#[derive(Properties, Clone, PartialEq)]
pub struct CircleMenuProps {
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub variant: Option<String>,
}

#[function_component(CircleMenu)]
pub fn circle_menu(props: &CircleMenuProps) -> Html {
    let mut classes = classes!("circle-menu");
    if let Some(variant) = &props.variant {
        classes.push(format!("circle-menu-{}", variant));
    }
    classes.push(props.class.clone());
    let style = circle_menu_style(props.variant.as_deref());

    html! {
        <div class={classes} style={style}>
            { for props.children.iter() }
        </div>
    }
}

fn circle_menu_style(variant: Option<&str>) -> AttrValue {
    if variants::vars(variant).is_empty() {
        return AttrValue::default();
    }

    variants::style(
        variant,
        "color: var(--component-content); --circle-menu-btn-bg: var(--component-solid, var(--component-color)); --circle-menu-bar-color: var(--component-content); --circle-menu-item-bg: var(--component-solid, var(--component-color)); --circle-menu-item-color: var(--component-content); --circle-menu-item-ring: color-mix(in oklch, var(--component-color) 40%, transparent);",
    )
}
