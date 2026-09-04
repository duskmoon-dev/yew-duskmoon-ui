use crate::routes::components::catalog::ComponentSpec;
use crate::routes::components::detail::page::{ApiRow, ComponentPage};
use yew::prelude::*;
use yew_duskmoon::{
    Badge, BadgeAppearance, BadgeSize, ButtonAppearance, ButtonSize, Color, IconButton, Table,
    TableBorders, TableDensity, Tooltip, TooltipPlacement,
};

const TABLE_API: &[ApiRow] = &[
    ApiRow {
        prop: "class",
        ty: "Classes",
        default: "empty",
        docs: "Extra CSS classes appended to the semantic table element.",
    },
    ApiRow {
        prop: "children",
        ty: "Children",
        default: "empty",
        docs: "Semantic caption, colgroup, thead, tbody, tfoot, tr, th, and td children.",
    },
    ApiRow {
        prop: "hoverable",
        ty: "bool",
        default: "false",
        docs: "Applies the core hover treatment to body rows.",
    },
    ApiRow {
        prop: "striped",
        ty: "bool",
        default: "false",
        docs: "Applies the core zebra treatment to alternating body rows.",
    },
    ApiRow {
        prop: "density",
        ty: "TableDensity",
        default: "Default",
        docs: "Mutually exclusive Default, Compact, or Comfortable density.",
    },
    ApiRow {
        prop: "borders",
        ty: "TableBorders",
        default: "Default",
        docs: "Mutually exclusive Default, Bordered, or Borderless treatment.",
    },
    ApiRow {
        prop: "sticky_header",
        ty: "bool",
        default: "false",
        docs: "Emits table-sticky for a consumer-provided vertical scroll container; core issue #56 tracks current Chromium behavior.",
    },
    ApiRow {
        prop: "selectable",
        ty: "bool",
        default: "false",
        docs: "Applies selectable-row affordances; consumers remain responsible for row interaction and selection state.",
    },
    ApiRow {
        prop: "surface",
        ty: "bool",
        default: "false",
        docs: "Applies the core surface background treatment to the table.",
    },
    ApiRow {
        prop: "responsive",
        ty: "bool",
        default: "false",
        docs: "Wraps the table in the core table-responsive overflow container.",
    },
    ApiRow {
        prop: "wrapper_class",
        ty: "Classes",
        default: "empty",
        docs: "Extra classes for the responsive wrapper; table classes remain on the table.",
    },
    ApiRow {
        prop: "aria_label",
        ty: "Option<AttrValue>",
        default: "None",
        docs: "Optional accessible name when a visible caption is not appropriate.",
    },
    ApiRow {
        prop: "aria_describedby",
        ty: "Option<AttrValue>",
        default: "None",
        docs: "Optional id of supporting table guidance.",
    },
    ApiRow {
        prop: "variant",
        ty: "Option<String>",
        default: "None",
        docs: "Legacy escape hatch that appends table-{variant}. The core table contract does not define a normal color palette.",
    },
];

pub fn page(spec: &'static ComponentSpec) -> ComponentPage {
    ComponentPage::without_color_variants(spec, usage, TABLE_API, demo)
}

fn usage(_: &ComponentSpec) -> String {
    "use yew_duskmoon::{Table, TableBorders, TableDensity};\n\nhtml! {\n    <Table\n        responsive={true}\n        hoverable={true}\n        striped={true}\n        density={TableDensity::Compact}\n        borders={TableBorders::Bordered}\n    >\n        <caption>{ \"Managed records\" }</caption>\n        <thead>\n            <tr>\n                <th scope=\"col\">{ \"Record\" }</th>\n                <th scope=\"col\">{ \"Status\" }</th>\n            </tr>\n        </thead>\n        <tbody>\n            <tr>\n                <th scope=\"row\">{ \"Record A-104\" }</th>\n                <td>{ \"Ready\" }</td>\n            </tr>\n        </tbody>\n    </Table>\n}".to_owned()
}

fn demo(_: &ComponentSpec) -> Html {
    html! {
        <div class="component-detail-data-management">
            <p id="managed-records-help" class="component-detail-table-help">
                { "A generic composition of reusable badges, semantic table markup, and native tooltip actions. Scroll the records to exercise the sticky-header contract; core issue #56 tracks current Chromium behavior." }
            </p>
            <Table
                class="component-detail-table-demo"
                wrapper_class="component-detail-table-wrap component-detail-table-scroll"
                responsive={true}
                hoverable={true}
                striped={true}
                density={TableDensity::Compact}
                borders={TableBorders::Bordered}
                sticky_header={true}
                surface={true}
                aria_describedby="managed-records-help"
            >
                <caption>{ "Managed records — sample data" }</caption>
                <thead>
                    <tr>
                        <th scope="col">{ "Record" }</th>
                        <th scope="col">{ "Category" }</th>
                        <th scope="col">{ "Status" }</th>
                        <th scope="col">{ "Updated" }</th>
                        <th scope="col">{ "Actions" }</th>
                    </tr>
                </thead>
                <tbody>
                    { data_row("Record A-104", "Standard", "Ready", Color::Success, "a104") }
                    { data_row("Record B-208", "External", "Pending", Color::Warning, "b208") }
                    { data_row("Record C-315", "Archived", "Paused", Color::Neutral, "c315") }
                    { data_row("Record D-421", "Priority", "Review", Color::Info, "d421") }
                    { data_row("Record E-509", "Standard", "Ready", Color::Success, "e509") }
                    { data_row("Record F-612", "External", "Blocked", Color::Error, "f612") }
                </tbody>
            </Table>
        </div>
    }
}

fn data_row(
    record: &'static str,
    category: &'static str,
    status: &'static str,
    status_color: Color,
    id_suffix: &'static str,
) -> Html {
    let inspect_tooltip_id: AttrValue = format!("inspect-{id_suffix}-tooltip").into();
    let archive_tooltip_id: AttrValue = format!("archive-{id_suffix}-tooltip").into();
    let inspect_label: AttrValue = format!("Inspect {record}").into();
    let archive_label: AttrValue = format!("Archive {record}").into();
    let status_label: AttrValue = format!("Status: {status}").into();

    html! {
        <tr>
            <th scope="row">{ record }</th>
            <td>
                <Badge
                    color={Color::Secondary}
                    appearance={BadgeAppearance::Tonal}
                    size={BadgeSize::Small}
                >
                    { html! { category } }
                </Badge>
            </td>
            <td>
                <Badge
                    color={status_color}
                    appearance={BadgeAppearance::Outlined}
                    size={BadgeSize::Small}
                    aria_label={status_label}
                >
                    { html! { status } }
                </Badge>
            </td>
            <td><time datetime="2026-09-01">{ "Sep 1, 2026" }</time></td>
            <td>
                <div class="component-detail-row-actions">
                    <IconButton
                        label={inspect_label.clone()}
                        appearance={ButtonAppearance::Text}
                        color={Color::Primary}
                        size={ButtonSize::Small}
                        title={inspect_label}
                        tooltip_id={inspect_tooltip_id.clone()}
                    >
                        { inspect_icon() }
                    </IconButton>
                    <Tooltip id={inspect_tooltip_id} placement={TooltipPlacement::Left}>
                        { "Inspect record" }
                    </Tooltip>
                    <IconButton
                        label={archive_label.clone()}
                        appearance={ButtonAppearance::Text}
                        color={Color::Error}
                        size={ButtonSize::Small}
                        title={archive_label}
                        tooltip_id={archive_tooltip_id.clone()}
                    >
                        { archive_icon() }
                    </IconButton>
                    <Tooltip
                        id={archive_tooltip_id}
                        placement={TooltipPlacement::Left}
                        color={Color::Error}
                    >
                        { "Archive record" }
                    </Tooltip>
                </div>
            </td>
        </tr>
    }
}

fn inspect_icon() -> Html {
    html! {
        <svg
            aria-hidden="true"
            viewBox="0 0 24 24"
            width="16"
            height="16"
            fill="none"
            stroke="currentColor"
            stroke-width="2"
        >
            <circle cx="11" cy="11" r="7" />
            <path d="m20 20-3.5-3.5" />
        </svg>
    }
}

fn archive_icon() -> Html {
    html! {
        <svg
            aria-hidden="true"
            viewBox="0 0 24 24"
            width="16"
            height="16"
            fill="none"
            stroke="currentColor"
            stroke-width="2"
        >
            <path d="M4 7h16" />
            <path d="M6 7v12h12V7" />
            <path d="M9 11h6" />
            <path d="M8 4h8l1 3H7l1-3Z" />
        </svg>
    }
}
