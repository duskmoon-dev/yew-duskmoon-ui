use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct TooltipProps {
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub variant: Option<String>,
}

#[function_component(Tooltip)]
pub fn tooltip(props: &TooltipProps) -> Html {
    let mut classes = classes!("tooltip");
    if let Some(variant) = &props.variant {
        classes.push(format!("tooltip-{}", variant));
    }
    classes.push(props.class.clone());

    html! {
        <div class={classes}>
            { for props.children.iter() }
        </div>
    }
}
