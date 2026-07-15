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

    if language == CodeLanguage::PlainText {
        return vec![SyntaxToken::new(SyntaxTokenKind::Plain, source)];
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

        if let Some(len) = block_comment_len(language, rest) {
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
    let markers: &[&str] = match language {
        CodeLanguage::Rust
        | CodeLanguage::Zig
        | CodeLanguage::C
        | CodeLanguage::Cpp
        | CodeLanguage::ObjectiveC
        | CodeLanguage::CSharp
        | CodeLanguage::Java
        | CodeLanguage::Kotlin
        | CodeLanguage::Swift
        | CodeLanguage::JavaScript
        | CodeLanguage::TypeScript
        | CodeLanguage::Sass
        | CodeLanguage::Scss
        | CodeLanguage::Less => &["//"],
        CodeLanguage::Php => &["//", "#"],
        CodeLanguage::Python
        | CodeLanguage::Ruby
        | CodeLanguage::Elixir
        | CodeLanguage::Shell
        | CodeLanguage::Bash
        | CodeLanguage::Zsh
        | CodeLanguage::Nix
        | CodeLanguage::Toml
        | CodeLanguage::Yaml => &["#"],
        CodeLanguage::Erlang => &["%"],
        CodeLanguage::Lisp | CodeLanguage::EmacsLisp | CodeLanguage::Assembly => &[";"],
        CodeLanguage::WebAssembly => &[";;"],
        CodeLanguage::Html
        | CodeLanguage::Css
        | CodeLanguage::Json
        | CodeLanguage::Markdown
        | CodeLanguage::PlainText => &[],
    };

    for marker in markers {
        if rest.starts_with(*marker) {
            return Some(rest.find('\n').unwrap_or(rest.len()));
        }
    }

    None
}

fn block_comment_len(language: CodeLanguage, rest: &str) -> Option<usize> {
    let (opening, closing) = match language {
        CodeLanguage::Rust
        | CodeLanguage::Zig
        | CodeLanguage::C
        | CodeLanguage::Cpp
        | CodeLanguage::ObjectiveC
        | CodeLanguage::CSharp
        | CodeLanguage::Java
        | CodeLanguage::Kotlin
        | CodeLanguage::Swift
        | CodeLanguage::Php
        | CodeLanguage::JavaScript
        | CodeLanguage::TypeScript
        | CodeLanguage::Css
        | CodeLanguage::Sass
        | CodeLanguage::Scss
        | CodeLanguage::Less
        | CodeLanguage::Nix => ("/*", "*/"),
        CodeLanguage::Html | CodeLanguage::Markdown => ("<!--", "-->"),
        CodeLanguage::Lisp | CodeLanguage::EmacsLisp => ("#|", "|#"),
        CodeLanguage::WebAssembly => ("(;", ";)"),
        CodeLanguage::PlainText
        | CodeLanguage::Python
        | CodeLanguage::Ruby
        | CodeLanguage::Elixir
        | CodeLanguage::Erlang
        | CodeLanguage::Shell
        | CodeLanguage::Bash
        | CodeLanguage::Zsh
        | CodeLanguage::Assembly
        | CodeLanguage::Json
        | CodeLanguage::Toml
        | CodeLanguage::Yaml => return None,
    };

    if !rest.starts_with(opening) {
        return None;
    }

    Some(
        rest.find(closing)
            .map(|end| end + closing.len())
            .unwrap_or(rest.len()),
    )
}

fn is_string_delimiter(language: CodeLanguage, ch: char) -> bool {
    match language {
        CodeLanguage::Json
        | CodeLanguage::Lisp
        | CodeLanguage::EmacsLisp
        | CodeLanguage::WebAssembly => ch == '"',
        CodeLanguage::JavaScript
        | CodeLanguage::TypeScript
        | CodeLanguage::Php
        | CodeLanguage::Shell
        | CodeLanguage::Bash
        | CodeLanguage::Zsh => ch == '"' || ch == '\'' || ch == '`',
        CodeLanguage::Markdown => ch == '`',
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
            | '-'
            | '*'
            | '/'
            | '%'
            | '^'
            | '~'
            | ':'
            | ';'
            | ','
            | '.'
            | '|'
            | '&'
            | '!'
            | '?'
            | '@'
            | '#'
            | '$'
            | '\''
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
        CodeLanguage::Zig => matches!(
            word,
            "align"
                | "allowzero"
                | "and"
                | "anyframe"
                | "anytype"
                | "asm"
                | "async"
                | "await"
                | "break"
                | "catch"
                | "comptime"
                | "const"
                | "continue"
                | "defer"
                | "else"
                | "enum"
                | "errdefer"
                | "error"
                | "export"
                | "extern"
                | "false"
                | "fn"
                | "for"
                | "if"
                | "inline"
                | "noalias"
                | "null"
                | "opaque"
                | "or"
                | "orelse"
                | "packed"
                | "pub"
                | "return"
                | "struct"
                | "switch"
                | "test"
                | "threadlocal"
                | "true"
                | "try"
                | "union"
                | "unreachable"
                | "usingnamespace"
                | "var"
                | "volatile"
                | "while"
        ),
        CodeLanguage::C | CodeLanguage::Cpp | CodeLanguage::ObjectiveC => matches!(
            word,
            "alignas"
                | "alignof"
                | "asm"
                | "auto"
                | "bool"
                | "break"
                | "case"
                | "catch"
                | "char"
                | "class"
                | "const"
                | "constexpr"
                | "continue"
                | "default"
                | "delete"
                | "do"
                | "double"
                | "else"
                | "end"
                | "enum"
                | "explicit"
                | "export"
                | "extern"
                | "false"
                | "float"
                | "for"
                | "friend"
                | "goto"
                | "if"
                | "implementation"
                | "import"
                | "include"
                | "inline"
                | "int"
                | "interface"
                | "long"
                | "namespace"
                | "new"
                | "nullptr"
                | "operator"
                | "private"
                | "property"
                | "protected"
                | "protocol"
                | "public"
                | "register"
                | "return"
                | "selector"
                | "short"
                | "signed"
                | "sizeof"
                | "static"
                | "struct"
                | "super"
                | "switch"
                | "synthesize"
                | "template"
                | "this"
                | "throw"
                | "true"
                | "try"
                | "typedef"
                | "typename"
                | "union"
                | "unsigned"
                | "using"
                | "virtual"
                | "void"
                | "volatile"
                | "while"
        ),
        CodeLanguage::CSharp => matches!(
            word,
            "abstract"
                | "as"
                | "async"
                | "await"
                | "base"
                | "bool"
                | "break"
                | "byte"
                | "case"
                | "catch"
                | "char"
                | "class"
                | "const"
                | "continue"
                | "decimal"
                | "default"
                | "delegate"
                | "do"
                | "double"
                | "else"
                | "enum"
                | "event"
                | "false"
                | "finally"
                | "float"
                | "for"
                | "foreach"
                | "get"
                | "if"
                | "in"
                | "int"
                | "interface"
                | "internal"
                | "is"
                | "lock"
                | "long"
                | "namespace"
                | "new"
                | "null"
                | "object"
                | "out"
                | "override"
                | "params"
                | "private"
                | "protected"
                | "public"
                | "readonly"
                | "record"
                | "ref"
                | "return"
                | "sealed"
                | "set"
                | "short"
                | "static"
                | "string"
                | "struct"
                | "switch"
                | "this"
                | "throw"
                | "true"
                | "try"
                | "uint"
                | "ulong"
                | "unsafe"
                | "ushort"
                | "using"
                | "var"
                | "virtual"
                | "void"
                | "while"
                | "yield"
        ),
        CodeLanguage::Java => matches!(
            word,
            "abstract"
                | "assert"
                | "boolean"
                | "break"
                | "byte"
                | "case"
                | "catch"
                | "char"
                | "class"
                | "continue"
                | "default"
                | "do"
                | "double"
                | "else"
                | "enum"
                | "extends"
                | "false"
                | "final"
                | "finally"
                | "float"
                | "for"
                | "if"
                | "implements"
                | "import"
                | "instanceof"
                | "int"
                | "interface"
                | "long"
                | "native"
                | "new"
                | "null"
                | "package"
                | "permits"
                | "private"
                | "protected"
                | "public"
                | "record"
                | "return"
                | "sealed"
                | "short"
                | "static"
                | "strictfp"
                | "super"
                | "switch"
                | "synchronized"
                | "this"
                | "throw"
                | "throws"
                | "transient"
                | "true"
                | "try"
                | "var"
                | "void"
                | "volatile"
                | "while"
                | "yield"
        ),
        CodeLanguage::Kotlin => matches!(
            word,
            "abstract"
                | "actual"
                | "annotation"
                | "as"
                | "break"
                | "class"
                | "companion"
                | "const"
                | "continue"
                | "data"
                | "do"
                | "else"
                | "enum"
                | "expect"
                | "external"
                | "false"
                | "final"
                | "for"
                | "fun"
                | "if"
                | "in"
                | "infix"
                | "inline"
                | "inner"
                | "interface"
                | "internal"
                | "is"
                | "lateinit"
                | "noinline"
                | "null"
                | "object"
                | "open"
                | "operator"
                | "out"
                | "override"
                | "package"
                | "private"
                | "protected"
                | "public"
                | "reified"
                | "return"
                | "sealed"
                | "super"
                | "suspend"
                | "this"
                | "throw"
                | "true"
                | "try"
                | "typealias"
                | "val"
                | "var"
                | "vararg"
                | "when"
                | "while"
        ),
        CodeLanguage::Swift => matches!(
            word,
            "actor"
                | "any"
                | "associatedtype"
                | "async"
                | "await"
                | "break"
                | "case"
                | "catch"
                | "class"
                | "continue"
                | "default"
                | "defer"
                | "deinit"
                | "do"
                | "else"
                | "enum"
                | "extension"
                | "fallthrough"
                | "false"
                | "fileprivate"
                | "for"
                | "func"
                | "guard"
                | "if"
                | "import"
                | "in"
                | "init"
                | "inout"
                | "internal"
                | "is"
                | "let"
                | "nil"
                | "open"
                | "operator"
                | "private"
                | "protocol"
                | "public"
                | "repeat"
                | "return"
                | "self"
                | "Self"
                | "some"
                | "static"
                | "struct"
                | "subscript"
                | "super"
                | "switch"
                | "throw"
                | "throws"
                | "true"
                | "try"
                | "typealias"
                | "var"
                | "where"
                | "while"
        ),
        CodeLanguage::Python => matches!(
            word,
            "False"
                | "None"
                | "True"
                | "and"
                | "as"
                | "assert"
                | "async"
                | "await"
                | "break"
                | "case"
                | "class"
                | "continue"
                | "def"
                | "del"
                | "elif"
                | "else"
                | "except"
                | "finally"
                | "for"
                | "from"
                | "global"
                | "if"
                | "import"
                | "in"
                | "is"
                | "lambda"
                | "match"
                | "nonlocal"
                | "not"
                | "or"
                | "pass"
                | "raise"
                | "return"
                | "try"
                | "while"
                | "with"
                | "yield"
        ),
        CodeLanguage::Ruby => matches!(
            word,
            "BEGIN"
                | "END"
                | "alias"
                | "and"
                | "begin"
                | "break"
                | "case"
                | "class"
                | "def"
                | "defined"
                | "do"
                | "else"
                | "elsif"
                | "end"
                | "ensure"
                | "false"
                | "for"
                | "if"
                | "in"
                | "module"
                | "next"
                | "nil"
                | "not"
                | "or"
                | "redo"
                | "rescue"
                | "retry"
                | "return"
                | "self"
                | "super"
                | "then"
                | "true"
                | "undef"
                | "unless"
                | "until"
                | "when"
                | "while"
                | "yield"
        ),
        CodeLanguage::Php => matches!(
            word,
            "abstract"
                | "and"
                | "array"
                | "as"
                | "break"
                | "callable"
                | "case"
                | "catch"
                | "class"
                | "clone"
                | "const"
                | "continue"
                | "declare"
                | "default"
                | "do"
                | "echo"
                | "else"
                | "elseif"
                | "enum"
                | "extends"
                | "false"
                | "final"
                | "finally"
                | "fn"
                | "for"
                | "foreach"
                | "function"
                | "global"
                | "if"
                | "implements"
                | "include"
                | "instanceof"
                | "interface"
                | "isset"
                | "match"
                | "namespace"
                | "new"
                | "null"
                | "or"
                | "private"
                | "protected"
                | "public"
                | "readonly"
                | "require"
                | "return"
                | "static"
                | "switch"
                | "throw"
                | "trait"
                | "true"
                | "try"
                | "unset"
                | "use"
                | "var"
                | "while"
                | "xor"
                | "yield"
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
                | "implements"
                | "import"
                | "in"
                | "interface"
                | "let"
                | "namespace"
                | "new"
                | "null"
                | "private"
                | "protected"
                | "public"
                | "readonly"
                | "return"
                | "satisfies"
                | "switch"
                | "this"
                | "throw"
                | "true"
                | "try"
                | "type"
                | "typeof"
                | "undefined"
                | "var"
                | "while"
        ),
        CodeLanguage::Json => matches!(word, "true" | "false" | "null"),
        CodeLanguage::Html => matches!(
            word,
            "article"
                | "body"
                | "button"
                | "div"
                | "footer"
                | "form"
                | "h1"
                | "h2"
                | "head"
                | "header"
                | "html"
                | "input"
                | "label"
                | "main"
                | "nav"
                | "script"
                | "section"
                | "span"
                | "style"
        ),
        CodeLanguage::Css | CodeLanguage::Sass | CodeLanguage::Scss | CodeLanguage::Less => {
            matches!(
                word,
                "background"
                    | "block"
                    | "color"
                    | "display"
                    | "extend"
                    | "flex"
                    | "font"
                    | "grid"
                    | "important"
                    | "include"
                    | "media"
                    | "mixin"
                    | "none"
                    | "position"
                    | "relative"
                    | "supports"
                    | "when"
            )
        }
        CodeLanguage::Elixir => matches!(
            word,
            "after"
                | "alias"
                | "and"
                | "case"
                | "catch"
                | "cond"
                | "def"
                | "defmodule"
                | "defp"
                | "do"
                | "else"
                | "end"
                | "false"
                | "fn"
                | "for"
                | "if"
                | "import"
                | "in"
                | "nil"
                | "not"
                | "or"
                | "quote"
                | "raise"
                | "receive"
                | "require"
                | "rescue"
                | "super"
                | "true"
                | "try"
                | "unless"
                | "unquote"
                | "use"
                | "when"
                | "with"
        ),
        CodeLanguage::Erlang => matches!(
            word,
            "after"
                | "and"
                | "andalso"
                | "begin"
                | "case"
                | "catch"
                | "end"
                | "export"
                | "false"
                | "fun"
                | "if"
                | "module"
                | "not"
                | "of"
                | "or"
                | "orelse"
                | "receive"
                | "true"
                | "try"
                | "when"
                | "xor"
        ),
        CodeLanguage::Lisp | CodeLanguage::EmacsLisp => matches!(
            word,
            "car"
                | "cdr"
                | "cond"
                | "cons"
                | "defconst"
                | "defmacro"
                | "defpackage"
                | "defun"
                | "defvar"
                | "function"
                | "if"
                | "in-package"
                | "lambda"
                | "let"
                | "loop"
                | "nil"
                | "progn"
                | "quote"
                | "setf"
                | "setq"
                | "t"
                | "unless"
                | "when"
        ),
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
        CodeLanguage::Shell | CodeLanguage::Bash | CodeLanguage::Zsh => matches!(
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
                | "local"
                | "readonly"
                | "select"
                | "then"
                | "time"
                | "until"
                | "while"
        ),
        CodeLanguage::Assembly => matches!(
            word,
            "add"
                | "and"
                | "call"
                | "cmp"
                | "div"
                | "extern"
                | "global"
                | "je"
                | "jmp"
                | "jne"
                | "lea"
                | "mov"
                | "mul"
                | "nop"
                | "or"
                | "pop"
                | "push"
                | "ret"
                | "section"
                | "sub"
                | "xor"
        ),
        CodeLanguage::WebAssembly => matches!(
            word,
            "block"
                | "br"
                | "call"
                | "data"
                | "elem"
                | "else"
                | "end"
                | "export"
                | "func"
                | "global"
                | "if"
                | "import"
                | "local"
                | "loop"
                | "memory"
                | "module"
                | "mut"
                | "param"
                | "result"
                | "return"
                | "start"
                | "table"
                | "then"
        ),
        CodeLanguage::Toml | CodeLanguage::Yaml => matches!(word, "true" | "false" | "null"),
        CodeLanguage::Markdown | CodeLanguage::PlainText => false,
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
        CodeLanguage::Zig => matches!(
            word,
            "anyerror"
                | "anyopaque"
                | "bool"
                | "comptime_float"
                | "comptime_int"
                | "f16"
                | "f32"
                | "f64"
                | "f80"
                | "f128"
                | "i8"
                | "i16"
                | "i32"
                | "i64"
                | "i128"
                | "isize"
                | "noreturn"
                | "type"
                | "u8"
                | "u16"
                | "u32"
                | "u64"
                | "u128"
                | "usize"
                | "void"
        ),
        CodeLanguage::C | CodeLanguage::Cpp | CodeLanguage::ObjectiveC => matches!(
            word,
            "FILE"
                | "NSArray"
                | "NSDictionary"
                | "NSObject"
                | "NSString"
                | "size_t"
                | "std"
                | "string"
                | "uint8_t"
                | "uint16_t"
                | "uint32_t"
                | "uint64_t"
                | "vector"
        ),
        CodeLanguage::CSharp => {
            matches!(word, "DateTime" | "Dictionary" | "List" | "String" | "Task")
        }
        CodeLanguage::Java => matches!(
            word,
            "ArrayList" | "List" | "Map" | "Object" | "Optional" | "String"
        ),
        CodeLanguage::Kotlin => matches!(
            word,
            "Any"
                | "Boolean"
                | "Double"
                | "Float"
                | "Int"
                | "List"
                | "Long"
                | "Map"
                | "Nothing"
                | "String"
                | "Unit"
        ),
        CodeLanguage::Swift => matches!(
            word,
            "Any"
                | "Array"
                | "Bool"
                | "Dictionary"
                | "Double"
                | "Error"
                | "Float"
                | "Int"
                | "Optional"
                | "String"
                | "Void"
        ),
        CodeLanguage::Python => matches!(
            word,
            "bool" | "bytes" | "dict" | "float" | "int" | "list" | "set" | "str" | "tuple"
        ),
        CodeLanguage::Ruby => matches!(
            word,
            "Array" | "FalseClass" | "Hash" | "Integer" | "String" | "Symbol" | "TrueClass"
        ),
        CodeLanguage::Php => matches!(
            word,
            "bool"
                | "float"
                | "int"
                | "iterable"
                | "mixed"
                | "never"
                | "object"
                | "string"
                | "void"
        ),
        CodeLanguage::TypeScript => matches!(
            word,
            "any" | "boolean" | "never" | "number" | "string" | "unknown" | "void"
        ),
        CodeLanguage::Elixir => matches!(word, "Atom" | "Integer" | "Map" | "String" | "Tuple"),
        CodeLanguage::Erlang => matches!(word, "atom" | "binary" | "integer" | "map" | "tuple"),
        CodeLanguage::Assembly => matches!(
            word,
            "eax"
                | "ebp"
                | "ebx"
                | "ecx"
                | "edi"
                | "edx"
                | "esi"
                | "esp"
                | "rax"
                | "rbp"
                | "rbx"
                | "rcx"
                | "rdi"
                | "rdx"
                | "rsi"
                | "rsp"
        ),
        CodeLanguage::WebAssembly => matches!(
            word,
            "externref" | "f32" | "f64" | "funcref" | "i32" | "i64" | "v128"
        ),
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::{highlight_tokens, SyntaxToken, SyntaxTokenKind};
    use crate::CodeLanguage;

    fn has_token(tokens: &[SyntaxToken], kind: SyntaxTokenKind, text: &str) -> bool {
        tokens
            .iter()
            .any(|token| token.kind == kind && token.text == text)
    }

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

        assert!(has_token(&tokens, SyntaxTokenKind::Function, "println"));
    }

    #[test]
    fn leaves_plain_text_unclassified() {
        let source = "Mode: collaborative";
        let tokens = highlight_tokens(CodeLanguage::PlainText, source);

        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0].kind, SyntaxTokenKind::Plain);
        assert_eq!(tokens[0].text, source);
    }

    #[test]
    fn highlights_language_specific_line_comments() {
        let cases = [
            (CodeLanguage::Python, "# Python", "# Python"),
            (CodeLanguage::Erlang, "% Erlang", "% Erlang"),
            (CodeLanguage::Lisp, "; Lisp", "; Lisp"),
            (CodeLanguage::WebAssembly, ";; WAT", ";; WAT"),
        ];

        for (language, source, expected) in cases {
            let tokens = highlight_tokens(language, source);
            assert!(
                has_token(&tokens, SyntaxTokenKind::Comment, expected),
                "missing {language} comment token"
            );
        }
    }

    #[test]
    fn highlights_language_specific_block_comments() {
        let cases = [
            (CodeLanguage::Html, "<!-- HTML -->"),
            (CodeLanguage::Lisp, "#| Lisp |#"),
            (CodeLanguage::WebAssembly, "(; WAT ;)"),
        ];

        for (language, source) in cases {
            let tokens = highlight_tokens(language, source);
            assert!(
                has_token(&tokens, SyntaxTokenKind::Comment, source),
                "missing {language} block comment token"
            );
        }
    }

    #[test]
    fn highlights_representative_language_families() {
        let cases = [
            (CodeLanguage::Zig, "const", SyntaxTokenKind::Keyword),
            (CodeLanguage::Cpp, "namespace", SyntaxTokenKind::Keyword),
            (CodeLanguage::CSharp, "record", SyntaxTokenKind::Keyword),
            (CodeLanguage::Python, "def", SyntaxTokenKind::Keyword),
            (CodeLanguage::Elixir, "defmodule", SyntaxTokenKind::Keyword),
            (CodeLanguage::Assembly, "mov", SyntaxTokenKind::Keyword),
            (CodeLanguage::WebAssembly, "i32", SyntaxTokenKind::Type),
        ];

        for (language, source, kind) in cases {
            let tokens = highlight_tokens(language, source);
            assert!(
                has_token(&tokens, kind, source),
                "missing {language} token for {source}"
            );
        }
    }
}
