use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct ToastProps {
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub variant: Option<String>,
}

#[function_component(Toast)]
pub fn toast(props: &ToastProps) -> Html {
    let mut classes = classes!("toast");
    if let Some(variant) = &props.variant {
        classes.push(format!("toast-{}", variant));
    }
    classes.push(props.class.clone());

    html! {
        <div class={classes}>
            { for props.children.iter() }
        </div>
    }
}
