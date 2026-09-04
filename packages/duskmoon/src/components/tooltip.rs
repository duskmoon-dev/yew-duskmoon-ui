use super::variants::Color;
use yew::prelude::*;

/// Preferred position of a tooltip relative to its anchor.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum TooltipPlacement {
    #[default]
    Top,
    Bottom,
    Left,
    Right,
}

impl TooltipPlacement {
    const fn class(self) -> &'static str {
        match self {
            Self::Top => "tooltip-top",
            Self::Bottom => "tooltip-bottom",
            Self::Left => "tooltip-left",
            Self::Right => "tooltip-right",
        }
    }
}

/// Layout treatment for the tooltip's content.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum TooltipContent {
    #[default]
    Plain,
    Multiline,
    Rich,
}

impl TooltipContent {
    const fn class(self) -> Option<&'static str> {
        match self {
            Self::Plain => None,
            Self::Multiline => Some("tooltip-multiline"),
            Self::Rich => Some("tooltip-rich"),
        }
    }
}

/// Base tooltip tone used when no explicit color is selected.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum TooltipTone {
    #[default]
    Dark,
    Light,
}

#[derive(Properties, Clone, PartialEq)]
pub struct TooltipProps {
    /// Stable DOM id shared with the trigger's `interestfor` attribute.
    pub id: AttrValue,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub children: Children,
    /// Legacy modifier escape hatch. When set, it takes precedence over
    /// `color` and `tone` and produces `tooltip-{variant}`.
    #[prop_or_default]
    pub variant: Option<String>,
    /// Core palette modifier. This takes precedence over `tone`.
    #[prop_or_default]
    pub color: Option<Color>,
    #[prop_or_default]
    pub tone: TooltipTone,
    #[prop_or_default]
    pub placement: TooltipPlacement,
    #[prop_or_default]
    pub content: TooltipContent,
    #[prop_or_default]
    pub interactive: bool,
}

/// A native hint popover positioned relative to a separately rendered trigger.
///
/// The trigger must use this tooltip's `id` as its `interestfor` value and a
/// matching CSS `anchor-name`; the button components' tooltip integration
/// supplies both. Display and positioning rely on browser support for the
/// Popover API, `interestfor`, and CSS Anchor Positioning. Keep an accessible
/// trigger name independent of this surface.
#[function_component(Tooltip)]
pub fn tooltip(props: &TooltipProps) -> Html {
    tooltip_view(props)
}

/// Build the anchor name shared by a tooltip surface and its trigger.
///
/// CSS Anchor Positioning requires a dashed identifier. Prefixing the
/// sanitized DOM id also keeps otherwise-valid ids that begin with a digit
/// usable as anchor names.
pub(crate) fn tooltip_anchor_name(id: &str) -> String {
    if id.is_empty() {
        "--tooltip-empty".to_owned()
    } else if id
        .chars()
        .all(|character| character.is_ascii_alphanumeric() || matches!(character, '_' | '-'))
    {
        format!("--tooltip-s-{id}")
    } else {
        const HEX: &[u8; 16] = b"0123456789abcdef";
        let mut encoded = String::with_capacity(id.len() * 2);
        for byte in id.bytes() {
            encoded.push(HEX[(byte >> 4) as usize] as char);
            encoded.push(HEX[(byte & 0x0f) as usize] as char);
        }

        format!("--tooltip-x-{encoded}")
    }
}

/// Inline style required on a native tooltip trigger.
pub(crate) fn tooltip_trigger_style(id: &str) -> AttrValue {
    format!("anchor-name: {};", tooltip_anchor_name(id)).into()
}

fn tooltip_surface_style(id: &str) -> AttrValue {
    format!("position-anchor: {};", tooltip_anchor_name(id)).into()
}

fn tooltip_classes(props: &TooltipProps) -> Classes {
    let mut classes = classes!("tooltip", props.placement.class());

    if let Some(variant) = props.variant.as_deref() {
        classes.push(format!("tooltip-{variant}"));
    } else if let Some(color) = props.color {
        classes.push(format!("tooltip-{}", color.as_str()));
    } else if props.tone == TooltipTone::Light {
        classes.push("tooltip-light");
    }

    if let Some(content_class) = props.content.class() {
        classes.push(content_class);
    }
    if props.interactive {
        classes.push("tooltip-interactive");
    }
    classes.push(props.class.clone());
    classes
}

fn tooltip_view(props: &TooltipProps) -> Html {
    // TODO(upstream): duskmoon-dev/duskmoonui#55 - Core must suppress the
    // native popover overflow scrollbar without adapter-local visual CSS.
    html! {
        <div
            id={props.id.clone()}
            popover="hint"
            role="tooltip"
            class={tooltip_classes(props)}
            style={tooltip_surface_style(&props.id)}
        >
            { for props.children.iter() }
        </div>
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;
    use yew::virtual_dom::VNode;

    fn props() -> TooltipProps {
        TooltipProps {
            id: "row-actions".into(),
            class: Classes::new(),
            children: Children::default(),
            variant: None,
            color: None,
            tone: TooltipTone::Dark,
            placement: TooltipPlacement::Top,
            content: TooltipContent::Plain,
            interactive: false,
        }
    }

    #[test]
    fn creates_css_safe_matching_anchor_styles() {
        assert_eq!(
            tooltip_anchor_name("row action:编辑"),
            "--tooltip-x-726f7720616374696f6e3ae7bc96e8be91"
        );
        assert_eq!(
            tooltip_trigger_style("row action:编辑"),
            "anchor-name: --tooltip-x-726f7720616374696f6e3ae7bc96e8be91;"
        );
        assert_eq!(
            tooltip_surface_style("row action:编辑"),
            "position-anchor: --tooltip-x-726f7720616374696f6e3ae7bc96e8be91;"
        );
        assert_eq!(tooltip_anchor_name(""), "--tooltip-empty");
        assert_eq!(
            tooltip_anchor_name("row-actions"),
            "--tooltip-s-row-actions"
        );
        assert_ne!(tooltip_anchor_name("row:1"), tooltip_anchor_name("row/1"));
    }

    #[test]
    fn maps_placement_content_and_interactivity() {
        let mut props = props();
        props.placement = TooltipPlacement::Right;
        props.content = TooltipContent::Rich;
        props.interactive = true;

        let classes = tooltip_classes(&props);

        assert!(classes.contains("tooltip"));
        assert!(classes.contains("tooltip-right"));
        assert!(classes.contains("tooltip-rich"));
        assert!(classes.contains("tooltip-interactive"));
        assert!(!classes.contains("tooltip-top"));
    }

    #[test]
    fn defaults_to_top_plain_dark_surface() {
        let classes = tooltip_classes(&props());

        assert_eq!(classes.to_string(), "tooltip tooltip-top");
    }

    #[test]
    fn legacy_variant_precedes_typed_color_and_tone() {
        let mut props = props();
        props.variant = Some("custom".into());
        props.color = Some(Color::Error);
        props.tone = TooltipTone::Light;

        let classes = tooltip_classes(&props);

        assert!(classes.contains("tooltip-custom"));
        assert!(!classes.contains("tooltip-error"));
        assert!(!classes.contains("tooltip-light"));
    }

    #[test]
    fn typed_color_precedes_light_tone() {
        let mut props = props();
        props.color = Some(Color::Warning);
        props.tone = TooltipTone::Light;

        let classes = tooltip_classes(&props);

        assert!(classes.contains("tooltip-warning"));
        assert!(!classes.contains("tooltip-light"));
    }

    #[test]
    fn surface_uses_native_hint_popover_contract() {
        let props = props();
        let VNode::VTag(tag) = tooltip_view(&props) else {
            panic!("tooltip surface should be an HTML element");
        };
        let attributes: HashMap<_, _> = tag.attributes.iter().collect();

        assert_eq!(tag.tag(), "div");
        assert_eq!(attributes.get("id"), Some(&"row-actions"));
        assert_eq!(attributes.get("popover"), Some(&"hint"));
        assert_eq!(attributes.get("role"), Some(&"tooltip"));
        assert_eq!(
            attributes.get("style"),
            Some(&"position-anchor: --tooltip-s-row-actions;")
        );
        assert_eq!(attributes.get("class"), Some(&"tooltip tooltip-top"));
    }
}
