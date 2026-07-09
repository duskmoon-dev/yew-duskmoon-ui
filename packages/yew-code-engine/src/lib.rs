mod document;
mod language;

use web_sys::HtmlTextAreaElement;
use yew::prelude::*;
use yew::virtual_dom::AttrValue;
use yew::TargetCast;

pub use document::TextDocument;
pub use language::CodeLanguage;

#[derive(Properties, Clone, PartialEq)]
pub struct CodeEditorProps {
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub value: Option<AttrValue>,
    #[prop_or_default]
    pub default_value: AttrValue,
    #[prop_or_default]
    pub placeholder: AttrValue,
    #[prop_or_default]
    pub on_change: Callback<AttrValue>,
    #[prop_or_default]
    pub readonly: bool,
    #[prop_or(true)]
    pub show_line_numbers: bool,
    #[prop_or(CodeLanguage::PlainText)]
    pub language: CodeLanguage,
    #[prop_or(12)]
    pub rows: usize,
    #[prop_or_default]
    pub variant: Option<String>,
    #[prop_or_else(|| AttrValue::from("Code editor"))]
    pub aria_label: AttrValue,
}

#[function_component(CodeEditor)]
pub fn code_editor(props: &CodeEditorProps) -> Html {
    let draft = use_state(|| props.default_value.clone());
    let current_value = props.value.clone().unwrap_or_else(|| (*draft).clone());
    let document = TextDocument::new(current_value.to_string());
    let line_count = document.line_count();

    let oninput = {
        let draft = draft.clone();
        let on_change = props.on_change.clone();
        let controlled = props.value.is_some();

        Callback::from(move |event: InputEvent| {
            let textarea: HtmlTextAreaElement = event.target_unchecked_into();
            let next = AttrValue::from(textarea.value());
            if !controlled {
                draft.set(next.clone());
            }
            on_change.emit(next);
        })
    };

    let mut classes = classes!(
        "code-engine",
        format!("code-engine-language-{}", props.language.as_str()),
        props.readonly.then_some("is-readonly")
    );
    if let Some(variant) = &props.variant {
        classes.push(format!("code-engine-{}", variant));
    }
    classes.push(props.class.clone());

    let body_classes = classes!(
        "code-engine-body",
        (!props.show_line_numbers).then_some("without-line-numbers")
    );

    html! {
        <div
            class={classes}
            data-language={props.language.as_str()}
            style={format!("--code-engine-rows: {}", props.rows.max(1))}
        >
            <div class="code-engine-header">
                <span class="code-engine-language">{ props.language.label() }</span>
            </div>
            <div class={body_classes}>
                if props.show_line_numbers {
                    <div class="code-engine-gutter" aria-hidden="true">
                        { for (1..=line_count).map(|line| html! {
                            <span class="code-engine-line-number">{ line }</span>
                        }) }
                    </div>
                }
                <textarea
                    class="code-engine-input"
                    value={current_value}
                    placeholder={props.placeholder.clone()}
                    readonly={props.readonly}
                    spellcheck="false"
                    rows={props.rows.max(1).to_string()}
                    aria-label={props.aria_label.clone()}
                    oninput={oninput}
                />
            </div>
        </div>
    }
}
