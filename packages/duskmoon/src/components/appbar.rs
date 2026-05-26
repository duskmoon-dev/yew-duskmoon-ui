use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct AppbarProps {
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub variant: Option<String>,
}

#[function_component(Appbar)]
pub fn appbar(props: &AppbarProps) -> Html {
    let mut classes = classes!("appbar");
    if let Some(variant) = &props.variant {
        classes.push(format!("appbar-{}", variant));
    }
    classes.push(props.class.clone());

    html! {
        <div class={classes}>
            { for props.children.iter() }
        </div>
    }
}
