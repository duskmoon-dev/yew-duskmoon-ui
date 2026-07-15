use crate::routes::components::catalog::ComponentSpec;
use crate::routes::components::detail::page::{ApiRow, ComponentPage};
use crate::routes::components::palette::{variant, PaletteColor};
use yew::prelude::*;
use yew_duskmoon::{CodeEditor, CodeLanguage};

const CODE_ENGINE_API: &[ApiRow] = &[
    ApiRow {
        prop: "class",
        ty: "Classes",
        default: "empty",
        docs: "Extra CSS classes appended to the editor root.",
    },
    ApiRow {
        prop: "value",
        ty: "Option<AttrValue>",
        default: "None",
        docs: "Controlled source text. When set, the host owns updates.",
    },
    ApiRow {
        prop: "default_value",
        ty: "AttrValue",
        default: "empty",
        docs: "Initial source text for uncontrolled use.",
    },
    ApiRow {
        prop: "placeholder",
        ty: "AttrValue",
        default: "empty",
        docs: "Placeholder shown while the editor is empty.",
    },
    ApiRow {
        prop: "on_change",
        ty: "Callback<AttrValue>",
        default: "noop",
        docs: "Emitted with the next source text after input.",
    },
    ApiRow {
        prop: "readonly",
        ty: "bool",
        default: "false",
        docs: "Prevents edits while preserving source selection.",
    },
    ApiRow {
        prop: "show_line_numbers",
        ty: "bool",
        default: "true",
        docs: "Shows the line gutter beside the source field.",
    },
    ApiRow {
        prop: "show_status_bar",
        ty: "bool",
        default: "true",
        docs: "Shows cursor position, selection count, language, and line count below the editor.",
    },
    ApiRow {
        prop: "syntax_highlight",
        ty: "bool",
        default: "true",
        docs: "Renders a Rust-powered syntax highlight layer behind the editable textarea.",
    },
    ApiRow {
        prop: "language",
        ty: "CodeLanguage",
        default: "PlainText",
        docs: "Language mode used by the tokenizer, CSS class, badge, and status bar.",
    },
    ApiRow {
        prop: "rows",
        ty: "usize",
        default: "12",
        docs: "Initial visible row count for the source field.",
    },
    ApiRow {
        prop: "variant",
        ty: "Option<String>",
        default: "None",
        docs: "Appends a color modifier class such as code-engine-primary.",
    },
    ApiRow {
        prop: "aria_label",
        ty: "AttrValue",
        default: "Code editor",
        docs: "Accessible label for the underlying source field.",
    },
];

fn language_sample(language: CodeLanguage) -> &'static str {
    match language {
        CodeLanguage::PlainText => {
            "Editor session\nMode: collaborative\nAutosave: enabled\nStatus: ready"
        }
        CodeLanguage::Rust => {
            r#"fn main() {
    let language: &str = "Rust";
    println!("Hello, {language}!");
}"#
        }
        CodeLanguage::Zig => {
            r#"const std = @import("std");
pub fn main() void {
    std.debug.print("Hello, Zig!\n", .{});
}"#
        }
        CodeLanguage::C => {
            r#"#include <stdio.h>
int main(void) {
    printf("Hello, C!\n");
    return 0;
}"#
        }
        CodeLanguage::Cpp => {
            r#"#include <iostream>
int main() {
    std::cout << "Hello, C++!\n";
    return 0;
}"#
        }
        CodeLanguage::ObjectiveC => {
            r#"#import <Foundation/Foundation.h>
int main(void) {
    @autoreleasepool { NSLog(@"Hello, Objective-C!"); }
    return 0;
}"#
        }
        CodeLanguage::CSharp => {
            r#"using System;
public record Greeting(string Name);
var greeting = new Greeting("C#");
Console.WriteLine($"Hello, {greeting.Name}!");"#
        }
        CodeLanguage::Java => {
            r#"public class Hello {
    public static void main(String[] args) {
        System.out.println("Hello, Java!");
    }
}"#
        }
        CodeLanguage::Kotlin => {
            r#"data class Greeting(val language: String)
fun main() {
    val greeting = Greeting("Kotlin")
    println("Hello, ${greeting.language}!")
}"#
        }
        CodeLanguage::Swift => {
            r#"let language: String = "Swift"
func welcome(_ name: String) -> String {
    return "Hello, \(name)!"
}
print(welcome(language))"#
        }
        CodeLanguage::Python => {
            r#"def welcome(language: str) -> str:
    return f"Hello, {language}!"

print(welcome("Python"))"#
        }
        CodeLanguage::Ruby => {
            r##"def welcome(language)
  "Hello, #{language}!"
end
puts welcome("Ruby")"##
        }
        CodeLanguage::Php => {
            r#"<?php
function welcome(string $language): string {
    return "Hello, {$language}!";
}"#
        }
        CodeLanguage::JavaScript => {
            r#"const greeting = "Hello";
function welcome(name) {
    return `${greeting}, ${name}!`;
}"#
        }
        CodeLanguage::TypeScript => {
            r#"const welcome = (name: string): string => {
    const greeting: string = "Hello";
    return `${greeting}, ${name}!`;
};"#
        }
        CodeLanguage::Json => {
            r#"{
  "theme": "moonlight",
  "lineNumbers": true,
  "fontSize": 15
}"#
        }
        CodeLanguage::Markdown => {
            r#"# Code Engine
- Rust-powered tokenization
- Precise cursor positioning
`Status: ready`"#
        }
        CodeLanguage::Html => {
            r#"<main class="editor">
  <h1>Code Engine</h1>
  <button type="button">Run</button>
</main>"#
        }
        CodeLanguage::Css => {
            r#".editor {
  display: grid;
  color: #d97706;
  background: #f8fafc;
}"#
        }
        CodeLanguage::Sass => {
            r#"$accent: #d97706
.editor
  display: grid
  color: $accent"#
        }
        CodeLanguage::Scss => {
            r#"$accent: #d97706;
.editor {
  display: grid;
  color: $accent;
}"#
        }
        CodeLanguage::Less => {
            r#"@accent: #d97706;
.editor {
  display: grid;
  color: @accent;
}"#
        }
        CodeLanguage::Elixir => {
            r##"defmodule Greeting do
  def welcome(language) do
    "Hello, #{language}!"
  end
end"##
        }
        CodeLanguage::Erlang => {
            r#"-module(greeting).
-export([welcome/1]).
welcome(Language) ->
    io:format("Hello, ~s!~n", [Language])."#
        }
        CodeLanguage::Lisp => {
            r#"(defun welcome (language)
  (format nil "Hello, ~a!" language))

(welcome "Lisp")"#
        }
        CodeLanguage::EmacsLisp => {
            r#"(defun welcome (language)
  (message "Hello, %s!" language))

(welcome "Emacs Lisp")"#
        }
        CodeLanguage::Shell => {
            r#"#!/bin/sh
set -eu
language="Shell"
printf 'Hello, %s!\n' "$language""#
        }
        CodeLanguage::Bash => {
            r#"#!/usr/bin/env bash
set -euo pipefail
language="Bash"
printf 'Hello, %s!\n' "$language""#
        }
        CodeLanguage::Zsh => {
            r#"#!/usr/bin/env zsh
setopt errexit nounset pipefail
language="Zsh"
print "Hello, ${language}!""#
        }
        CodeLanguage::Nix => {
            r#"{ pkgs, ... }:
{
  environment.systemPackages = [ pkgs.rustc ];
  services.editor.enable = true;
}"#
        }
        CodeLanguage::Assembly => {
            r#"section .text
global add_one
add_one:
    lea rax, [rdi + 1]
    ret"#
        }
        CodeLanguage::WebAssembly => {
            r#"(module
  (func (export "add") (param i32 i32) (result i32)
    local.get 0
    local.get 1
    i32.add))"#
        }
        CodeLanguage::Toml => {
            r#"[package]
name = "code-engine"
version = "0.2.0"
edition = "2021""#
        }
        CodeLanguage::Yaml => "editor:\n  language: rust\n  line_numbers: true\n  tab_size: 4",
    }
}

pub fn page(spec: &'static ComponentSpec) -> ComponentPage {
    ComponentPage::new(spec, usage, CODE_ENGINE_API, demo, color_variant)
}

fn usage(_: &ComponentSpec) -> String {
    "use yew_duskmoon::{CodeEditor, CodeLanguage};\n\nhtml! {\n    <CodeEditor\n        variant={Some(\"primary\".to_owned())}\n        language={CodeLanguage::Rust}\n        default_value={\"fn main() {\\n    println!(\\\"hello\\\");\\n}\"}\n    />\n}".to_owned()
}

fn demo(_: &ComponentSpec) -> Html {
    html! {
        <div class="code-engine-language-grid">
            { for CodeLanguage::ALL.into_iter().map(|language| html! {
                <CodeEditor
                    class="component-detail-language-code-engine"
                    language={language}
                    default_value={language_sample(language)}
                    rows={5}
                    aria_label={format!("{} source", language.label())}
                />
            }) }
        </div>
    }
}

fn color_variant(color: PaletteColor) -> Html {
    html! {
        <CodeEditor
            variant={variant(color)}
            class="component-detail-color-code-engine"
            language={CodeLanguage::Nix}
            default_value={format!("services.{} = {{\n  enable = true;\n}};", color.key)}
            rows={4}
            aria_label={format!("{} source", color.label)}
        />
    }
}
