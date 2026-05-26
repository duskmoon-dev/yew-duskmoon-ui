use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct SliderProps {
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub variant: Option<String>,
}

#[function_component(Slider)]
pub fn slider(props: &SliderProps) -> Html {
    let mut classes = classes!("slider");
    if let Some(variant) = &props.variant {
        classes.push(format!("slider-{}", variant));
    }
    classes.push(props.class.clone());

    html! {
        <div class={classes}>
            { for props.children.iter() }
        </div>
    }
}
