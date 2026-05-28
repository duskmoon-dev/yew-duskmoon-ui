use yew::prelude::*;
use yew_duskmoon::{Card, Link, Button, Typography};
use yew_duskmoon::button::ButtonType;
use yew_duskmoon::typography::TypographyLevel;

use super::ComponentsRoute;

/// Components page
#[function_component(ComponentsRoot)]
pub fn components_root() -> Html {
  html! {
    <div class="app">
      <div style="text-shadow: #FC0 1px 0 10px;" class="flex justify-center items-center w-full min-h-[300px] bg-primary text-on-primary select-none bg-[url(./assets/moon.png)] bg-no-repeat bg-[size:20%] bg-right bg-blend-hard-light">
        <Typography level={TypographyLevel::H1} classes="flex text-8xl m-0 font-bold">
          { "Duskmoon Components" }
        </Typography>
      </div>
      <div class="app-main" style="gap: 2rem">
        <Card
          title={html!{
            <Typography level={TypographyLevel::H3}> { "General" } </Typography>
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
              <Link<ComponentsRoute> to={ComponentsRoute::CodeComponent}>
                {"Code"}
              </Link<ComponentsRoute>>
            </div>
            <div class="item">
              <Link<ComponentsRoute> to={ComponentsRoute::CodeComponent}>
                {"Markdown"}
              </Link<ComponentsRoute>>
            </div>
          </div>
        </Card>
        <Card title={ html!{
          <Typography level={TypographyLevel::H3}> { "Layout" } </Typography>
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
              <Link<ComponentsRoute> to={ComponentsRoute::LayoutComponent}>
                {"Divider"}
              </Link<ComponentsRoute>>
            </div>
            <div class="item">
              <Button r#type={ButtonType::Link} disabled={true}>
                {"Space"}
              </Button>
            </div>
          </div>
        </Card>
        <Card title={ html!{
          <Typography level={TypographyLevel::H3}> { "Form" } </Typography>
        }}>
          <div class="flex flex-row justify-center items-center gap-4">
            <div class="item">
              <Link<ComponentsRoute> to={ComponentsRoute::FormComponent}>
                {"Input"}
              </Link<ComponentsRoute>>
            </div>
            <div class="item">
              <Link<ComponentsRoute> to={ComponentsRoute::FormComponent}>
                {"Textarea"}
              </Link<ComponentsRoute>>
            </div>
            <div class="item">
              <Link<ComponentsRoute> to={ComponentsRoute::FormComponent}>
                {"Checkbox"}
              </Link<ComponentsRoute>>
            </div>
            <div class="item">
              <Link<ComponentsRoute> to={ComponentsRoute::FormComponent}>
                {"Radio"}
              </Link<ComponentsRoute>>
            </div>
            <div class="item">
              <Link<ComponentsRoute> to={ComponentsRoute::FormComponent}>
                {"Switch"}
              </Link<ComponentsRoute>>
            </div>
          </div>
        </Card>
        <Card title={ html!{
          <Typography level={TypographyLevel::H3}> { "Data Display" } </Typography>
        }}>
          <div class="flex flex-row justify-center items-center gap-4">
            <div class="item">
              <Link<ComponentsRoute> to={ComponentsRoute::DataDisplayComponent}>
                {"Card"}
              </Link<ComponentsRoute>>
            </div>
            <div class="item">
              <Link<ComponentsRoute> to={ComponentsRoute::DataDisplayComponent}>
                {"Table"}
              </Link<ComponentsRoute>>
            </div>
            <div class="item">
              <Link<ComponentsRoute> to={ComponentsRoute::DataDisplayComponent}>
                {"List"}
              </Link<ComponentsRoute>>
            </div>
            <div class="item">
              <Link<ComponentsRoute> to={ComponentsRoute::DataDisplayComponent}>
                {"Badge"}
              </Link<ComponentsRoute>>
            </div>
          </div>
        </Card>
        <Card title={ html!{
          <Typography level={TypographyLevel::H3}> { "Feedback" } </Typography>
        }}>
          <div class="flex flex-row justify-center items-center gap-4">
            <div class="item">
              <Link<ComponentsRoute> to={ComponentsRoute::FeedbackComponent}>
                {"Modal"}
              </Link<ComponentsRoute>>
            </div>
            <div class="item">
              <Link<ComponentsRoute> to={ComponentsRoute::FeedbackComponent}>
                {"Alert"}
              </Link<ComponentsRoute>>
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
          <Typography level={TypographyLevel::H3}> { "Navigation" } </Typography>
        }}>
          <div class="flex flex-row justify-center items-center gap-4">
            <div class="item">
              <Link<ComponentsRoute> to={ComponentsRoute::NavigationComponent}>
                {"Breadcrumbs"}
              </Link<ComponentsRoute>>
            </div>
            <div class="item">
              <Link<ComponentsRoute> to={ComponentsRoute::NavigationComponent}>
                {"Menu"}
              </Link<ComponentsRoute>>
            </div>
            <div class="item">
              <Link<ComponentsRoute> to={ComponentsRoute::NavigationComponent}>
                {"Pagination"}
              </Link<ComponentsRoute>>
            </div>
            <div class="item">
              <Link<ComponentsRoute> to={ComponentsRoute::NavigationComponent}>
                {"Stepper"}
              </Link<ComponentsRoute>>
            </div>
          </div>
        </Card>
      </div>
    </div>
  }
}
