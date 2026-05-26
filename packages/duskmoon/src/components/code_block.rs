use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct CodeBlockProps {
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub variant: Option<String>,
}

#[function_component(CodeBlock)]
pub fn code_block(props: &CodeBlockProps) -> Html {
    let mut classes = classes!("code-block");
    if let Some(variant) = &props.variant {
        classes.push(format!("code-block-{}", variant));
    }
    classes.push(props.class.clone());

    html! {
        <div class={classes}>
            { for props.children.iter() }
        </div>
    }
}
