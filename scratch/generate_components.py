import os
import glob

components_src_dir = "/Users/gao/Workspace/duskmoon-dev/duskmoonui/packages/core/src/components"
yew_components_dir = "/Users/gao/Workspace/duskmoon-dev/yew-duskmoon-ui/packages/duskmoon/src/components"

os.makedirs(yew_components_dir, exist_ok=True)

# Find all CSS files
css_files = glob.glob(os.path.join(components_src_dir, "*.css"))

generated_modules = []

for filepath in css_files:
    filename = os.path.basename(filepath)
    base_name = os.path.splitext(filename)[0]
    
    # Skip index.css, button.css, card.css, and file-upload.css (since we have fileupload.ts or similar, wait let's just skip button/card/index)
    if base_name in ["index", "button", "card"]:
        continue
        
    # Convert base name (e.g. bottom-navigation) to snake_case for file/module and PascalCase for Component
    module_name = base_name.replace("-", "_")
    
    # Capitalize each word for PascalCase
    words = base_name.split("-")
    pascal_name = "".join(w.capitalize() for w in words)
    
    # Specific adjustments if needed
    if pascal_name == "Fileupload":
        # Handle file-upload vs fileupload
        pascal_name = "FileUpload"
        
    rs_filename = f"{module_name}.rs"
    rs_filepath = os.path.join(yew_components_dir, rs_filename)
    
    # Yew component template
    component_content = f"""use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct {pascal_name}Props {{
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub variant: Option<String>,
}}

#[function_component({pascal_name})]
pub fn {module_name}(props: &{pascal_name}Props) -> Html {{
    let mut classes = classes!("{base_name}");
    if let Some(variant) = &props.variant {{
        classes.push(format!("{base_name}-{{}}", variant));
    }}
    classes.push(props.class.clone());

    html! {{
        <div class={{classes}}>
            {{ for props.children.iter() }}
        </div>
    }}
}}
"""
    with open(rs_filepath, "w") as f:
        f.write(component_content)
        
    generated_modules.append((module_name, pascal_name))

# Sort modules
generated_modules.sort()

# Generate mod.rs
mod_filepath = os.path.join(yew_components_dir, "mod.rs")
mod_content = ""
for module_name, pascal_name in generated_modules:
    mod_content += f"pub mod {module_name};\n"
    mod_content += f"pub use {module_name}::{pascal_name};\n\n"

with open(mod_filepath, "w") as f:
    f.write(mod_content)

print(f"Successfully generated {len(generated_modules)} components in {yew_components_dir}!")
