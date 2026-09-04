use yew_duskmoon::Color;

#[derive(Clone, Copy, PartialEq)]
pub struct PaletteColor {
    pub key: &'static str,
    pub label: &'static str,
    pub color: Color,
}

pub const PALETTE: [PaletteColor; 10] = [
    PaletteColor {
        key: "primary",
        label: "Primary",
        color: Color::Primary,
    },
    PaletteColor {
        key: "secondary",
        label: "Secondary",
        color: Color::Secondary,
    },
    PaletteColor {
        key: "tertiary",
        label: "Tertiary",
        color: Color::Tertiary,
    },
    PaletteColor {
        key: "accent",
        label: "Accent",
        color: Color::Accent,
    },
    PaletteColor {
        key: "neutral",
        label: "Neutral",
        color: Color::Neutral,
    },
    PaletteColor {
        key: "base",
        label: "Base",
        color: Color::Base,
    },
    PaletteColor {
        key: "info",
        label: "Info",
        color: Color::Info,
    },
    PaletteColor {
        key: "success",
        label: "Success",
        color: Color::Success,
    },
    PaletteColor {
        key: "warning",
        label: "Warning",
        color: Color::Warning,
    },
    PaletteColor {
        key: "error",
        label: "Error",
        color: Color::Error,
    },
];

pub fn variant(color: PaletteColor) -> Option<String> {
    Some(color.key.to_owned())
}
