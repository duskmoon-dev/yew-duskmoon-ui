use yew::prelude::*;
use yew_duskmoon::typography::TypographyLevel;
use yew_duskmoon::{Link, Typography};

use super::ComponentsRoute;

fn catalog_group(
    index: &'static str,
    title: &'static str,
    description: &'static str,
    items: Vec<(&'static str, ComponentsRoute)>,
) -> Html {
    html! {
        <article class="catalog-card">
            <span class="catalog-index">{ index }</span>
            <div class="catalog-copy">
                <h3 class="catalog-title">{ title }</h3>
                <p>{ description }</p>
            </div>
            <div class="catalog-links">
                { for items.into_iter().map(|(label, route)| html! {
                    <Link<ComponentsRoute> to={route} classes="catalog-link">
                        <span>{ label }</span>
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
                        <span>{ "routes" }</span>
                        <strong>{ "8" }</strong>
                    </div>
                    <div>
                        <span>{ "families" }</span>
                        <strong>{ "6" }</strong>
                    </div>
                    <div>
                        <span>{ "colors" }</span>
                        <strong>{ "10" }</strong>
                    </div>
                </section>

                <section class="catalog-grid">
                    { catalog_group(
                        "01",
                        "General",
                        "Foundational pieces used throughout the rest of the example shell.",
                        vec![
                            ("Button", ComponentsRoute::ButtonComponent),
                            ("Typography", ComponentsRoute::TypographyComponent),
                            ("Code", ComponentsRoute::CodeComponent),
                            ("Markdown", ComponentsRoute::CodeComponent),
                        ],
                    ) }
                    { catalog_group(
                        "02",
                        "Layout",
                        "Spacing and separation primitives for composing interface regions.",
                        vec![
                            ("Divider", ComponentsRoute::LayoutComponent),
                            ("Space", ComponentsRoute::LayoutComponent),
                        ],
                    ) }
                    { catalog_group(
                        "03",
                        "Form",
                        "Input controls for capture, selection, and preference toggles.",
                        vec![
                            ("Input", ComponentsRoute::FormComponent),
                            ("Textarea", ComponentsRoute::FormComponent),
                            ("Checkbox", ComponentsRoute::FormComponent),
                            ("Radio", ComponentsRoute::FormComponent),
                            ("Switch", ComponentsRoute::FormComponent),
                        ],
                    ) }
                    { catalog_group(
                        "04",
                        "Data Display",
                        "Compact components for status, records, and structured data.",
                        vec![
                            ("Card", ComponentsRoute::DataDisplayComponent),
                            ("Table", ComponentsRoute::DataDisplayComponent),
                            ("List", ComponentsRoute::DataDisplayComponent),
                            ("Badge", ComponentsRoute::DataDisplayComponent),
                        ],
                    ) }
                    { catalog_group(
                        "05",
                        "Feedback",
                        "Response surfaces for success, warning, error, and transient events.",
                        vec![
                            ("Modal", ComponentsRoute::FeedbackComponent),
                            ("Alert", ComponentsRoute::FeedbackComponent),
                            ("Toast", ComponentsRoute::FeedbackComponent),
                        ],
                    ) }
                    { catalog_group(
                        "06",
                        "Navigation",
                        "Route, hierarchy, progress, and page movement primitives.",
                        vec![
                            ("Breadcrumbs", ComponentsRoute::NavigationComponent),
                            ("Menu", ComponentsRoute::NavigationComponent),
                            ("Pagination", ComponentsRoute::NavigationComponent),
                            ("Stepper", ComponentsRoute::NavigationComponent),
                        ],
                    ) }
                </section>
            </main>
        </div>
    }
}
