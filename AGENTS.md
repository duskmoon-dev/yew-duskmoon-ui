# Repository Guidelines

## Project Structure & Module Organization

The root `Cargo.toml` defines a Rust workspace over `packages/*`. `packages/duskmoon/src/` contains the public Yew components; keep reusable components in focused modules and export them through `components/mod.rs` or `lib.rs`. `packages/code-engine/src/` contains the editor’s document, cursor, language, and syntax logic. The excluded `example/` crate is the Trunk demo: routes live in `example/src/routes/`, styles and the HTML shell in `example/static/`, and component demos in `example/src/routes/components/detail/pages/`. Unit tests are colocated with source under `#[cfg(test)]`. Do not commit generated `target/`, `dist/`, `node_modules/`, or `example/static/tailwind.css` output.

## Build, Test, and Development Commands

- `cargo build --workspace --all-features` builds both library crates.
- `cargo test --workspace --all-features` runs the complete Rust test suite.
- `cargo fmt --all -- --check` verifies formatting.
- `cargo clippy --workspace --all-targets --all-features -- -D warnings` matches the CI lint gate.
- `rustup target add wasm32-unknown-unknown`, `cargo install trunk`, and `cargo install wasm-bindgen-cli` install demo prerequisites; Bun is also required, and CI uses its latest release.
- `cd example && bun install && bun run build:css && trunk serve` builds CSS and serves the demo at port `3240`. For live CSS work, run `bun run watch:css` beside `trunk serve` in a second terminal. First place DuskMoon core at `<repo-root>/../duskmoonui`; `example/package.json` resolves it as `../../duskmoonui/packages/core`.

## Coding Style & Naming Conventions

Use Rust 2021 idioms and let `rustfmt` determine Rust indentation. Elsewhere, `.editorconfig` specifies UTF-8, LF endings, final newlines, and two-space indentation. Name modules, functions, fields, and tests in `snake_case`; use `PascalCase` for public types, Yew components, and property structs such as `TextareaProps`. Keep changes scoped and do not hand-edit the generated paths listed above.

## Testing Guidelines

For behavioral changes, add focused regression tests beside the changed module with names such as `adds_auto_resize_class_only_when_enabled`. During development, run a targeted command such as `cargo test -p yew-duskmoon auto_resize`; before opening a PR, run the full format, Clippy, build, and test gates above. Because `example/` is outside the workspace, validate demo changes separately with `cd example && trunk build`. No coverage percentage is currently enforced.

## Commit & Pull Request Guidelines

Follow the repository’s Conventional Commit style: `feat(textarea): ...`, `fix(markdown): ...`, `docs(code-engine): ...`, or `ci(github): ...`. PRs should include a concise summary, linked issues (`Fixes #N` when applicable), and the exact validation commands run. Add screenshots for visible demo changes. Maintainers dispatch `.github/workflows/release.yml`; contributors should not manually bump or publish workspace crates.
