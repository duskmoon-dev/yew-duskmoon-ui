use crate::routes::components::catalog::ComponentSpec;
use crate::routes::components::detail::page::{ComponentPage, STANDARD_API};
use crate::routes::components::palette::{variant, PaletteColor};
use yew::prelude::*;
use yew_duskmoon::Datepicker;

pub fn page(spec: &'static ComponentSpec) -> ComponentPage {
    ComponentPage::new(spec, usage, STANDARD_API, demo, color_variant)
}

fn usage(_: &ComponentSpec) -> String {
    r#"use yew_duskmoon::DatePicker;

html! {
    <DatePicker variant={Some("primary".to_owned())}>
        <div class="datepicker-input-container">
            <input class="datepicker-input" type="date" />
            <button class="datepicker-trigger" type="button"><span class="datepicker-icon"></span></button>
        </div>
        <div class="datepicker-dropdown datepicker-inline">
            <div class="datepicker-header">
                <button class="datepicker-nav-btn" type="button">{ "<" }</button>
                <div class="datepicker-title">{ "January 2026" }</div>
                <button class="datepicker-nav-btn" type="button">{ ">" }</button>
            </div>
        </div>
    </DatePicker>
}"#
    .to_owned()
}

fn demo(_: &ComponentSpec) -> Html {
    html! {
        <Datepicker variant={Some("primary".to_owned())} class="component-detail-demo-control datepicker-inline">
            <div class="datepicker-input-container">
                <input class="datepicker-input" type="date" value="2026-07-07" readonly={true} />
                <button class="datepicker-trigger" type="button" aria-label="Open calendar">
                    <span class="datepicker-icon"></span>
                </button>
            </div>
            <div class="datepicker-dropdown datepicker-inline">
                <div class="datepicker-header">
                    <button class="datepicker-nav-btn" type="button">{ "<" }</button>
                    <div class="datepicker-title">{ "July 2026" }</div>
                    <button class="datepicker-nav-btn" type="button">{ ">" }</button>
                </div>
                <div class="datepicker-calendar">
                    { for ["Su", "Mo", "Tu", "We", "Th", "Fr", "Sa"].into_iter().map(|day| html! {
                        <div class="datepicker-weekday">{ day }</div>
                    }) }
                    { for (1..=14).map(|day| {
                        let class = if day == 7 {
                            "datepicker-day datepicker-day-selected datepicker-day-today"
                        } else if (8..=10).contains(&day) {
                            "datepicker-day datepicker-day-in-range"
                        } else {
                            "datepicker-day"
                        };

                        html! {
                            <button class={class} type="button">{ day }</button>
                        }
                    }) }
                </div>
            </div>
        </Datepicker>
    }
}

fn color_variant(color: PaletteColor) -> Html {
    html! {
        <Datepicker variant={variant(color)} class="component-detail-color-demo">
            <label class="form-label">{ color.label }</label>
            <div class="datepicker-input-container">
                <input class="datepicker-input" type="text" placeholder={format!("{} date", color.label)} />
                <button class="datepicker-trigger" type="button" aria-label="Open calendar">
                    <span class="datepicker-icon"></span>
                </button>
            </div>
            <code>{ format!("datepicker-{}", color.key) }</code>
        </Datepicker>
    }
}
