use yew::prelude::*;
use yew::virtual_dom::AttrValue;

use super::variants;

#[derive(Properties, Clone, PartialEq)]
pub struct AvatarProps {
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub variant: Option<String>,
}

#[function_component(Avatar)]
pub fn avatar(props: &AvatarProps) -> Html {
    let mut classes = classes!("avatar");
    if let Some(variant) = &props.variant {
        classes.push(format!("avatar-{}", variant));
    }
    classes.push(props.class.clone());
    let style = avatar_style(props.variant.as_deref());

    html! {
        <div class={classes} style={style}>
            { for props.children.iter() }
        </div>
    }
}

fn avatar_style(variant: Option<&str>) -> AttrValue {
    if variant.is_none() {
        return AttrValue::default();
    }

    variants::style(
        variant,
        "background-color: var(--component-solid, var(--component-color, var(--color-surface-container))); color: var(--component-content, var(--color-on-surface));",
    )
}
