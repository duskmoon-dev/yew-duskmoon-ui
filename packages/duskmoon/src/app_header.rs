use yew::prelude::*;

/// Props for [`AppHeader`]
#[derive(Properties, Clone, PartialEq)]
pub struct AppHeaderProps {
    /// CSS classes to add to the container element (optional).
    #[prop_or_default]
    pub classes: Classes,
    #[prop_or_default]
    pub variant: Option<String>,
    /// logo part
    pub logo: Html,
    /// menu part
    #[prop_or_default]
    pub menu: Html,
    /// info part
    #[prop_or_default]
    pub info: Html,
}

/// AppHeader component using Tailwind CSS navbar classes
#[function_component(AppHeader)]
pub fn app_header(props: &AppHeaderProps) -> Html {
    let owned_props = props.clone();
    let mut classes = classes!(
        "app-header",
        "navbar",
        "bg-surface-container-high",
        "shadow-md",
        "h-[60px]",
        "px-8"
    );

    if let Some(variant) = &owned_props.variant {
        classes.push(format!("app-header-{}", variant));
    }

    classes.push(owned_props.classes.clone());

    html! {
      <nav class={classes}>
        <div class="navbar-start flex items-center h-full gap-4">
          <div class="logo flex items-center h-full">
              { owned_props.logo }
          </div>
          <div class="menu flex items-center h-full gap-5">
              { owned_props.menu }
          </div>
        </div>
        <div class="navbar-end flex items-center h-full">
          <div class="info flex items-center h-full">
              { owned_props.info }
          </div>
        </div>
      </nav>
    }
}
