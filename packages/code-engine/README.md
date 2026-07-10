# code-engine

`code-engine` is the pure Rust editor foundation used by `yew-duskmoon`.

The first milestone intentionally exposes a small but stable surface:

- `TextDocument` for line-aware source text operations
- `CodeLanguage` for editor language metadata
- `CursorStatus` for line and column reporting
- `highlight_tokens` for pure Rust syntax tokenization
- `CodeEditor` for a Yew editor component with controlled and uncontrolled value modes

The crate is structured to grow into a fuller editor engine over time without making
`yew-duskmoon` own the editor internals.

```rust
use yew::prelude::*;
use code_engine::{CodeEditor, CodeLanguage};

html! {
    <CodeEditor
        language={CodeLanguage::Rust}
        default_value={"fn main() {\n    println!(\"hello\");\n}"}
    />
}
```
