use yew::prelude::*;

use super::dialog::{dialog_view, DialogProps};

#[derive(Properties, Clone, PartialEq)]
pub struct ModalProps {
    /// Stable DOM id targeted by `commandfor` on dialog invoker buttons.
    pub id: AttrValue,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub variant: Option<String>,
}

/// Legacy component name implementing the native [`super::Dialog`] contract.
/// Existing `.modal-*` markup must be migrated to `.dialog-*` children.
#[function_component(Modal)]
pub fn modal(props: &ModalProps) -> Html {
    modal_view(props)
}

fn modal_view(props: &ModalProps) -> Html {
    dialog_view(&DialogProps {
        id: props.id.clone(),
        class: props.class.clone(),
        children: props.children.clone(),
        variant: props.variant.clone(),
    })
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;
    use yew::virtual_dom::VNode;

    #[test]
    fn legacy_name_uses_the_native_dialog_contract() {
        let props = ModalProps {
            id: "legacy-modal".into(),
            class: Classes::new(),
            children: Children::default(),
            variant: None,
        };
        let VNode::VTag(tag) = modal_view(&props) else {
            panic!("modal compatibility component should render a native dialog");
        };
        let attributes: HashMap<_, _> = tag.attributes.iter().collect();

        assert_eq!(tag.tag(), "dialog");
        assert_eq!(attributes.get("id"), Some(&"legacy-modal"));
        assert_eq!(attributes.get("class"), Some(&"dialog"));
    }
}
