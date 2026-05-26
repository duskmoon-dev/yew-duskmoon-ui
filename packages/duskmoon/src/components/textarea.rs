use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct TextareaProps {
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub variant: Option<String>,
}

#[function_component(Textarea)]
pub fn textarea(props: &TextareaProps) -> Html {
    let mut classes = classes!("textarea");
    if let Some(variant) = &props.variant {
        classes.push(format!("textarea-{}", variant));
    }
    classes.push(props.class.clone());

    html! {
        <div class={classes}>
            { for props.children.iter() }
        </div>
    }
}
