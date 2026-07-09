use std::fmt;
use std::str::FromStr;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum CodeLanguage {
    #[default]
    PlainText,
    Rust,
    JavaScript,
    TypeScript,
    Json,
    Markdown,
    Html,
    Css,
    Nix,
    Shell,
    Toml,
    Yaml,
}

impl CodeLanguage {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::PlainText => "text",
            Self::Rust => "rust",
            Self::JavaScript => "javascript",
            Self::TypeScript => "typescript",
            Self::Json => "json",
            Self::Markdown => "markdown",
            Self::Html => "html",
            Self::Css => "css",
            Self::Nix => "nix",
            Self::Shell => "shell",
            Self::Toml => "toml",
            Self::Yaml => "yaml",
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            Self::PlainText => "Plain text",
            Self::Rust => "Rust",
            Self::JavaScript => "JavaScript",
            Self::TypeScript => "TypeScript",
            Self::Json => "JSON",
            Self::Markdown => "Markdown",
            Self::Html => "HTML",
            Self::Css => "CSS",
            Self::Nix => "Nix",
            Self::Shell => "Shell",
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
            "js" | "javascript" => Ok(Self::JavaScript),
            "ts" | "typescript" => Ok(Self::TypeScript),
            "json" => Ok(Self::Json),
            "md" | "markdown" => Ok(Self::Markdown),
            "html" | "htm" => Ok(Self::Html),
            "css" => Ok(Self::Css),
            "nix" => Ok(Self::Nix),
            "sh" | "shell" | "bash" => Ok(Self::Shell),
            "toml" => Ok(Self::Toml),
            "yaml" | "yml" => Ok(Self::Yaml),
            _ => Err(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::CodeLanguage;

    #[test]
    fn parses_common_aliases() {
        assert_eq!(CodeLanguage::from_str("rs"), Ok(CodeLanguage::Rust));
        assert_eq!(CodeLanguage::from_str("bash"), Ok(CodeLanguage::Shell));
        assert_eq!(CodeLanguage::from_str("yml"), Ok(CodeLanguage::Yaml));
    }

    #[test]
    fn exposes_css_safe_language_ids() {
        assert_eq!(CodeLanguage::Rust.as_str(), "rust");
        assert_eq!(CodeLanguage::PlainText.as_str(), "text");
    }
}
