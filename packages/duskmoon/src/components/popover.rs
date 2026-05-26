use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct PopoverProps {
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub variant: Option<String>,
}

#[function_component(Popover)]
pub fn popover(props: &PopoverProps) -> Html {
    let mut classes = classes!("popover");
    if let Some(variant) = &props.variant {
        classes.push(format!("popover-{}", variant));
    }
    classes.push(props.class.clone());

    html! {
        <div class={classes}>
            { for props.children.iter() }
        </div>
    }
}
