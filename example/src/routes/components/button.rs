use yew::prelude::*;
use yew_duskmoon::button::ButtonType;
use yew_duskmoon::Button;
use yew_duskmoon::Card;
use strum::IntoEnumIterator;

/// Components page
#[function_component(ButtonComponent)]
pub fn component() -> Html {
  html! {
    <div class="app">
      <div class="app-main">
        <Card title={ html!{ "Duskmoon Components - Button" } } classes="w-[90%]">
          <div class="code">
            <pre>
            {"use yew_duskmoon::Button;\n"}
            {"use yew_duskmoon::button::ButtonType;\n"}
            {"\n\n"}
            </pre>
          </div>
          <ul class="flex flex-col justify-start items-start gap-8 list-none w-full p-0">
            {ButtonType::iter().into_iter().map(|t| {
              html!{ 
                <li class="flex flex-col w-full gap-3">
                  <label class="flex after:content-[':'] after:inline-flex after:px-0.5 font-semibold">{ format!("ButtonType::{:?}", t) }</label>
                  <div class="flex">
                    <Button r#type={t.clone()}>{html! { format!("{:?}", t) }}</Button>
                  </div>
                  <div class="flex">
                    <Button r#type={t.clone()} disabled={true}>{"Disabled"}</Button>
                  </div>
                  <div class="flex">
                    <Button r#type={t.clone()} loading={true}>{"Loading"}</Button>
                  </div>
                  <code class="flex">
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
