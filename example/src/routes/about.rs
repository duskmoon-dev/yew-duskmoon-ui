use serde::{Deserialize, Serialize};
use yew::prelude::*;
use yew_duskmoon::{Button, Card, Alert, Typography};
use yew_duskmoon::button::ButtonType;
use yew_duskmoon::typography::TypographyLevel;
use yew_hooks::prelude::*;

/// About page
#[function_component(About)]
pub fn about() -> Html {
  let state = use_async(async move { fetch_repo(("yew-duskmoon-ui".to_string()).clone()).await });

  {
      let state = state.clone();
      use_effect_with(
          (),
          move |_| {
              state.run();
          },
      );
  }

  html! {
    <div class="app">
      <div style="text-shadow: #FC0 1px 0 10px;" class="flex justify-center items-center w-full min-h-[300px] bg-primary text-on-primary select-none bg-[url(./assets/moon.png)] bg-no-repeat bg-[size:20%] bg-right bg-blend-hard-light">
        <Typography level={TypographyLevel::H1} classes="flex text-8xl m-0 font-bold">
          {if let Some(repo) = &state.data {
            html!{ format!("Stars {}", repo.stargazers_count) }
          } else {
            html!{ format!("Stars {}", 0) }
          }}
        </Typography>
      </div>
      <div class="app-main">
        <Card>
          <div>
            {
              if state.loading {
                html! {
                  <Alert variant={Some("info".to_string())}>
                    { "Loading, wait a sec..." }
                  </Alert>
                }
              } else {
                if let Some(repo) = &state.data {
                  html! {
                    <div key={repo.id} class="space-h">
                        <div class="flex items-center gap-2">
                          <Typography level={TypographyLevel::Default} classes="text-muted">{ "repo name: " }</Typography>
                          <Typography level={TypographyLevel::Default} classes="font-bold">{ html! { &repo.name } }</Typography>
                        </div>
                        <div class="flex items-center gap-2">
                          <Typography level={TypographyLevel::Default} classes="text-muted">{ "repo web url: " }</Typography>
                          <Typography level={TypographyLevel::Default} classes="font-mono">{ html! { &repo.html_url } }</Typography>
                        </div>
                        <div class="flex items-center gap-2">
                            <Typography level={TypographyLevel::Default} classes="text-muted">{ "Go to Github Repo: " }</Typography>
                            <Button
                                r#type={ ButtonType::Link }
                                href={ repo.html_url.clone() }
                                target="_blank"
                                rel="noopener noreferrer"
                            >
                                { html! { &repo.name } }
                            </Button>
                        </div>
                    </div>
                  }
                } else if let Some(error) = &state.error {
                  let err_msg = match error {
                      Error::DeserializeError => "Deserialize Error",
                      Error::RequestError => "Request Error",
                  };
                  html! {
                    <Alert variant={Some("error".to_string())}>
                      { html! { err_msg } }
                    </Alert>
                  }
                } else {
                    html! {}
                }
              }
            }
          </div>
        </Card>
      </div>
    </div>
  }
}

async fn fetch_repo(repo: String) -> Result<Repo, Error> {
  let url = format!("https://api.github.com/repos/gsmlg-dev/{}", repo);
  let response = reqwest::get(url).await;
  if let Ok(data) = response {
    if let Ok(repo) = data.json::<Repo>().await {
      Ok(repo)
    } else {
      Err(Error::DeserializeError)
    }
  } else {
    Err(Error::RequestError)
  }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
struct Repo {
  id: i32,
  name: String,
  html_url: String,
  created_at: String,
  pushed_at: String,
  size: i32,
  stargazers_count: i32,
  watchers_count: i32,
  forks_count: i32,
}

// You can use thiserror to define your errors.
#[derive(Clone, Debug, PartialEq)]
enum Error {
  RequestError,
  DeserializeError,
  // etc.
}
