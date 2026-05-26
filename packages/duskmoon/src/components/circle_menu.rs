use yew::prelude::*;

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

    html! {
        <div class={classes}>
            { for props.children.iter() }
        </div>
    }
}
