use yew::prelude::*;
use yew_duskmoon::typography::TypographyLevel;
use yew_duskmoon::Typography;
use yew_duskmoon::Card;
use stylist::css;
use stylist::yew::use_style;
use strum::IntoEnumIterator;

/// Components page
#[function_component(TypographyComponent)]
pub fn component() -> Html {
  let style = use_style(css!(
    r#"
    display: flex;
    flex-direction: column;
    justify-content: flex-start;
    align-items: flex-start;
    gap: 2rem;
    list-style: none;
    width: 100%;
    padding: 0;
    li {
      display: flex;
      flex-direction: column;
      width: 100%;
      gap: 0.82rem;
    }
    .t::after {
      content: ":";
      display: inline-flex;
      padding: 0 0.1em;
    }
    .t,.v,.c {
      display: flex;
    }
  "#
  ));

  html! {
    <div class="app">
      <div class="app-main">
        <Card title={ html!{ "Duskmoon Components - Typography" } }>
          <div class={ style }>
            {TypographyLevel::iter().into_iter().map(|l| {
                html!{
                <div>
                    <label class="t">{ format!("TypographyLevel::{:?}", l) }</label>
                    <div class="v">
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
