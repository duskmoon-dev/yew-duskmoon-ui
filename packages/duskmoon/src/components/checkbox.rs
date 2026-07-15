use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct CheckboxProps {
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub variant: Option<String>,
}

#[function_component(Checkbox)]
pub fn checkbox(props: &CheckboxProps) -> Html {
    let mut classes = if props.children.is_empty() {
        classes!("checkbox")
    } else {
        classes!("checkbox-group")
    };
    if let Some(variant) = &props.variant {
        classes.push(format!("checkbox-{}", variant));
    }
    classes.push(props.class.clone());

    html! {
        <div class={classes}>
            { for props.children.iter() }
        </div>
    }
}
