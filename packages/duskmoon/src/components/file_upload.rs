use yew::prelude::*;
use yew::virtual_dom::AttrValue;

use super::variants;

#[derive(Properties, Clone, PartialEq)]
pub struct FileUploadProps {
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub variant: Option<String>,
    #[prop_or_default]
    pub id: AttrValue,
    #[prop_or_default]
    pub name: AttrValue,
    #[prop_or_default]
    pub accept: AttrValue,
    #[prop_or_default]
    pub multiple: bool,
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or(AttrValue::from("Upload files"))]
    pub title: AttrValue,
    #[prop_or(AttrValue::from("Drag files here or"))]
    pub subtitle: AttrValue,
    #[prop_or(AttrValue::from("browse from your device"))]
    pub browse_label: AttrValue,
    #[prop_or_default]
    pub onchange: Callback<Event>,
}

#[function_component(FileUpload)]
pub fn file_upload(props: &FileUploadProps) -> Html {
    let mut classes = classes!("file-upload");
    if let Some(variant) = &props.variant {
        classes.push(format!("file-upload-{}", variant));
    }
    classes.push(props.class.clone());

    let mut dropzone_classes = classes!("file-upload-dropzone");
    if props.disabled {
        dropzone_classes.push("disabled");
    }

    let dropzone_style = variants::style(
        props.variant.as_deref(),
        "color: var(--component-color, var(--color-primary)); border-color: var(--component-color, var(--color-primary)); background: color-mix(in oklch, var(--component-container, var(--color-surface-container)) 36%, var(--color-surface));",
    );
    let title_style = variants::style(
        props.variant.as_deref(),
        "color: var(--component-color, var(--color-primary));",
    );
    let subtitle_style = variants::style(
        props.variant.as_deref(),
        "color: color-mix(in oklch, var(--component-color, var(--dm-paper)) 58%, var(--dm-paper));",
    );

    html! {
        <div class={classes}>
            <label class={dropzone_classes} style={dropzone_style}>
                <input
                    id={optional_attr(&props.id)}
                    class="file-upload-input"
                    type="file"
                    name={props.name.clone()}
                    accept={props.accept.clone()}
                    multiple={props.multiple}
                    disabled={props.disabled}
                    onchange={props.onchange.clone()}
                />
                <span class="file-upload-icon" aria-hidden="true">{ "File" }</span>
                <div class="file-upload-text">
                    {
                        if props.children.is_empty() {
                            html! {
                                <>
                                    <div class="file-upload-title" style={title_style}>{ props.title.clone() }</div>
                                    <div class="file-upload-subtitle" style={subtitle_style}>
                                        { props.subtitle.clone() }
                                        {
                                            if props.browse_label.is_empty() {
                                                html! {}
                                            } else {
                                                html! {
                                                    <>
                                                        { " " }
                                                        <span class="file-upload-browse">{ props.browse_label.clone() }</span>
                                                    </>
                                                }
                                            }
                                        }
                                    </div>
                                </>
                            }
                        } else {
                            html! { <>{ for props.children.iter() }</> }
                        }
                    }
                </div>
            </label>
        </div>
    }
}

fn optional_attr(value: &AttrValue) -> Option<AttrValue> {
    if value.is_empty() {
        None
    } else {
        Some(value.clone())
    }
}
