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

## Markdown rendering

`DmMarkdown` renders initial YAML front matter by default and adds color previews to inline code containing a complete HEX, RGB(A), or HSL(A) color value. Named colors and unrecognized CSS expressions remain ordinary inline code.

```rust
use yew::prelude::*;
use yew_duskmoon::{DmMarkdown, FrontMatterMode};

html! {
    <DmMarkdown
        markdown={"---\ntitle: Example\n---\n# Document\n\nColor: `#4C86FC`"}
        base_url={Some("/api/notes/42/attachments/".to_owned())}
        color_chips={true}
        front_matter={FrontMatterMode::Render}
    />
}
```

Relative Markdown link and image destinations are preserved by default. Set `base_url` to resolve them, plus `href` and `src` attributes in allowed raw HTML, against a directory URL; absolute, external, fragment, and query-only destinations remain unchanged.

For direct string rendering, configure the same behavior with `DmMarkdownOptions` and `render_markdown_to_html_with_options`. `FrontMatterMode::Hidden` removes initial front matter while keeping the body, and `FrontMatterMode::Disabled` parses the complete source as ordinary Markdown.

## Components

Common exports include `Accordion`, `Alert`, `Appbar`, `Autocomplete`, `Avatar`, `Badge`, `BottomNavigation`, `Breadcrumbs`, `Button`, `Card`, `Checkbox`, `Chip`, `CodeBlock`, `CodeEditor`, `DatePicker`, `Dialog`, `DmMarkdown`, `Drawer`, `FileUpload`, `Form`, `Grid`, `Input`, `Link`, `List`, `MarkdownBody`, `Menu`, `Modal`, `Pagination`, `Progress`, `Radio`, `Select`, `Switch`, `Table`, `Tabs`, `Textarea`, `ThemeController`, `Toast`, `Tooltip`, `TreeSelect`, and `Typography`.

See the [demo](https://duskmoon-dev.github.io/yew-duskmoon-ui/) for the full catalog and component examples.
