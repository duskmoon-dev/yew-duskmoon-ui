use yew::prelude::*;
use yew_duskmoon::{Card, Typography, Breadcrumbs, Menu, Pagination, Stepper};
use yew_duskmoon::typography::TypographyLevel;

#[function_component(NavigationComponent)]
pub fn navigation_component() -> Html {
    let current_page = use_state(|| 1);
    let select_page = {
        let current_page = current_page.clone();
        Callback::from(move |p: usize| current_page.set(p))
    };

    html! {
        <div class="app">
            <div class="app-main w-[90%] mx-auto flex flex-col gap-6">
                <Typography level={TypographyLevel::H2}>{"Navigation Components"}</Typography>
                
                <Card title={html! { <Typography level={TypographyLevel::H4}>{"Breadcrumbs"}</Typography> }}>
                    <div class="flex flex-col gap-4 w-full">
                        <Breadcrumbs class="flex items-center gap-2 text-sm text-gray-500">
                            <span>{"Home"}</span>
                            <span>{"/"}</span>
                            <span>{"Components"}</span>
                            <span>{"/"}</span>
                            <span class="font-bold text-primary">{"Navigation"}</span>
                        </Breadcrumbs>
                    </div>
                </Card>

                <Card title={html! { <Typography level={TypographyLevel::H4}>{"Menu"}</Typography> }}>
                    <div class="flex flex-col gap-4 w-[250px]">
                        <Menu class="border rounded p-1 flex flex-col gap-1">
                            <a href="#profile" class="p-2 hover:bg-gray-100 dark:hover:bg-gray-800 rounded">{"My Profile"}</a>
                            <a href="#settings" class="p-2 hover:bg-gray-100 dark:hover:bg-gray-800 rounded">{"Settings"}</a>
                            <hr class="my-1" />
                            <a href="#logout" class="p-2 hover:bg-gray-100 dark:hover:bg-gray-800 text-red-500 rounded">{"Logout"}</a>
                        </Menu>
                    </div>
                </Card>

                <Card title={html! { <Typography level={TypographyLevel::H4}>{"Pagination"}</Typography> }}>
                    <div class="flex flex-col gap-4 w-full">
                        <Pagination class="flex items-center gap-2">
                            {
                                for (1..=5).map(|p| {
                                    let select = select_page.clone();
                                    let active = p == *current_page;
                                    let btn_class = if active {
                                        "px-3 py-1.5 bg-primary text-white rounded font-bold"
                                    } else {
                                        "px-3 py-1.5 border rounded hover:bg-gray-100 dark:hover:bg-gray-800"
                                    };
                                    html! {
                                        <button onclick={move |_| select.emit(p)} class={btn_class}>
                                            { p }
                                        </button>
                                    }
                                })
                            }
                        </Pagination>
                        <Typography level={TypographyLevel::Default}>
                            { html! { format!("Current Page: {}", *current_page) } }
                        </Typography>
                    </div>
                </Card>

                <Card title={html! { <Typography level={TypographyLevel::H4}>{"Stepper"}</Typography> }}>
                    <div class="flex flex-col gap-4 w-full">
                        <Stepper class="flex justify-between items-center w-full max-w-lg">
                            <div class="flex flex-col items-center">
                                <div class="w-8 h-8 rounded-full bg-primary text-white flex items-center justify-center font-bold">{"1"}</div>
                                <span class="text-xs mt-1">{"Step 1"}</span>
                            </div>
                            <div class="flex-1 h-0.5 bg-primary mx-4"></div>
                            <div class="flex flex-col items-center">
                                <div class="w-8 h-8 rounded-full bg-primary text-white flex items-center justify-center font-bold">{"2"}</div>
                                <span class="text-xs mt-1">{"Step 2"}</span>
                            </div>
                            <div class="flex-1 h-0.5 bg-gray-200 dark:bg-gray-700 mx-4"></div>
                            <div class="flex flex-col items-center">
                                <div class="w-8 h-8 rounded-full bg-gray-200 dark:bg-gray-700 text-gray-500 flex items-center justify-center font-bold">{"3"}</div>
                                <span class="text-xs mt-1">{"Step 3"}</span>
                            </div>
                        </Stepper>
                    </div>
                </Card>
            </div>
        </div>
    }
}
