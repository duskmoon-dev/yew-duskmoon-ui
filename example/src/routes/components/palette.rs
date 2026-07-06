#[derive(Clone, Copy, PartialEq)]
pub struct PaletteColor {
    pub key: &'static str,
    pub label: &'static str,
}

pub const PALETTE: [PaletteColor; 10] = [
    PaletteColor {
        key: "primary",
        label: "Primary",
    },
    PaletteColor {
        key: "secondary",
        label: "Secondary",
    },
    PaletteColor {
        key: "tertiary",
        label: "Tertiary",
    },
    PaletteColor {
        key: "accent",
        label: "Accent",
    },
    PaletteColor {
        key: "neutral",
        label: "Neutral",
    },
    PaletteColor {
        key: "base",
        label: "Base",
    },
    PaletteColor {
        key: "info",
        label: "Info",
    },
    PaletteColor {
        key: "success",
        label: "Success",
    },
    PaletteColor {
        key: "warning",
        label: "Warning",
    },
    PaletteColor {
        key: "error",
        label: "Error",
    },
];

pub fn variant(color: PaletteColor) -> Option<String> {
    Some(color.key.to_owned())
}
