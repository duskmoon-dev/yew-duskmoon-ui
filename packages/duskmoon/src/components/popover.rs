use yew::prelude::*;
use yew::virtual_dom::AttrValue;

use super::variants;

#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub enum PopoverTrigger {
    #[default]
    Click,
    Hover,
    Focus,
}

#[derive(Properties, Clone, PartialEq)]
pub struct PopoverProps {
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub variant: Option<String>,
    #[prop_or_default]
    pub trigger: PopoverTrigger,
    #[prop_or(AttrValue::from("Show popover"))]
    pub trigger_label: AttrValue,
    #[prop_or_else(default_trigger_class)]
    pub trigger_class: Classes,
}

#[function_component(Popover)]
pub fn popover(props: &PopoverProps) -> Html {
    let is_open = use_state_eq(|| false);
    let trigger = props.trigger;
    let placement = PopoverPlacement::from_classes(&props.class);
    let mut classes = classes!("popover");
    if let Some(variant) = &props.variant {
        classes.push(format!("popover-{}", variant));
    }
    if trigger == PopoverTrigger::Hover {
        classes.push("popover-hover");
    }
    if *is_open {
        classes.push("show");
        classes.push("popover-show");
    }
    classes.push(props.class.clone());

    let onclick = {
        let is_open = is_open.clone();
        Callback::from(move |_| {
            if trigger == PopoverTrigger::Click {
                is_open.set(!*is_open);
            }
        })
    };
    let onmouseenter = {
        let is_open = is_open.clone();
        Callback::from(move |_| {
            if trigger == PopoverTrigger::Hover {
                is_open.set(true);
            }
        })
    };
    let onmouseleave = {
        let is_open = is_open.clone();
        Callback::from(move |_| {
            if trigger == PopoverTrigger::Hover {
                is_open.set(false);
            }
        })
    };
    let onfocus = {
        let is_open = is_open.clone();
        Callback::from(move |_| {
            if trigger == PopoverTrigger::Focus {
                is_open.set(true);
            }
        })
    };
    let onblur = {
        let is_open = is_open.clone();
        Callback::from(move |_| {
            if trigger == PopoverTrigger::Focus {
                is_open.set(false);
            }
        })
    };

    let content_style = popover_content_style(props.variant.as_deref(), placement);
    let arrow_style = popover_arrow_style(props.variant.as_deref(), placement);
    let root_style = AttrValue::from(
        "position: relative; display: inline-block; width: max-content; justify-self: start; opacity: 1; visibility: visible; z-index: auto; min-width: 0; max-width: none; padding: 0; background: transparent; border: 0; box-shadow: none;",
    );
    let expanded = if *is_open { "true" } else { "false" };

    html! {
        <div class={classes} style={root_style} onmouseenter={onmouseenter} onmouseleave={onmouseleave}>
            <button
                type="button"
                class={props.trigger_class.clone()}
                aria-haspopup="dialog"
                aria-expanded={expanded}
                onclick={onclick}
                onfocus={onfocus}
                onblur={onblur}
            >
                { props.trigger_label.clone() }
            </button>
            <div class="popover-content" role="dialog" style={content_style}>
                {
                    if props.children.is_empty() {
                        html! { <div class="popover-body">{ "Popover content" }</div> }
                    } else {
                        html! { <>{ for props.children.iter() }</> }
                    }
                }
                <span class="popover-arrow" aria-hidden="true" style={arrow_style}></span>
            </div>
        </div>
    }
}

fn default_trigger_class() -> Classes {
    classes!("btn", "btn-primary")
}

#[derive(Clone, Copy)]
enum PopoverPlacement {
    Top,
    Bottom,
    Left,
    Right,
}

impl PopoverPlacement {
    fn from_classes(classes: &Classes) -> Self {
        if classes.contains("popover-bottom") {
            Self::Bottom
        } else if classes.contains("popover-left") {
            Self::Left
        } else if classes.contains("popover-right") {
            Self::Right
        } else {
            Self::Top
        }
    }

    fn content_style(self) -> &'static str {
        match self {
            Self::Top => "top: auto; right: auto; bottom: 100%; left: 50%; height: auto; margin-top: 0; margin-right: 0; margin-bottom: 0.75rem; margin-left: 0;",
            Self::Bottom => "top: 100%; right: auto; bottom: auto; left: 50%; height: auto; margin-top: 0.75rem; margin-right: 0; margin-bottom: 0; margin-left: 0;",
            Self::Left => "top: 50%; right: 100%; bottom: auto; left: auto; height: auto; margin-top: 0; margin-right: 0.75rem; margin-bottom: 0; margin-left: 0;",
            Self::Right => "top: 50%; right: auto; bottom: auto; left: 100%; height: auto; margin-top: 0; margin-right: 0; margin-bottom: 0; margin-left: 0.75rem;",
        }
    }

    fn arrow_style(self) -> &'static str {
        match self {
            Self::Top => "top: auto; right: auto; bottom: -0.375rem; left: 50%; transform: translateX(-50%) rotate(45deg); border-style: solid; border-width: 1px; border-top-width: 0; border-left-width: 0;",
            Self::Bottom => "top: -0.375rem; right: auto; bottom: auto; left: 50%; transform: translateX(-50%) rotate(45deg); border-style: solid; border-width: 1px; border-bottom-width: 0; border-right-width: 0;",
            Self::Left => "top: 50%; right: -0.375rem; bottom: auto; left: auto; transform: translateY(-50%) rotate(45deg); border-style: solid; border-width: 1px; border-left-width: 0; border-bottom-width: 0;",
            Self::Right => "top: 50%; right: auto; bottom: auto; left: -0.375rem; transform: translateY(-50%) rotate(45deg); border-style: solid; border-width: 1px; border-right-width: 0; border-top-width: 0;",
        }
    }
}

fn popover_content_style(variant: Option<&str>, placement: PopoverPlacement) -> AttrValue {
    let declaration = format!(
        "{} {}",
        placement.content_style(),
        "background-color: color-mix(in oklch, var(--component-solid, var(--component-color, var(--color-surface))) var(--popover-color-intensity, 30%), var(--color-surface)); border-color: var(--component-color, var(--color-outline-variant)); color: var(--component-on-container, var(--color-on-surface)); --color-on-surface: var(--component-on-container, var(--color-on-surface)); --color-on-surface-variant: var(--component-on-container, var(--color-on-surface-variant));"
    );

    variants::style(variant, &declaration)
}

fn popover_arrow_style(variant: Option<&str>, placement: PopoverPlacement) -> AttrValue {
    let declaration = format!(
        "{} {}",
        placement.arrow_style(),
        "background-color: color-mix(in oklch, var(--component-solid, var(--component-color, var(--color-surface))) var(--popover-color-intensity, 30%), var(--color-surface)); border-color: var(--component-color, var(--color-outline-variant));"
    );

    variants::style(variant, &declaration)
}
