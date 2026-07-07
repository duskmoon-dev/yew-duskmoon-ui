use yew::prelude::*;

use super::super::catalog::ComponentSpec;
use super::super::palette::{PaletteColor, PALETTE};

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct ApiRow {
    pub prop: &'static str,
    pub ty: &'static str,
    pub default: &'static str,
    pub docs: &'static str,
}

pub type UsageRenderer = fn(&ComponentSpec) -> String;
pub type FeatureDemoRenderer = fn(&ComponentSpec) -> Html;
pub type ColorRenderer = fn(PaletteColor) -> Html;

#[derive(Clone, Copy)]
pub struct ComponentPage {
    pub spec: &'static ComponentSpec,
    pub usage: UsageRenderer,
    pub api_rows: &'static [ApiRow],
    pub feature_demo: FeatureDemoRenderer,
    pub color_variant: ColorRenderer,
}

impl ComponentPage {
    pub fn new(
        spec: &'static ComponentSpec,
        usage: UsageRenderer,
        api_rows: &'static [ApiRow],
        feature_demo: FeatureDemoRenderer,
        color_variant: ColorRenderer,
    ) -> Self {
        Self {
            spec,
            usage,
            api_rows,
            feature_demo,
            color_variant,
        }
    }

    pub fn render_color_matrix(self) -> Html {
        html! {
            <>
                { for PALETTE.into_iter().map(|color| (self.color_variant)(color)) }
            </>
        }
    }
}

pub const STANDARD_API: &[ApiRow] = &[
    ApiRow {
        prop: "class",
        ty: "Classes",
        default: "empty",
        docs: "Extra CSS classes appended to the component root.",
    },
    ApiRow {
        prop: "children",
        ty: "Children",
        default: "empty",
        docs: "Content rendered inside the component root.",
    },
    ApiRow {
        prop: "variant",
        ty: "Option<String>",
        default: "None",
        docs: "Appends a component-specific modifier class such as component-primary.",
    },
];

pub const BUTTON_API: &[ApiRow] = &[
    ApiRow {
        prop: "classes",
        ty: "Classes",
        default: "empty",
        docs: "Extra CSS classes appended to the button or anchor.",
    },
    ApiRow {
        prop: "type",
        ty: "ButtonType",
        default: "Default",
        docs: "Visual and semantic button mode.",
    },
    ApiRow {
        prop: "variant",
        ty: "Option<String>",
        default: "None",
        docs: "Appends a color modifier class such as btn-primary.",
    },
    ApiRow {
        prop: "href",
        ty: "AttrValue",
        default: "empty",
        docs: "Anchor href used when type is Link.",
    },
    ApiRow {
        prop: "target",
        ty: "AttrValue",
        default: "empty",
        docs: "Anchor target used when type is Link.",
    },
    ApiRow {
        prop: "rel",
        ty: "AttrValue",
        default: "empty",
        docs: "Anchor rel used when type is Link.",
    },
    ApiRow {
        prop: "disabled",
        ty: "bool",
        default: "false",
        docs: "Prevents interaction and marks the button disabled.",
    },
    ApiRow {
        prop: "loading",
        ty: "bool",
        default: "false",
        docs: "Applies the loading state and prevents interaction.",
    },
    ApiRow {
        prop: "children",
        ty: "Children",
        default: "empty",
        docs: "Button label or icon content.",
    },
    ApiRow {
        prop: "onclick",
        ty: "Callback<MouseEvent>",
        default: "noop",
        docs: "Click handler emitted when the button is interactive.",
    },
];

pub const CARD_API: &[ApiRow] = &[
    ApiRow {
        prop: "classes",
        ty: "Classes",
        default: "empty",
        docs: "Extra CSS classes appended to the card root.",
    },
    ApiRow {
        prop: "title",
        ty: "Option<Html>",
        default: "None",
        docs: "Optional title rendered in the card header.",
    },
    ApiRow {
        prop: "variant",
        ty: "Option<String>",
        default: "None",
        docs: "Appends a color modifier class such as card-primary.",
    },
    ApiRow {
        prop: "children",
        ty: "Children",
        default: "empty",
        docs: "Card body content.",
    },
];

pub const GRID_API: &[ApiRow] = &[
    ApiRow {
        prop: "class",
        ty: "Classes",
        default: "empty",
        docs: "Extra CSS classes appended to the grid root.",
    },
    ApiRow {
        prop: "children",
        ty: "Children",
        default: "empty",
        docs: "Grid item content.",
    },
    ApiRow {
        prop: "variant",
        ty: "Option<String>",
        default: "None",
        docs: "Appends a color modifier class such as grid-primary.",
    },
    ApiRow {
        prop: "columns",
        ty: "Option<GridColumns>",
        default: "None",
        docs: "Column preset for fixed, auto-fill, or auto-fit tracks.",
    },
    ApiRow {
        prop: "gap",
        ty: "Option<GridGap>",
        default: "None",
        docs: "Gap preset between grid items.",
    },
    ApiRow {
        prop: "inline",
        ty: "bool",
        default: "false",
        docs: "Renders inline-grid instead of grid.",
    },
    ApiRow {
        prop: "style",
        ty: "AttrValue",
        default: "empty",
        docs: "Additional inline style appended after preset grid styles.",
    },
];

pub fn primary_variant() -> Option<String> {
    Some("primary".to_owned())
}

pub fn secondary_variant() -> Option<String> {
    Some("secondary".to_owned())
}

pub fn tertiary_variant() -> Option<String> {
    Some("tertiary".to_owned())
}
