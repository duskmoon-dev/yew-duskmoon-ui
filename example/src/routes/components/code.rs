use yew::prelude::*;
use yew_duskmoon::{Card, Typography, CodeBlock, MarkdownBody};
use yew_duskmoon::typography::TypographyLevel;

#[function_component(CodeComponent)]
pub fn code_component() -> Html {
    html! {
        <div class="app">
            <div class="app-main w-[90%] mx-auto flex flex-col gap-6">
                <Typography level={TypographyLevel::H2}>{"Code / Markdown Components"}</Typography>
                
                <Card title={html! { <Typography level={TypographyLevel::H4}>{"CodeBlock Component"}</Typography> }}>
                    <div class="flex flex-col gap-4 w-full">
                        <CodeBlock>
                            <pre>
                                { "fn main() {\n    println!(\"Hello, Duskmoon UI!\");\n}" }
                            </pre>
                        </CodeBlock>
                        <CodeBlock variant="dark">
                            <pre>
                                { "const greet = () => {\n    console.log(\"Hello from Javascript\");\n};" }
                            </pre>
                        </CodeBlock>
                    </div>
                </Card>

                <Card title={html! { <Typography level={TypographyLevel::H4}>{"MarkdownBody Component"}</Typography> }}>
                    <div class="flex flex-col gap-4 w-full">
                        <MarkdownBody>
                            <h3>{"Markdown Heading 3"}</h3>
                            <p>{"This is an example of "}<strong>{"MarkdownBody"}</strong>{" component rendering some text with markdown classes."}</p>
                            <ul>
                                <li>{"Item 1"}</li>
                                <li>{"Item 2"}</li>
                            </ul>
                        </MarkdownBody>
                    </div>
                </Card>
            </div>
        </div>
    }
}
