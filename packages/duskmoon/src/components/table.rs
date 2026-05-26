use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct TableProps {
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub variant: Option<String>,
}

#[function_component(Table)]
pub fn table(props: &TableProps) -> Html {
    let mut classes = classes!("table");
    if let Some(variant) = &props.variant {
        classes.push(format!("table-{}", variant));
    }
    classes.push(props.class.clone());

    html! {
        <div class={classes}>
            { for props.children.iter() }
        </div>
    }
}
