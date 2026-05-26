use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct DatepickerProps {
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub variant: Option<String>,
}

#[function_component(Datepicker)]
pub fn datepicker(props: &DatepickerProps) -> Html {
    let mut classes = classes!("datepicker");
    if let Some(variant) = &props.variant {
        classes.push(format!("datepicker-{}", variant));
    }
    classes.push(props.class.clone());

    html! {
        <div class={classes}>
            { for props.children.iter() }
        </div>
    }
}
