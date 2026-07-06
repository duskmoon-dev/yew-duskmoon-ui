use yew::prelude::*;
use yew_duskmoon::typography::TypographyLevel;
use yew_duskmoon::{Alert, Button, Card, Modal, Toast, Typography};

use super::palette::{variant, PALETTE};

#[function_component(FeedbackComponent)]
pub fn feedback_component() -> Html {
    let show_modal = use_state(|| false);
    let open_modal = {
        let show_modal = show_modal.clone();
        Callback::from(move |_| show_modal.set(true))
    };
    let close_modal = {
        let show_modal = show_modal.clone();
        Callback::from(move |_| show_modal.set(false))
    };

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

                <Card title={html! { <Typography level={TypographyLevel::H4}>{"Modal Overlay"}</Typography> }} classes="component-card">
                    <div class="flex flex-col gap-4 w-full">
                        <Typography level={TypographyLevel::Default}>
                            {"Click the button below to toggle the modal overlay."}
                        </Typography>
                        <div>
                            <Button onclick={open_modal} classes="btn-primary">{"Open Modal"}</Button>
                        </div>
                    </div>
                </Card>

                <Card title={html! { <Typography level={TypographyLevel::H4}>{"Toast Notification"}</Typography> }} classes="component-card">
                    <div class="flex flex-col gap-4 w-full">
                        <Typography level={TypographyLevel::Default}>
                            {"Click the button below to show a toast message."}
                        </Typography>
                        <div>
                            <Button onclick={trigger_toast} classes="btn-secondary">{"Show Toast"}</Button>
                        </div>
                    </div>
                </Card>

                // Modal Rendering
                {
                    if *show_modal {
                        html! {
                            <div class="fixed inset-0 z-50 flex items-center justify-center bg-black/40 backdrop-blur-sm">
                                <Modal class="demo-modal">
                                    <div class="p-6">
                                        <Typography level={TypographyLevel::H4} classes="mb-4">{"Interactive Modal"}</Typography>
                                        <Typography level={TypographyLevel::Default} classes="mb-6">
                                            {"This modal wrapper uses our custom Modal component and Yew state to control overlay visibility."}
                                        </Typography>
                                        <div class="flex justify-end gap-2">
                                            <Button onclick={close_modal} classes="btn-tertiary">{"Close Dialog"}</Button>
                                        </div>
                                    </div>
                                </Modal>
                            </div>
                        }
                    } else {
                        html! {}
                    }
                }

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
