use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct CascaderProps {
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub variant: Option<String>,
}

#[function_component(Cascader)]
pub fn cascader(props: &CascaderProps) -> Html {
    let mut classes = classes!("cascader");
    if let Some(variant) = &props.variant {
        classes.push(format!("cascader-{}", variant));
    }
    classes.push(props.class.clone());

    html! {
        <div class={classes}>
            { for props.children.iter() }
        </div>
    }
}
