use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct SkeletonProps {
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub variant: Option<String>,
}

#[function_component(Skeleton)]
pub fn skeleton(props: &SkeletonProps) -> Html {
    let mut classes = classes!("skeleton");
    if let Some(variant) = &props.variant {
        classes.push(format!("skeleton-{}", variant));
    }
    classes.push(props.class.clone());

    html! {
        <div class={classes}>
            { for props.children.iter() }
        </div>
    }
}
