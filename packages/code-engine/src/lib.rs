mod cursor;
mod document;
mod language;
mod syntax;

use web_sys::{Event, FocusEvent, HtmlElement, HtmlTextAreaElement, KeyboardEvent, MouseEvent};
use yew::prelude::*;
use yew::virtual_dom::AttrValue;
use yew::TargetCast;

pub use cursor::{CursorPosition, CursorStatus};
pub use document::TextDocument;
pub use language::CodeLanguage;
pub use syntax::{highlight_tokens, SyntaxToken, SyntaxTokenKind};

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
    #[prop_or(true)]
    pub show_status_bar: bool,
    #[prop_or(true)]
    pub syntax_highlight: bool,
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
    let source = current_value.to_string();
    let document = TextDocument::new(source.clone());
    let line_count = document.line_count();
    let cursor_status = use_state(CursorStatus::default);
    let scroll_position = use_state(|| (0, 0));
    let is_focused = use_state(|| false);
    let highlight_ref = use_node_ref();
    let gutter_ref = use_node_ref();

    let oninput = {
        let draft = draft.clone();
        let on_change = props.on_change.clone();
        let controlled = props.value.is_some();
        let cursor_status = cursor_status.clone();

        Callback::from(move |event: InputEvent| {
            let textarea: HtmlTextAreaElement = event.target_unchecked_into();
            let next = AttrValue::from(textarea.value());
            cursor_status.set(CursorStatus::from_textarea(&textarea));
            if !controlled {
                draft.set(next.clone());
            }
            on_change.emit(next);
        })
    };

    let onscroll = {
        let highlight_ref = highlight_ref.clone();
        let gutter_ref = gutter_ref.clone();
        let scroll_position = scroll_position.clone();

        Callback::from(move |event: Event| {
            let textarea: HtmlTextAreaElement = event.target_unchecked_into();
            let scroll_top = textarea.scroll_top();
            let scroll_left = textarea.scroll_left();
            scroll_position.set((scroll_top, scroll_left));

            if let Some(highlight) = highlight_ref.cast::<HtmlElement>() {
                highlight.set_scroll_top(scroll_top);
                highlight.set_scroll_left(scroll_left);
            }

            if let Some(gutter) = gutter_ref.cast::<HtmlElement>() {
                gutter.set_scroll_top(scroll_top);
            }
        })
    };

    let onselect = {
        let cursor_status = cursor_status.clone();
        Callback::from(move |event: Event| {
            let textarea: HtmlTextAreaElement = event.target_unchecked_into();
            cursor_status.set(CursorStatus::from_textarea(&textarea));
        })
    };
    let onkeyup = {
        let cursor_status = cursor_status.clone();
        Callback::from(move |event: KeyboardEvent| {
            let textarea: HtmlTextAreaElement = event.target_unchecked_into();
            cursor_status.set(CursorStatus::from_textarea(&textarea));
        })
    };
    let onclick = {
        let cursor_status = cursor_status.clone();
        Callback::from(move |event: MouseEvent| {
            let textarea: HtmlTextAreaElement = event.target_unchecked_into();
            cursor_status.set(CursorStatus::from_textarea(&textarea));
        })
    };
    let onfocus = {
        let cursor_status = cursor_status.clone();
        let is_focused = is_focused.clone();
        Callback::from(move |event: FocusEvent| {
            let textarea: HtmlTextAreaElement = event.target_unchecked_into();
            is_focused.set(true);
            cursor_status.set(CursorStatus::from_textarea(&textarea));
        })
    };
    let onblur = {
        let is_focused = is_focused.clone();
        Callback::from(move |_event: FocusEvent| {
            is_focused.set(false);
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
    if props.syntax_highlight {
        classes.push("is-highlighted");
    }
    classes.push("has-block-cursor");
    classes.push(props.class.clone());

    let body_classes = classes!(
        "code-engine-body",
        (!props.show_line_numbers).then_some("without-line-numbers")
    );
    let body_style = if props.show_line_numbers {
        "display: grid; grid-template-columns: minmax(48px, auto) minmax(0, 1fr); min-width: 0;"
    } else {
        "display: grid; grid-template-columns: minmax(0, 1fr); min-width: 0;"
    };
    let row_count = props.rows.max(1);
    let highlight_enabled = props.syntax_highlight && !source.is_empty();
    let input_style = if highlight_enabled {
        "position: relative; z-index: 1; display: block; width: 100%; min-width: 0; min-height: calc((var(--code-engine-rows, 12) * 1.55em) + 32px); box-sizing: border-box; padding: 16px; border: 0; outline: 0; resize: vertical; overflow: auto; color: transparent; -webkit-text-fill-color: transparent; caret-color: transparent; background: transparent; font-family: var(--font-mono, ui-monospace, SFMono-Regular, Menlo, monospace); font-size: 0.95rem; line-height: 1.55; tab-size: 4; white-space: pre;"
    } else {
        "position: relative; z-index: 1; display: block; width: 100%; min-width: 0; min-height: calc((var(--code-engine-rows, 12) * 1.55em) + 32px); box-sizing: border-box; padding: 16px; border: 0; outline: 0; resize: vertical; overflow: auto; color: var(--code-token-plain, #1f2937); caret-color: transparent; background: transparent; font-family: var(--font-mono, ui-monospace, SFMono-Regular, Menlo, monospace); font-size: 0.95rem; line-height: 1.55; tab-size: 4; white-space: pre;"
    };
    let highlight_tokens = highlight_tokens(props.language, &source);
    let selected_units = cursor_status.selected_units;
    let selected_label = if selected_units == 0 {
        String::new()
    } else {
        format!(" · {selected_units} selected")
    };
    let (scroll_top, scroll_left) = *scroll_position;
    let cursor_left = cursor_status.visual_column.saturating_sub(1);
    let cursor_top = cursor_status.position.line.saturating_sub(1) as f32 * 1.55;
    let cursor_opacity = if *is_focused { "0.88" } else { "0.58" };
    let cursor_visible = !props.readonly && selected_units == 0;
    let cursor_style = format!(
        "position: absolute; z-index: 2; left: calc(16px + {cursor_left}ch - {scroll_left}px); top: calc(16px + {cursor_top:.3}em - {scroll_top}px); width: 1ch; height: 1.55em; box-sizing: border-box; border: 1px solid color-mix(in oklch, var(--code-editor-caret, #111827) 82%, transparent); border-radius: 2px; opacity: {cursor_opacity}; background: color-mix(in oklch, var(--code-editor-caret, #111827) 70%, transparent); pointer-events: none; mix-blend-mode: multiply;"
    );

    html! {
        <div
            class={classes}
            data-language={props.language.as_str()}
            style={format!("--code-engine-rows: {row_count}; --code-editor-caret: #111827; --code-token-plain: #1f2937; --code-token-keyword: #d97706; --code-token-type: #7c3aed; --code-token-string: #047857; --code-token-number: #dc2626; --code-token-comment: #64748b; --code-token-function: #2563eb; --code-token-punctuation: #6b7280; display: flex; width: min(100%, 860px); min-width: 0; flex-direction: column; overflow: hidden;")}
        >
            <div
                class="code-engine-header"
                style="display: flex; align-items: center; justify-content: flex-end; min-width: 0;"
            >
                <span class="code-engine-language">{ props.language.label() }</span>
            </div>
            <div class={body_classes} style={body_style}>
                if props.show_line_numbers {
                    <div
                        ref={gutter_ref}
                        class="code-engine-gutter"
                        aria-hidden="true"
                        style="display: flex; min-width: 48px; max-height: calc((var(--code-engine-rows, 12) * 1.55em) + 32px); overflow: hidden; flex-direction: column; align-items: flex-end; box-sizing: border-box;"
                    >
                        { for (1..=line_count).map(|line| html! {
                            <span
                                class="code-engine-line-number"
                                style="display: block; min-height: 1.55em; font-variant-numeric: tabular-nums;"
                            >
                                { line }
                            </span>
                        }) }
                    </div>
                }
                <div
                    class="code-engine-editor"
                    style="position: relative; min-width: 0; min-height: calc((var(--code-engine-rows, 12) * 1.55em) + 32px); overflow: hidden;"
                >
                    if highlight_enabled {
                        <pre
                            ref={highlight_ref}
                            class="code-engine-highlight"
                            aria-hidden="true"
                            style="position: absolute; inset: 0; z-index: 0; box-sizing: border-box; margin: 0; min-height: calc((var(--code-engine-rows, 12) * 1.55em) + 32px); padding: 16px; overflow: hidden; border: 0; color: var(--code-token-plain, #1f2937); background: transparent; box-shadow: none; font-family: var(--font-mono, ui-monospace, SFMono-Regular, Menlo, monospace); font-size: 0.95rem; line-height: 1.55; tab-size: 4; white-space: pre; pointer-events: none;"
                        >
                            <code
                                class="code-engine-highlight-code"
                                style="display: block; padding: 0; border: 0; color: inherit; background: transparent; box-shadow: none; font: inherit; white-space: pre;"
                            >
                                { for highlight_tokens.iter().map(render_syntax_token) }
                            </code>
                        </pre>
                    }
                    <textarea
                        class="code-engine-input"
                        value={current_value}
                        placeholder={props.placeholder.clone()}
                        readonly={props.readonly}
                        spellcheck="false"
                        rows={row_count.to_string()}
                        wrap="off"
                        aria-label={props.aria_label.clone()}
                        style={input_style}
                        oninput={oninput}
                        onscroll={onscroll}
                        onselect={onselect}
                        onkeyup={onkeyup}
                        onclick={onclick}
                        onfocus={onfocus}
                        onblur={onblur}
                    />
                    if cursor_visible {
                        <span
                            class="code-engine-cursor code-engine-cursor-block"
                            aria-hidden="true"
                            style={cursor_style}
                        />
                    }
                </div>
            </div>
            if props.show_status_bar {
                <div
                    class="code-engine-statusbar"
                    style="display: flex; min-height: 34px; align-items: center; justify-content: space-between; gap: 12px; padding: 0 12px; border-top: 1px solid var(--dm-line, rgba(148, 163, 184, 0.24)); color: var(--dm-paper-muted, #94a3b8); background: color-mix(in oklch, var(--color-surface-container-high, #0f172a) 72%, transparent); font-family: var(--font-mono, ui-monospace, SFMono-Regular, Menlo, monospace); font-size: 0.78rem;"
                >
                    <span class="code-engine-status-position">
                        { format!("Ln {}, Col {}{}", cursor_status.position.line, cursor_status.position.column, selected_label) }
                    </span>
                    <span class="code-engine-status-meta">
                        { format!("{} · {} lines", props.language.label(), line_count) }
                    </span>
                </div>
            }
        </div>
    }
}

fn render_syntax_token(token: &SyntaxToken) -> Html {
    html! {
        <span class={token.kind.class_name()} style={token.kind.style()}>
            { token.text.clone() }
        </span>
    }
}
