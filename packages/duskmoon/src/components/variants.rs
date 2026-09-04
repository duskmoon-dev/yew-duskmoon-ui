use yew::virtual_dom::AttrValue;

/// Color tokens shared by components whose core CSS exposes the complete
/// DuskMoon palette.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Color {
    Primary,
    Secondary,
    Tertiary,
    Accent,
    Neutral,
    Base,
    Info,
    Success,
    Warning,
    Error,
}

impl Color {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Primary => "primary",
            Self::Secondary => "secondary",
            Self::Tertiary => "tertiary",
            Self::Accent => "accent",
            Self::Neutral => "neutral",
            Self::Base => "base",
            Self::Info => "info",
            Self::Success => "success",
            Self::Warning => "warning",
            Self::Error => "error",
        }
    }
}

pub fn vars(variant: Option<&str>) -> &'static str {
    match variant {
        Some("primary") => "--component-color: var(--color-primary); --component-content: var(--color-primary-content); --component-solid: var(--color-primary); --component-container: var(--color-primary-container); --component-on-container: var(--color-on-primary-container);",
        Some("secondary") => "--component-color: var(--color-secondary); --component-content: var(--color-secondary-content); --component-solid: var(--color-secondary); --component-container: var(--color-secondary-container); --component-on-container: var(--color-on-secondary-container);",
        Some("tertiary") => "--component-color: var(--color-tertiary); --component-content: var(--color-tertiary-content); --component-solid: var(--color-tertiary); --component-container: var(--color-tertiary-container); --component-on-container: var(--color-on-tertiary-container);",
        Some("accent") => "--component-color: var(--color-accent); --component-content: var(--color-accent-content); --component-solid: var(--color-accent); --component-container: color-mix(in oklch, var(--color-accent) 22%, var(--color-surface)); --component-on-container: var(--color-accent-content);",
        Some("neutral") => "--component-color: var(--color-neutral); --component-content: var(--color-neutral-content); --component-solid: var(--color-neutral); --component-container: color-mix(in oklch, var(--color-neutral) 12%, var(--color-surface)); --component-on-container: var(--color-neutral-content);",
        Some("base") => "--component-color: var(--color-base-content); --component-content: var(--color-base-content); --component-solid: var(--color-base-300); --component-container: var(--color-base-200); --component-on-container: var(--color-base-content);",
        Some("info") => "--component-color: var(--color-info); --component-content: var(--color-info-content); --component-solid: var(--color-info); --component-container: var(--color-info-container); --component-on-container: var(--color-on-info-container);",
        Some("success") => "--component-color: var(--color-success); --component-content: var(--color-success-content); --component-solid: var(--color-success); --component-container: var(--color-success-container); --component-on-container: var(--color-on-success-container);",
        Some("warning") => "--component-color: var(--color-warning); --component-content: var(--color-warning-content); --component-solid: var(--color-warning); --component-container: var(--color-warning-container); --component-on-container: var(--color-on-warning-container);",
        Some("error") => "--component-color: var(--color-error); --component-content: var(--color-error-content); --component-solid: var(--color-error); --component-container: var(--color-error-container); --component-on-container: var(--color-on-error-container);",
        _ => "",
    }
}

pub fn style(variant: Option<&str>, declaration: &str) -> AttrValue {
    let vars = vars(variant);

    match (vars.is_empty(), declaration.is_empty()) {
        (true, true) => AttrValue::default(),
        (true, false) => AttrValue::from(declaration.to_owned()),
        (false, true) => AttrValue::from(vars),
        (false, false) => AttrValue::from(format!("{vars} {declaration}")),
    }
}

#[cfg(test)]
mod tests {
    use super::Color;

    #[test]
    fn maps_every_shared_color_to_its_core_suffix() {
        let mappings = [
            (Color::Primary, "primary"),
            (Color::Secondary, "secondary"),
            (Color::Tertiary, "tertiary"),
            (Color::Accent, "accent"),
            (Color::Neutral, "neutral"),
            (Color::Base, "base"),
            (Color::Info, "info"),
            (Color::Success, "success"),
            (Color::Warning, "warning"),
            (Color::Error, "error"),
        ];

        for (color, suffix) in mappings {
            assert_eq!(color.as_str(), suffix);
        }
    }
}
