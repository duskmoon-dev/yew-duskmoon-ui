use yew::prelude::*;
use yew_duskmoon::typography::TypographyLevel;
use yew_duskmoon::{Alert, Button, Card, Dialog, Toast, Typography};

use super::palette::{variant, PALETTE};

#[function_component(FeedbackComponent)]
pub fn feedback_component() -> Html {
    let show_toast = use_state(|| false);
    let trigger_toast = {
        let show_toast = show_toast.clone();
        Callback::from(move |_| show_toast.set(true))
    };
    let hide_toast = {
        let show_toast = show_toast.clone();
        Callback::from(move |_| show_toast.set(false))
    };

    html! {
        <div class="app">
            <div class="app-main component-main">
                <Typography level={TypographyLevel::H2}>{"Feedback Components"}</Typography>

                <Card title={html! { <Typography level={TypographyLevel::H4}>{"Alerts"}</Typography> }} classes="component-card">
                    <div class="color-grid alert-color-grid">
                        { for PALETTE.into_iter().map(|color| html! {
                            <Alert variant={variant(color)} class="alert-compact">
                                <span class="alert-icon">{ "•" }</span>
                                <span class="alert-content">
                                    <strong class="alert-title">{ color.label }</strong>
                                    <span class="alert-description">{ format!("alert-{}", color.key) }</span>
                                </span>
                            </Alert>
                        }) }
                    </div>
                </Card>

                <Card title={html! { <Typography level={TypographyLevel::H4}>{"Toast Palette"}</Typography> }} classes="component-card">
                    <div class="color-grid toast-color-grid">
                        { for PALETTE.into_iter().map(|color| html! {
                            <Toast variant={variant(color)} class="toast-show toast-compact">
                                <span class="toast-icon">{ "●" }</span>
                                <span class="toast-content">
                                    <strong class="toast-title">{ color.label }</strong>
                                    <span class="toast-message">{ format!("toast-{}", color.key) }</span>
                                </span>
                            </Toast>
                        }) }
                    </div>
                </Card>

                <Card title={html! { <Typography level={TypographyLevel::H4}>{"Native Dialog"}</Typography> }} classes="component-card">
                    <div class="flex flex-col gap-4 w-full">
                        <Typography level={TypographyLevel::Default}>
                            {"Open the modal dialog with a declarative HTML command."}
                        </Typography>
                        <div>
                            <Button
                                command="show-modal"
                                command_for="feedback-dialog"
                                variant={Some("primary".to_owned())}
                            >
                                {"Open Dialog"}
                            </Button>
                        </div>
                    </div>
                </Card>

                <Card title={html! { <Typography level={TypographyLevel::H4}>{"Toast Notification"}</Typography> }} classes="component-card">
                    <div class="flex flex-col gap-4 w-full">
                        <Typography level={TypographyLevel::Default}>
                            {"Click the button below to show a toast message."}
                        </Typography>
                        <div>
                            <Button onclick={trigger_toast} variant={Some("secondary".to_owned())}>{"Show Toast"}</Button>
                        </div>
                    </div>
                </Card>

                <Dialog id="feedback-dialog">
                    <div class="dialog-box">
                        <div class="dialog-header">
                            <Typography level={TypographyLevel::H4} classes="dialog-title">{"Interactive Dialog"}</Typography>
                        </div>
                        <div class="dialog-body">
                            <Typography level={TypographyLevel::Default}>
                                {"The native dialog owns the backdrop, focus handling, Escape key, and top-layer visibility."}
                            </Typography>
                        </div>
                        <div class="dialog-footer">
                            <Button
                                command="close"
                                command_for="feedback-dialog"
                                variant={Some("tertiary".to_owned())}
                            >
                                {"Close Dialog"}
                            </Button>
                        </div>
                    </div>
                </Dialog>

                // Toast Rendering
                {
                    if *show_toast {
                        html! {
                            <div class="fixed bottom-4 right-4 z-50">
                                <Toast variant={Some("success".to_string())} class="toast-show toast-filled">
                                    <span class="toast-icon">{ "✓" }</span>
                                    <span class="toast-content">
                                        <Typography level={TypographyLevel::Default}>{"Notification message triggered!"}</Typography>
                                    </span>
                                    <Button onclick={hide_toast} classes="btn btn-sm btn-ghost">{"Dismiss"}</Button>
                                </Toast>
                            </div>
                        }
                    } else {
                        html! {}
                    }
                }
            </div>
        </div>
    }
}
