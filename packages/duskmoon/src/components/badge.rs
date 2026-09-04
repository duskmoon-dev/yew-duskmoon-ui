use super::variants::Color;
use yew::prelude::*;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum BadgeAppearance {
    #[default]
    Filled,
    Tonal,
    Outlined,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum BadgeSize {
    Small,
    #[default]
    Medium,
    Large,
}

#[derive(Properties, Clone, PartialEq)]
pub struct BadgeProps {
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub children: Children,
    /// Legacy free-form modifier. When present, `badge-{variant}` replaces the
    /// typed color class; typed appearance and size modifiers still apply.
    #[prop_or_default]
    pub variant: Option<String>,
    #[prop_or_default]
    pub color: Option<Color>,
    #[prop_or_default]
    pub appearance: BadgeAppearance,
    #[prop_or_default]
    pub size: BadgeSize,
    #[prop_or_default]
    pub role: Option<AttrValue>,
    #[prop_or_default]
    pub aria_label: Option<AttrValue>,
    #[prop_or_default]
    pub title: Option<AttrValue>,
}

#[function_component(Badge)]
pub fn badge(props: &BadgeProps) -> Html {
    badge_view(props)
}

fn badge_view(props: &BadgeProps) -> Html {
    let classes = badge_classes(props);

    html! {
        <span
            class={classes}
            role={props.role.clone()}
            aria-label={props.aria_label.clone()}
            title={props.title.clone()}
        >
            { for props.children.iter() }
        </span>
    }
}

fn badge_classes(props: &BadgeProps) -> Classes {
    let mut classes = classes!("badge");
    let color = props
        .variant
        .as_deref()
        .or_else(|| props.color.map(Color::as_str));

    if let Some(color) = color {
        classes.push(format!("badge-{color}"));
    }

    match props.appearance {
        BadgeAppearance::Filled => {}
        BadgeAppearance::Tonal => classes.push(match color {
            Some("secondary") => "badge-tonal-secondary",
            Some("tertiary") => "badge-tonal-tertiary",
            _ => "badge-tonal",
        }),
        BadgeAppearance::Outlined => classes.push(match color {
            Some("secondary") => "badge-outlined-secondary",
            Some("tertiary") => "badge-outlined-tertiary",
            _ => "badge-outlined",
        }),
    }

    match props.size {
        BadgeSize::Small => classes.push("badge-sm"),
        BadgeSize::Medium => {}
        BadgeSize::Large => classes.push("badge-lg"),
    }

    classes.push(props.class.clone());
    classes
}

#[cfg(test)]
mod tests {
    use super::*;
    use yew::virtual_dom::VNode;

    fn props() -> BadgeProps {
        BadgeProps {
            class: Classes::new(),
            children: Children::default(),
            variant: None,
            color: None,
            appearance: BadgeAppearance::default(),
            size: BadgeSize::default(),
            role: None,
            aria_label: None,
            title: None,
        }
    }

    #[test]
    fn defaults_to_the_base_filled_medium_badge() {
        let props = props();

        assert_eq!(badge_classes(&props).to_string(), "badge");
        assert_eq!(props.appearance, BadgeAppearance::Filled);
        assert_eq!(props.size, BadgeSize::Medium);

        let VNode::VTag(tag) = badge_view(&props) else {
            panic!("badge view should render a native element");
        };
        let attributes: Vec<_> = tag.attributes.iter().collect();

        assert!(!attributes.iter().any(|(name, _)| *name == "role"));
        assert!(!attributes.iter().any(|(name, _)| *name == "aria-label"));
    }

    #[test]
    fn maps_typed_color_appearance_and_size_to_core_classes() {
        let mut props = props();
        props.color = Some(Color::Warning);
        props.appearance = BadgeAppearance::Tonal;
        props.size = BadgeSize::Small;

        assert_eq!(
            badge_classes(&props).to_string(),
            "badge badge-warning badge-tonal badge-sm"
        );

        props.color = Some(Color::Secondary);
        props.size = BadgeSize::Medium;

        assert_eq!(
            badge_classes(&props).to_string(),
            "badge badge-secondary badge-tonal-secondary"
        );

        props.color = Some(Color::Tertiary);
        props.appearance = BadgeAppearance::Outlined;
        props.size = BadgeSize::Large;

        assert_eq!(
            badge_classes(&props).to_string(),
            "badge badge-tertiary badge-outlined-tertiary badge-lg"
        );
    }

    #[test]
    fn legacy_variant_replaces_typed_color_and_preserves_other_modifiers() {
        let mut props = props();
        props.class = classes!("consumer-class");
        props.variant = Some("custom".to_owned());
        props.color = Some(Color::Error);
        props.appearance = BadgeAppearance::Outlined;

        let classes = badge_classes(&props);

        assert_eq!(
            classes.to_string(),
            "badge badge-custom badge-outlined consumer-class"
        );
        assert!(!classes.contains("badge-error"));
    }

    #[test]
    fn renders_an_inline_root_with_optional_accessibility_attributes() {
        let mut props = props();
        props.role = Some("status".into());
        props.aria_label = Some("Build status".into());
        props.title = Some("Ready".into());

        let VNode::VTag(tag) = badge_view(&props) else {
            panic!("badge view should render a native element");
        };
        let attributes: Vec<_> = tag.attributes.iter().collect();

        assert_eq!(tag.tag(), "span");
        assert!(attributes.contains(&("role", "status")));
        assert!(attributes.contains(&("aria-label", "Build status")));
        assert!(attributes.contains(&("title", "Ready")));
    }
}
