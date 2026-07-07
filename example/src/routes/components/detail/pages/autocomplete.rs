use crate::routes::components::catalog::ComponentSpec;
use crate::routes::components::detail::page::{ComponentPage, STANDARD_API};
use crate::routes::components::palette::{variant, PaletteColor};
use yew::prelude::*;
use yew_duskmoon::Autocomplete;

pub fn page(spec: &'static ComponentSpec) -> ComponentPage {
    ComponentPage::new(spec, usage, STANDARD_API, demo, color_variant)
}

fn usage(_: &ComponentSpec) -> String {
    r#"use yew_duskmoon::Autocomplete;

html! {
    <Autocomplete variant={Some("primary".to_owned())} class="autocomplete-open">
        <div class="autocomplete-input-wrapper">
            <input class="autocomplete-input" type="text" placeholder="Search..." aria-autocomplete="list" />
            <button class="autocomplete-toggle" type="button" aria-label="Toggle dropdown">{ "v" }</button>
        </div>
        <div class="autocomplete-dropdown">
            <ul class="autocomplete-options" role="listbox">
                <li class="autocomplete-option selected" role="option">{ "Option 1" }</li>
                <li class="autocomplete-option" role="option">{ "Option 2" }</li>
            </ul>
        </div>
    </Autocomplete>
}"#
    .to_owned()
}

fn demo(_: &ComponentSpec) -> Html {
    html! {
        <Autocomplete variant={Some("primary".to_owned())} class="component-detail-demo-control autocomplete-open">
            <div class="autocomplete-input-wrapper">
                <input class="autocomplete-input" type="text" value="Can" readonly={true} aria-autocomplete="list" />
                <button class="autocomplete-toggle" type="button" aria-label="Toggle dropdown">{ "v" }</button>
            </div>
            <div class="autocomplete-dropdown">
                <ul class="autocomplete-options" role="listbox">
                    <li class="autocomplete-option highlighted" role="option">
                        <span class="autocomplete-option-content">
                            <span class="autocomplete-option-label">{ "Canada" }</span>
                            <span class="autocomplete-option-description">{ "North America" }</span>
                        </span>
                    </li>
                    <li class="autocomplete-option" role="option">{ "Canary Islands" }</li>
                    <li class="autocomplete-option" role="option">{ "Cancun" }</li>
                </ul>
            </div>
        </Autocomplete>
    }
}

fn color_variant(color: PaletteColor) -> Html {
    html! {
        <Autocomplete variant={variant(color)} class="component-detail-color-demo">
            <label class="form-label">{ color.label }</label>
            <input class={classes!("autocomplete-input", format!("autocomplete-{}", color.key))} type="text" placeholder={format!("{} autocomplete", color.label)} />
            <code>{ format!("autocomplete-{}", color.key) }</code>
        </Autocomplete>
    }
}
