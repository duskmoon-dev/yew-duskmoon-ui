use web_sys::MouseEvent as Event;
use yew::prelude::*;
use yew::virtual_dom::AttrValue;
use strum_macros::Display;
use strum_macros::EnumIter;
use stylist::css;
use stylist::yew::use_style;

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
    /// CSS classes to add to the anchor element (optional).
    #[prop_or_default]
    pub classes: Classes,
    #[prop_or(ButtonType::Default)]
    pub r#type: ButtonType,
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
    /// infor part
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub onclick: Callback<Event>,
}

/// Button component
#[function_component(Button)]
pub fn button(props: &ButtonProps) -> Html {
    let style = use_style(css!(
        r#"
        line-height: 1.5715;
        position: relative;
        display: inline-flex;
        justify-content: center;
        align-items: center;
        font-weight: 400;
        white-space: nowrap;
        background-image: none;
        border: 1px solid transparent;
        box-shadow: 0 2px 0 rgba(0, 0, 0, 0.015);
        cursor: pointer;
        transition: all 0.3s cubic-bezier(0.645, 0.045, 0.355, 1);
        user-select: none;
        touch-action: manipulation;
        height: 2.25rem;
        padding: 0.3rem 1.1rem;
        font-size: 1rem;
        border-radius: 0.2rem;
        color: rgba(0, 0, 0, 0.85);
        border-color: var(--btn-border, #d9d9d9);
        background: #fff;
        text-decoration: none;

        & icon-spin {
            margin-left: -0.4rem;
            margin-right: 0.4rem;
        }
        &.btn-primary {
            color: #fff;
            border-color: var(--btn-primary, #1890ff);
            background: var(--btn-primary, #1890ff);
        }
        &.btn-dashed {
            border: 1px dashed transparent;
            background: transparent;
            border-color: rgba(0, 0, 0, 0.85);
        }
        &.btn-danger {
            color: rgb(255, 255, 255);
            border-color: var(--btn-danger, #ff4d4f);
            background: var(--btn-danger, #ff4d4f);
            text-shadow: rgb(0 0 0 / 12%) 0px -1px 0px;
            box-shadow: rgb(0 0 0 / 4%) 0px 2px 0px;
        }
        &.btn-link,
        &.btn-text {
            color: rgba(0, 0, 0, 0.85);
            border: none;
            background: transparent;
            text-shadow: rgb(0 0 0 / 12%) 0px -1px 0px;
            box-shadow: none;
        }
        &.btn-link {
            color: var(--btn-link, #1890ff);
        }
        &.btn-round {
            height: 2.25rem;
            padding: 0.3rem 1.1rem;
            font-size: 1rem;
            border-radius: 2.25rem;
        }
        &.btn-circle {
            height: 2.25rem;
            width: 2.25rem;
            padding: 0.3rem 1.1rem;
            font-size: 1rem;
            border-radius: 50%;
        }
        &.btn-circle icon-spin {
            position: absolute;
            margin: 0;
        }
        &.btn-block {
            height: 2.25rem;
            width: 100%;
            padding: 0.3rem 1.1rem;
            font-size: 1rem;
        }
        &:hover {
            color: var(--btn-hover, #40a9ff);
            border-color: var(--btn-hover, #40a9ff);
        }
        &.btn-primary:hover {
            color: #fff;
            background: var(--btn-primary-hover, #40a9ff);
            border-color: var(--btn-primary-hover, #40a9ff);
        }
        &.btn-danger:hover {
            color: #fff;
            background: var(--btn-danger-hover, #ff7875);
            border-color: var(--btn-danger-hover, #ff7875);
        }
        &[disabled] {
            cursor: not-allowed;
        }
        &[disabled].btn-loading {
            cursor: progress;
        }
        &[disabled]:not(.btn-loading),
        &[disabled].disabled {
            color: rgba(0, 0, 0, 0.25);
            background: var(--btn-disabled, #f5f5f5);
            broder-color: var(--btn-border-disabled, #d9d9d9);
            border: none;
        }
    "#
    ));
    let owned_props = props.clone();
    let onclick_func = props.onclick.clone();
    let type_class = match props.r#type {
        ButtonType::Primary => "btn-primary",
        ButtonType::Dashed => "btn-dashed",
        ButtonType::Danger => "btn-danger",
        ButtonType::Link => "btn-link",
        ButtonType::Text => "btn-text",
        ButtonType::Circle => "btn-circle",
        ButtonType::Round => "btn-round",
        ButtonType::Block => "btn-block",
        _ => "",
    };
    let disabled_class = if owned_props.disabled { "disabled" } else { "" };
    let loading_class = if owned_props.loading { "btn-loading" } else { "" };

    match props.r#type {
        ButtonType::Link => html! {
            <a
                class={ classes!(style, type_class, disabled_class, loading_class, owned_props.classes) }
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
                class={ classes!(style, type_class, disabled_class, loading_class, owned_props.classes) }
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
