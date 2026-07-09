# yew-duskmoon

Duskmoon UI components for [Yew](https://yew.rs/) applications.

[![yew-duskmoon crates version](https://badgen.net/crates/v/yew-duskmoon)](https://crates.io/crates/yew-duskmoon)
[![yew-duskmoon crates download](https://badgen.net/crates/d/yew-duskmoon)](https://crates.io/crates/yew-duskmoon)

## Links

- [Documentation and demo](https://duskmoon-dev.github.io/yew-duskmoon-ui/)
- [Repository](https://github.com/duskmoon-dev/yew-duskmoon-ui)

## Install

```toml
[dependencies]
yew = { version = "0.23", features = ["csr"] }
yew-duskmoon = "0.4"
```

The default `full` feature enables the feature-gated components: `AppHeader`, `Button`, `Card`, `Link`, and `Typography`.

For explicit feature selection:

```toml
[dependencies]
yew-duskmoon = { version = "0.4", default-features = false, features = ["button", "card"] }
```

## CSS

This crate renders Duskmoon CSS class names. Include Duskmoon UI styles in the consuming app.

The repository demo uses Tailwind CSS 4 with `@duskmoon-dev/core`:

```css
@import "tailwindcss";
@plugin "@duskmoon-dev/core/plugin";
@import "@duskmoon-dev/core";
```

If your CSS build removes unused classes, make sure it scans your Yew source files and the component sources you use.

## Usage

```rust
use yew::prelude::*;
use yew_duskmoon::{Button, Card};

#[function_component(App)]
fn app() -> Html {
    html! {
        <Card title={html! { "Quick start" }}>
            <Button variant={Some("primary".to_owned())}>
                { "Save" }
            </Button>
        </Card>
    }
}
```

## Components

Common exports include `Accordion`, `Alert`, `Appbar`, `Autocomplete`, `Avatar`, `Badge`, `BottomNavigation`, `Breadcrumbs`, `Button`, `Card`, `Checkbox`, `Chip`, `CodeBlock`, `DatePicker`, `Dialog`, `DmMarkdown`, `Drawer`, `FileUpload`, `Form`, `Grid`, `Input`, `Link`, `List`, `MarkdownBody`, `Menu`, `Modal`, `Pagination`, `Progress`, `Radio`, `Select`, `Switch`, `Table`, `Tabs`, `Textarea`, `ThemeController`, `Toast`, `Tooltip`, `TreeSelect`, and `Typography`.

See the [demo](https://duskmoon-dev.github.io/yew-duskmoon-ui/) for the full catalog and component examples.
