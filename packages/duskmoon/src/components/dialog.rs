use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct DialogProps {
    /// Stable DOM id targeted by `commandfor` on dialog invoker buttons.
    pub id: AttrValue,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub variant: Option<String>,
}

#[function_component(Dialog)]
pub fn dialog(props: &DialogProps) -> Html {
    dialog_view(props)
}

pub(crate) fn dialog_view(props: &DialogProps) -> Html {
    let mut classes = classes!("dialog");
    if let Some(variant) = &props.variant {
        classes.push(format!("dialog-{}", variant));
    }
    classes.push(props.class.clone());

    html! {
        <dialog id={props.id.clone()} class={classes}>
            { for props.children.iter() }
        </dialog>
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;
    use yew::virtual_dom::VNode;

    #[test]
    fn renders_a_native_dialog_target() {
        let props = DialogProps {
            id: "confirm-deploy".into(),
            class: classes!("dialog-divider"),
            children: Children::default(),
            variant: Some("primary".to_owned()),
        };
        let VNode::VTag(tag) = dialog_view(&props) else {
            panic!("dialog should render an HTML element");
        };
        let attributes: HashMap<_, _> = tag.attributes.iter().collect();

        assert_eq!(tag.tag(), "dialog");
        assert_eq!(attributes.get("id"), Some(&"confirm-deploy"));
        assert_eq!(
            attributes.get("class"),
            Some(&"dialog dialog-primary dialog-divider")
        );
        assert_eq!(attributes.get("open"), None);
    }
}
