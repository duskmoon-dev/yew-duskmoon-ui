use yew::prelude::*;
use yew_duskmoon::typography::TypographyLevel;
use yew_duskmoon::{Link, Typography};

use self::page::{ApiRow, ComponentPage};
use super::ComponentsRoute;

mod page;
mod pages;

#[derive(Properties, Clone, PartialEq)]
pub struct ComponentDetailProps {
    pub slug: String,
}

#[function_component(ComponentDetail)]
pub fn component_detail(props: &ComponentDetailProps) -> Html {
    match pages::component_page(&props.slug) {
        Some(page) => render_component_detail(page),
        None => html! {
            <div class="app example-page component-detail-page">
                <main class="app-main component-detail-main">
                    <section class="detail-hero">
                        <span class="eyebrow">{ "Component catalog" }</span>
                        <Typography level={TypographyLevel::H1} classes="page-title">
                            { "Component not found." }
                        </Typography>
                        <Link<ComponentsRoute> to={ComponentsRoute::ComponentsRoot} classes="detail-back-link">
                            { "Back to catalog" }
                        </Link<ComponentsRoute>>
                    </section>
                </main>
            </div>
        },
    }
}

fn render_component_detail(page: ComponentPage) -> Html {
    let spec = page.spec;

    html! {
        <div class="app example-page component-detail-page">
            <section class="detail-hero component-detail-hero">
                <div>
                    <span class="eyebrow">{ spec.category }</span>
                    <h1 class="page-title">{ spec.name }</h1>
                </div>
                <div class="detail-hero-copy">
                    <p>{ spec.description }</p>
                    <Link<ComponentsRoute> to={ComponentsRoute::ComponentsRoot} classes="detail-back-link">
                        { "Back to catalog" }
                    </Link<ComponentsRoute>>
                </div>
            </section>

            <main class="app-main component-detail-main">
                <section id="docs" class="detail-section">
                    <div class="detail-section-head">
                        <span class="section-kicker">{ "Docs" }</span>
                        <h2>{ "Usage guidance" }</h2>
                    </div>
                    <div class="detail-copy">
                        <p>{ spec.docs }</p>
                        <p>
                            { "The Yew component is " }
                            <code>{ spec.component_name }</code>
                            { " and its base style class is " }
                            <code>{ spec.class_name }</code>
                            { "." }
                        </p>
                    </div>
                    <pre class="detail-code"><code>{ (page.usage)(spec) }</code></pre>
                </section>

                <section id="api" class="detail-section">
                    <div class="detail-section-head">
                        <span class="section-kicker">{ "API" }</span>
                        <h2>{ "Props" }</h2>
                    </div>
                    <div class="detail-api-content">
                        { render_api_table(page.api_rows) }
                        if let Some((title, rows)) = page.additional_api {
                            <section class="api-subsection" aria-labelledby="additional-api-title">
                                <h3 id="additional-api-title">{ title }</h3>
                                { render_api_table(rows) }
                            </section>
                        }
                    </div>
                </section>

                <section id="demo" class="detail-section detail-demo-section">
                    <div class="detail-section-head">
                        <span class="section-kicker">{ "Feature demo" }</span>
                        <h2>{ "Component behavior" }</h2>
                    </div>
                    <div class="demo-stage">
                        { (page.feature_demo)(spec) }
                    </div>
                </section>

                if page.color_variant.is_some() {
                    <section id="colors" class="detail-section detail-colors-section">
                        <div class="detail-section-head">
                            <span class="section-kicker">{ "Color demo" }</span>
                            <h2>{ "Theme variants" }</h2>
                        </div>
                        <div class="detail-color-grid">
                            { page.render_color_matrix() }
                        </div>
                    </section>
                }
            </main>
        </div>
    }
}

fn render_api_table(rows: &'static [ApiRow]) -> Html {
    html! {
        <div class="api-table-wrap">
            <table class="api-table">
                <thead>
                    <tr>
                        <th>{ "Prop" }</th>
                        <th>{ "Type" }</th>
                        <th>{ "Default" }</th>
                        <th>{ "Description" }</th>
                    </tr>
                </thead>
                <tbody>
                    { for rows.iter().map(|row| html! {
                        <tr>
                            <td><code>{ row.prop }</code></td>
                            <td><code>{ row.ty }</code></td>
                            <td>{ row.default }</td>
                            <td>{ row.docs }</td>
                        </tr>
                    }) }
                </tbody>
            </table>
        </div>
    }
}
