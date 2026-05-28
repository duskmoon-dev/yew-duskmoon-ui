use yew::prelude::*;
use yew_duskmoon::{Card, Typography, Divider};
use yew_duskmoon::typography::TypographyLevel;

#[function_component(LayoutComponent)]
pub fn layout_component() -> Html {
    html! {
        <div class="app">
            <div class="app-main w-[90%] mx-auto flex flex-col gap-6">
                <Typography level={TypographyLevel::H2}>{"Layout Components"}</Typography>
                
                <Card title={html! { <Typography level={TypographyLevel::H4}>{"Divider Component"}</Typography> }}>
                    <div class="flex flex-col gap-4 w-full">
                        <Typography level={TypographyLevel::Default}>
                            {"Above the divider."}
                        </Typography>
                        <Divider />
                        <Typography level={TypographyLevel::Default}>
                            {"Below the divider."}
                        </Typography>
                        <Divider variant="dashed" />
                        <Typography level={TypographyLevel::Default}>
                            {"Below a dashed divider."}
                        </Typography>
                    </div>
                </Card>
            </div>
        </div>
    }
}
