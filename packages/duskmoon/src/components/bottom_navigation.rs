use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct BottomNavigationProps {
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub variant: Option<String>,
}

#[function_component(BottomNavigation)]
pub fn bottom_navigation(props: &BottomNavigationProps) -> Html {
    let mut classes = classes!("bottom-navigation");
    if let Some(variant) = &props.variant {
        classes.push(format!("bottom-navigation-{}", variant));
    }
    classes.push(props.class.clone());

    html! {
        <div class={classes}>
            { for props.children.iter() }
        </div>
    }
}
