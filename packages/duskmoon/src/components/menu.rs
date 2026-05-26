use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct MenuProps {
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub variant: Option<String>,
}

#[function_component(Menu)]
pub fn menu(props: &MenuProps) -> Html {
    let mut classes = classes!("menu");
    if let Some(variant) = &props.variant {
        classes.push(format!("menu-{}", variant));
    }
    classes.push(props.class.clone());

    html! {
        <div class={classes}>
            { for props.children.iter() }
        </div>
    }
}
