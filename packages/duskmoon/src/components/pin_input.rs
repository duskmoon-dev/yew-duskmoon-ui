use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct PinInputProps {
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub variant: Option<String>,
}

#[function_component(PinInput)]
pub fn pin_input(props: &PinInputProps) -> Html {
    let mut classes = classes!("pin-input");
    if let Some(variant) = &props.variant {
        classes.push(format!("pin-input-{}", variant));
    }
    classes.push(props.class.clone());

    html! {
        <div class={classes}>
            { for props.children.iter() }
        </div>
    }
}
