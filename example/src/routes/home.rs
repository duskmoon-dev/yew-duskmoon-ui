use yew::prelude::*;
use yew_duskmoon::button::ButtonType;
use yew_duskmoon::typography::TypographyLevel;
use yew_duskmoon::{Button, Card, Link, Typography};
use yewdux::prelude::*;

use crate::routes::components::ComponentsRoute;
use crate::routes::AppRoute;
use crate::states::config::ConfigStore;

/// Home page
#[function_component(Home)]
pub fn home() -> Html {
    let (_state, dispatch) = use_store::<ConfigStore>();
    let set_title_case = dispatch.reduce_mut_callback(|l| l.name = "capitalize".to_string());
    let set_upper = dispatch.reduce_mut_callback(|l| l.name = "uppercase".to_string());
    let set_lower = dispatch.reduce_mut_callback(|l| l.name = "lowercase".to_string());

    html! {
      <div class="app example-page home-page">
        <section class="hero-stage">
          <div class="hero-copy">
            <span class="eyebrow">{ "Yew component kit" }</span>
            <Typography level={TypographyLevel::H1} classes="hero-title">
              { "Duskmoon UI" }
            </Typography>
            <p class="hero-lede">
              { "A Rust-first component gallery with crisp controls, dark surfaces, and enough structure to audit real interface states quickly." }
            </p>
            <div class="hero-actions">
              <Link<AppRoute> to={AppRoute::ComponentsRoot} classes="hero-link hero-link-primary">
                { "Browse catalog" }
              </Link<AppRoute>>
              <Button
                r#type={ButtonType::Link}
                href={"https://github.com/gsmlg-dev/yew-duskmoon-ui"}
                target={"_blank"}
                rel={"noopener noreferrer"}
                classes="hero-link hero-link-secondary"
              >
                { "Open source" }
              </Button>
            </div>
          </div>

          <div class="hero-visual" aria-hidden="true">
            <div class="moonplate">
              <img src="./assets/moon.png" alt="" />
            </div>
            <div class="instrument-readout">
              <span>{ "crate" }</span>
              <strong>{ "yew-duskmoon" }</strong>
              <small>{ "csr / wasm / md3" }</small>
            </div>
          </div>
        </section>

        <main class="app-main home-main">
          <section class="signal-grid" aria-label="Project highlights">
            <article class="signal-panel">
              <span>{ "runtime" }</span>
              <strong>{ "Yew 0.23" }</strong>
              <p>{ "Client-side rendering with router-backed examples." }</p>
            </article>
            <article class="signal-panel signal-panel-accent">
              <span>{ "surface" }</span>
              <strong>{ "40+ controls" }</strong>
              <p>{ "Forms, feedback, navigation, data display, and layout pieces." }</p>
            </article>
            <article class="signal-panel">
              <span>{ "styling" }</span>
              <strong>{ "Tailwind plugin" }</strong>
              <p>{ "Duskmoon core tokens plus example-specific presentation CSS." }</p>
            </article>
          </section>

          <section class="showcase-band">
            <div class="section-kicker">{ "Start points" }</div>
            <div class="route-grid">
              <Link<ComponentsRoute> to={ComponentsRoute::ButtonComponent} classes="route-tile">
                <span>{ "01" }</span>
                <strong>{ "Buttons" }</strong>
                <small>{ "States, variants, loading" }</small>
              </Link<ComponentsRoute>>
              <Link<ComponentsRoute> to={ComponentsRoute::FormComponent} classes="route-tile">
                <span>{ "02" }</span>
                <strong>{ "Forms" }</strong>
                <small>{ "Inputs, choices, toggles" }</small>
              </Link<ComponentsRoute>>
              <Link<ComponentsRoute> to={ComponentsRoute::FeedbackComponent} classes="route-tile">
                <span>{ "03" }</span>
                <strong>{ "Feedback" }</strong>
                <small>{ "Alerts, modal, toast" }</small>
              </Link<ComponentsRoute>>
              <Link<ComponentsRoute> to={ComponentsRoute::DataDisplayComponent} classes="route-tile">
                <span>{ "04" }</span>
                <strong>{ "Data" }</strong>
                <small>{ "Badge, list, table" }</small>
              </Link<ComponentsRoute>>
            </div>
          </section>

          <Card
            title={ html! { <Typography level={TypographyLevel::H4} classes="control-title">{"Header Text Transform"}</Typography> } }
            classes="control-card"
          >
            <div class="control-row">
              <div>
                <span class="control-label">{ "Live config store" }</span>
                <p>{ "Navigation casing preset for the shared header." }</p>
              </div>
              <div class="segmented-actions">
                <Button onclick={set_lower} classes="segment-button">{ "lowercase" }</Button>
                <Button onclick={set_title_case} classes="segment-button">{ "Capitalize" }</Button>
                <Button r#type={ButtonType::Primary} onclick={set_upper} classes="segment-button">{ "UPPERCASE" }</Button>
              </div>
            </div>
          </Card>
        </main>
      </div>
    }
}
