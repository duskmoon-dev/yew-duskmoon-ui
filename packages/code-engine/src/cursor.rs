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
        let mut column = 1;

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
}

impl Default for CursorPosition {
    fn default() -> Self {
        Self { line: 1, column: 1 }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct CursorStatus {
    pub position: CursorPosition,
    pub selected_units: usize,
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
}
