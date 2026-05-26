use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct TabsProps {
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub variant: Option<String>,
}

#[function_component(Tabs)]
pub fn tabs(props: &TabsProps) -> Html {
    let mut classes = classes!("tabs");
    if let Some(variant) = &props.variant {
        classes.push(format!("tabs-{}", variant));
    }
    classes.push(props.class.clone());

    html! {
        <div class={classes}>
            { for props.children.iter() }
        </div>
    }
}
