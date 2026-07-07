use crate::routes::components::catalog::ComponentSpec;
use crate::routes::components::detail::page::{ComponentPage, STANDARD_API};
use crate::routes::components::palette::{variant, PaletteColor};
use yew::prelude::*;
use yew_duskmoon::Timeline;

pub fn page(spec: &'static ComponentSpec) -> ComponentPage {
    ComponentPage::new(spec, usage, STANDARD_API, demo, color_variant)
}

fn usage(_: &ComponentSpec) -> String {
    r#"use yew_duskmoon::Timeline;

html! {
    <Timeline variant={Some("primary".to_owned())}>
        <div class="timeline-item completed">
            <div class="timeline-marker"><span class="timeline-marker-icon">{ "OK" }</span></div>
            <div class="timeline-content">
                <time class="timeline-time" datetime="2026-07-07">{ "09:00" }</time>
                <h3 class="timeline-title">{ "Release cut" }</h3>
                <p class="timeline-description">{ "Artifacts were built and signed." }</p>
            </div>
        </div>
    </Timeline>
}"#
    .to_owned()
}

fn demo(_: &ComponentSpec) -> Html {
    html! {
        <Timeline variant={Some("primary".to_owned())} class="component-detail-demo-control timeline-progress">
            <div class="timeline-item completed">
                <div class="timeline-marker"><span class="timeline-marker-icon">{ "OK" }</span></div>
                <div class="timeline-content">
                    <time class="timeline-time" datetime="2026-07-07T09:00:00">{ "09:00" }</time>
                    <h3 class="timeline-title">{ "Build completed" }</h3>
                    <p class="timeline-description">{ "Package artifacts passed verification." }</p>
                </div>
            </div>
            <div class="timeline-item active">
                <div class="timeline-marker"><span class="timeline-marker-dot"></span></div>
                <div class="timeline-content">
                    <time class="timeline-time" datetime="2026-07-07T10:15:00">{ "10:15" }</time>
                    <h3 class="timeline-title">{ "Deploying" }</h3>
                    <p class="timeline-description">{ "The release is rolling through production." }</p>
                </div>
            </div>
            <div class="timeline-item timeline-item-warning">
                <div class="timeline-marker"><span class="timeline-marker-dot"></span></div>
                <div class="timeline-content">
                    <time class="timeline-time" datetime="2026-07-07T11:00:00">{ "11:00" }</time>
                    <h3 class="timeline-title">{ "Observe metrics" }</h3>
                    <p class="timeline-description">{ "Watch latency and error budgets after rollout." }</p>
                </div>
            </div>
        </Timeline>
    }
}

fn color_variant(color: PaletteColor) -> Html {
    html! {
        <Timeline variant={variant(color)} class="component-detail-color-demo">
            <div class={classes!("timeline-item", format!("timeline-item-{}", color.key))}>
                <div class="timeline-marker"><span class="timeline-marker-dot"></span></div>
                <div class="timeline-content">
                    <strong class="timeline-title">{ color.label }</strong>
                    <span class="timeline-time">{ format!("timeline-{}", color.key) }</span>
                </div>
            </div>
        </Timeline>
    }
}
