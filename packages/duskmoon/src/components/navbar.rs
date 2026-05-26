use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct NavbarProps {
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub variant: Option<String>,
}

#[function_component(Navbar)]
pub fn navbar(props: &NavbarProps) -> Html {
    let mut classes = classes!("navbar");
    if let Some(variant) = &props.variant {
        classes.push(format!("navbar-{}", variant));
    }
    classes.push(props.class.clone());

    html! {
        <div class={classes}>
            { for props.children.iter() }
        </div>
    }
}
