use serde::{Deserialize, Serialize};
use yew::prelude::*;
use yew_duskmoon::button::ButtonType;
use yew_duskmoon::typography::TypographyLevel;
use yew_duskmoon::{Alert, Button, Card, Typography};
use yew_hooks::prelude::*;

/// About page
#[function_component(About)]
pub fn about() -> Html {
    let state = use_async(async move { fetch_repo(("yew-duskmoon-ui".to_string()).clone()).await });

    {
        let state = state.clone();
        use_effect_with((), move |_| {
            state.run();
        });
    }

    html! {
      <div class="app example-page about-page">
        <section class="about-hero">
          <div>
            <span class="eyebrow">{ "Repository signal" }</span>
            <Typography level={TypographyLevel::H1} classes="page-title">
              { "Open workbench, live metadata." }
            </Typography>
          </div>
          <div class="about-score">
            <span>{ "stars" }</span>
            <strong>
              {if let Some(repo) = &state.data {
                html!{ repo.stargazers_count }
              } else {
                html!{ 0 }
              }}
            </strong>
          </div>
        </section>

        <main class="app-main about-main">
          <Card classes="repo-card">
            {
              if state.loading {
                html! {
                  <Alert variant={Some("info".to_string())}>
                    { "Fetching repository metadata..." }
                  </Alert>
                }
              } else if let Some(repo) = &state.data {
                html! {
                  <div key={repo.id} class="repo-report">
                    <div class="repo-report-head">
                      <span class="section-kicker">{ "GitHub" }</span>
                      <Typography level={TypographyLevel::H2} classes="repo-name">
                        { html! { &repo.name } }
                      </Typography>
                    </div>

                    <div class="repo-stat-grid">
                      <div>
                        <span>{ "watchers" }</span>
                        <strong>{ repo.watchers_count }</strong>
                      </div>
                      <div>
                        <span>{ "forks" }</span>
                        <strong>{ repo.forks_count }</strong>
                      </div>
                      <div>
                        <span>{ "size" }</span>
                        <strong>{ format!("{} KB", repo.size) }</strong>
                      </div>
                    </div>

                    <div class="repo-meta">
                      <div>
                        <span>{ "created" }</span>
                        <strong>{ html! { &repo.created_at } }</strong>
                      </div>
                      <div>
                        <span>{ "pushed" }</span>
                        <strong>{ html! { &repo.pushed_at } }</strong>
                      </div>
                      <div>
                        <span>{ "url" }</span>
                        <strong>{ html! { &repo.html_url } }</strong>
                      </div>
                    </div>

                    <Button
                      r#type={ ButtonType::Link }
                      href={ repo.html_url.clone() }
                      target="_blank"
                      rel="noopener noreferrer"
                      classes="hero-link hero-link-primary repo-cta"
                    >
                      { "Open repository" }
                    </Button>
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
          </Card>
        </main>
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
