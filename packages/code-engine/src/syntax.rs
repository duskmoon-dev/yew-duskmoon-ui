use crate::CodeLanguage;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SyntaxTokenKind {
    Plain,
    Keyword,
    Type,
    String,
    Number,
    Comment,
    Function,
    Punctuation,
}

impl SyntaxTokenKind {
    pub fn class_name(self) -> &'static str {
        match self {
            Self::Plain => "code-engine-token-plain",
            Self::Keyword => "code-engine-token-keyword",
            Self::Type => "code-engine-token-type",
            Self::String => "code-engine-token-string",
            Self::Number => "code-engine-token-number",
            Self::Comment => "code-engine-token-comment",
            Self::Function => "code-engine-token-function",
            Self::Punctuation => "code-engine-token-punctuation",
        }
    }

    pub fn style(self) -> &'static str {
        match self {
            Self::Plain => "color: var(--code-token-plain, #dbeafe);",
            Self::Keyword => "color: var(--code-token-keyword, #7dd3fc); font-weight: 700;",
            Self::Type => "color: var(--code-token-type, #c4b5fd);",
            Self::String => "color: var(--code-token-string, #86efac);",
            Self::Number => "color: var(--code-token-number, #fbbf24);",
            Self::Comment => "color: var(--code-token-comment, #64748b); font-style: italic;",
            Self::Function => "color: var(--code-token-function, #f0abfc);",
            Self::Punctuation => "color: var(--code-token-punctuation, #94a3b8);",
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SyntaxToken {
    pub kind: SyntaxTokenKind,
    pub text: String,
}

impl SyntaxToken {
    fn new(kind: SyntaxTokenKind, text: impl Into<String>) -> Self {
        Self {
            kind,
            text: text.into(),
        }
    }
}

pub fn highlight_tokens(language: CodeLanguage, source: &str) -> Vec<SyntaxToken> {
    if source.is_empty() {
        return Vec::new();
    }

    let mut tokens = Vec::new();
    let mut index = 0;

    while index < source.len() {
        let rest = &source[index..];

        if let Some(len) = line_comment_len(language, rest) {
            push_token(&mut tokens, SyntaxTokenKind::Comment, &rest[..len]);
            index += len;
            continue;
        }

        if rest.starts_with("/*") {
            let len = rest.find("*/").map(|end| end + 2).unwrap_or(rest.len());
            push_token(&mut tokens, SyntaxTokenKind::Comment, &rest[..len]);
            index += len;
            continue;
        }

        let ch = rest.chars().next().unwrap_or_default();

        if is_string_delimiter(language, ch) {
            let len = string_len(rest, ch);
            push_token(&mut tokens, SyntaxTokenKind::String, &rest[..len]);
            index += len;
            continue;
        }

        if ch.is_ascii_digit() {
            let len = take_while(rest, is_number_char);
            push_token(&mut tokens, SyntaxTokenKind::Number, &rest[..len]);
            index += len;
            continue;
        }

        if is_identifier_start(ch) {
            let len = take_while(rest, is_identifier_continue);
            let word = &rest[..len];
            let kind = if is_keyword(language, word) {
                SyntaxTokenKind::Keyword
            } else if is_type_name(language, word) {
                SyntaxTokenKind::Type
            } else if starts_function_call(&rest[len..]) {
                SyntaxTokenKind::Function
            } else {
                SyntaxTokenKind::Plain
            };
            push_token(&mut tokens, kind, word);
            index += len;
            continue;
        }

        let len = ch.len_utf8();
        let kind = if is_punctuation(ch) {
            SyntaxTokenKind::Punctuation
        } else {
            SyntaxTokenKind::Plain
        };
        push_token(&mut tokens, kind, &rest[..len]);
        index += len;
    }

    tokens
}

fn push_token(tokens: &mut Vec<SyntaxToken>, kind: SyntaxTokenKind, text: &str) {
    if let Some(last) = tokens.last_mut() {
        if last.kind == kind {
            last.text.push_str(text);
            return;
        }
    }

    tokens.push(SyntaxToken::new(kind, text));
}

fn line_comment_len(language: CodeLanguage, rest: &str) -> Option<usize> {
    let marker = match language {
        CodeLanguage::Rust
        | CodeLanguage::JavaScript
        | CodeLanguage::TypeScript
        | CodeLanguage::Json
        | CodeLanguage::Css => "//",
        CodeLanguage::Shell | CodeLanguage::Nix | CodeLanguage::Toml | CodeLanguage::Yaml => "#",
        CodeLanguage::Html | CodeLanguage::Markdown | CodeLanguage::PlainText => return None,
    };

    if !rest.starts_with(marker) {
        return None;
    }

    Some(rest.find('\n').unwrap_or(rest.len()))
}

fn is_string_delimiter(language: CodeLanguage, ch: char) -> bool {
    match language {
        CodeLanguage::Json => ch == '"',
        CodeLanguage::Shell => ch == '"' || ch == '\'',
        CodeLanguage::JavaScript | CodeLanguage::TypeScript => ch == '"' || ch == '\'' || ch == '`',
        CodeLanguage::PlainText => false,
        _ => ch == '"' || ch == '\'',
    }
}

fn string_len(rest: &str, delimiter: char) -> usize {
    let mut escaped = false;

    for (offset, ch) in rest.char_indices().skip(1) {
        if escaped {
            escaped = false;
            continue;
        }

        if ch == '\\' {
            escaped = true;
            continue;
        }

        if ch == delimiter {
            return offset + ch.len_utf8();
        }
    }

    rest.len()
}

fn take_while(rest: &str, predicate: impl Fn(char) -> bool) -> usize {
    for (offset, ch) in rest.char_indices() {
        if !predicate(ch) {
            return offset;
        }
    }

    rest.len()
}

fn is_number_char(ch: char) -> bool {
    ch.is_ascii_alphanumeric() || matches!(ch, '.' | '_' | '-')
}

fn is_identifier_start(ch: char) -> bool {
    ch == '_' || ch.is_ascii_alphabetic()
}

fn is_identifier_continue(ch: char) -> bool {
    ch == '_' || ch == '-' || ch.is_ascii_alphanumeric()
}

fn is_punctuation(ch: char) -> bool {
    matches!(
        ch,
        '{' | '}'
            | '['
            | ']'
            | '('
            | ')'
            | '<'
            | '>'
            | '='
            | '+'
            | '*'
            | '/'
            | '%'
            | ':'
            | ';'
            | ','
            | '.'
            | '|'
            | '&'
            | '!'
            | '?'
    )
}

fn starts_function_call(rest: &str) -> bool {
    let mut chars = rest.chars().filter(|ch| !ch.is_whitespace());
    match chars.next() {
        Some('(') => true,
        Some('!') => chars.next() == Some('('),
        _ => false,
    }
}

fn is_keyword(language: CodeLanguage, word: &str) -> bool {
    match language {
        CodeLanguage::Rust => matches!(
            word,
            "as" | "async"
                | "await"
                | "break"
                | "const"
                | "continue"
                | "crate"
                | "dyn"
                | "else"
                | "enum"
                | "extern"
                | "false"
                | "fn"
                | "for"
                | "if"
                | "impl"
                | "in"
                | "let"
                | "loop"
                | "match"
                | "mod"
                | "move"
                | "mut"
                | "pub"
                | "ref"
                | "return"
                | "self"
                | "Self"
                | "static"
                | "struct"
                | "super"
                | "trait"
                | "true"
                | "type"
                | "unsafe"
                | "use"
                | "where"
                | "while"
        ),
        CodeLanguage::JavaScript | CodeLanguage::TypeScript => matches!(
            word,
            "async"
                | "await"
                | "break"
                | "case"
                | "catch"
                | "class"
                | "const"
                | "continue"
                | "default"
                | "else"
                | "export"
                | "extends"
                | "false"
                | "finally"
                | "for"
                | "from"
                | "function"
                | "if"
                | "import"
                | "in"
                | "let"
                | "new"
                | "null"
                | "return"
                | "switch"
                | "this"
                | "throw"
                | "true"
                | "try"
                | "typeof"
                | "undefined"
                | "var"
                | "while"
        ),
        CodeLanguage::Json => matches!(word, "true" | "false" | "null"),
        CodeLanguage::Nix => matches!(
            word,
            "assert"
                | "else"
                | "false"
                | "if"
                | "in"
                | "inherit"
                | "let"
                | "null"
                | "or"
                | "rec"
                | "then"
                | "true"
                | "with"
        ),
        CodeLanguage::Shell => matches!(
            word,
            "case"
                | "do"
                | "done"
                | "elif"
                | "else"
                | "esac"
                | "fi"
                | "for"
                | "function"
                | "if"
                | "in"
                | "then"
                | "while"
        ),
        CodeLanguage::Toml | CodeLanguage::Yaml => matches!(word, "true" | "false" | "null"),
        CodeLanguage::Html
        | CodeLanguage::Css
        | CodeLanguage::Markdown
        | CodeLanguage::PlainText => false,
    }
}

fn is_type_name(language: CodeLanguage, word: &str) -> bool {
    match language {
        CodeLanguage::Rust => matches!(
            word,
            "bool"
                | "char"
                | "f32"
                | "f64"
                | "i8"
                | "i16"
                | "i32"
                | "i64"
                | "i128"
                | "isize"
                | "str"
                | "String"
                | "u8"
                | "u16"
                | "u32"
                | "u64"
                | "u128"
                | "usize"
                | "Vec"
                | "Option"
                | "Result"
        ),
        CodeLanguage::TypeScript => matches!(
            word,
            "any" | "boolean" | "never" | "number" | "string" | "unknown" | "void"
        ),
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::{highlight_tokens, SyntaxTokenKind};
    use crate::CodeLanguage;

    #[test]
    fn highlights_rust_keywords_strings_and_comments() {
        let tokens = highlight_tokens(CodeLanguage::Rust, "fn main() { \"ok\" } // done");

        assert!(tokens
            .iter()
            .any(|token| token.kind == SyntaxTokenKind::Keyword && token.text == "fn"));
        assert!(tokens
            .iter()
            .any(|token| token.kind == SyntaxTokenKind::String && token.text == "\"ok\""));
        assert!(tokens
            .iter()
            .any(|token| token.kind == SyntaxTokenKind::Comment && token.text == "// done"));
    }

    #[test]
    fn highlights_function_like_identifiers() {
        let tokens = highlight_tokens(CodeLanguage::Rust, "println!(\"ok\")");

        assert!(tokens
            .iter()
            .any(|token| token.kind == SyntaxTokenKind::Function && token.text == "println"));
    }
}
