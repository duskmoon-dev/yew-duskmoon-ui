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
yew-duskmoon = "0.9.1"
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
yew-duskmoon = { version = "0.9.1", default-features = false, features = ["button", "card"] }
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
use yew_duskmoon::{Button, ButtonAppearance, Card, Color};

#[function_component(App)]
fn app() -> Html {
    html! {
        <Card title={html! { "Quick start" }}>
            <Button
                appearance={Some(ButtonAppearance::Filled)}
                color={Some(Color::Primary)}
            >
                { "Save" }
            </Button>
        </Card>
    }
}
```

## Data-management primitives

`Color` provides the shared core palette (`Primary`, `Secondary`, `Tertiary`, `Accent`, `Neutral`, `Base`, `Info`, `Success`, `Warning`, and `Error`) for components whose CSS contracts support it. Legacy `variant` strings and custom classes remain available as escape hatches.

Typed badge and button modifiers cover their common core-supported forms:

```rust
use yew::prelude::*;
use yew_duskmoon::{
    Badge, BadgeAppearance, BadgeSize, Button, ButtonAppearance, ButtonSize,
    Color, NativeButtonType,
};

html! {
    <>
        <Badge
            appearance={BadgeAppearance::Tonal}
            size={BadgeSize::Small}
            color={Some(Color::Neutral)}
            title="Internal account"
        >
            { "Internal" }
        </Badge>

        <Button
            native_type={NativeButtonType::Submit}
            appearance={Some(ButtonAppearance::Outlined)}
            size={ButtonSize::Small}
            color={Some(Color::Primary)}
        >
            { "Save" }
        </Button>
    </>
}
```

`BadgeAppearance` supports `Filled`, `Tonal`, and `Outlined`; `BadgeSize` supports `Small`, `Medium`, and `Large`. Badges render as `<span>` elements, accept optional `role`, `aria_label`, and `title` attributes, and do not receive a status role by default. If both `variant` and `color` are set, `variant` wins while typed appearance and size still apply.

`ButtonAppearance` supports `Filled`, `Outlined`, `Tonal`, and `Text`; `ButtonSize` supports `Small`, `Medium`, and `Large`. Native buttons default to `NativeButtonType::Button`, so they do not submit a surrounding form unless `Submit` is selected. Buttons also accept `aria_label`, `aria_describedby`, `aria_pressed`, `aria_expanded`, `title`, and `tooltip_id`. Disabled and loading controls suppress their callbacks, and loading controls expose `aria-busy="true"`.

`IconButton` renders a native `<button>` and requires a `label`, which becomes its `aria-label`. It defaults to `NativeButtonType::Button`, `ButtonAppearance::Text`, and `ButtonSize::Medium`; use `appearance`, the shared `Color` enum, and `size` for supported visual combinations, with `class` as an escape hatch. It accepts `native_type`, suppresses callbacks while `disabled` or `loading`, exposes loading state through `aria-busy`, and supports `title` as a fallback plus `tooltip_id` for a native tooltip relationship.

### Semantic tables and native tooltips

`Table` accepts semantic table children directly, optional `aria_label` and `aria_describedby` attributes, and the core responsive wrapper. `TableDensity` selects `Default`, `Compact`, or `Comfortable`; `TableBorders` selects `Default`, `Bordered`, or `Borderless`. The `hoverable`, `striped`, `sticky_header`, `selectable`, and `surface` booleans map to their matching core modifiers. Use `sticky_header` inside a scroll container where the sticky behavior has a meaningful boundary.

```rust
use yew::prelude::*;
use yew_duskmoon::{
    Badge, BadgeAppearance, BadgeSize, ButtonSize, Color, IconButton, Table,
    TableBorders, TableDensity, Tooltip, TooltipContent, TooltipPlacement,
    TooltipTone,
};

html! {
    <Table
        responsive={true}
        hoverable={true}
        density={TableDensity::Compact}
        borders={TableBorders::Bordered}
    >
        <caption>{ "Account access" }</caption>
        <thead>
            <tr>
                <th scope="col">{ "Account" }</th>
                <th scope="col">{ "Status" }</th>
                <th scope="col">{ "Actions" }</th>
            </tr>
        </thead>
        <tbody>
            <tr>
                <td>{ "Example Co." }</td>
                <td>
                    <Badge
                        appearance={BadgeAppearance::Tonal}
                        size={BadgeSize::Small}
                        color={Some(Color::Success)}
                    >
                        { "Active" }
                    </Badge>
                </td>
                <td>
                    <IconButton
                        label="Edit Example Co."
                        size={ButtonSize::Small}
                        color={Some(Color::Primary)}
                        tooltip_id="edit-example-tooltip"
                        title="Edit Example Co."
                    >
                        <span aria-hidden="true">{ "✎" }</span>
                    </IconButton>
                    <Tooltip
                        id="edit-example-tooltip"
                        placement={TooltipPlacement::Bottom}
                        content={TooltipContent::Plain}
                        tone={TooltipTone::Dark}
                    >
                        { "Edit account" }
                    </Tooltip>
                </td>
            </tr>
        </tbody>
    </Table>
}
```

Each `Tooltip` is a sibling surface with a required stable `id`. Setting a `Button` or `IconButton` `tooltip_id` to that same value supplies `interestfor`, `aria-describedby`, and the matching CSS anchor without cloning the trigger. Keep visible text or an independent `aria_label`/`label` as the trigger's accessible name; `title` is a useful fallback. Placements are `Top`, `Bottom`, `Left`, and `Right`; content forms are `Plain`, `Multiline`, and `Rich`; tones are `Dark` and `Light`, with `Color` available for palette variants. A legacy `variant` wins over `color`, which wins over `tone`. Native display relies on the Popover API, `interestfor`, and CSS Anchor Positioning, so tooltip behavior is progressive enhancement on browsers without those features.

Native tooltip overflow and sticky table-header fixes are available in `@duskmoon-dev/core` 1.18.6 and later. This Yew wrapper emits those contracts without local CSS overrides.

### Native popovers and dialogs

`Popover` requires a stable `id` and renders a command trigger next to a native `popover` surface. The default `PopoverMode::Auto` provides light-dismiss; use `Manual` with an explicit `hide-popover` command when dismissal must be controlled. The component also emits matching CSS anchor styles, so no Yew visibility state or `popover-show` class is needed.

`Dialog` renders a native `<dialog>` and requires an `id`. `Button` and `IconButton` accept `command` and `command_for`, which map to the HTML `command` and `commandfor` attributes:

```rust
use yew::prelude::*;
use yew_duskmoon::{Button, Dialog, Popover};

html! {
    <>
        <Popover id="account-menu" class="popover-bottom">
            <div class="popover-body">{ "Account options" }</div>
        </Popover>

        <Button command="show-modal" command_for="confirm-delete">{ "Delete" }</Button>
        <Dialog id="confirm-delete">
            <div class="dialog-box">
                <div class="dialog-body">{ "Delete this item?" }</div>
                <div class="dialog-footer">
                    <Button command="close" command_for="confirm-delete">{ "Cancel" }</Button>
                </div>
            </div>
        </Dialog>
    </>
}
```

`Modal` remains as a legacy name but now implements the new native dialog contract. Migrating existing `Modal` calls is required; prefer `Dialog` in new code.

## Migration notes

- `Badge` now renders a `<span>` instead of a `<div>`. Update selectors or layout assumptions that depended on the old root.
- `Button` now defaults to native `type="button"`; set `native_type={NativeButtonType::Submit}` for form submission. `ButtonType::Link` now uses the supported `btn-text` class instead of the old unsupported `btn-link`. A disabled or loading link no longer emits `disabled` or `href`, is removed from normal keyboard activation, and exposes `aria-disabled`.
- `Table` now renders a semantic `<table>`. Replace the old `<div>` row markup with `<caption>`, `<thead>`, `<tbody>`, `<tr>`, `<th>`, and `<td>` as appropriate. With `responsive={true}`, custom `class` stays on the table and `wrapper_class` customizes the `.table-responsive` wrapper.
- `Tooltip` replaces the obsolete wrapper/`.tooltip-content`/`.tooltip-open` structure with a `popover="hint"` surface. Give it a stable `id` and associate a separate trigger through `tooltip_id`.
- `Popover` no longer accepts `PopoverTrigger` or manages open state. Give it a stable `id`; the generated button uses `command="toggle-popover"`, and browser state is represented by `:popover-open`.
- `Dialog` and the legacy `Modal` name now require an `id` and render a native `<dialog>`. This is a breaking markup migration: replace `.modal-open`, old `.modal-*` children, conditional overlay wrappers, and click-state handlers with the `.dialog-*` structure and `show-modal` / `close` commands.

## Components

The crate includes components for actions, data display, data entry, feedback, layout, navigation, and surfaces.

Common exports include:

`Accordion`, `Alert`, `Appbar`, `Autocomplete`, `Avatar`, `Badge`, `BottomNavigation`, `Breadcrumbs`, `Button`, `Card`, `Checkbox`, `Chip`, `CodeBlock`, `CodeEditor`, `DatePicker`, `Dialog`, `DmMarkdown`, `Drawer`, `FileUpload`, `Form`, `Grid`, `IconButton`, `Input`, `Link`, `List`, `MarkdownBody`, `Menu`, `Modal`, `Pagination`, `Progress`, `Radio`, `Select`, `Switch`, `Table`, `Tabs`, `Textarea`, `ThemeController`, `Toast`, `Tooltip`, `TreeSelect`, and `Typography`.

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
