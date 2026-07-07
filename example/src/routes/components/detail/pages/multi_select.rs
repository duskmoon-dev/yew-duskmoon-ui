use crate::routes::components::catalog::ComponentSpec;
use crate::routes::components::detail::page::{ComponentPage, STANDARD_API};
use crate::routes::components::palette::{variant, PaletteColor};
use yew::prelude::*;
use yew_duskmoon::MultiSelect;

pub fn page(spec: &'static ComponentSpec) -> ComponentPage {
    ComponentPage::new(spec, usage, STANDARD_API, demo, color_variant)
}

fn usage(_: &ComponentSpec) -> String {
    r#"use yew_duskmoon::MultiSelect;

html! {
    <MultiSelect variant={Some("primary".to_owned())} class="multi-select-open">
        <button class="multi-select-trigger" type="button">
            <div class="multi-select-tags">
                <span class="multi-select-tag">{ "React" }</span>
                <span class="multi-select-tag">{ "Yew" }</span>
            </div>
            <span class="multi-select-arrow">{ "v" }</span>
        </button>
        <div class="multi-select-dropdown">
            <div class="multi-select-options">
                <button class="multi-select-option multi-select-option-selected" type="button">{ "React" }</button>
                <button class="multi-select-option" type="button">{ "Svelte" }</button>
            </div>
        </div>
    </MultiSelect>
}"#
    .to_owned()
}

fn demo(_: &ComponentSpec) -> Html {
    html! {
        <MultiSelect variant={Some("primary".to_owned())} class="component-detail-demo-control multi-select-open">
            <button class="multi-select-trigger" type="button">
                <div class="multi-select-tags">
                    <span class="multi-select-tag">
                        <span class="multi-select-tag-text">{ "Yew" }</span>
                        <span class="multi-select-tag-remove">{ "x" }</span>
                    </span>
                    <span class="multi-select-tag">
                        <span class="multi-select-tag-text">{ "Rust" }</span>
                        <span class="multi-select-tag-remove">{ "x" }</span>
                    </span>
                    <span class="multi-select-tag-overflow">{ "+2 more" }</span>
                </div>
                <span class="multi-select-arrow">{ "v" }</span>
            </button>
            <div class="multi-select-dropdown">
                <div class="multi-select-search">
                    <input class="multi-select-search-input" type="text" value="r" readonly={true} />
                </div>
                <div class="multi-select-options">
                    <button class="multi-select-option multi-select-option-selected" type="button">
                        <span class="multi-select-option-checkbox">{ "x" }</span>
                        <span class="multi-select-option-text">{ "Rust" }</span>
                    </button>
                    <button class="multi-select-option multi-select-option-selected" type="button">
                        <span class="multi-select-option-checkbox">{ "x" }</span>
                        <span class="multi-select-option-text">{ "Yew" }</span>
                    </button>
                    <button class="multi-select-option" type="button">
                        <span class="multi-select-option-checkbox"></span>
                        <span class="multi-select-option-text">{ "Leptos" }</span>
                    </button>
                </div>
            </div>
        </MultiSelect>
    }
}

fn color_variant(color: PaletteColor) -> Html {
    html! {
        <MultiSelect variant={variant(color)} class="component-detail-color-demo">
            <button class="multi-select-trigger" type="button">
                <span class="multi-select-placeholder">{ color.label }</span>
                <span class="multi-select-counter">{ "2" }</span>
            </button>
        </MultiSelect>
    }
}
