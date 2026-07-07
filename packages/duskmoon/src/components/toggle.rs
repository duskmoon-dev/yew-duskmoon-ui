use yew::prelude::*;
use yew::virtual_dom::AttrValue;

use super::variants;

#[derive(Properties, Clone, PartialEq)]
pub struct ToggleProps {
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub variant: Option<String>,
    #[prop_or_default]
    pub options: Vec<AttrValue>,
    #[prop_or_default]
    pub active: usize,
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or(AttrValue::from("Toggle options"))]
    pub aria_label: AttrValue,
    #[prop_or_default]
    pub onclick: Callback<usize>,
}

#[function_component(Toggle)]
pub fn toggle(props: &ToggleProps) -> Html {
    let active = use_state_eq(|| props.active);

    let mut classes = classes!("toggle-group");
    if let Some(variant) = &props.variant {
        classes.push(format!("toggle-group-{}", variant));
    }
    classes.push(props.class.clone());

    let group_style = variants::style(
        props.variant.as_deref(),
        "border-color: color-mix(in oklch, var(--component-color, var(--color-primary)) 42%, var(--dm-line)); background: color-mix(in oklch, var(--component-container, var(--color-surface-container)) 34%, var(--color-surface));",
    );

    let options = if props.options.is_empty() && props.children.is_empty() {
        vec![
            AttrValue::from("List"),
            AttrValue::from("Grid"),
            AttrValue::from("Details"),
        ]
    } else {
        props.options.clone()
    };

    html! {
        <div class={classes} role="group" aria-label={props.aria_label.clone()} style={group_style}>
            {
                if options.is_empty() {
                    let mut button_classes = toggle_button_classes(props.variant.as_deref(), true);
                    let button_style = toggle_button_style(props.variant.as_deref(), true);
                    if props.disabled {
                        button_classes.push("toggle-btn-disabled");
                    }

                    html! {
                        <button
                            type="button"
                            class={button_classes}
                            style={button_style}
                            aria-pressed="true"
                            disabled={props.disabled}
                        >
                            { for props.children.iter() }
                        </button>
                    }
                } else {
                    html! {
                        <>
                            {
                                for options.into_iter().enumerate().map(|(index, option)| {
                                    let is_active = index == *active;
                                    let mut button_classes = toggle_button_classes(props.variant.as_deref(), is_active);
                                    let button_style = toggle_button_style(props.variant.as_deref(), is_active);
                                    if props.disabled {
                                        button_classes.push("toggle-btn-disabled");
                                    }

                                    let onclick = {
                                        let active = active.clone();
                                        let callback = props.onclick.clone();
                                        Callback::from(move |_| {
                                            active.set(index);
                                            callback.emit(index);
                                        })
                                    };

                                    html! {
                                        <button
                                            type="button"
                                            class={button_classes}
                                            style={button_style}
                                            aria-pressed={if is_active { "true" } else { "false" }}
                                            disabled={props.disabled}
                                            onclick={onclick}
                                        >
                                            { option }
                                        </button>
                                    }
                                })
                            }
                        </>
                    }
                }
            }
        </div>
    }
}

fn toggle_button_classes(variant: Option<&str>, active: bool) -> Classes {
    let mut classes = classes!("toggle-btn");

    if let Some(variant) = variant {
        classes.push(format!("toggle-btn-{}", variant));
    }

    if active {
        classes.push("toggle-btn-active");
        classes.push("active");
    }

    classes
}

fn toggle_button_style(variant: Option<&str>, active: bool) -> AttrValue {
    if active {
        variants::style(
            variant,
            "color: var(--component-on-container, var(--dm-ink)); background: var(--component-container, var(--color-primary-container)); box-shadow: 0 1px 2px color-mix(in oklch, var(--component-color, var(--color-primary)) 28%, transparent);",
        )
    } else {
        variants::style(
            variant,
            "color: var(--component-color, var(--color-primary));",
        )
    }
}
