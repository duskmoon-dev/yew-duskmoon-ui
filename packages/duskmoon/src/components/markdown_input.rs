use web_sys::HtmlTextAreaElement;
use yew::prelude::*;
use yew::virtual_dom::AttrValue;
use yew::TargetCast;

use super::dm_markdown::DmMarkdown;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum MarkdownInputView {
    Write,
    Preview,
}

#[derive(Properties, Clone, PartialEq)]
pub struct MarkdownInputProps {
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
    pub preview: bool,
    #[prop_or_default]
    pub variant: Option<String>,
}

#[function_component(MarkdownInput)]
pub fn markdown_input(props: &MarkdownInputProps) -> Html {
    let draft = use_state(|| props.default_value.clone());
    let active_view = use_state(|| MarkdownInputView::Write);
    let current_value = props.value.clone().unwrap_or_else(|| (*draft).clone());

    let onchange = {
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

    let show_write = {
        let active_view = active_view.clone();
        Callback::from(move |_| active_view.set(MarkdownInputView::Write))
    };
    let show_preview = {
        let active_view = active_view.clone();
        Callback::from(move |_| active_view.set(MarkdownInputView::Preview))
    };

    let mut classes = classes!("markdown-input");
    if let Some(variant) = &props.variant {
        classes.push(format!("markdown-input-{}", variant));
    }
    classes.push(props.class.clone());

    let is_write = *active_view == MarkdownInputView::Write || !props.preview;

    html! {
        <div class={classes}>
            if props.preview {
                <div class="markdown-input-tabs" role="tablist" aria-label="Markdown editor mode">
                    <button
                        type="button"
                        class={classes!("markdown-input-tab", is_write.then_some("is-active"))}
                        role="tab"
                        aria-selected={is_write.to_string()}
                        onclick={show_write}
                    >
                        { "Write" }
                    </button>
                    <button
                        type="button"
                        class={classes!("markdown-input-tab", (!is_write).then_some("is-active"))}
                        role="tab"
                        aria-selected={(!is_write).to_string()}
                        onclick={show_preview}
                    >
                        { "Preview" }
                    </button>
                </div>
            }

            if is_write {
                <textarea
                    class="markdown-input-field"
                    value={current_value.clone()}
                    placeholder={props.placeholder.clone()}
                    readonly={props.readonly}
                    aria-label="Markdown source"
                    oninput={onchange}
                />
            } else {
                <DmMarkdown class="markdown-input-preview" markdown={current_value} />
            }
        </div>
    }
}
