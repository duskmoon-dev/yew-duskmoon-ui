use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct TimelineProps {
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub variant: Option<String>,
}

#[function_component(Timeline)]
pub fn timeline(props: &TimelineProps) -> Html {
    let mut classes = classes!("timeline");
    if let Some(variant) = &props.variant {
        classes.push(format!("timeline-{}", variant));
    }
    classes.push(props.class.clone());

    html! {
        <div class={classes}>
            { for props.children.iter() }
        </div>
    }
}
