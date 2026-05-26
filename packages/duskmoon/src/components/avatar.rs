use yew::prelude::*;

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

    html! {
        <div class={classes}>
            { for props.children.iter() }
        </div>
    }
}
