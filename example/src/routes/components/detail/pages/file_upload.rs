use crate::routes::components::catalog::ComponentSpec;
use crate::routes::components::detail::page::{primary_variant, ApiRow, ComponentPage};
use crate::routes::components::palette::{variant, PaletteColor};
use yew::prelude::*;
use yew_duskmoon::FileUpload;

const FILE_UPLOAD_API: &[ApiRow] = &[
    ApiRow {
        prop: "class",
        ty: "Classes",
        default: "empty",
        docs: "Extra CSS classes appended to the file upload root.",
    },
    ApiRow {
        prop: "children",
        ty: "Children",
        default: "empty",
        docs: "Optional custom dropzone content.",
    },
    ApiRow {
        prop: "variant",
        ty: "Option<String>",
        default: "None",
        docs: "Applies the file-upload color variant.",
    },
    ApiRow {
        prop: "id",
        ty: "AttrValue",
        default: "empty",
        docs: "Optional id forwarded to the native file input.",
    },
    ApiRow {
        prop: "name",
        ty: "AttrValue",
        default: "empty",
        docs: "Optional name forwarded to the native file input.",
    },
    ApiRow {
        prop: "accept",
        ty: "AttrValue",
        default: "empty",
        docs: "File input accept filter.",
    },
    ApiRow {
        prop: "multiple",
        ty: "bool",
        default: "false",
        docs: "Allows selecting multiple files.",
    },
    ApiRow {
        prop: "disabled",
        ty: "bool",
        default: "false",
        docs: "Disables the file input.",
    },
    ApiRow {
        prop: "title",
        ty: "AttrValue",
        default: "Upload files",
        docs: "Primary dropzone label.",
    },
    ApiRow {
        prop: "subtitle",
        ty: "AttrValue",
        default: "Drag files here or",
        docs: "Secondary dropzone guidance.",
    },
    ApiRow {
        prop: "browse_label",
        ty: "AttrValue",
        default: "browse from your device",
        docs: "Inline browse action label appended to the subtitle.",
    },
    ApiRow {
        prop: "onchange",
        ty: "Callback<Event>",
        default: "noop",
        docs: "Native file input change handler.",
    },
];

pub fn page(spec: &'static ComponentSpec) -> ComponentPage {
    ComponentPage::new(spec, usage, FILE_UPLOAD_API, demo, color_variant)
}

fn usage(_: &ComponentSpec) -> String {
    "use yew_duskmoon::FileUpload;\n\nhtml! {\n    <FileUpload\n        id=\"asset-upload\"\n        name=\"assets\"\n        variant={Some(\"primary\".to_owned())}\n        accept=\"image/*,.pdf\"\n        multiple={true}\n        title=\"Upload project assets\"\n        subtitle=\"Drop images or PDFs here or\"\n        browse_label=\"browse files\"\n    />\n}".to_owned()
}

fn demo(_: &ComponentSpec) -> Html {
    html! {
        <div class="detail-demo-stack">
            <FileUpload
                id="component-detail-file-upload"
                name="component-detail-assets"
                variant={primary_variant()}
                class="component-detail-file-upload-demo"
                accept="image/*,.pdf"
                multiple={true}
                title="Upload project assets"
                subtitle="Drop images or PDFs here or"
                browse_label="browse files"
            />
            <div class="file-upload-list" aria-label="Example selected files">
                <div class="file-upload-item">
                    <div class="file-upload-item-info">
                        <span class="file-upload-item-name">{ "hero-cover.png" }</span>
                        <span class="file-upload-item-size">{ "1.8 MB" }</span>
                    </div>
                </div>
                <div class="file-upload-item">
                    <div class="file-upload-item-info">
                        <span class="file-upload-item-name">{ "release-notes.pdf" }</span>
                        <span class="file-upload-item-size">{ "420 KB" }</span>
                    </div>
                </div>
            </div>
        </div>
    }
}

fn color_variant(color: PaletteColor) -> Html {
    html! {
        <FileUpload
            variant={variant(color)}
            class="component-detail-color-file-upload"
            title={color.label}
            subtitle={format!("file-upload-{}", color.key)}
            browse_label=""
        />
    }
}
