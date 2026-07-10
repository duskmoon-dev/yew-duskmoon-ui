use web_sys::{Event, HtmlInputElement, HtmlSelectElement, MouseEvent};
use yew::prelude::*;
use yew::virtual_dom::AttrValue;
use yew::TargetCast;

#[derive(Properties, Clone, PartialEq)]
pub struct PaginationProps {
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub variant: Option<String>,
    #[prop_or_default]
    pub total: Option<usize>,
    #[prop_or(10)]
    pub page_size: usize,
    #[prop_or(1)]
    pub current: usize,
    #[prop_or_else(default_page_size_options)]
    pub page_size_options: Vec<usize>,
    #[prop_or_default]
    pub on_change: Callback<usize>,
    #[prop_or_default]
    pub on_page_size_change: Callback<usize>,
    #[prop_or_default]
    pub on_refresh: Callback<MouseEvent>,
    #[prop_or(true)]
    pub show_refresh: bool,
    #[prop_or_else(|| AttrValue::from("items/page"))]
    pub page_size_label: AttrValue,
    #[prop_or_else(|| AttrValue::from("Refresh"))]
    pub refresh_label: AttrValue,
    #[prop_or_else(|| AttrValue::from("Pagination"))]
    pub aria_label: AttrValue,
}

#[function_component(Pagination)]
pub fn pagination(props: &PaginationProps) -> Html {
    let mut classes = classes!("pagination");
    if let Some(variant) = &props.variant {
        classes.push(format!("pagination-{}", variant));
    }
    classes.push(props.class.clone());

    if let Some(total) = props.total {
        classes.push("pagination-auto");
        return render_automatic_pagination(props, classes, total);
    }

    html! {
        <div class={classes}>
            { for props.children.iter() }
        </div>
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct PaginationState {
    current: usize,
    page_size: usize,
    total_pages: usize,
    range_start: usize,
    range_end: usize,
}

fn default_page_size_options() -> Vec<usize> {
    vec![10, 20, 30, 50, 100]
}

fn calculate_state(total: usize, page_size: usize, current: usize) -> PaginationState {
    let page_size = page_size.max(1);
    let total_pages = total.div_ceil(page_size).max(1);
    let current = current.clamp(1, total_pages);
    let range_start = if total == 0 {
        0
    } else {
        ((current - 1) * page_size) + 1
    };
    let range_end = (current * page_size).min(total);

    PaginationState {
        current,
        page_size,
        total_pages,
        range_start,
        range_end,
    }
}

fn render_automatic_pagination(props: &PaginationProps, classes: Classes, total: usize) -> Html {
    let state = calculate_state(total, props.page_size, props.current);
    let is_first = state.current <= 1;
    let is_last = state.current >= state.total_pages;

    let previous = {
        let on_change = props.on_change.clone();
        let current = state.current;
        Callback::from(move |_| {
            if current > 1 {
                on_change.emit(current - 1);
            }
        })
    };

    let next = {
        let on_change = props.on_change.clone();
        let current = state.current;
        let total_pages = state.total_pages;
        Callback::from(move |_| {
            if current < total_pages {
                on_change.emit(current + 1);
            }
        })
    };

    let jump_to_page = {
        let on_change = props.on_change.clone();
        let total_pages = state.total_pages;
        Callback::from(move |event: Event| {
            let input = event.target_unchecked_into::<HtmlInputElement>();
            if let Ok(page) = input.value().parse::<usize>() {
                on_change.emit(page.clamp(1, total_pages));
            }
        })
    };

    let change_page_size = {
        let on_page_size_change = props.on_page_size_change.clone();
        Callback::from(move |event: Event| {
            let select = event.target_unchecked_into::<HtmlSelectElement>();
            if let Ok(page_size) = select.value().parse::<usize>() {
                on_page_size_change.emit(page_size.max(1));
            }
        })
    };

    let mut page_size_options = props.page_size_options.clone();
    if !page_size_options.contains(&state.page_size) {
        page_size_options.push(state.page_size);
    }
    page_size_options.sort_unstable();
    page_size_options.dedup();

    html! {
        <div class={classes} role="navigation" aria-label={props.aria_label.clone()}>
            <span class="pagination-total">
                { format!("{}-{} / {}", state.range_start, state.range_end, total) }
            </span>

            <button
                class="pagination-page-button pagination-prev"
                type="button"
                disabled={is_first}
                aria-label="Previous page"
                onclick={previous}
            >
                { "<" }
            </button>

            <input
                class="pagination-current-input"
                type="number"
                name="pagination-current"
                min="1"
                max={state.total_pages.to_string()}
                value={state.current.to_string()}
                aria-label="Current page"
                onchange={jump_to_page}
            />

            <span class="pagination-slash">{ "/" }</span>
            <span class="pagination-page-total">{ state.total_pages }</span>

            <button
                class="pagination-page-button pagination-next"
                type="button"
                disabled={is_last}
                aria-label="Next page"
                onclick={next}
            >
                { ">" }
            </button>

            <label class="pagination-page-size">
                <select
                    class="pagination-page-size-select"
                    name="pagination-page-size"
                    value={state.page_size.to_string()}
                    aria-label="Page size"
                    onchange={change_page_size}
                >
                    {
                        for page_size_options.into_iter().map(|page_size| html! {
                            <option value={page_size.to_string()} selected={page_size == state.page_size}>
                                { page_size }
                            </option>
                        })
                    }
                </select>
                <span class="pagination-page-size-label">{ props.page_size_label.clone() }</span>
                <span class="pagination-page-size-caret" aria-hidden="true"></span>
            </label>

            {
                if props.show_refresh {
                    html! {
                        <button
                            class="pagination-refresh"
                            type="button"
                            aria-label={props.refresh_label.clone()}
                            onclick={props.on_refresh.clone()}
                        >
                            <span class="pagination-refresh-icon" aria-hidden="true"></span>
                            <span>{ props.refresh_label.clone() }</span>
                        </button>
                    }
                } else {
                    Html::default()
                }
            }
        </div>
    }
}

#[cfg(test)]
mod tests {
    use super::calculate_state;

    #[test]
    fn calculates_page_count_and_visible_range() {
        let state = calculate_state(8038, 30, 1);

        assert_eq!(state.total_pages, 268);
        assert_eq!(state.range_start, 1);
        assert_eq!(state.range_end, 30);
    }

    #[test]
    fn clamps_current_page_to_available_range() {
        let state = calculate_state(95, 30, 99);

        assert_eq!(state.current, 4);
        assert_eq!(state.range_start, 91);
        assert_eq!(state.range_end, 95);
    }

    #[test]
    fn handles_empty_totals_and_zero_page_size() {
        let state = calculate_state(0, 0, 4);

        assert_eq!(state.current, 1);
        assert_eq!(state.page_size, 1);
        assert_eq!(state.total_pages, 1);
        assert_eq!(state.range_start, 0);
        assert_eq!(state.range_end, 0);
    }
}
