use strum_macros::Display;
use strum_macros::EnumIter;
use web_sys::MouseEvent as Event;
use yew::prelude::*;
use yew::virtual_dom::AttrValue;

#[derive(Clone, PartialEq, Debug, Display, EnumIter)]
pub enum ButtonType {
    Default,
    Primary,
    Dashed,
    Danger,
    Link,
    Text,
    Circle,
    Round,
    Block,
}

/// Props for [`Button`]
#[derive(Properties, Clone, PartialEq)]
pub struct ButtonProps {
    /// CSS classes to add to the element (optional).
    #[prop_or_default]
    pub classes: Classes,
    #[prop_or(ButtonType::Default)]
    pub r#type: ButtonType,
    #[prop_or_default]
    pub variant: Option<String>,
    #[prop_or_default]
    pub href: AttrValue,
    #[prop_or_default]
    pub target: AttrValue,
    #[prop_or_default]
    pub rel: AttrValue,
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or_default]
    pub loading: bool,
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub onclick: Callback<Event>,
}

/// Button component using Tailwind CSS classes
#[function_component(Button)]
pub fn button(props: &ButtonProps) -> Html {
    let owned_props = props.clone();
    let onclick_func = props.onclick.clone();

    // Base Tailwind class
    let mut class_list = classes!("btn");
    let has_variant = props.variant.is_some();

    // Map ButtonType to Tailwind class modifiers
    match props.r#type {
        ButtonType::Primary if !has_variant => class_list.push("btn-primary"),
        ButtonType::Dashed => class_list.push("btn-outlined"), // Dashed behaves like outlined
        ButtonType::Danger if !has_variant => class_list.push("btn-error"),
        ButtonType::Link => class_list.push("btn-link"),
        ButtonType::Text => class_list.push("btn-text"),
        ButtonType::Circle => class_list.push("btn-icon"),
        ButtonType::Round => {} // MD3 buttons are round by default
        ButtonType::Block => class_list.push("btn-block"),
        ButtonType::Primary | ButtonType::Danger | ButtonType::Default => {}
    }

    if let Some(variant) = &owned_props.variant {
        class_list.push(format!("btn-{}", variant));
    }

    if owned_props.disabled {
        class_list.push("disabled");
    }
    if owned_props.loading {
        class_list.push("btn-loading");
    }

    class_list.push(owned_props.classes);

    match props.r#type {
        ButtonType::Link => html! {
            <a
                class={ class_list }
                onclick={ move |e: Event| {
                  if !owned_props.disabled && !owned_props.loading {
                    onclick_func.emit(e)
                  }
                }}
                href={ owned_props.href }
                target={ owned_props.target }
                rel={ owned_props.rel }
                disabled={ owned_props.disabled || owned_props.loading }
            >
                { for owned_props.children.iter() }
            </a>
        },
        _ => html! {
            <button
                class={ class_list }
                onclick={ move |e: Event| {
                  if !owned_props.disabled && !owned_props.loading {
                    onclick_func.emit(e)
                  }
                }}
                disabled={ owned_props.disabled || owned_props.loading }
            >
                { for owned_props.children.iter() }
            </button>
        },
    }
}
