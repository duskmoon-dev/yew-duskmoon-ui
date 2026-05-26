use yew::prelude::*;
use yew_duskmoon::button::ButtonType;
use yew_duskmoon::Button;
use yew_duskmoon::Card;
use strum::IntoEnumIterator;
use stylist::css;
use stylist::yew::use_style;

/// Components page
#[function_component(ButtonComponent)]
pub fn component() -> Html {
  let list_style = use_style(css!(
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
        <Card title={ html!{ "Duskmoon Components - Button" } }>
          { "Button" }
          <div>
            <pre>
            {"use yew_duskmoon::Button;\n"}
            {"use yew_duskmoon::button::ButtonType;\n"}
            {"\n\n"}
            </pre>
          </div>
          <ul class={ list_style }>
            {ButtonType::iter().into_iter().map(|t| {
              html!{ 
                <li>
                  <label class="t">{ format!("ButtonType::{:?}", t) }</label>
                  <div class="v">
                    <Button r#type={t.clone()}>{html! { format!("{:?}", t) }}</Button>
                  </div>
                  <div class="v">
                    <Button r#type={t.clone()} disabled={true}>{"Disabled"}</Button>
                  </div>
                  <div class="v">
                    <Button r#type={t.clone()} loading={true}>{"Loading"}</Button>
                  </div>
                  <code class="c">
                  {format!("html!{{ <Button type={{ButtonType::{}}}>Button<Button> }}", t.clone())}
                  </code>
                  <code class="c">
                  {format!("html!{{ <Button type={{ButtonType::{}}} disabled={{true}}>Button<Button> }}", t.clone())}
                  </code>
                  <code class="c">
                  {format!("html!{{ <Button type={{ButtonType::{}}} loading={{true}}>Button<Button> }}", t.clone())}
                  </code>
                </li>
              }
            }).collect::<Html>()}
          </ul>
        </Card>
      </div>
    </div>
  }
}
