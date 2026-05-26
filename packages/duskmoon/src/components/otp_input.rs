use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct OtpInputProps {
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub variant: Option<String>,
}

#[function_component(OtpInput)]
pub fn otp_input(props: &OtpInputProps) -> Html {
    let mut classes = classes!("otp-input");
    if let Some(variant) = &props.variant {
        classes.push(format!("otp-input-{}", variant));
    }
    classes.push(props.class.clone());

    html! {
        <div class={classes}>
            { for props.children.iter() }
        </div>
    }
}
