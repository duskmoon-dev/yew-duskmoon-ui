use strum_macros::Display;
use strum_macros::EnumIter;
use web_sys::MouseEvent as Event;
use yew::prelude::*;
use yew::virtual_dom::AttrValue;

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
    /// CSS classes to add to the element (optional).
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
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub onclick: Callback<Event>,
}

/// Typography component using Tailwind CSS utility classes
#[function_component(Typography)]
pub fn typography(props: &TypographyProps) -> Html {
    let owned_props = props.clone();
    let onclick_func = props.onclick.clone();

    // Map TypographyLevel to standard Tailwind heading classes
    let level_class = match props.r#level {
        TypographyLevel::H1 => "text-4xl font-bold mb-4",
        TypographyLevel::H2 => "text-3xl font-bold mb-3",
        TypographyLevel::H3 => "text-2xl font-semibold mb-2",
        TypographyLevel::H4 => "text-xl font-semibold mb-2",
        TypographyLevel::H5 => "text-lg font-medium mb-1",
        TypographyLevel::H6 => "text-base font-medium mb-1",
        TypographyLevel::Default => "text-base leading-relaxed",
    };

    let class_list = classes!(level_class, owned_props.classes);

    match props.r#level {
        TypographyLevel::H1 => {
            html! {
                <h1 class={ class_list } onclick={ move |e: Event| onclick_func.emit(e) }>
                    { for owned_props.children.iter() }
                </h1>
            }
        }
        TypographyLevel::H2 => {
            html! {
                <h2 class={ class_list } onclick={ move |e: Event| onclick_func.emit(e) }>
                    { for owned_props.children.iter() }
                </h2>
            }
        }
        TypographyLevel::H3 => {
            html! {
                <h3 class={ class_list } onclick={ move |e: Event| onclick_func.emit(e) }>
                    { for owned_props.children.iter() }
                </h3>
            }
        }
        TypographyLevel::H4 => {
            html! {
                <h4 class={ class_list } onclick={ move |e: Event| onclick_func.emit(e) }>
                    { for owned_props.children.iter() }
                </h4>
            }
        }
        TypographyLevel::H5 => {
            html! {
                <h5 class={ class_list } onclick={ move |e: Event| onclick_func.emit(e) }>
                    { for owned_props.children.iter() }
                </h5>
            }
        }
        TypographyLevel::H6 => {
            html! {
                <h6 class={ class_list } onclick={ move |e: Event| onclick_func.emit(e) }>
                    { for owned_props.children.iter() }
                </h6>
            }
        }
        _ => html! {
            <p
                class={ class_list }
                onclick={ move |e: Event| onclick_func.emit(e) }
            >
                { for owned_props.children.iter() }
            </p>
        },
    }
}
