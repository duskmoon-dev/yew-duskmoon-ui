use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct TimeInputProps {
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub variant: Option<String>,
}

#[function_component(TimeInput)]
pub fn time_input(props: &TimeInputProps) -> Html {
    let mut classes = classes!("time-input");
    if let Some(variant) = &props.variant {
        classes.push(format!("time-input-{}", variant));
    }
    classes.push(props.class.clone());

    html! {
        <div class={classes}>
            { for props.children.iter() }
        </div>
    }
}
