use web_sys::HtmlTextAreaElement;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CursorPosition {
    pub line: usize,
    pub column: usize,
}

impl CursorPosition {
    pub const fn new(line: usize, column: usize) -> Self {
        Self { line, column }
    }

    pub fn from_utf16_offset(text: &str, offset: usize) -> Self {
        let mut remaining = offset;
        let mut line = 1;
        let mut column: usize = 1;

        for ch in text.chars() {
            if remaining == 0 {
                break;
            }

            let width = ch.len_utf16();
            if width > remaining {
                break;
            }

            remaining -= width;
            if ch == '\n' {
                line += 1;
                column = 1;
            } else {
                column += 1;
            }
        }

        Self { line, column }
    }

    pub fn visual_column_from_utf16_offset(text: &str, offset: usize, tab_size: usize) -> usize {
        let tab_size = tab_size.max(1);
        let mut remaining = offset;
        let mut column: usize = 1;

        for ch in text.chars() {
            if remaining == 0 {
                break;
            }

            let width = ch.len_utf16();
            if width > remaining {
                break;
            }

            remaining -= width;
            if ch == '\n' {
                column = 1;
            } else if ch == '\t' {
                let zero_based = column.saturating_sub(1);
                column = ((zero_based / tab_size) + 1) * tab_size + 1;
            } else {
                column += 1;
            }
        }

        column
    }
}

impl Default for CursorPosition {
    fn default() -> Self {
        Self { line: 1, column: 1 }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CursorStatus {
    pub position: CursorPosition,
    pub visual_column: usize,
    pub selected_units: usize,
}

impl Default for CursorStatus {
    fn default() -> Self {
        Self {
            position: CursorPosition::default(),
            visual_column: 1,
            selected_units: 0,
        }
    }
}

impl CursorStatus {
    pub fn from_textarea(textarea: &HtmlTextAreaElement) -> Self {
        let value = textarea.value();
        let start = textarea.selection_start().ok().flatten().unwrap_or(0) as usize;
        let end = textarea
            .selection_end()
            .ok()
            .flatten()
            .unwrap_or(start as u32) as usize;

        Self {
            position: CursorPosition::from_utf16_offset(&value, start),
            visual_column: CursorPosition::visual_column_from_utf16_offset(&value, start, 4),
            selected_units: start.abs_diff(end),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::CursorPosition;

    #[test]
    fn starts_at_first_line_and_column() {
        assert_eq!(
            CursorPosition::from_utf16_offset("abc", 0),
            CursorPosition::new(1, 1)
        );
    }

    #[test]
    fn tracks_line_and_column_after_newline() {
        assert_eq!(
            CursorPosition::from_utf16_offset("one\ntwo", 5),
            CursorPosition::new(2, 2)
        );
    }

    #[test]
    fn handles_utf16_wide_characters() {
        assert_eq!(
            CursorPosition::from_utf16_offset("a😀b", 3),
            CursorPosition::new(1, 3)
        );
    }

    #[test]
    fn expands_tabs_for_visual_columns() {
        assert_eq!(
            CursorPosition::visual_column_from_utf16_offset("\tlet", 1, 4),
            5
        );
        assert_eq!(
            CursorPosition::visual_column_from_utf16_offset("ab\tlet", 3, 4),
            5
        );
    }
}
