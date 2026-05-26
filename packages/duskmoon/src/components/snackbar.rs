use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct SnackbarProps {
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub variant: Option<String>,
}

#[function_component(Snackbar)]
pub fn snackbar(props: &SnackbarProps) -> Html {
    let mut classes = classes!("snackbar");
    if let Some(variant) = &props.variant {
        classes.push(format!("snackbar-{}", variant));
    }
    classes.push(props.class.clone());

    html! {
        <div class={classes}>
            { for props.children.iter() }
        </div>
    }
}
