use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct FormGroupProps {
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub variant: Option<String>,
}

#[function_component(FormGroup)]
pub fn form_group(props: &FormGroupProps) -> Html {
    let mut classes = classes!("form-group");
    if let Some(variant) = &props.variant {
        classes.push(format!("form-group-{}", variant));
    }
    classes.push(props.class.clone());

    html! {
        <div class={classes}>
            { for props.children.iter() }
        </div>
    }
}
