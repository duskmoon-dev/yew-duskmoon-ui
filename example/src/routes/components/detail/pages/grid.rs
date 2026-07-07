use crate::routes::components::catalog::ComponentSpec;
use crate::routes::components::detail::page::{primary_variant, ComponentPage, GRID_API};
use crate::routes::components::palette::{variant, PaletteColor};
use yew::prelude::*;
use yew_duskmoon::{Grid, GridColumns, GridGap};

pub fn page(spec: &'static ComponentSpec) -> ComponentPage {
    ComponentPage::new(spec, usage, GRID_API, demo, color_variant)
}

fn usage(_: &ComponentSpec) -> String {
    "use yew_duskmoon::{Grid, GridColumns, GridGap};\n\nhtml! {\n    <Grid variant={Some(\"primary\".to_owned())} columns={Some(GridColumns::AutoFit48)} gap={Some(GridGap::Md)}>\n        <div>{ \"Grid item\" }</div>\n    </Grid>\n}".to_owned()
}

fn demo(_: &ComponentSpec) -> Html {
    html! {
        <Grid variant={primary_variant()} columns={Some(GridColumns::AutoFit48)} gap={Some(GridGap::Md)} class="component-detail-grid-demo">
            { for ["Auto", "Fit", "Grid"].into_iter().map(|label| html! {
                <div class="component-detail-grid-item">{ label }</div>
            }) }
        </Grid>
    }
}

fn color_variant(color: PaletteColor) -> Html {
    html! {
        <Grid variant={variant(color)} columns={Some(GridColumns::Two)} gap={Some(GridGap::Xs)} class="component-detail-color-grid-demo">
            <span>{ color.label }</span>
            <code>{ format!("grid-{}", color.key) }</code>
        </Grid>
    }
}
