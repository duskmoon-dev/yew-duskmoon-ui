use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct MultiSelectProps {
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub variant: Option<String>,
}

#[function_component(MultiSelect)]
pub fn multi_select(props: &MultiSelectProps) -> Html {
    let mut classes = classes!("multi-select");
    if let Some(variant) = &props.variant {
        classes.push(format!("multi-select-{}", variant));
    }
    classes.push(props.class.clone());

    html! {
        <div class={classes}>
            { for props.children.iter() }
        </div>
    }
}
