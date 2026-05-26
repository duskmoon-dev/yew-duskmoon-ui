use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct ThemeControllerProps {
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub variant: Option<String>,
}

#[function_component(ThemeController)]
pub fn theme_controller(props: &ThemeControllerProps) -> Html {
    let mut classes = classes!("theme-controller");
    if let Some(variant) = &props.variant {
        classes.push(format!("theme-controller-{}", variant));
    }
    classes.push(props.class.clone());

    html! {
        <div class={classes}>
            { for props.children.iter() }
        </div>
    }
}
