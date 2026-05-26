use web_sys::MouseEvent as Event;
use yew::prelude::*;
use yew::virtual_dom::AttrValue;
use strum_macros::Display;
use strum_macros::EnumIter;
use stylist::css;
use stylist::yew::use_style;

#[derive(Clone, PartialEq, Debug, Display, EnumIter)]
pub enum TypographyLevel {
    Default,
    H1,
    H2,
    H3,
    H4,
    H5,
    H6,
}

/// Props for [`Typography`]
#[derive(Properties, Clone, PartialEq)]
pub struct TypographyProps {
    #[prop_or("p".to_string())]
    pub r#tag: String,
    /// CSS classes to add to the anchor element (optional).
    #[prop_or_default]
    pub classes: Classes,
    #[prop_or(TypographyLevel::Default)]
    pub r#level: TypographyLevel,
    #[prop_or_default]
    pub href: AttrValue,
    #[prop_or_default]
    pub target: AttrValue,
    #[prop_or_default]
    pub rel: AttrValue,
    /// infor part
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub onclick: Callback<Event>,
}

/// Typography component
#[function_component(Typography)]
pub fn typography(props: &TypographyProps) -> Html {
    let style = use_style(css!(
        r#"
        &.h1 {
            font-size: 2.5rem;
        }
        &.h2 {
            font-size: 2rem;
        }
        &.h3 {
            font-size: 1.75rem;
        }
        &.h4 {
            font-size: 1.5rem;
        }
        &.h5 {
            font-size: 1.25rem;
        }
        &.h6 {
            font-size: 1rem;
        }
    "#
    ));
    let owned_props = props.clone();
    let onclick_func = props.onclick.clone();

    match props.r#level {
        TypographyLevel::H1 => {
            html! {
                <h1 class={ classes!(style, "h1", owned_props.classes) }>
                { for owned_props.children.iter() }
                </h1>
            }
        },
        TypographyLevel::H2 => {
            html! {
                <h2 class={ classes!(style, "h2", owned_props.classes) }>
                { for owned_props.children.iter() }
                </h2>
            }
        },
        TypographyLevel::H3 => {
            html! {
                <h3 class={ classes!(style, "h3", owned_props.classes) }>
                { for owned_props.children.iter() }
                </h3>
            }
        },
        TypographyLevel::H4 => {
            html! {
                <h4 class={ classes!(style, "h4", owned_props.classes) }>
                { for owned_props.children.iter() }
                </h4>
            }
        },
        TypographyLevel::H5 => {
            html! {
                <h5 class={ classes!(style, "h5", owned_props.classes) }>
                { for owned_props.children.iter() }
                </h5>
            }
        },
        TypographyLevel::H6 => {
            html! {
                <h6 class={ classes!(style, "h6", owned_props.classes) }>
                { for owned_props.children.iter() }
                </h6>
            }
        },
        _ => html! {
            <p
                class={ classes!(style, "text", owned_props.classes) }
                onclick={ move |e: Event| onclick_func.emit(e) }
            >
                { for owned_props.children.iter() }
            </p>
        },
    }
}

