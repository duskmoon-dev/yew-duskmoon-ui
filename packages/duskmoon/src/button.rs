use strum_macros::Display;
use strum_macros::EnumIter;
use web_sys::MouseEvent as Event;
use yew::prelude::*;
use yew::virtual_dom::AttrValue;

use crate::components::tooltip::tooltip_trigger_style;
use crate::components::Color;

/// Legacy visual and semantic modes for [`Button`].
///
/// New code can combine `appearance`, `color`, and `size` while retaining this
/// prop for source compatibility. `Link`, `Circle`, and `Block` continue to
/// control their existing structural behavior.
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

/// Native HTML `type` used when [`Button`] renders a `<button>`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum NativeButtonType {
    #[default]
    Button,
    Submit,
    Reset,
}

impl NativeButtonType {
    const fn as_str(self) -> &'static str {
        match self {
            Self::Button => "button",
            Self::Submit => "submit",
            Self::Reset => "reset",
        }
    }
}

/// Core-supported button appearances.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ButtonAppearance {
    #[default]
    Filled,
    Outlined,
    Tonal,
    Text,
}

impl ButtonAppearance {
    const fn class(self) -> Option<&'static str> {
        match self {
            Self::Filled => None,
            Self::Outlined => Some("btn-outlined"),
            Self::Tonal => Some("btn-tonal"),
            Self::Text => Some("btn-text"),
        }
    }
}

/// Core-supported button sizes.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ButtonSize {
    Small,
    #[default]
    Medium,
    Large,
}

impl ButtonSize {
    const fn class(self) -> Option<&'static str> {
        match self {
            Self::Small => Some("btn-sm"),
            Self::Medium => None,
            Self::Large => Some("btn-lg"),
        }
    }

    const fn icon_class(self) -> Option<&'static str> {
        match self {
            Self::Small => Some("btn-icon-sm"),
            Self::Medium => None,
            Self::Large => Some("btn-icon-lg"),
        }
    }
}

/// Props for [`Button`].
///
/// `variant` is the legacy color escape hatch and takes precedence over
/// `color`. A typed `appearance` takes precedence over the legacy appearance
/// implied by `ButtonType::Dashed`, `ButtonType::Text`, or `ButtonType::Link`.
#[derive(Properties, Clone, PartialEq)]
pub struct ButtonProps {
    /// CSS classes to add to the element (optional).
    #[prop_or_default]
    pub classes: Classes,
    #[prop_or(ButtonType::Default)]
    pub r#type: ButtonType,
    #[prop_or_default]
    pub native_type: NativeButtonType,
    #[prop_or_default]
    pub appearance: Option<ButtonAppearance>,
    #[prop_or_default]
    pub color: Option<Color>,
    #[prop_or_default]
    pub size: ButtonSize,
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
    pub aria_label: Option<AttrValue>,
    #[prop_or_default]
    pub aria_describedby: Option<AttrValue>,
    #[prop_or_default]
    pub aria_pressed: Option<bool>,
    #[prop_or_default]
    pub aria_expanded: Option<bool>,
    #[prop_or_default]
    pub title: Option<AttrValue>,
    /// Associates the control with a sibling native [`Tooltip`](crate::Tooltip)
    /// surface and supplies the required CSS anchor on this trigger.
    #[prop_or_default]
    pub tooltip_id: Option<AttrValue>,
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub onclick: Callback<Event>,
}

#[function_component(Button)]
pub fn button(props: &ButtonProps) -> Html {
    button_view(props)
}

/// Props for an accessible icon-only [`IconButton`].
///
/// `label` is required and becomes the control's accessible name. `variant`
/// remains a free-form color escape hatch and takes precedence over `color`.
#[derive(Properties, Clone, PartialEq)]
pub struct IconButtonProps {
    /// Accessible action name, independent of the rendered icon or tooltip.
    pub label: AttrValue,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub native_type: NativeButtonType,
    #[prop_or(ButtonAppearance::Text)]
    pub appearance: ButtonAppearance,
    #[prop_or_default]
    pub color: Option<Color>,
    #[prop_or_default]
    pub size: ButtonSize,
    #[prop_or_default]
    pub variant: Option<String>,
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or_default]
    pub loading: bool,
    #[prop_or_default]
    pub aria_describedby: Option<AttrValue>,
    #[prop_or_default]
    pub aria_pressed: Option<bool>,
    #[prop_or_default]
    pub aria_expanded: Option<bool>,
    #[prop_or_default]
    pub title: Option<AttrValue>,
    /// Associates the control with a sibling native [`Tooltip`](crate::Tooltip)
    /// surface and supplies the required CSS anchor on this trigger.
    #[prop_or_default]
    pub tooltip_id: Option<AttrValue>,
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub onclick: Callback<Event>,
}

/// A native, accessibly named button for icon-only actions.
#[function_component(IconButton)]
pub fn icon_button(props: &IconButtonProps) -> Html {
    icon_button_view(props)
}

fn button_view(props: &ButtonProps) -> Html {
    let class_list = button_classes(props);
    let inactive = !is_interactive(props.disabled, props.loading);
    let aria_busy = props.loading.then_some(AttrValue::from("true"));
    let aria_pressed = aria_bool(props.aria_pressed);
    let aria_expanded = aria_bool(props.aria_expanded);
    let tooltip_id = props.tooltip_id.clone();
    let aria_describedby =
        merge_describedby(props.aria_describedby.as_deref(), tooltip_id.as_deref());
    let anchor_style = tooltip_id.as_deref().map(tooltip_trigger_style);
    let onclick = guarded_onclick(props.onclick.clone(), inactive);

    match props.r#type {
        ButtonType::Link => {
            let href = (!inactive).then_some(props.href.clone());
            let aria_disabled = inactive.then_some(AttrValue::from("true"));
            let tabindex = inactive.then_some(AttrValue::from("-1"));
            let role = inactive.then_some(AttrValue::from("link"));

            html! {
                <a
                    class={class_list}
                    onclick={onclick}
                    href={href}
                    target={props.target.clone()}
                    rel={props.rel.clone()}
                    role={role}
                    tabindex={tabindex}
                    aria-disabled={aria_disabled}
                    aria-busy={aria_busy}
                    aria-label={props.aria_label.clone()}
                    aria-describedby={aria_describedby}
                    aria-expanded={aria_expanded}
                    title={props.title.clone()}
                    interestfor={tooltip_id}
                    style={anchor_style}
                >
                    { for props.children.iter() }
                </a>
            }
        }
        _ => html! {
            <button
                type={props.native_type.as_str()}
                class={class_list}
                onclick={onclick}
                disabled={inactive}
                aria-busy={aria_busy}
                aria-label={props.aria_label.clone()}
                aria-describedby={aria_describedby}
                aria-pressed={aria_pressed}
                aria-expanded={aria_expanded}
                title={props.title.clone()}
                interestfor={tooltip_id}
                style={anchor_style}
            >
                { for props.children.iter() }
            </button>
        },
    }
}

fn icon_button_view(props: &IconButtonProps) -> Html {
    let inactive = !is_interactive(props.disabled, props.loading);
    let tooltip_id = props.tooltip_id.clone();

    html! {
        <button
            type={props.native_type.as_str()}
            class={icon_button_classes(props)}
            onclick={guarded_onclick(props.onclick.clone(), inactive)}
            disabled={inactive}
            aria-busy={props.loading.then_some(AttrValue::from("true"))}
            aria-label={props.label.clone()}
            aria-describedby={merge_describedby(
                props.aria_describedby.as_deref(),
                tooltip_id.as_deref(),
            )}
            aria-pressed={aria_bool(props.aria_pressed)}
            aria-expanded={aria_bool(props.aria_expanded)}
            title={props.title.clone()}
            interestfor={tooltip_id.clone()}
            style={tooltip_id.as_deref().map(tooltip_trigger_style)}
        >
            { for props.children.iter() }
        </button>
    }
}

fn button_classes(props: &ButtonProps) -> Classes {
    let mut class_list = classes!("btn");
    let has_explicit_color = props.variant.is_some() || props.color.is_some();

    match props.r#type {
        ButtonType::Primary if !has_explicit_color => class_list.push("btn-primary"),
        ButtonType::Danger if !has_explicit_color => class_list.push("btn-error"),
        ButtonType::Dashed if props.appearance.is_none() => class_list.push("btn-outlined"),
        ButtonType::Link | ButtonType::Text if props.appearance.is_none() => {
            class_list.push("btn-text")
        }
        ButtonType::Circle => class_list.push("btn-icon"),
        ButtonType::Block => class_list.push("btn-block"),
        ButtonType::Primary
        | ButtonType::Danger
        | ButtonType::Dashed
        | ButtonType::Link
        | ButtonType::Text
        | ButtonType::Default
        | ButtonType::Round => {}
    }

    if let Some(appearance) = props.appearance.and_then(ButtonAppearance::class) {
        class_list.push(appearance);
    }

    if let Some(variant) = &props.variant {
        class_list.push(format!("btn-{variant}"));
    } else if let Some(color) = props.color {
        class_list.push(format!("btn-{}", color.as_str()));
    }

    let size = if props.r#type == ButtonType::Circle {
        props.size.icon_class()
    } else {
        props.size.class()
    };
    if let Some(size) = size {
        class_list.push(size);
    }

    if props.disabled {
        class_list.push("disabled");
    }
    if props.loading {
        class_list.push("btn-loading");
    }
    if props.tooltip_id.is_some() {
        class_list.push("tooltip-delay");
    }

    class_list.push(props.classes.clone());
    class_list
}

fn icon_button_classes(props: &IconButtonProps) -> Classes {
    let mut class_list = classes!("btn", "btn-icon");

    if let Some(appearance) = props.appearance.class() {
        class_list.push(appearance);
    }

    if let Some(variant) = &props.variant {
        class_list.push(format!("btn-{variant}"));
    } else if let Some(color) = props.color {
        class_list.push(format!("btn-{}", color.as_str()));
    }

    if let Some(size) = props.size.icon_class() {
        class_list.push(size);
    }
    if props.disabled {
        class_list.push("disabled");
    }
    if props.loading {
        class_list.push("btn-loading");
    }
    if props.tooltip_id.is_some() {
        class_list.push("tooltip-delay");
    }

    class_list.push(props.class.clone());
    class_list
}

fn guarded_onclick(callback: Callback<Event>, inactive: bool) -> Callback<Event> {
    Callback::from(move |event: Event| {
        if inactive {
            event.prevent_default();
        } else {
            callback.emit(event);
        }
    })
}

fn is_interactive(disabled: bool, loading: bool) -> bool {
    !disabled && !loading
}

fn aria_bool(value: Option<bool>) -> Option<AttrValue> {
    value.map(|value| AttrValue::from(if value { "true" } else { "false" }))
}

fn merge_describedby(explicit: Option<&str>, tooltip_id: Option<&str>) -> Option<AttrValue> {
    let mut ids = Vec::new();
    for value in [explicit, tooltip_id].into_iter().flatten() {
        for id in value.split_whitespace() {
            if !ids.contains(&id) {
                ids.push(id);
            }
        }
    }

    (!ids.is_empty()).then(|| ids.join(" ").into())
}

#[cfg(test)]
mod tests {
    use std::rc::Rc;

    use super::*;
    use yew::virtual_dom::{VNode, VTag};

    fn props() -> ButtonProps {
        ButtonProps {
            classes: Classes::new(),
            r#type: ButtonType::Default,
            native_type: NativeButtonType::default(),
            appearance: None,
            color: None,
            size: ButtonSize::default(),
            variant: None,
            href: AttrValue::default(),
            target: AttrValue::default(),
            rel: AttrValue::default(),
            disabled: false,
            loading: false,
            aria_label: None,
            aria_describedby: None,
            aria_pressed: None,
            aria_expanded: None,
            title: None,
            tooltip_id: None,
            children: Children::default(),
            onclick: Callback::default(),
        }
    }

    fn icon_props() -> IconButtonProps {
        IconButtonProps {
            label: AttrValue::from("Edit item"),
            class: Classes::new(),
            native_type: NativeButtonType::default(),
            appearance: ButtonAppearance::Text,
            color: None,
            size: ButtonSize::default(),
            variant: None,
            disabled: false,
            loading: false,
            aria_describedby: None,
            aria_pressed: None,
            aria_expanded: None,
            title: None,
            tooltip_id: None,
            children: Children::default(),
            onclick: Callback::default(),
        }
    }

    fn vtag(node: Html) -> Rc<VTag> {
        match node {
            VNode::VTag(tag) => tag,
            other => panic!("expected VTag, got {other:?}"),
        }
    }

    fn attr(tag: &VTag, name: &str) -> Option<String> {
        tag.attributes
            .iter()
            .find_map(|(key, value)| (key == name).then(|| value.to_owned()))
    }

    #[test]
    fn maps_typed_appearance_color_and_size() {
        let mut props = props();
        props.appearance = Some(ButtonAppearance::Outlined);
        props.color = Some(Color::Error);
        props.size = ButtonSize::Small;

        let classes = button_classes(&props);

        assert!(classes.contains("btn"));
        assert!(classes.contains("btn-outlined"));
        assert!(classes.contains("btn-error"));
        assert!(classes.contains("btn-sm"));
    }

    #[test]
    fn legacy_variant_takes_precedence_over_typed_color() {
        let mut props = props();
        props.r#type = ButtonType::Danger;
        props.color = Some(Color::Success);
        props.variant = Some("warning".to_owned());

        let classes = button_classes(&props);

        assert!(classes.contains("btn-warning"));
        assert!(!classes.contains("btn-success"));
        assert!(!classes.contains("btn-error"));
    }

    #[test]
    fn typed_appearance_overrides_legacy_appearance() {
        let mut props = props();
        props.r#type = ButtonType::Text;
        props.appearance = Some(ButtonAppearance::Tonal);

        let classes = button_classes(&props);

        assert!(classes.contains("btn-tonal"));
        assert!(!classes.contains("btn-text"));
    }

    #[test]
    fn defaults_to_a_non_submitting_native_button() {
        let tag = vtag(button_view(&props()));

        assert_eq!(tag.tag(), "button");
        assert_eq!(attr(&tag, "type").as_deref(), Some("button"));
        assert_eq!(NativeButtonType::Submit.as_str(), "submit");
        assert_eq!(NativeButtonType::Reset.as_str(), "reset");
    }

    #[test]
    fn maps_each_typed_size() {
        let mut props = props();
        props.size = ButtonSize::Small;
        assert!(button_classes(&props).contains("btn-sm"));

        props.size = ButtonSize::Medium;
        assert!(!button_classes(&props).contains("btn-sm"));
        assert!(!button_classes(&props).contains("btn-lg"));

        props.size = ButtonSize::Large;
        assert!(button_classes(&props).contains("btn-lg"));
    }

    #[test]
    fn legacy_circle_uses_icon_specific_size_classes() {
        let mut props = props();
        props.r#type = ButtonType::Circle;
        props.size = ButtonSize::Small;

        let classes = button_classes(&props);

        assert!(classes.contains("btn-icon"));
        assert!(classes.contains("btn-icon-sm"));
        assert!(!classes.contains("btn-circle"));
        assert!(!classes.contains("btn-sm"));
    }

    #[test]
    fn icon_button_defaults_to_named_text_medium_native_button() {
        let props = icon_props();
        let classes = icon_button_classes(&props);
        let tag = vtag(icon_button_view(&props));

        assert_eq!(classes.to_string(), "btn btn-icon btn-text");
        assert_eq!(tag.tag(), "button");
        assert_eq!(attr(&tag, "type").as_deref(), Some("button"));
        assert_eq!(attr(&tag, "aria-label").as_deref(), Some("Edit item"));
    }

    #[test]
    fn icon_button_maps_each_size_without_generic_size_classes() {
        let mut props = icon_props();

        props.size = ButtonSize::Small;
        let classes = icon_button_classes(&props);
        assert!(classes.contains("btn-icon"));
        assert!(classes.contains("btn-icon-sm"));
        assert!(!classes.contains("btn-sm"));

        props.size = ButtonSize::Medium;
        let classes = icon_button_classes(&props);
        assert!(classes.contains("btn-icon"));
        assert!(!classes.contains("btn-icon-sm"));
        assert!(!classes.contains("btn-icon-lg"));

        props.size = ButtonSize::Large;
        let classes = icon_button_classes(&props);
        assert!(classes.contains("btn-icon"));
        assert!(classes.contains("btn-icon-lg"));
        assert!(!classes.contains("btn-lg"));
    }

    #[test]
    fn icon_button_supports_destructive_appearance_and_variant_precedence() {
        let mut props = icon_props();
        props.appearance = ButtonAppearance::Outlined;
        props.color = Some(Color::Error);

        let classes = icon_button_classes(&props);
        assert!(classes.contains("btn-outlined"));
        assert!(classes.contains("btn-error"));

        props.variant = Some("warning".to_owned());
        let classes = icon_button_classes(&props);
        assert!(classes.contains("btn-warning"));
        assert!(!classes.contains("btn-error"));
    }

    #[test]
    fn icon_button_disabled_and_loading_states_are_inactive() {
        let mut props = icon_props();
        props.disabled = true;
        let disabled = vtag(icon_button_view(&props));

        assert!(attr(&disabled, "disabled").is_some());
        assert!(!is_interactive(props.disabled, props.loading));

        props.disabled = false;
        props.loading = true;
        let loading = vtag(icon_button_view(&props));

        assert!(attr(&loading, "disabled").is_some());
        assert_eq!(attr(&loading, "aria-busy").as_deref(), Some("true"));
        assert!(icon_button_classes(&props).contains("btn-loading"));
        assert!(!is_interactive(props.disabled, props.loading));
    }

    #[test]
    fn icon_button_composes_tooltip_and_accessibility_attributes() {
        let mut props = icon_props();
        props.aria_describedby = Some(AttrValue::from("keyboard-help"));
        props.aria_pressed = Some(false);
        props.aria_expanded = Some(true);
        props.tooltip_id = Some(AttrValue::from("edit-tooltip"));
        props.title = Some(AttrValue::from("Edit item"));

        let tag = vtag(icon_button_view(&props));

        assert_eq!(
            attr(&tag, "aria-describedby").as_deref(),
            Some("keyboard-help edit-tooltip")
        );
        assert_eq!(attr(&tag, "aria-pressed").as_deref(), Some("false"));
        assert_eq!(attr(&tag, "aria-expanded").as_deref(), Some("true"));
        assert_eq!(attr(&tag, "interestfor").as_deref(), Some("edit-tooltip"));
        assert_eq!(attr(&tag, "title").as_deref(), Some("Edit item"));
        assert!(icon_button_classes(&props).contains("tooltip-delay"));
    }

    #[test]
    fn disabled_link_has_no_invalid_or_activating_attributes() {
        let mut props = props();
        props.r#type = ButtonType::Link;
        props.href = AttrValue::from("/settings");
        props.disabled = true;
        props.aria_pressed = Some(true);

        let tag = vtag(button_view(&props));

        assert_eq!(tag.tag(), "a");
        assert_eq!(attr(&tag, "aria-disabled").as_deref(), Some("true"));
        assert_eq!(attr(&tag, "tabindex").as_deref(), Some("-1"));
        assert_eq!(attr(&tag, "role").as_deref(), Some("link"));
        assert_eq!(attr(&tag, "href"), None);
        assert_eq!(attr(&tag, "disabled"), None);
        assert_eq!(attr(&tag, "aria-pressed"), None);
    }

    #[test]
    fn loading_is_busy_and_suppresses_callbacks() {
        let mut props = props();
        props.loading = true;

        let tag = vtag(button_view(&props));

        assert_eq!(attr(&tag, "aria-busy").as_deref(), Some("true"));
        assert!(!is_interactive(false, true));
        assert!(!is_interactive(true, false));
        assert!(is_interactive(false, false));
    }

    #[test]
    fn emits_optional_accessibility_states() {
        let mut props = props();
        props.aria_label = Some(AttrValue::from("Toggle preview"));
        props.aria_pressed = Some(false);
        props.aria_expanded = Some(true);

        let tag = vtag(button_view(&props));

        assert_eq!(attr(&tag, "aria-label").as_deref(), Some("Toggle preview"));
        assert_eq!(attr(&tag, "aria-pressed").as_deref(), Some("false"));
        assert_eq!(attr(&tag, "aria-expanded").as_deref(), Some("true"));
    }

    #[test]
    fn associates_a_native_tooltip_without_cloning_children() {
        let mut props = props();
        props.tooltip_id = Some(AttrValue::from("edit-release"));

        let tag = vtag(button_view(&props));

        assert_eq!(attr(&tag, "interestfor").as_deref(), Some("edit-release"));
        assert_eq!(
            attr(&tag, "aria-describedby").as_deref(),
            Some("edit-release")
        );
        assert_eq!(
            attr(&tag, "style").as_deref(),
            Some("anchor-name: --tooltip-s-edit-release;")
        );
        assert!(attr(&tag, "class").is_some_and(|classes| classes.contains("tooltip-delay")));
    }

    #[test]
    fn preserves_existing_descriptions_when_associating_a_tooltip() {
        let mut props = props();
        props.aria_describedby = Some(AttrValue::from("field-help"));
        props.tooltip_id = Some(AttrValue::from("edit-release"));

        let tag = vtag(button_view(&props));

        assert_eq!(
            attr(&tag, "aria-describedby").as_deref(),
            Some("field-help edit-release")
        );
        assert_eq!(
            merge_describedby(Some("field-help edit-release"), Some("edit-release")),
            Some(AttrValue::from("field-help edit-release"))
        );
    }
}
