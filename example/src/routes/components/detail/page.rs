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
    pub additional_api: Option<(&'static str, &'static [ApiRow])>,
    pub feature_demo: FeatureDemoRenderer,
    pub color_variant: Option<ColorRenderer>,
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
            additional_api: None,
            feature_demo,
            color_variant: Some(color_variant),
        }
    }

    pub fn without_color_variants(
        spec: &'static ComponentSpec,
        usage: UsageRenderer,
        api_rows: &'static [ApiRow],
        feature_demo: FeatureDemoRenderer,
    ) -> Self {
        Self {
            spec,
            usage,
            api_rows,
            additional_api: None,
            feature_demo,
            color_variant: None,
        }
    }

    pub fn with_additional_api(mut self, title: &'static str, rows: &'static [ApiRow]) -> Self {
        self.additional_api = Some((title, rows));
        self
    }

    pub fn render_color_matrix(self) -> Html {
        self.color_variant.map_or_else(Html::default, |render| {
            html! {
                <>
                    { for PALETTE.into_iter().map(render) }
                </>
            }
        })
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
        docs: "Legacy visual or structural mode. Link, Circle, and Block still select their existing root behavior.",
    },
    ApiRow {
        prop: "native_type",
        ty: "NativeButtonType",
        default: "Button",
        docs: "Native button type: Button, Submit, or Reset. Ignored when type is Link.",
    },
    ApiRow {
        prop: "appearance",
        ty: "Option<ButtonAppearance>",
        default: "None",
        docs: "Filled, Outlined, Tonal, or Text treatment. When set, it overrides the appearance implied by legacy ButtonType values.",
    },
    ApiRow {
        prop: "color",
        ty: "Option<Color>",
        default: "None",
        docs: "Typed DuskMoon palette modifier.",
    },
    ApiRow {
        prop: "size",
        ty: "ButtonSize",
        default: "Medium",
        docs: "Core-supported Small, Medium, or Large size.",
    },
    ApiRow {
        prop: "variant",
        ty: "Option<String>",
        default: "None",
        docs: "Legacy escape hatch that appends btn-{variant}. It takes precedence over color; typed appearance and size still apply.",
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
        prop: "aria_label",
        ty: "Option<AttrValue>",
        default: "None",
        docs: "Optional accessible name. Prefer IconButton for icon-only controls.",
    },
    ApiRow {
        prop: "aria_describedby",
        ty: "Option<AttrValue>",
        default: "None",
        docs: "Id of descriptive content. tooltip_id supplies this automatically when present.",
    },
    ApiRow {
        prop: "aria_pressed",
        ty: "Option<bool>",
        default: "None",
        docs: "Optional pressed state for toggle buttons.",
    },
    ApiRow {
        prop: "aria_expanded",
        ty: "Option<bool>",
        default: "None",
        docs: "Optional expanded state for disclosure controls.",
    },
    ApiRow {
        prop: "title",
        ty: "Option<AttrValue>",
        default: "None",
        docs: "Native title fallback and supplemental pointer hint.",
    },
    ApiRow {
        prop: "tooltip_id",
        ty: "Option<AttrValue>",
        default: "None",
        docs: "Associates a sibling Tooltip and supplies interestfor, aria-describedby, and a matching CSS anchor.",
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

pub const ICON_BUTTON_API: &[ApiRow] = &[
    ApiRow {
        prop: "label",
        ty: "AttrValue",
        default: "required",
        docs: "Required accessible action name, emitted as aria-label independently of the icon or tooltip.",
    },
    ApiRow {
        prop: "class",
        ty: "Classes",
        default: "empty",
        docs: "Extra CSS classes appended to the native button.",
    },
    ApiRow {
        prop: "native_type",
        ty: "NativeButtonType",
        default: "Button",
        docs: "Native button type: Button, Submit, or Reset.",
    },
    ApiRow {
        prop: "appearance",
        ty: "ButtonAppearance",
        default: "Text",
        docs: "Filled, Outlined, Tonal, or Text core treatment.",
    },
    ApiRow {
        prop: "color",
        ty: "Option<Color>",
        default: "None",
        docs: "Typed DuskMoon palette modifier.",
    },
    ApiRow {
        prop: "size",
        ty: "ButtonSize",
        default: "Medium",
        docs: "Core-supported Small, Medium, or Large icon-button size.",
    },
    ApiRow {
        prop: "variant",
        ty: "Option<String>",
        default: "None",
        docs: "Free-form color escape hatch that appends btn-{variant} and takes precedence over color.",
    },
    ApiRow {
        prop: "disabled",
        ty: "bool",
        default: "false",
        docs: "Disables the native button and suppresses its click callback.",
    },
    ApiRow {
        prop: "loading",
        ty: "bool",
        default: "false",
        docs: "Applies the loading state, emits aria-busy, and suppresses its click callback.",
    },
    ApiRow {
        prop: "aria_describedby",
        ty: "Option<AttrValue>",
        default: "None",
        docs: "Ids of descriptive content. tooltip_id is merged into this relationship.",
    },
    ApiRow {
        prop: "aria_pressed",
        ty: "Option<bool>",
        default: "None",
        docs: "Optional pressed state for icon-only toggle actions.",
    },
    ApiRow {
        prop: "aria_expanded",
        ty: "Option<bool>",
        default: "None",
        docs: "Optional expanded state for icon-only disclosure actions.",
    },
    ApiRow {
        prop: "title",
        ty: "Option<AttrValue>",
        default: "None",
        docs: "Optional native title fallback; it does not replace the required label.",
    },
    ApiRow {
        prop: "tooltip_id",
        ty: "Option<AttrValue>",
        default: "None",
        docs: "Associates a sibling Tooltip and supplies interestfor, aria-describedby, and a matching CSS anchor.",
    },
    ApiRow {
        prop: "children",
        ty: "Children",
        default: "empty",
        docs: "Icon content; decorative SVGs should use aria-hidden=true.",
    },
    ApiRow {
        prop: "onclick",
        ty: "Callback<MouseEvent>",
        default: "noop",
        docs: "Click handler emitted only while the icon button is interactive.",
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
