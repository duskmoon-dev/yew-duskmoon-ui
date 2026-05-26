use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct ChatProps {
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub variant: Option<String>,
}

#[function_component(Chat)]
pub fn chat(props: &ChatProps) -> Html {
    let mut classes = classes!("chat");
    if let Some(variant) = &props.variant {
        classes.push(format!("chat-{}", variant));
    }
    classes.push(props.class.clone());

    html! {
        <div class={classes}>
            { for props.children.iter() }
        </div>
    }
}
