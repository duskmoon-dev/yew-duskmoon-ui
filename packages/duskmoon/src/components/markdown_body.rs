use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct MarkdownBodyProps {
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub variant: Option<String>,
}

#[function_component(MarkdownBody)]
pub fn markdown_body(props: &MarkdownBodyProps) -> Html {
    let mut classes = classes!("markdown-body");
    if let Some(variant) = &props.variant {
        classes.push(format!("markdown-body-{}", variant));
    }
    classes.push(props.class.clone());

    html! {
        <div class={classes}>
            { for props.children.iter() }
        </div>
    }
}
