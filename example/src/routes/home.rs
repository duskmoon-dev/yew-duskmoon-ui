use yew::prelude::*;
use yew_duskmoon::{Button, Card, Typography};
use yew_duskmoon::button::ButtonType;
use yew_duskmoon::typography::TypographyLevel;
use yewdux::prelude::*;

use crate::states::config::ConfigStore;

/// Home page
#[function_component(Home)]
pub fn home() -> Html {
    let (_state, dispatch) = use_store::<ConfigStore>();
    let set_lower = dispatch.reduce_mut_callback(|l| l.name = "capitalize".to_string());
    let set_upper = dispatch.reduce_mut_callback(|l| l.name = "uppercase".to_string());

    html! {
      <div class="app">
        <div style="text-shadow: #FC0 1px 0 10px;" class="flex justify-center items-center w-full min-h-[300px] bg-primary text-on-primary select-none bg-[url(./assets/moon.png)] bg-no-repeat bg-[size:20%] bg-right bg-blend-hard-light">
          <Typography level={TypographyLevel::H1} classes="flex text-8xl m-0 font-bold">
            { "Duskmoon UI" }
          </Typography>
        </div>
        <div class="app-main">
          <Card title={ html! { <Typography level={TypographyLevel::H4} classes="text-primary text-xl font-bold">{"Config Header Text Transform"}</Typography> } } classes="w-[90%]">
              <div class="space">
                <div class="space-item">
                  <Button onclick={set_lower}>{ "captialize" }</Button>
                </div>
                <div class="space-item">
                  <Button r#type={ButtonType::Primary} onclick={set_upper}>{ "UPPERCASE" }</Button>
                </div>
              </div>
          </Card>
        </div>
      </div>
    }
}
