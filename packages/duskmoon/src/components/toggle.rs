use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct ToggleProps {
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub variant: Option<String>,
}

#[function_component(Toggle)]
pub fn toggle(props: &ToggleProps) -> Html {
    let mut classes = classes!("toggle");
    if let Some(variant) = &props.variant {
        classes.push(format!("toggle-{}", variant));
    }
    classes.push(props.class.clone());

    html! {
        <div class={classes}>
            { for props.children.iter() }
        </div>
    }
}
