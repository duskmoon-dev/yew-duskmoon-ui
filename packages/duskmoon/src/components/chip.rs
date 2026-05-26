use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct ChipProps {
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub variant: Option<String>,
}

#[function_component(Chip)]
pub fn chip(props: &ChipProps) -> Html {
    let mut classes = classes!("chip");
    if let Some(variant) = &props.variant {
        classes.push(format!("chip-{}", variant));
    }
    classes.push(props.class.clone());

    html! {
        <div class={classes}>
            { for props.children.iter() }
        </div>
    }
}
