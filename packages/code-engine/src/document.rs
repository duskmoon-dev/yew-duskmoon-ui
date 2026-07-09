#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct TextDocument {
    text: String,
}

impl TextDocument {
    pub fn new(text: impl Into<String>) -> Self {
        Self { text: text.into() }
    }

    pub fn text(&self) -> &str {
        &self.text
    }

    pub fn is_empty(&self) -> bool {
        self.text.is_empty()
    }

    pub fn line_count(&self) -> usize {
        if self.text.is_empty() {
            1
        } else {
            self.text.lines().count() + usize::from(self.text.ends_with('\n'))
        }
    }

    pub fn line(&self, index: usize) -> Option<&str> {
        self.lines().get(index).copied()
    }

    pub fn lines(&self) -> Vec<&str> {
        if self.text.is_empty() {
            return vec![""];
        }

        let mut lines = self.text.lines().collect::<Vec<_>>();
        if self.text.ends_with('\n') {
            lines.push("");
        }
        lines
    }
}

impl From<&str> for TextDocument {
    fn from(value: &str) -> Self {
        Self::new(value)
    }
}

impl From<String> for TextDocument {
    fn from(value: String) -> Self {
        Self::new(value)
    }
}

#[cfg(test)]
mod tests {
    use super::TextDocument;

    #[test]
    fn empty_document_has_one_line() {
        let document = TextDocument::new("");

        assert_eq!(document.line_count(), 1);
        assert_eq!(document.line(0), Some(""));
    }

    #[test]
    fn tracks_trailing_blank_line() {
        let document = TextDocument::new("one\ntwo\n");

        assert_eq!(document.line_count(), 3);
        assert_eq!(document.lines(), vec!["one", "two", ""]);
    }

    #[test]
    fn returns_requested_line() {
        let document = TextDocument::new("one\ntwo");

        assert_eq!(document.line(1), Some("two"));
        assert_eq!(document.line(2), None);
    }
}
