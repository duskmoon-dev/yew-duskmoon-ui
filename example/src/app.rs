use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::header::Header;
use crate::routes::{switch, AppRoute};
use crate::theme::Theme;

/// Root app component
#[function_component(App)]
pub fn app() -> Html {
    let theme = use_memo(
        (),
        |_| Theme {
            foreground: "#afe9dc".to_owned(),
            background: "#3a1f5d".to_owned(),
        },
    );

    html! {
        <ContextProvider<Theme> context={(*theme).clone()}>
            <BrowserRouter>
                <Header />
                <Switch<AppRoute> render={switch} />
            </BrowserRouter>
        </ContextProvider<Theme>>
    }
}
