# Yew Duskmoon UI

`yew-duskmoon` provides Duskmoon UI components for [Yew](https://yew.rs/) applications.

[![CI](https://github.com/duskmoon-dev/yew-duskmoon-ui/actions/workflows/ci.yml/badge.svg)](https://github.com/duskmoon-dev/yew-duskmoon-ui/actions/workflows/ci.yml)
[![yew-duskmoon crates version](https://badgen.net/crates/v/yew-duskmoon)](https://crates.io/crates/yew-duskmoon)
[![yew-duskmoon crates download](https://badgen.net/crates/d/yew-duskmoon)](https://crates.io/crates/yew-duskmoon)
[![yew-duskmoon crates latest download](https://badgen.net/crates/dl/yew-duskmoon)](https://crates.io/crates/yew-duskmoon)

## Links

- [Documentation and demo](https://duskmoon-dev.github.io/yew-duskmoon-ui/)
- [Crate](https://crates.io/crates/yew-duskmoon)
- [Repository](https://github.com/duskmoon-dev/yew-duskmoon-ui)

## Install

```toml
[dependencies]
yew = { version = "0.23", features = ["csr"] }
yew-duskmoon = "0.4"
```

The default `full` feature enables the core components:

- `app_header`
- `button`
- `card`
- `link`
- `typography`

Use explicit features when you want a smaller enabled component surface:

```toml
[dependencies]
yew-duskmoon = { version = "0.4", default-features = false, features = ["button", "card"] }
```

## CSS

The Rust components render Duskmoon class names. Include Duskmoon UI styles in the app that consumes this crate.

The demo uses Tailwind CSS 4 with `@duskmoon-dev/core`:

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

The crate includes components for actions, data display, data entry, feedback, layout, navigation, and surfaces.

Common exports include:

`Accordion`, `Alert`, `Appbar`, `Autocomplete`, `Avatar`, `Badge`, `BottomNavigation`, `Breadcrumbs`, `Button`, `Card`, `Checkbox`, `Chip`, `CodeBlock`, `DatePicker`, `Dialog`, `DmMarkdown`, `Drawer`, `FileUpload`, `Form`, `Grid`, `Input`, `Link`, `List`, `MarkdownBody`, `Menu`, `Modal`, `Pagination`, `Progress`, `Radio`, `Select`, `Switch`, `Table`, `Tabs`, `Textarea`, `ThemeController`, `Toast`, `Tooltip`, `TreeSelect`, and `Typography`.

See the [demo](https://duskmoon-dev.github.io/yew-duskmoon-ui/) for the full catalog.

## Development

Install the Rust WebAssembly target and Trunk:

```sh
rustup target add wasm32-unknown-unknown
cargo install trunk
cargo install wasm-bindgen-cli
```

Run the demo:

```sh
cd example
bun install
bun run build:css
trunk serve
```

Run crate checks from the repository root:

```sh
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-features
```
