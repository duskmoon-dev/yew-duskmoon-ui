use super::super::catalog::component_by_slug;
use super::page::ComponentPage;

pub fn component_page(slug: &str) -> Option<ComponentPage> {
    let spec = component_by_slug(slug)?;

    Some(match slug {
        "accordion" => accordion::page(spec),
        "alert" => alert::page(spec),
        "appbar" => appbar::page(spec),
        "autocomplete" => autocomplete::page(spec),
        "avatar" => avatar::page(spec),
        "badge" => badge::page(spec),
        "bottom-navigation" => bottom_navigation::page(spec),
        "bottom-sheet" => bottom_sheet::page(spec),
        "breadcrumbs" => breadcrumbs::page(spec),
        "button" => button::page(spec),
        "card" => card::page(spec),
        "cascader" => cascader::page(spec),
        "checkbox" => checkbox::page(spec),
        "chip" => chip::page(spec),
        "circle-menu" => circle_menu::page(spec),
        "code-block" => code_block::page(spec),
        "code-engine" => code_engine::page(spec),
        "collapse" => collapse::page(spec),
        "datepicker" => datepicker::page(spec),
        "dialog" => dialog::page(spec),
        "divider" => divider::page(spec),
        "dm-markdown" => dm_markdown::page(spec),
        "drawer" => drawer::page(spec),
        "file-upload" => file_upload::page(spec),
        "form" => form::page(spec),
        "form-group" => form_group::page(spec),
        "grid" => grid::page(spec),
        "input" => input::page(spec),
        "list" => list::page(spec),
        "markdown-body" => markdown_body::page(spec),
        "markdown-input" => markdown_input::page(spec),
        "menu" => menu::page(spec),
        "modal" => modal::page(spec),
        "multi-select" => multi_select::page(spec),
        "navbar" => navbar::page(spec),
        "nested-menu" => nested_menu::page(spec),
        "otp-input" => otp_input::page(spec),
        "pagination" => pagination::page(spec),
        "pin-input" => pin_input::page(spec),
        "popover" => popover::page(spec),
        "progress" => progress::page(spec),
        "radio" => radio::page(spec),
        "rating" => rating::page(spec),
        "segment-control" => segment_control::page(spec),
        "select" => select::page(spec),
        "skeleton" => skeleton::page(spec),
        "slider" => slider::page(spec),
        "snackbar" => snackbar::page(spec),
        "stepper" => stepper::page(spec),
        "switch" => switch::page(spec),
        "table" => table::page(spec),
        "tabs" => tabs::page(spec),
        "textarea" => textarea::page(spec),
        "time-input" => time_input::page(spec),
        "timeline" => timeline::page(spec),
        "toast" => toast::page(spec),
        "toggle" => toggle::page(spec),
        "tooltip" => tooltip::page(spec),
        "tree-select" => tree_select::page(spec),
        _ => return None,
    })
}

mod accordion;
mod alert;
mod appbar;
mod autocomplete;
mod avatar;
mod badge;
mod bottom_navigation;
mod bottom_sheet;
mod breadcrumbs;
mod button;
mod card;
mod cascader;
mod checkbox;
mod chip;
mod circle_menu;
mod code_block;
mod code_engine;
mod collapse;
mod datepicker;
mod dialog;
mod divider;
mod dm_markdown;
mod drawer;
mod file_upload;
mod form;
mod form_group;
mod grid;
mod input;
mod list;
mod markdown_body;
mod markdown_input;
mod menu;
mod modal;
mod multi_select;
mod navbar;
mod nested_menu;
mod otp_input;
mod pagination;
mod pin_input;
mod popover;
mod progress;
mod radio;
mod rating;
mod segment_control;
mod select;
mod skeleton;
mod slider;
mod snackbar;
mod stepper;
mod switch;
mod table;
mod tabs;
mod textarea;
mod time_input;
mod timeline;
mod toast;
mod toggle;
mod tooltip;
mod tree_select;
