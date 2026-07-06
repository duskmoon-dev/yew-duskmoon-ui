use yew::prelude::*;
use yew::virtual_dom::AttrValue;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum GridColumns {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Twelve,
    AutoFill32,
    AutoFill48,
    AutoFill56,
    AutoFill64,
    AutoFill80,
    AutoFit32,
    AutoFit48,
    AutoFit56,
    AutoFit64,
    AutoFit80,
}

impl GridColumns {
    fn class(self) -> &'static str {
        match self {
            Self::One => "grid-cols-1",
            Self::Two => "grid-cols-2",
            Self::Three => "grid-cols-3",
            Self::Four => "grid-cols-4",
            Self::Five => "grid-cols-5",
            Self::Six => "grid-cols-6",
            Self::Twelve => "grid-cols-12",
            Self::AutoFill32 => "grid-cols-auto-fill-32",
            Self::AutoFill48 => "grid-cols-auto-fill-48",
            Self::AutoFill56 => "grid-cols-auto-fill-56",
            Self::AutoFill64 => "grid-cols-auto-fill-64",
            Self::AutoFill80 => "grid-cols-auto-fill-80",
            Self::AutoFit32 => "grid-cols-auto-fit-32",
            Self::AutoFit48 => "grid-cols-auto-fit-48",
            Self::AutoFit56 => "grid-cols-auto-fit-56",
            Self::AutoFit64 => "grid-cols-auto-fit-64",
            Self::AutoFit80 => "grid-cols-auto-fit-80",
        }
    }

    fn style(self) -> &'static str {
        match self {
            Self::One => "grid-template-columns: repeat(1, minmax(0, 1fr));",
            Self::Two => "grid-template-columns: repeat(2, minmax(0, 1fr));",
            Self::Three => "grid-template-columns: repeat(3, minmax(0, 1fr));",
            Self::Four => "grid-template-columns: repeat(4, minmax(0, 1fr));",
            Self::Five => "grid-template-columns: repeat(5, minmax(0, 1fr));",
            Self::Six => "grid-template-columns: repeat(6, minmax(0, 1fr));",
            Self::Twelve => "grid-template-columns: repeat(12, minmax(0, 1fr));",
            Self::AutoFill32 => "grid-template-columns: repeat(auto-fill, minmax(8rem, 1fr));",
            Self::AutoFill48 => "grid-template-columns: repeat(auto-fill, minmax(12rem, 1fr));",
            Self::AutoFill56 => "grid-template-columns: repeat(auto-fill, minmax(14rem, 1fr));",
            Self::AutoFill64 => "grid-template-columns: repeat(auto-fill, minmax(16rem, 1fr));",
            Self::AutoFill80 => "grid-template-columns: repeat(auto-fill, minmax(20rem, 1fr));",
            Self::AutoFit32 => "grid-template-columns: repeat(auto-fit, minmax(8rem, 1fr));",
            Self::AutoFit48 => "grid-template-columns: repeat(auto-fit, minmax(12rem, 1fr));",
            Self::AutoFit56 => "grid-template-columns: repeat(auto-fit, minmax(14rem, 1fr));",
            Self::AutoFit64 => "grid-template-columns: repeat(auto-fit, minmax(16rem, 1fr));",
            Self::AutoFit80 => "grid-template-columns: repeat(auto-fit, minmax(20rem, 1fr));",
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum GridGap {
    None,
    Xs,
    Sm,
    Md,
    Lg,
    Xl,
}

impl GridGap {
    fn class(self) -> &'static str {
        match self {
            Self::None => "gap-0",
            Self::Xs => "gap-1",
            Self::Sm => "gap-2",
            Self::Md => "gap-4",
            Self::Lg => "gap-6",
            Self::Xl => "gap-8",
        }
    }

    fn style(self) -> &'static str {
        match self {
            Self::None => "gap: 0;",
            Self::Xs => "gap: 0.25rem;",
            Self::Sm => "gap: 0.5rem;",
            Self::Md => "gap: 1rem;",
            Self::Lg => "gap: 1.5rem;",
            Self::Xl => "gap: 2rem;",
        }
    }
}

#[derive(Properties, Clone, PartialEq)]
pub struct GridProps {
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub columns: Option<GridColumns>,
    #[prop_or_default]
    pub gap: Option<GridGap>,
    #[prop_or_default]
    pub inline: bool,
    #[prop_or_default]
    pub style: AttrValue,
}

#[function_component(Grid)]
pub fn grid(props: &GridProps) -> Html {
    let mut classes = if props.inline {
        classes!("inline-grid")
    } else {
        classes!("grid")
    };

    if let Some(columns) = props.columns {
        classes.push(columns.class());
    }

    if let Some(gap) = props.gap {
        classes.push(gap.class());
    }

    classes.push(props.class.clone());

    let mut styles = vec![if props.inline {
        "display: inline-grid;"
    } else {
        "display: grid;"
    }];

    if let Some(columns) = props.columns {
        styles.push(columns.style());
    }

    if let Some(gap) = props.gap {
        styles.push(gap.style());
    }

    let custom_style = props.style.to_string();
    let style = if custom_style.is_empty() {
        styles.join(" ")
    } else {
        format!("{} {}", styles.join(" "), custom_style)
    };

    html! {
        <div class={classes} style={style}>
            { for props.children.iter() }
        </div>
    }
}
