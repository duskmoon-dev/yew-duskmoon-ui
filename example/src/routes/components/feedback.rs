use yew::prelude::*;
use yew_duskmoon::{Card, Typography, Alert, Modal, Toast, Button};
use yew_duskmoon::typography::TypographyLevel;

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
            <div class="app-main w-[90%] mx-auto flex flex-col gap-6">
                <Typography level={TypographyLevel::H2}>{"Feedback Components"}</Typography>
                
                <Card title={html! { <Typography level={TypographyLevel::H4}>{"Alerts"}</Typography> }}>
                    <div class="flex flex-col gap-3 w-full">
                        <Alert variant={Some("info".to_string())}>
                            {"Info alert: Something is happening."}
                        </Alert>
                        <Alert variant={Some("success".to_string())}>
                            {"Success alert: Operation completed successfully."}
                        </Alert>
                        <Alert variant={Some("warning".to_string())}>
                            {"Warning alert: Please pay attention to this."}
                        </Alert>
                        <Alert variant={Some("error".to_string())}>
                            {"Error alert: Something went wrong."}
                        </Alert>
                    </div>
                </Card>

                <Card title={html! { <Typography level={TypographyLevel::H4}>{"Modal Overlay"}</Typography> }}>
                    <div class="flex flex-col gap-4 w-full">
                        <Typography level={TypographyLevel::Default}>
                            {"Click the button below to toggle the modal overlay."}
                        </Typography>
                        <div>
                            <Button onclick={open_modal}>{"Open Modal"}</Button>
                        </div>
                    </div>
                </Card>

                <Card title={html! { <Typography level={TypographyLevel::H4}>{"Toast Notification"}</Typography> }}>
                    <div class="flex flex-col gap-4 w-full">
                        <Typography level={TypographyLevel::Default}>
                            {"Click the button below to show a toast message."}
                        </Typography>
                        <div>
                            <Button onclick={trigger_toast}>{"Show Toast"}</Button>
                        </div>
                    </div>
                </Card>

                // Modal Rendering
                {
                    if *show_modal {
                        html! {
                            <div class="fixed inset-0 z-50 flex items-center justify-center bg-black/40 backdrop-blur-sm">
                                <Modal class="relative bg-white dark:bg-gray-900 rounded-lg shadow-xl max-w-md w-full mx-4 overflow-hidden border">
                                    <div class="p-6">
                                        <Typography level={TypographyLevel::H4} classes="mb-4">{"Interactive Modal"}</Typography>
                                        <Typography level={TypographyLevel::Default} classes="mb-6">
                                            {"This modal wrapper uses our custom Modal component and Yew state to control overlay visibility."}
                                        </Typography>
                                        <div class="flex justify-end gap-2">
                                            <Button onclick={close_modal}>{"Close Dialog"}</Button>
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
                                <Toast class="bg-gray-800 text-white p-4 rounded-lg shadow-lg flex items-center gap-4">
                                    <Typography level={TypographyLevel::Default}>{"Notification message triggered!"}</Typography>
                                    <Button onclick={hide_toast} classes="text-xs py-1 px-2 bg-gray-700 hover:bg-gray-600 rounded">{"Dismiss"}</Button>
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
