use yew::prelude::*;
use yew::virtual_dom::AttrValue;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum TableDensity {
    #[default]
    Default,
    Compact,
    Comfortable,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum TableBorders {
    #[default]
    Default,
    Bordered,
    Borderless,
}

#[derive(Properties, Clone, PartialEq)]
pub struct TableProps {
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub variant: Option<String>,
    #[prop_or_default]
    pub hoverable: bool,
    #[prop_or_default]
    pub striped: bool,
    #[prop_or_default]
    pub density: TableDensity,
    #[prop_or_default]
    pub borders: TableBorders,
    #[prop_or_default]
    pub sticky_header: bool,
    #[prop_or_default]
    pub selectable: bool,
    #[prop_or_default]
    pub surface: bool,
    #[prop_or_default]
    pub responsive: bool,
    #[prop_or_default]
    pub wrapper_class: Classes,
    #[prop_or_default]
    pub aria_label: Option<AttrValue>,
    #[prop_or_default]
    pub aria_describedby: Option<AttrValue>,
}

#[function_component(Table)]
pub fn table(props: &TableProps) -> Html {
    table_view(props)
}

fn table_view(props: &TableProps) -> Html {
    let table = html! {
        <table
            class={table_classes(props)}
            aria-label={props.aria_label.clone()}
            aria-describedby={props.aria_describedby.clone()}
        >
            { for props.children.iter() }
        </table>
    };

    if props.responsive {
        html! {
            <div class={classes!("table-responsive", props.wrapper_class.clone())}>
                { table }
            </div>
        }
    } else {
        table
    }
}

fn table_classes(props: &TableProps) -> Classes {
    let mut classes = classes!("table");

    if let Some(variant) = &props.variant {
        classes.push(format!("table-{variant}"));
    }
    if props.hoverable {
        classes.push("table-hover");
    }
    if props.striped {
        classes.push("table-striped");
    }
    match props.density {
        TableDensity::Default => {}
        TableDensity::Compact => classes.push("table-compact"),
        TableDensity::Comfortable => classes.push("table-comfortable"),
    }
    match props.borders {
        TableBorders::Default => {}
        TableBorders::Bordered => classes.push("table-bordered"),
        TableBorders::Borderless => classes.push("table-borderless"),
    }
    if props.sticky_header {
        // TODO(upstream): duskmoon-dev/duskmoonui#56 - Core must keep the
        // header visible inside a bounded overflow container in Chromium.
        classes.push("table-sticky");
    }
    if props.selectable {
        classes.push("table-selectable");
    }
    if props.surface {
        classes.push("table-surface");
    }

    classes.push(props.class.clone());
    classes
}

#[cfg(test)]
mod tests {
    use super::*;
    use yew::virtual_dom::{VNode, VTag};

    fn default_props() -> TableProps {
        TableProps {
            class: Classes::new(),
            children: Children::default(),
            variant: None,
            hoverable: false,
            striped: false,
            density: TableDensity::Default,
            borders: TableBorders::Default,
            sticky_header: false,
            selectable: false,
            surface: false,
            responsive: false,
            wrapper_class: Classes::new(),
            aria_label: None,
            aria_describedby: None,
        }
    }

    fn tag(node: &VNode) -> &VTag {
        match node {
            VNode::VTag(tag) => tag,
            other => panic!("expected an element, got {other:?}"),
        }
    }

    fn attribute<'a>(tag: &'a VTag, name: &str) -> Option<&'a str> {
        tag.attributes
            .iter()
            .find_map(|(key, value)| (key == name).then_some(value))
    }

    #[test]
    fn maps_supported_modifiers_to_core_classes() {
        let props = TableProps {
            hoverable: true,
            striped: true,
            density: TableDensity::Compact,
            borders: TableBorders::Bordered,
            sticky_header: true,
            selectable: true,
            surface: true,
            ..default_props()
        };

        assert_eq!(
            table_classes(&props).to_string(),
            "table table-hover table-striped table-compact table-bordered table-sticky table-selectable table-surface"
        );

        let props = TableProps {
            density: TableDensity::Comfortable,
            ..default_props()
        };

        assert_eq!(table_classes(&props).to_string(), "table table-comfortable");
    }

    #[test]
    fn border_treatment_is_mutually_exclusive() {
        let default = TableProps {
            borders: TableBorders::Default,
            ..default_props()
        };
        let bordered = TableProps {
            borders: TableBorders::Bordered,
            ..default_props()
        };
        let borderless = TableProps {
            borders: TableBorders::Borderless,
            ..default_props()
        };

        assert_eq!(table_classes(&default).to_string(), "table");
        assert_eq!(table_classes(&bordered).to_string(), "table table-bordered");
        assert_eq!(
            table_classes(&borderless).to_string(),
            "table table-borderless"
        );
    }

    #[test]
    fn preserves_default_variant_and_custom_class_behavior() {
        assert_eq!(table_classes(&default_props()).to_string(), "table");

        let props = TableProps {
            class: classes!("custom-table"),
            variant: Some("legacy".to_owned()),
            ..default_props()
        };

        assert_eq!(
            table_classes(&props).to_string(),
            "table table-legacy custom-table"
        );
    }

    #[test]
    fn renders_a_semantic_table_root() {
        let view = table_view(&default_props());

        assert_eq!(tag(&view).tag(), "table");
    }

    #[test]
    fn responsive_table_uses_wrapper_and_preserves_table_attributes() {
        let props = TableProps {
            class: classes!("custom-table"),
            responsive: true,
            wrapper_class: classes!("custom-wrapper"),
            aria_label: Some("Accounts".into()),
            aria_describedby: Some("accounts-help".into()),
            ..default_props()
        };

        let view = table_view(&props);
        let wrapper = tag(&view);
        assert_eq!(wrapper.tag(), "div");
        assert_eq!(
            attribute(wrapper, "class"),
            Some("table-responsive custom-wrapper")
        );

        let table = tag(wrapper
            .children()
            .expect("wrapper should contain the table"));
        assert_eq!(table.tag(), "table");
        assert_eq!(attribute(table, "class"), Some("table custom-table"));
        assert_eq!(attribute(table, "aria-label"), Some("Accounts"));
        assert_eq!(attribute(table, "aria-describedby"), Some("accounts-help"));
    }
}
