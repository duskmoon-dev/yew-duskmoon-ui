use yew::prelude::*;
use yew_router::prelude::*;

use self::button::ButtonComponent;
use self::code::CodeComponent;
use self::data_display::DataDisplayComponent;
use self::feedback::FeedbackComponent;
use self::form::FormComponent;
use self::layout::LayoutComponent;
use self::navigation::NavigationComponent;
use self::typography::TypographyComponent;
use crate::routes::AppRoute;

pub mod button;
pub mod code;
pub mod data_display;
pub mod feedback;
pub mod form;
pub mod layout;
pub mod navigation;
pub mod root;
pub mod typography;
pub use self::root::ComponentsRoot;

#[derive(Clone, Routable, PartialEq)]
pub enum ComponentsRoute {
    #[at("/components")]
    ComponentsRoot,
    #[at("/components/button")]
    ButtonComponent,
    #[at("/components/typography")]
    TypographyComponent,
    #[at("/components/code")]
    CodeComponent,
    #[at("/components/layout")]
    LayoutComponent,
    #[at("/components/form")]
    FormComponent,
    #[at("/components/data-display")]
    DataDisplayComponent,
    #[at("/components/feedback")]
    FeedbackComponent,
    #[at("/components/navigation")]
    NavigationComponent,
    #[not_found]
    #[at("/components/404")]
    NotFound,
}

/// Switch components routes
pub fn switch_components(route: ComponentsRoute) -> Html {
    match route {
        ComponentsRoute::ComponentsRoot => html! { <ComponentsRoot /> },
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
