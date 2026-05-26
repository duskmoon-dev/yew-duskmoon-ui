use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct AutocompleteProps {
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub variant: Option<String>,
}

#[function_component(Autocomplete)]
pub fn autocomplete(props: &AutocompleteProps) -> Html {
    let mut classes = classes!("autocomplete");
    if let Some(variant) = &props.variant {
        classes.push(format!("autocomplete-{}", variant));
    }
    classes.push(props.class.clone());

    html! {
        <div class={classes}>
            { for props.children.iter() }
        </div>
    }
}
