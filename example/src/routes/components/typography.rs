use yew::prelude::*;
use yew_duskmoon::typography::TypographyLevel;
use yew_duskmoon::Typography;
use yew_duskmoon::Card;
use strum::IntoEnumIterator;

/// Components page
#[function_component(TypographyComponent)]
pub fn component() -> Html {
  html! {
    <div class="app">
      <div class="app-main">
        <Card title={ html!{ "Duskmoon Components - Typography" } } classes="w-[90%]">
          <div class="flex flex-col justify-start items-start gap-8 list-none w-full p-0">
            {TypographyLevel::iter().into_iter().map(|l| {
                html!{
                <div class="flex flex-col w-full gap-3">
                    <label class="flex after:content-[':'] after:inline-flex after:px-0.5 font-semibold">{ format!("TypographyLevel::{:?}", l) }</label>
                    <div class="flex">
                    <Typography level={l.clone()}>{html! { format!("Typography Level {:?}", l) }}</Typography>
                    </div>
                    <code class="c">
                    {format!("html!{{ <Typography level={{TypographyLevel::{}}}>Typography {}<Typography> }}", l, l)}
                    </code>
                </div>
                }
            }).collect::<Html>()}
          </div>
        </Card>
      </div>
    </div>
  }
}
