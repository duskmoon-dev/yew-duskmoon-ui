use std::fmt;
use std::str::FromStr;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum CodeLanguage {
    #[default]
    PlainText,
    Rust,
    Zig,
    C,
    Cpp,
    ObjectiveC,
    CSharp,
    Java,
    Kotlin,
    Swift,
    Python,
    Ruby,
    Php,
    JavaScript,
    TypeScript,
    Json,
    Markdown,
    Html,
    Css,
    Sass,
    Scss,
    Less,
    Elixir,
    Erlang,
    Lisp,
    EmacsLisp,
    Shell,
    Bash,
    Zsh,
    Nix,
    Assembly,
    WebAssembly,
    Toml,
    Yaml,
}

impl CodeLanguage {
    pub const ALL: [Self; 34] = [
        Self::PlainText,
        Self::Rust,
        Self::Zig,
        Self::C,
        Self::Cpp,
        Self::ObjectiveC,
        Self::CSharp,
        Self::Java,
        Self::Kotlin,
        Self::Swift,
        Self::Python,
        Self::Ruby,
        Self::Php,
        Self::JavaScript,
        Self::TypeScript,
        Self::Json,
        Self::Markdown,
        Self::Html,
        Self::Css,
        Self::Sass,
        Self::Scss,
        Self::Less,
        Self::Elixir,
        Self::Erlang,
        Self::Lisp,
        Self::EmacsLisp,
        Self::Shell,
        Self::Bash,
        Self::Zsh,
        Self::Nix,
        Self::Assembly,
        Self::WebAssembly,
        Self::Toml,
        Self::Yaml,
    ];

    pub fn as_str(self) -> &'static str {
        match self {
            Self::PlainText => "text",
            Self::Rust => "rust",
            Self::Zig => "zig",
            Self::C => "c",
            Self::Cpp => "cpp",
            Self::ObjectiveC => "objective-c",
            Self::CSharp => "csharp",
            Self::Java => "java",
            Self::Kotlin => "kotlin",
            Self::Swift => "swift",
            Self::Python => "python",
            Self::Ruby => "ruby",
            Self::Php => "php",
            Self::JavaScript => "javascript",
            Self::TypeScript => "typescript",
            Self::Json => "json",
            Self::Markdown => "markdown",
            Self::Html => "html",
            Self::Css => "css",
            Self::Sass => "sass",
            Self::Scss => "scss",
            Self::Less => "less",
            Self::Elixir => "elixir",
            Self::Erlang => "erlang",
            Self::Lisp => "lisp",
            Self::EmacsLisp => "emacs-lisp",
            Self::Shell => "shell",
            Self::Bash => "bash",
            Self::Zsh => "zsh",
            Self::Nix => "nix",
            Self::Assembly => "assembly",
            Self::WebAssembly => "wasm",
            Self::Toml => "toml",
            Self::Yaml => "yaml",
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            Self::PlainText => "Plain text",
            Self::Rust => "Rust",
            Self::Zig => "Zig",
            Self::C => "C",
            Self::Cpp => "C++",
            Self::ObjectiveC => "Objective-C",
            Self::CSharp => "C# / .NET",
            Self::Java => "Java",
            Self::Kotlin => "Kotlin",
            Self::Swift => "Swift",
            Self::Python => "Python",
            Self::Ruby => "Ruby",
            Self::Php => "PHP",
            Self::JavaScript => "JavaScript",
            Self::TypeScript => "TypeScript",
            Self::Json => "JSON",
            Self::Markdown => "Markdown",
            Self::Html => "HTML",
            Self::Css => "CSS",
            Self::Sass => "Sass",
            Self::Scss => "SCSS",
            Self::Less => "Less",
            Self::Elixir => "Elixir",
            Self::Erlang => "Erlang",
            Self::Lisp => "Lisp",
            Self::EmacsLisp => "Emacs Lisp",
            Self::Shell => "Shell",
            Self::Bash => "Bash",
            Self::Zsh => "Zsh",
            Self::Nix => "Nix",
            Self::Assembly => "Assembly",
            Self::WebAssembly => "WebAssembly",
            Self::Toml => "TOML",
            Self::Yaml => "YAML",
        }
    }
}

impl fmt::Display for CodeLanguage {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for CodeLanguage {
    type Err = ();

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value.trim().to_ascii_lowercase().as_str() {
            "" | "text" | "plain" | "plaintext" | "plain_text" => Ok(Self::PlainText),
            "rs" | "rust" => Ok(Self::Rust),
            "zig" => Ok(Self::Zig),
            "c" => Ok(Self::C),
            "c++" | "cc" | "cpp" | "cxx" => Ok(Self::Cpp),
            "object c" | "object-c" | "objc" | "obj-c" | "objectivec" | "objective-c" => {
                Ok(Self::ObjectiveC)
            }
            ".net" | "c#" | "cs" | "csharp" | "dotnet" => Ok(Self::CSharp),
            "java" => Ok(Self::Java),
            "kt" | "kotlin" => Ok(Self::Kotlin),
            "swift" => Ok(Self::Swift),
            "py" | "python" => Ok(Self::Python),
            "rb" | "ruby" => Ok(Self::Ruby),
            "php" => Ok(Self::Php),
            "js" | "javascript" => Ok(Self::JavaScript),
            "ts" | "typescript" => Ok(Self::TypeScript),
            "json" => Ok(Self::Json),
            "md" | "markdown" => Ok(Self::Markdown),
            "html" | "htm" => Ok(Self::Html),
            "css" => Ok(Self::Css),
            "sass" => Ok(Self::Sass),
            "scss" => Ok(Self::Scss),
            "less" => Ok(Self::Less),
            "ex" | "exs" | "elixir" => Ok(Self::Elixir),
            "erl" | "erlang" => Ok(Self::Erlang),
            "cl" | "common-lisp" | "lisp" => Ok(Self::Lisp),
            "el" | "elisp" | "emacs-lisp" => Ok(Self::EmacsLisp),
            "sh" | "shell" => Ok(Self::Shell),
            "bash" => Ok(Self::Bash),
            "zsh" => Ok(Self::Zsh),
            "nix" => Ok(Self::Nix),
            "asm" | "assembly" => Ok(Self::Assembly),
            "wat" | "wasm" | "webassembly" => Ok(Self::WebAssembly),
            "toml" => Ok(Self::Toml),
            "yaml" | "yml" => Ok(Self::Yaml),
            _ => Err(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;
    use std::str::FromStr;

    use super::CodeLanguage;

    #[test]
    fn parses_common_aliases() {
        assert_eq!(CodeLanguage::from_str("rs"), Ok(CodeLanguage::Rust));
        assert_eq!(CodeLanguage::from_str("c++"), Ok(CodeLanguage::Cpp));
        assert_eq!(CodeLanguage::from_str("C#"), Ok(CodeLanguage::CSharp));
        assert_eq!(
            CodeLanguage::from_str("objective-c"),
            Ok(CodeLanguage::ObjectiveC)
        );
        assert_eq!(CodeLanguage::from_str("elisp"), Ok(CodeLanguage::EmacsLisp));
        assert_eq!(CodeLanguage::from_str("bash"), Ok(CodeLanguage::Bash));
        assert_eq!(CodeLanguage::from_str("wat"), Ok(CodeLanguage::WebAssembly));
        assert_eq!(CodeLanguage::from_str("yml"), Ok(CodeLanguage::Yaml));
    }

    #[test]
    fn canonical_ids_are_unique_and_round_trip() {
        let ids = CodeLanguage::ALL
            .into_iter()
            .map(CodeLanguage::as_str)
            .collect::<HashSet<_>>();

        assert_eq!(ids.len(), CodeLanguage::ALL.len());
        for language in CodeLanguage::ALL {
            assert_eq!(CodeLanguage::from_str(language.as_str()), Ok(language));
        }
    }
}
