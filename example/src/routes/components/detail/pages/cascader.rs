use crate::routes::components::catalog::ComponentSpec;
use crate::routes::components::detail::page::{ComponentPage, STANDARD_API};
use crate::routes::components::palette::{variant, PaletteColor};
use yew::prelude::*;
use yew_duskmoon::Cascader;

pub fn page(spec: &'static ComponentSpec) -> ComponentPage {
    ComponentPage::new(spec, usage, STANDARD_API, demo, color_variant)
}

fn usage(_: &ComponentSpec) -> String {
    r#"use yew_duskmoon::Cascader;

html! {
    <Cascader variant={Some("primary".to_owned())} class="cascader-open">
        <button class="cascader-trigger" type="button">
            <span class="cascader-placeholder">{ "Select location" }</span>
            <span class="cascader-arrow">{ ">" }</span>
        </button>
        <div class="cascader-dropdown">
            <div class="cascader-panels">
                <div class="cascader-panel">
                    <button class="cascader-option cascader-option-active" type="button">
                        <span class="cascader-option-label">{ "United States" }</span>
                    </button>
                </div>
            </div>
        </div>
    </Cascader>
}"#
    .to_owned()
}

fn demo(_: &ComponentSpec) -> Html {
    html! {
        <Cascader variant={Some("primary".to_owned())} class="component-detail-demo-control cascader-open">
            <button class="cascader-trigger" type="button">
                <span class="cascader-value">{ "United States / California / San Francisco" }</span>
                <span class="cascader-arrow">{ ">" }</span>
            </button>
            <div class="cascader-dropdown">
                <div class="cascader-panels">
                    <div class="cascader-panel">
                        <div class="cascader-panel-header">{ "Country" }</div>
                        <div class="cascader-options">
                            <button class="cascader-option cascader-option-active" type="button">
                                <span class="cascader-option-label">{ "United States" }</span>
                                <span class="cascader-option-arrow">{ ">" }</span>
                            </button>
                            <button class="cascader-option" type="button">
                                <span class="cascader-option-label">{ "Canada" }</span>
                                <span class="cascader-option-arrow">{ ">" }</span>
                            </button>
                        </div>
                    </div>
                    <div class="cascader-panel">
                        <div class="cascader-panel-header">{ "State" }</div>
                        <div class="cascader-options">
                            <button class="cascader-option cascader-option-active" type="button">
                                <span class="cascader-option-label">{ "California" }</span>
                                <span class="cascader-option-arrow">{ ">" }</span>
                            </button>
                            <button class="cascader-option" type="button">
                                <span class="cascader-option-label">{ "Texas" }</span>
                                <span class="cascader-option-arrow">{ ">" }</span>
                            </button>
                        </div>
                    </div>
                    <div class="cascader-panel">
                        <div class="cascader-panel-header">{ "City" }</div>
                        <div class="cascader-options">
                            <button class="cascader-option cascader-option-selected" type="button">
                                <span class="cascader-option-label">{ "San Francisco" }</span>
                            </button>
                            <button class="cascader-option" type="button">
                                <span class="cascader-option-label">{ "Los Angeles" }</span>
                            </button>
                        </div>
                    </div>
                </div>
            </div>
        </Cascader>
    }
}

fn color_variant(color: PaletteColor) -> Html {
    html! {
        <Cascader variant={variant(color)} class="component-detail-color-demo">
            <button class="cascader-trigger" type="button">
                <span class="cascader-value">{ color.label }</span>
                <span class="cascader-arrow">{ ">" }</span>
            </button>
        </Cascader>
    }
}
