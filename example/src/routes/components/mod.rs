use yew::prelude::*;
use yew_router::prelude::*;

use self::button::ButtonComponent;
use self::code::CodeComponent;
use self::data_display::DataDisplayComponent;
use self::detail::ComponentDetail;
use self::feedback::FeedbackComponent;
use self::form::FormComponent;
use self::layout::LayoutComponent;
use self::navigation::NavigationComponent;
use self::typography::TypographyComponent;
use crate::routes::AppRoute;

pub mod button;
pub mod catalog;
pub mod code;
pub mod data_display;
pub mod detail;
pub mod feedback;
pub mod form;
pub mod layout;
pub mod navigation;
pub mod palette;
pub mod root;
pub mod typography;
pub use self::root::ComponentsRoot;

#[derive(Clone, Routable, PartialEq)]
pub enum ComponentsRoute {
    #[at("/components")]
    ComponentsRoot,
    #[at("/components/:slug")]
    ComponentDetail { slug: String },
    #[at("/components/examples/button")]
    ButtonComponent,
    #[at("/components/examples/typography")]
    TypographyComponent,
    #[at("/components/examples/code")]
    CodeComponent,
    #[at("/components/examples/layout")]
    LayoutComponent,
    #[at("/components/examples/form")]
    FormComponent,
    #[at("/components/examples/data-display")]
    DataDisplayComponent,
    #[at("/components/examples/feedback")]
    FeedbackComponent,
    #[at("/components/examples/navigation")]
    NavigationComponent,
    #[not_found]
    #[at("/components/404")]
    NotFound,
}

/// Switch components routes
pub fn switch_components(route: ComponentsRoute) -> Html {
    match route {
        ComponentsRoute::ComponentsRoot => html! { <ComponentsRoot /> },
        ComponentsRoute::ComponentDetail { slug } => html! { <ComponentDetail slug={slug} /> },
        ComponentsRoute::ButtonComponent => html! { <ButtonComponent /> },
        ComponentsRoute::TypographyComponent => html! { <TypographyComponent /> },
        ComponentsRoute::CodeComponent => html! { <CodeComponent /> },
        ComponentsRoute::LayoutComponent => html! { <LayoutComponent /> },
        ComponentsRoute::FormComponent => html! { <FormComponent /> },
        ComponentsRoute::DataDisplayComponent => html! { <DataDisplayComponent /> },
        ComponentsRoute::FeedbackComponent => html! { <FeedbackComponent /> },
        ComponentsRoute::NavigationComponent => html! { <NavigationComponent /> },
        ComponentsRoute::NotFound => html! { <Redirect<AppRoute> to={AppRoute::NotFound} /> },
    }
}
