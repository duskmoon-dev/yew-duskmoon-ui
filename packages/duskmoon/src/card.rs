use yew::prelude::*;

/// Props for [`Card`]
#[derive(Properties, Clone, PartialEq)]
pub struct CardProps {
    /// CSS classes to add to the container element (optional).
    #[prop_or_default]
    pub classes: Classes,
    /// title part
    #[prop_or_default]
    pub title: Option<Html>,
    /// content part
    #[prop_or_default]
    pub children: Children,
}

/// Card component using Tailwind CSS classes
#[function_component(Card)]
pub fn card(props: &CardProps) -> Html {
    let owned_props = props.clone();

    html! {
        <div class={classes!("card", owned_props.classes)}>
            <div class="card-body">
                {
                    match owned_props.title {
                        Some(title) => {
                            html! {
                                <h2 class="card-title">
                                    { title }
                                </h2>
                            }
                        }
                        None => {
                            html! {}
                        }
                    }
                }
                { for owned_props.children.iter() }
            </div>
        </div>
    }
}
