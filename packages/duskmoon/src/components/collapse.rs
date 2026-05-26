use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct CollapseProps {
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub variant: Option<String>,
}

#[function_component(Collapse)]
pub fn collapse(props: &CollapseProps) -> Html {
    let mut classes = classes!("collapse");
    if let Some(variant) = &props.variant {
        classes.push(format!("collapse-{}", variant));
    }
    classes.push(props.class.clone());

    html! {
        <div class={classes}>
            { for props.children.iter() }
        </div>
    }
}
