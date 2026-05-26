use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct ModalProps {
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub variant: Option<String>,
}

#[function_component(Modal)]
pub fn modal(props: &ModalProps) -> Html {
    let mut classes = classes!("modal");
    if let Some(variant) = &props.variant {
        classes.push(format!("modal-{}", variant));
    }
    classes.push(props.class.clone());

    html! {
        <div class={classes}>
            { for props.children.iter() }
        </div>
    }
}
