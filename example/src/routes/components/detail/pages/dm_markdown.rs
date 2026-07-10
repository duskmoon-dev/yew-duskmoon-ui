use crate::routes::components::catalog::ComponentSpec;
use crate::routes::components::detail::page::{primary_variant, ApiRow, ComponentPage};
use crate::routes::components::palette::{variant, PaletteColor};
use yew::prelude::*;
use yew_duskmoon::DmMarkdown;

const DM_MARKDOWN_API: &[ApiRow] = &[
    ApiRow {
        prop: "class",
        ty: "Classes",
        default: "empty",
        docs: "Extra CSS classes appended to the markdown root.",
    },
    ApiRow {
        prop: "allow_html",
        ty: "bool",
        default: "true",
        docs: "Renders safe raw HTML while escaping style, script, and object tags.",
    },
    ApiRow {
        prop: "custom_elements",
        ty: "Vec<String>",
        default: "empty",
        docs: "Allowed custom element tag names. Empty disables custom elements.",
    },
    ApiRow {
        prop: "markdown",
        ty: "AttrValue",
        default: "empty",
        docs: "Markdown source rendered into HTML.",
    },
    ApiRow {
        prop: "variant",
        ty: "Option<String>",
        default: "None",
        docs: "Appends a color modifier class such as markdown-body-primary.",
    },
];

const SAMPLE_MARKDOWN: &str = r#"# DmMarkdown rendering

Dm Markdown renders source text into the same typography scope as `MarkdownBody`, with highlighted fenced code and Mermaid diagrams.

## Raw HTML

<aside><strong>Safe HTML:</strong> raw tags render by default.</aside>

<el-dm-note>Custom elements are escaped unless their tag name is allowed.</el-dm-note>

<script>alert("blocked")</script>

## Mermaid diagrams

```mermaid
flowchart LR
  Source[Markdown Source] --> Renderer[DmMarkdown]
  Renderer --> Html[Highlighted HTML]
  Renderer --> Chart[Mermaid Chart]
```

```mermaid
swimlane-beta LR
  subgraph Customer
    request[Request service]
    update[Receive update]
  end
  subgraph Support
    triage[Triage request]
    answer[Send answer]
  end
  subgraph Engineering
    investigate[Investigate issue]
    fix[Prepare fix]
  end
  request --> triage
  triage -->|Known issue| answer
  triage -->|Needs code change| investigate
  investigate --> fix
  fix --> answer
  answer --> update
```

```mermaid
sequenceDiagram
  participant Author
  participant Renderer
  Author->>Renderer: Submit markdown
  Renderer-->>Author: Return safe HTML
```

```mermaid
classDiagram
  namespace Company {
    class CEO {
      +makeDecisions()
    }
  }
  namespace Company.Engineering.Backend {
    class Developer {
      +writeCode()
    }
  }
  namespace Company.Engineering.Frontend {
    class Designer {
      +createMockup()
    }
  }
  CEO --> Developer : oversees
  CEO --> Designer : oversees
```

```mermaid
stateDiagram-v2
  [*] --> Still
  Still --> Moving
  Moving --> Crash
  Crash --> [*]
  Still --> [*]
  Moving --> Still
```

```mermaid
erDiagram
  CUSTOMER ||--o{ ORDER : places
  CUSTOMER ||--o{ DELIVERY_ADDRESS : uses
  ORDER ||--|{ LINE_ITEM : contains
```

```mermaid
journey
  title My working day
  section Morning
    Go to work: 5: Me
    Make tea: 5: Cat, Me
    Go upstairs: 3: Me
  section Work
    Do work: 2: Cat, Me
  section Evening
    Go downstairs: 5: Me
    Go home: 5: Me
    Sit down: 5: Me
```

```mermaid
gantt
  title A Gantt Diagram
  dateFormat  YYYY-MM-DD
  axisFormat  %Y-%m-%d
  section Section
  A task :2014-01-01, 2014-01-31
  Another task :2014-01-31, 2014-02-18
  section Another
  Task in Another :2014-01-12, 2014-01-24
  another task :2014-01-24, 2014-02-10
```

```mermaid
pie showData
  title Demo content
  "Code" : 42
  "Mermaid" : 34
  "Markdown" : 24
```

```mermaid
quadrantChart
  title Documentation value
  x-axis Low effort --> High effort
  y-axis Low impact --> High impact
  "Markdown": [0.30, 0.65]
  "Mermaid": [0.58, 0.82]
  "Highlight": [0.42, 0.74]
```

```mermaid
requirementDiagram
  requirement safe_render {
    id: DM-1
    text: Strip unsafe raw HTML
    risk: high
    verifymethod: test
  }
```

```mermaid
gitGraph
  commit id: "markdown"
  branch demo
  checkout demo
  commit id: "mermaid"
```

```mermaid
C4Context
  title Markdown rendering context
  Person(author, "Author", "Writes docs")
  System(renderer, "DmMarkdown", "Renders safe HTML")
  Rel(author, renderer, "submits markdown")
```

```mermaid
mindmap
  root((DmMarkdown))
    Markdown
      Tables
      Task lists
    Code
      Rust
      TypeScript
    Mermaid
      Flowchart
      Sequence
```

```mermaid
timeline
  title Render path
  Source : Markdown arrives
  Parse : Events are sanitized
  Output : HTML is emitted
```

```mermaid
zenuml
  Author->Renderer: markdown
  Renderer->Author: html
```

```mermaid
sankey-beta
  Markdown,Parser,40
  Parser,Highlighter,22
  Parser,Mermaid,18
  Highlighter,HTML,22
  Mermaid,HTML,18
```

```mermaid
xychart-beta
  title "Render coverage"
  x-axis [Markdown, Code, Mermaid]
  y-axis "Coverage" 0 --> 100
  bar [92, 88, 76]
```

```mermaid
block-beta
  columns 3
  source["Source"] parser["Parser"] html["HTML"]
  source --> parser
  parser --> html
```

```mermaid
packet-beta
  title Markdown payload
  0-7: "type"
  8-31: "content length"
  32-63: "source bytes"
```

```mermaid
kanban
  Todo
    Add docs
  Doing
    Render diagrams
  Done
    Highlight code
```

```mermaid
architecture-beta
  group app(cloud)[Example app]
  service markdown(server)[DmMarkdown] in app
  service browser(internet)[Browser] in app
  browser:R -- L:markdown
```

```mermaid
radar-beta
  title Renderer qualities
  axis Safety, Coverage, Readability, Speed
  curve Demo{90, 80, 88, 74}
```

```mermaid
eventModeling
  event MarkdownSubmitted
  command RenderMarkdown
  view HtmlPreview
```

```mermaid
treemap-beta
  "DmMarkdown"
    "Markdown": 32
    "Code": 26
    "Mermaid": 42
```

```mermaid
venn
  Markdown: 40
  Mermaid: 35
  Markdown & Mermaid: 15
```

```mermaid
ishikawa
  root((Renderer quality))
    Safety
      Escaped HTML
    Readability
      Highlighted code
    Coverage
      Mermaid diagrams
```

```mermaid
wardley
  title Documentation map
  anchor User [0.95, 0.65]
  component Markdown [0.65, 0.55]
  component Mermaid [0.48, 0.44]
  User->Markdown
```

```mermaid
cynefin
  title Rendering decisions
  Simple: Escape HTML
  Complicated: Highlight syntax
  Complex: Diagram layout
```

```mermaid
treeview
  root
    markdown
      code
      mermaid
    html
```

## Language fences

```elixir
defmodule Demo.Greeting do
  def hello(name) do
    "hello #{name}"
  end
end
```

```go
package main

func hello(name string) string {
    return "hello " + name
}
```

```rust
pub fn hello(name: &str) -> String {
    let message = format!("hello {name}");
    message
}
```

```zig
const std = @import("std");

pub fn hello(writer: anytype) !void {
    try writer.print("hello {s}", .{"moon"});
}
```

```typescript
type User = {
  readonly name: string;
};

export function hello(user: User): string {
  return `hello ${user.name}`;
}
```

- [x] Parse Markdown
- [x] Highlight code fences
- [x] Render Mermaid chart blocks
"#;

pub fn page(spec: &'static ComponentSpec) -> ComponentPage {
    ComponentPage::new(spec, usage, DM_MARKDOWN_API, demo, color_variant)
}

fn usage(_: &ComponentSpec) -> String {
    "use yew_duskmoon::DmMarkdown;\n\nhtml! {\n    <DmMarkdown markdown={\"# Demo\\n\\n```rust\\nfn main() {}\\n```\"} />\n}".to_owned()
}

fn demo(_: &ComponentSpec) -> Html {
    html! {
        <DmMarkdown
            variant={primary_variant()}
            class="component-detail-demo-control"
            markdown={SAMPLE_MARKDOWN}
        />
    }
}

fn color_variant(color: PaletteColor) -> Html {
    html! {
        <DmMarkdown
            variant={variant(color)}
            class="component-detail-color-demo"
            markdown={format!("**{}** markdown renderer\n\n`markdown-body-{}`", color.label, color.key)}
        />
    }
}
