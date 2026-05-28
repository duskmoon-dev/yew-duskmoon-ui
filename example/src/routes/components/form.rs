use yew::prelude::*;
use yew_duskmoon::{Card, Typography, Input, Textarea, Checkbox, Radio, Switch};
use yew_duskmoon::typography::TypographyLevel;

#[function_component(FormComponent)]
pub fn form_component() -> Html {
    html! {
        <div class="app">
            <div class="app-main w-[90%] mx-auto flex flex-col gap-6">
                <Typography level={TypographyLevel::H2}>{"Form Components"}</Typography>
                
                <Card title={html! { <Typography level={TypographyLevel::H4}>{"Inputs and Textareas"}</Typography> }}>
                    <div class="flex flex-col gap-4 w-full max-w-md">
                        <div class="flex flex-col gap-1">
                            <label class="text-sm font-semibold">{"Standard Input:"}</label>
                            <Input class="border p-2 rounded" />
                        </div>
                        
                        <div class="flex flex-col gap-1">
                            <label class="text-sm font-semibold">{"Standard Textarea:"}</label>
                            <Textarea class="border p-2 rounded h-24" />
                        </div>
                    </div>
                </Card>

                <Card title={html! { <Typography level={TypographyLevel::H4}>{"Checkboxes, Radios, and Switches"}</Typography> }}>
                    <div class="flex flex-col gap-4 w-full">
                        <div class="flex items-center gap-2">
                            <Checkbox />
                            <label>{"Accept terms and conditions"}</label>
                        </div>

                        <div class="flex flex-col gap-2">
                            <Typography level={TypographyLevel::Default} classes="font-semibold">{"Select an Option:"}</Typography>
                            <div class="flex items-center gap-2">
                                <Radio />
                                <label>{"Option A"}</label>
                            </div>
                            <div class="flex items-center gap-2">
                                <Radio />
                                <label>{"Option B"}</label>
                            </div>
                        </div>

                        <div class="flex items-center gap-2">
                            <Switch />
                            <label>{"Enable notifications"}</label>
                        </div>
                    </div>
                </Card>
            </div>
        </div>
    }
}
