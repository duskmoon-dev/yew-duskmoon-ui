use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct BottomsheetProps {
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub variant: Option<String>,
}

#[function_component(Bottomsheet)]
pub fn bottomsheet(props: &BottomsheetProps) -> Html {
    let mut classes = classes!("bottomsheet");
    if let Some(variant) = &props.variant {
        classes.push(format!("bottomsheet-{}", variant));
    }
    classes.push(props.class.clone());

    html! {
        <div class={classes}>
            { for props.children.iter() }
        </div>
    }
}
