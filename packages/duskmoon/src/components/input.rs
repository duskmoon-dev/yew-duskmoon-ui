use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct InputProps {
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub variant: Option<String>,
}

#[function_component(Input)]
pub fn input(props: &InputProps) -> Html {
    let mut classes = classes!("input");
    if let Some(variant) = &props.variant {
        classes.push(format!("input-{}", variant));
    }
    classes.push(props.class.clone());

    html! {
        <div class={classes}>
            { for props.children.iter() }
        </div>
    }
}
