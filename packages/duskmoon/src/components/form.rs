use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct FormProps {
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub variant: Option<String>,
}

#[function_component(Form)]
pub fn form(props: &FormProps) -> Html {
    let mut classes = classes!("form");
    if let Some(variant) = &props.variant {
        classes.push(format!("form-{}", variant));
    }
    classes.push(props.class.clone());

    html! {
        <div class={classes}>
            { for props.children.iter() }
        </div>
    }
}
