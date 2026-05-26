use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct RadioProps {
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub variant: Option<String>,
}

#[function_component(Radio)]
pub fn radio(props: &RadioProps) -> Html {
    let mut classes = classes!("radio");
    if let Some(variant) = &props.variant {
        classes.push(format!("radio-{}", variant));
    }
    classes.push(props.class.clone());

    html! {
        <div class={classes}>
            { for props.children.iter() }
        </div>
    }
}
