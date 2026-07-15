use yew::prelude::*;
use yew::virtual_dom::AttrValue;

#[derive(Properties, Clone, PartialEq)]
pub struct RadioProps {
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub variant: Option<String>,
    #[prop_or(AttrValue::from("Radio options"))]
    pub aria_label: AttrValue,
}

#[function_component(Radio)]
pub fn radio(props: &RadioProps) -> Html {
    let is_group = !props.children.is_empty();
    let mut classes = if is_group {
        classes!("radio-group")
    } else {
        classes!("radio")
    };
    if let Some(variant) = &props.variant {
        classes.push(format!("radio-{}", variant));
    }
    classes.push(props.class.clone());
    let role = is_group.then(|| AttrValue::from("radiogroup"));
    let aria_label = is_group.then(|| props.aria_label.clone());

    html! {
        <div class={classes} role={role} aria-label={aria_label}>
            { for props.children.iter() }
        </div>
    }
}
