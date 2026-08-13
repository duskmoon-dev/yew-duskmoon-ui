use yew::prelude::*;
use yew::virtual_dom::VNode;

#[derive(Properties, Clone, PartialEq)]
pub struct TextareaProps {
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub auto_size: bool,
    #[prop_or_default]
    pub variant: Option<String>,
}

#[function_component(Textarea)]
pub fn textarea(props: &TextareaProps) -> Html {
    let classes = textarea_classes(props);
    let default_value = children_text(&props.children);

    html! {
        <textarea class={classes} defaultvalue={default_value} />
    }
}

fn textarea_classes(props: &TextareaProps) -> Classes {
    let mut classes = classes!("textarea");
    if let Some(variant) = &props.variant {
        classes.push(format!("textarea-{}", variant));
    }
    if props.auto_size {
        classes.push("textarea-auto-resize");
    }
    classes.push(props.class.clone());
    classes
}

fn children_text(children: &Children) -> AttrValue {
    fn push_text(node: &VNode, output: &mut String) {
        match node {
            VNode::VText(text) => output.push_str(&text.text),
            VNode::VList(children) => {
                for child in children.iter() {
                    push_text(child, output);
                }
            }
            VNode::VTag(tag) => {
                if let Some(children) = tag.children() {
                    push_text(children, output);
                }
            }
            _ => {}
        }
    }

    let mut output = String::new();
    for child in children.iter() {
        push_text(&child, &mut output);
    }
    output.into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adds_auto_resize_class_only_when_enabled() {
        let mut props = TextareaProps {
            class: Classes::new(),
            children: Children::default(),
            auto_size: false,
            variant: None,
        };

        assert!(!textarea_classes(&props).contains("textarea-auto-resize"));

        props.auto_size = true;

        assert!(textarea_classes(&props).contains("textarea-auto-resize"));
    }

    #[test]
    fn converts_text_children_to_the_initial_value() {
        let children = Children::new(vec![
            html! { "first" },
            html! { <span>{ " second" }</span> },
        ]);

        assert_eq!(children_text(&children), "first second");
    }
}
