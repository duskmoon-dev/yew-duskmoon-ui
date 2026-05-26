use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct ProgressProps {
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub variant: Option<String>,
}

#[function_component(Progress)]
pub fn progress(props: &ProgressProps) -> Html {
    let mut classes = classes!("progress");
    if let Some(variant) = &props.variant {
        classes.push(format!("progress-{}", variant));
    }
    classes.push(props.class.clone());

    html! {
        <div class={classes}>
            { for props.children.iter() }
        </div>
    }
}
