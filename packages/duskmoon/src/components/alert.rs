use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct AlertProps {
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub variant: Option<String>,
}

#[function_component(Alert)]
pub fn alert(props: &AlertProps) -> Html {
    let mut classes = classes!("alert");
    if let Some(variant) = &props.variant {
        classes.push(format!("alert-{}", variant));
    }
    classes.push(props.class.clone());

    html! {
        <div class={classes}>
            { for props.children.iter() }
        </div>
    }
}
