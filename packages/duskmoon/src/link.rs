use serde::Serialize;
use yew::prelude::*;

use yew_router::components::Link as YewLink;
use yew_router::Routable;

/// Props for [`Link`]
#[derive(Properties, Clone, PartialEq)]
pub struct LinkProps<R, Q = ()>
where
    R: Routable,
    Q: Clone + PartialEq + Serialize,
{
    /// CSS classes to add to the anchor element (optional).
    #[prop_or_default]
    pub classes: Classes,
    #[prop_or_default]
    pub variant: Option<String>,
    /// Route that will be pushed when the anchor is clicked.
    pub to: R,
    /// Route query data
    #[prop_or_default]
    pub query: Option<Q>,
    #[prop_or_default]
    pub disabled: bool,
    /// [`NodeRef`](yew::html::NodeRef) for the `<a>` element.
    #[prop_or_default]
    pub anchor_ref: NodeRef,
    #[prop_or_default]
    pub children: Children,
}

/// Link component using Tailwind CSS utility classes
#[function_component(Link)]
pub fn link<R, Q = ()>(props: &LinkProps<R, Q>) -> Html
where
    R: Routable + 'static,
    Q: Clone + PartialEq + Serialize + 'static,
{
    let np = props.clone();

    // Default link styles mimicking original layout with Tailwind
    let link_classes = classes!(
        "link",
        "inline-flex",
        "items-center",
        "justify-center",
        "px-3",
        "py-1.5",
        "text-center",
        "cursor-pointer",
        "transition-all",
        "duration-300",
        "no-underline",
        "hover:text-opacity-80",
    );
    let mut link_classes = link_classes;

    if let Some(variant) = &np.variant {
        link_classes.push(format!("link-{}", variant));
    } else {
        link_classes.push("text-primary");
    }

    link_classes.push(np.classes.clone());

    html! {
        <YewLink<R,Q> classes={link_classes}
            to={np.to}
            query={np.query}
            disabled={np.disabled}
            anchor_ref={np.anchor_ref}
        >
            { np.children }
        </YewLink<R,Q>>
    }
}
