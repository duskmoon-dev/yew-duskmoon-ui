use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct DialogProps {
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub variant: Option<String>,
}

#[function_component(Dialog)]
pub fn dialog(props: &DialogProps) -> Html {
    let mut classes = classes!("dialog");
    if let Some(variant) = &props.variant {
        classes.push(format!("dialog-{}", variant));
    }
    classes.push(props.class.clone());

    html! {
        <div class={classes}>
            { for props.children.iter() }
        </div>
    }
}
