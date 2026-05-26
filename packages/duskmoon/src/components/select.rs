use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct SelectProps {
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub variant: Option<String>,
}

#[function_component(Select)]
pub fn select(props: &SelectProps) -> Html {
    let mut classes = classes!("select");
    if let Some(variant) = &props.variant {
        classes.push(format!("select-{}", variant));
    }
    classes.push(props.class.clone());

    html! {
        <div class={classes}>
            { for props.children.iter() }
        </div>
    }
}
