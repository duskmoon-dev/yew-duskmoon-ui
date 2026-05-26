use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct DrawerProps {
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub variant: Option<String>,
}

#[function_component(Drawer)]
pub fn drawer(props: &DrawerProps) -> Html {
    let mut classes = classes!("drawer");
    if let Some(variant) = &props.variant {
        classes.push(format!("drawer-{}", variant));
    }
    classes.push(props.class.clone());

    html! {
        <div class={classes}>
            { for props.children.iter() }
        </div>
    }
}
