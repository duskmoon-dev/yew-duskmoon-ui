use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct StepperProps {
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub variant: Option<String>,
}

#[function_component(Stepper)]
pub fn stepper(props: &StepperProps) -> Html {
    let mut classes = classes!("stepper");
    if let Some(variant) = &props.variant {
        classes.push(format!("stepper-{}", variant));
    }
    classes.push(props.class.clone());

    html! {
        <div class={classes}>
            { for props.children.iter() }
        </div>
    }
}
