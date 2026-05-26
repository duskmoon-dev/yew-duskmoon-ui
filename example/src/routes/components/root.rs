use yew::prelude::*;
use yew_duskmoon::Card;
use yew_duskmoon::Link;
use yew_duskmoon::Button;
use yew_duskmoon::button::ButtonType;

use super::ComponentsRoute;

/// Components page
#[function_component(ComponentsRoot)]
pub fn components_root() -> Html {
  html! {
    <div class="app">
      <div class="flex justify-center items-center w-full min-h-[300px] bg-primary text-on-primary select-none bg-[url(./assets/moon.png)] bg-no-repeat bg-[size:20%] bg-right bg-blend-hard-light">
        <h1 style="text-shadow: #FC0 1px 0 10px;" class="flex text-8xl">
          { "Duskmoon Components" }
        </h1>
      </div>
      <div class="app-main" style="gap: 2rem">
        <Card
          title={html!{
            <h3> { "General" } </h3>
          }}
        >
          <div class="flex flex-row justify-center items-center gap-4">
            <div class="item">
              <Link<ComponentsRoute> to={ComponentsRoute::ButtonComponent}>
                {"Button"}
              </Link<ComponentsRoute>>
            </div>
            <div class="item">
              <Link<ComponentsRoute> to={ComponentsRoute::TypographyComponent}>
                {"Typography"}
              </Link<ComponentsRoute>>
            </div>
            <div class="item">
              <Button r#type={ButtonType::Link} disabled={true}>
                {"Code"}
              </Button>
            </div>
            <div class="item">
              <Button r#type={ButtonType::Link} disabled={true}>
                {"Markdown"}
              </Button>
            </div>
          </div>
        </Card>
        <Card title={ html!{
          <h3> { "Layout" } </h3>
        }}>
          <div class="flex flex-row justify-center items-center gap-4">
            <div class="item">
              <Button r#type={ButtonType::Link} disabled={true}>
                {"Layout"}
              </Button>
            </div>
            <div class="item">
              <Button r#type={ButtonType::Link} disabled={true}>
                {"Row / Col"}
              </Button>
            </div>
            <div class="item">
              <Button r#type={ButtonType::Link} disabled={true}>
                {"Divider"}
              </Button>
            </div>
            <div class="item">
              <Button r#type={ButtonType::Link} disabled={true}>
                {"Space"}
              </Button>
            </div>
          </div>
        </Card>
        <Card title={ html!{
          <h3> { "Form" } </h3>
        }}>
          <div class="flex flex-row justify-center items-center gap-4">
            <div class="item">
              <Button r#type={ButtonType::Link} disabled={true}>
                {"Layout"}
              </Button>
            </div>
            <div class="item">
              <Button r#type={ButtonType::Link} disabled={true}>
                {"AppHeader"}
              </Button>
            </div>
            <div class="item">
              <Button r#type={ButtonType::Link} disabled={true}>
                {"Row / Col"}
              </Button>
            </div>
            <div class="item">
              <Button r#type={ButtonType::Link} disabled={true}>
                {"Divider"}
              </Button>
            </div>
            <div class="item">
              <Button r#type={ButtonType::Link} disabled={true}>
                {"Space"}
              </Button>
            </div>
          </div>
        </Card>
        <Card title={ html!{
          <h3> { "Data Display" } </h3>
        }}>
          <div class="flex flex-row justify-center items-center gap-4">
            <div class="item">
              <Button r#type={ButtonType::Link} disabled={true}>
                {"Card"}
              </Button>
            </div>
            <div class="item">
              <Button r#type={ButtonType::Link} disabled={true}>
                {"Table"}
              </Button>
            </div>
            <div class="item">
              <Button r#type={ButtonType::Link} disabled={true}>
                {"List"}
              </Button>
            </div>
            <div class="item">
              <Button r#type={ButtonType::Link} disabled={true}>
                {"Tree"}
              </Button>
            </div>
          </div>
        </Card>
        <Card title={ html!{
          <h3> { "Feedback" } </h3>
        }}>
          <div class="flex flex-row justify-center items-center gap-4">
            <div class="item">
              <Button r#type={ButtonType::Link} disabled={true}>
                {"Modal"}
              </Button>
            </div>
            <div class="item">
              <Button r#type={ButtonType::Link} disabled={true}>
                {"Alert"}
              </Button>
            </div>
            <div class="item">
              <Button r#type={ButtonType::Link} disabled={true}>
                {"Notification"}
              </Button>
            </div>
            <div class="item">
              <Button r#type={ButtonType::Link} disabled={true}>
                {"Message"}
              </Button>
            </div>
          </div>
        </Card>
        <Card title={ html!{
          <h3> { "Navigation" } </h3>
        }}>
          <div class="flex flex-row justify-center items-center gap-4">
            <div class="item">
              <Button r#type={ButtonType::Link} disabled={true}>
                {"Breadcrumb"}
              </Button>
            </div>
            <div class="item">
              <Button r#type={ButtonType::Link} disabled={true}>
                {"Menu"}
              </Button>
            </div>
            <div class="item">
              <Button r#type={ButtonType::Link} disabled={true}>
                {"Pagination"}
              </Button>
            </div>
            <div class="item">
              <Button r#type={ButtonType::Link} disabled={true}>
                {"Steps"}
              </Button>
            </div>
          </div>
        </Card>
      </div>
    </div>
  }
}
