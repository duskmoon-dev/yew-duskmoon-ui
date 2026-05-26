use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct DividerProps {
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub variant: Option<String>,
}

#[function_component(Divider)]
pub fn divider(props: &DividerProps) -> Html {
    let mut classes = classes!("divider");
    if let Some(variant) = &props.variant {
        classes.push(format!("divider-{}", variant));
    }
    classes.push(props.class.clone());

    html! {
        <div class={classes}>
            { for props.children.iter() }
        </div>
    }
}
