use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct SegmentControlProps {
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub variant: Option<String>,
}

#[function_component(SegmentControl)]
pub fn segment_control(props: &SegmentControlProps) -> Html {
    let mut classes = classes!("segment-control");
    if let Some(variant) = &props.variant {
        classes.push(format!("segment-control-{}", variant));
    }
    classes.push(props.class.clone());

    html! {
        <div class={classes}>
            { for props.children.iter() }
        </div>
    }
}
