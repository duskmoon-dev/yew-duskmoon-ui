use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct NestedMenuProps {
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub variant: Option<String>,
}

#[function_component(NestedMenu)]
pub fn nested_menu(props: &NestedMenuProps) -> Html {
    let mut classes = classes!("nested-menu");
    if let Some(variant) = &props.variant {
        classes.push(format!("nested-menu-{}", variant));
    }
    classes.push(props.class.clone());

    html! {
        <div class={classes}>
            { for props.children.iter() }
        </div>
    }
}
