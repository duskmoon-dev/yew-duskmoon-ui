use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct BreadcrumbsProps {
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub variant: Option<String>,
}

#[function_component(Breadcrumbs)]
pub fn breadcrumbs(props: &BreadcrumbsProps) -> Html {
    let mut classes = classes!("breadcrumbs");
    if let Some(variant) = &props.variant {
        classes.push(format!("breadcrumbs-{}", variant));
    }
    classes.push(props.class.clone());

    html! {
        <div class={classes}>
            { for props.children.iter() }
        </div>
    }
}
