use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct ListProps {
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub variant: Option<String>,
}

#[function_component(List)]
pub fn list(props: &ListProps) -> Html {
    let mut classes = classes!("list");
    if let Some(variant) = &props.variant {
        classes.push(format!("list-{}", variant));
    }
    classes.push(props.class.clone());

    html! {
        <div class={classes}>
            { for props.children.iter() }
        </div>
    }
}
