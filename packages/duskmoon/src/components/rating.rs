use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct RatingProps {
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub variant: Option<String>,
}

#[function_component(Rating)]
pub fn rating(props: &RatingProps) -> Html {
    let mut classes = classes!("rating");
    if let Some(variant) = &props.variant {
        classes.push(format!("rating-{}", variant));
    }
    classes.push(props.class.clone());

    html! {
        <div class={classes}>
            { for props.children.iter() }
        </div>
    }
}
