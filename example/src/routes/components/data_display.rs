use yew::prelude::*;
use yew_duskmoon::{Card, Typography, Table, List, Badge};
use yew_duskmoon::typography::TypographyLevel;

#[function_component(DataDisplayComponent)]
pub fn data_display_component() -> Html {
    html! {
        <div class="app">
            <div class="app-main w-[90%] mx-auto flex flex-col gap-6">
                <Typography level={TypographyLevel::H2}>{"Data Display Components"}</Typography>
                
                <Card title={html! { <Typography level={TypographyLevel::H4}>{"Badge Component"}</Typography> }}>
                    <div class="flex items-center gap-4">
                        <Badge variant={Some("primary".to_string())}>{"New"}</Badge>
                        <Badge variant={Some("success".to_string())}>{"Success"}</Badge>
                        <Badge variant={Some("warning".to_string())}>{"Warning"}</Badge>
                        <Badge variant={Some("error".to_string())}>{"Error"}</Badge>
                    </div>
                </Card>

                <Card title={html! { <Typography level={TypographyLevel::H4}>{"List Component"}</Typography> }}>
                    <List class="border rounded p-2">
                        <div class="p-2 border-b">{"List Item 1"}</div>
                        <div class="p-2 border-b">{"List Item 2"}</div>
                        <div class="p-2">{"List Item 3"}</div>
                    </List>
                </Card>

                <Card title={html! { <Typography level={TypographyLevel::H4}>{"Table Component"}</Typography> }}>
                    <Table class="w-full border-collapse border">
                        <thead>
                            <tr class="bg-gray-100 dark:bg-gray-800">
                                <th class="border p-2 text-left">{"Name"}</th>
                                <th class="border p-2 text-left">{"Role"}</th>
                            </tr>
                        </thead>
                        <tbody>
                            <tr>
                                <td class="border p-2">{"Duskmoon"}</td>
                                <td class="border p-2">{"Admin"}</td>
                            </tr>
                            <tr class="bg-gray-50 dark:bg-gray-900">
                                <td class="border p-2">{"Yew"}</td>
                                <td class="border p-2">{"Developer"}</td>
                            </tr>
                        </tbody>
                    </Table>
                </Card>
            </div>
        </div>
    }
}
