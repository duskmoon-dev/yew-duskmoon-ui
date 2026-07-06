use yew::prelude::*;
use yew_duskmoon::typography::TypographyLevel;
use yew_duskmoon::{Link, Typography};

use super::catalog::{ComponentCategory, COMPONENTS, CATEGORIES};
use super::ComponentsRoute;

fn catalog_group(category: &ComponentCategory) -> Html {
    html! {
        <article class="catalog-card">
            <span class="catalog-index">{ category.index }</span>
            <div class="catalog-copy">
                <h3 class="catalog-title">{ category.title }</h3>
                <p>{ category.description }</p>
            </div>
            <div class="catalog-links">
                { for COMPONENTS.iter().filter(|component| component.category == category.title).map(|component| html! {
                    <Link<ComponentsRoute> to={ComponentsRoute::ComponentDetail { slug: component.slug.to_owned() }} classes="catalog-link">
                        <span>{ component.name }</span>
                    </Link<ComponentsRoute>>
                }) }
            </div>
        </article>
    }
}

/// Components page
#[function_component(ComponentsRoot)]
pub fn components_root() -> Html {
    html! {
        <div class="app example-page catalog-page">
            <section class="catalog-hero">
                <div>
                    <span class="eyebrow">{ "Component catalog" }</span>
                    <Typography level={TypographyLevel::H1} classes="page-title">
                        { "Inspect the system by behavior." }
                    </Typography>
                </div>
                <p class="page-lede">
                    { "Each route groups related controls into a compact lab surface with complete primary, secondary, tertiary, accent, neutral, base, info, success, warning, and error color coverage." }
                </p>
            </section>

            <main class="app-main catalog-main">
                <section class="catalog-overview" aria-label="Catalog metrics">
                    <div>
                        <span>{ "demo routes" }</span>
                        <strong>{ "8" }</strong>
                    </div>
                    <div>
                        <span>{ "families" }</span>
                        <strong>{ CATEGORIES.len().to_string() }</strong>
                    </div>
                    <div>
                        <span>{ "components" }</span>
                        <strong>{ COMPONENTS.len().to_string() }</strong>
                    </div>
                </section>

                <section class="catalog-grid">
                    { for CATEGORIES.iter().map(catalog_group) }
                </section>
            </main>
        </div>
    }
}
