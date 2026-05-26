use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct FileUploadProps {
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub variant: Option<String>,
}

#[function_component(FileUpload)]
pub fn file_upload(props: &FileUploadProps) -> Html {
    let mut classes = classes!("file-upload");
    if let Some(variant) = &props.variant {
        classes.push(format!("file-upload-{}", variant));
    }
    classes.push(props.class.clone());

    html! {
        <div class={classes}>
            { for props.children.iter() }
        </div>
    }
}
