use pulldown_cmark::{html, CodeBlockKind, CowStr, Event, LinkType, Options, Parser, Tag, TagEnd};
use yew::prelude::*;
use yew::virtual_dom::AttrValue;

/// Controls how YAML front matter at the start of a Markdown document is handled.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum FrontMatterMode {
    /// Render the extracted YAML through the fenced-code renderer before the body.
    #[default]
    Render,
    /// Remove the extracted YAML and render only the Markdown body.
    Hidden,
    /// Parse the complete source as ordinary Markdown without recognizing front matter.
    Disabled,
}

#[derive(Properties, Clone, PartialEq)]
pub struct DmMarkdownProps {
    /// Extra CSS classes appended to the Markdown root.
    #[prop_or_default]
    pub class: Classes,
    /// Whether safe raw HTML is passed through to the rendered output.
    #[prop_or(true)]
    pub allow_html: bool,
    /// Optional directory URL used to resolve relative Markdown and raw HTML URLs.
    #[prop_or_default]
    pub base_url: Option<String>,
    /// Custom element tag names that may be rendered instead of entity-escaped.
    #[prop_or_default]
    pub custom_elements: Vec<String>,
    /// Whether valid CSS colors in inline code receive a visual color chip.
    #[prop_or(true)]
    pub color_chips: bool,
    /// Controls extraction and presentation of YAML front matter.
    #[prop_or(FrontMatterMode::Render)]
    pub front_matter: FrontMatterMode,
    /// Markdown source to render.
    #[prop_or_default]
    pub markdown: AttrValue,
    /// Optional color variant used to build the `markdown-body-{variant}` class.
    #[prop_or_default]
    pub variant: Option<String>,
}

/// Options used by [`render_markdown_to_html_with_options`].
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DmMarkdownOptions {
    /// Whether safe raw HTML is passed through to the rendered output.
    pub allow_html: bool,
    /// Optional directory URL used to resolve relative Markdown and raw HTML URLs.
    pub base_url: Option<String>,
    /// Custom element tag names that may be rendered instead of entity-escaped.
    pub custom_elements: Vec<String>,
    /// Whether valid CSS colors in inline code receive a visual color chip.
    pub color_chips: bool,
    /// Controls extraction and presentation of YAML front matter.
    pub front_matter: FrontMatterMode,
}

impl Default for DmMarkdownOptions {
    fn default() -> Self {
        Self {
            allow_html: true,
            base_url: None,
            custom_elements: Vec::new(),
            color_chips: true,
            front_matter: FrontMatterMode::Render,
        }
    }
}

#[function_component(DmMarkdown)]
pub fn dm_markdown(props: &DmMarkdownProps) -> Html {
    let mut classes = classes!("markdown-body");
    if let Some(variant) = &props.variant {
        classes.push(format!("markdown-body-{}", variant));
    }
    classes.push(props.class.clone());

    let rendered = render_markdown_to_html_with_options(
        &props.markdown,
        DmMarkdownOptions {
            allow_html: props.allow_html,
            base_url: props.base_url.clone(),
            custom_elements: props.custom_elements.clone(),
            color_chips: props.color_chips,
            front_matter: props.front_matter,
        },
    );

    html! {
        <div class={classes}>
            { Html::from_html_unchecked(AttrValue::from(rendered)) }
        </div>
    }
}

pub fn render_markdown_to_html(markdown: &str) -> String {
    render_markdown_to_html_with_options(markdown, DmMarkdownOptions::default())
}

pub fn render_markdown_to_html_with_options(markdown: &str, options: DmMarkdownOptions) -> String {
    let front_matter = if options.front_matter == FrontMatterMode::Disabled {
        None
    } else {
        split_front_matter(markdown)
    };
    let body = front_matter
        .as_ref()
        .map_or(markdown, |front_matter| front_matter.body);

    let parser = coalesce_html_blocks(Parser::new_ext(body, markdown_options()))
        .map(|event| sanitize_event(event, &options));
    let parser = transform_inline_colors(parser, options.color_chips);
    let parser = render_special_blocks(parser);
    let mut output = String::new();

    if options.front_matter == FrontMatterMode::Render {
        if let Some(front_matter) = front_matter {
            output.push_str(&render_front_matter(front_matter.source));
        }
    }

    html::push_html(&mut output, parser.into_iter());
    output
}

#[derive(Debug, PartialEq, Eq)]
struct FrontMatter<'a> {
    source: &'a str,
    body: &'a str,
}

fn split_front_matter(markdown: &str) -> Option<FrontMatter<'_>> {
    let markdown = markdown.strip_prefix('\u{feff}').unwrap_or(markdown);
    let (opening, mut cursor, has_line_ending) = logical_line(markdown, 0)?;

    if opening != "---" || !has_line_ending {
        return None;
    }

    let source_start = cursor;
    while cursor < markdown.len() {
        let line_start = cursor;
        let (line, next, _) = logical_line(markdown, cursor)?;

        if matches!(line, "---" | "...") {
            return Some(FrontMatter {
                source: &markdown[source_start..line_start],
                body: &markdown[next..],
            });
        }

        cursor = next;
    }

    None
}

fn logical_line(source: &str, start: usize) -> Option<(&str, usize, bool)> {
    let rest = source.get(start..)?;
    if let Some(newline) = rest.find('\n') {
        let line_end = start + newline;
        let line = rest[..newline]
            .strip_suffix('\r')
            .unwrap_or(&rest[..newline]);
        Some((line, line_end + 1, true))
    } else {
        Some((rest, source.len(), false))
    }
}

fn markdown_options() -> Options {
    Options::ENABLE_TABLES
        | Options::ENABLE_STRIKETHROUGH
        | Options::ENABLE_TASKLISTS
        | Options::ENABLE_GFM
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum CssColor {
    Hex,
    Rgb,
    Rgba,
    Hsl,
    Hsla,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum ColorChannelUnit {
    Number,
    Percentage,
}

fn transform_inline_colors<'a>(
    events: impl IntoIterator<Item = Event<'a>>,
    enabled: bool,
) -> Vec<Event<'a>> {
    events
        .into_iter()
        .map(|event| match event {
            Event::Code(code) if enabled && parse_css_color(&code).is_some() => {
                Event::InlineHtml(CowStr::Boxed(render_color_chip(&code).into_boxed_str()))
            }
            event => event,
        })
        .collect()
}

fn parse_css_color(value: &str) -> Option<CssColor> {
    if value.is_empty() || !value.is_ascii() || value.trim() != value {
        return None;
    }

    if let Some(hex) = value.strip_prefix('#') {
        return (matches!(hex.len(), 3 | 4 | 6 | 8)
            && hex.chars().all(|ch| ch.is_ascii_hexdigit()))
        .then_some(CssColor::Hex);
    }

    let opening = value.find('(')?;
    let name = &value[..opening];
    let arguments = value.get(opening + 1..)?.strip_suffix(')')?;
    if name.is_empty() || arguments.contains('(') || arguments.contains(')') {
        return None;
    }

    let components = arguments.split(',').map(str::trim).collect::<Vec<_>>();
    match name.to_ascii_lowercase().as_str() {
        "rgb" if valid_rgb_components(&components, false) => Some(CssColor::Rgb),
        "rgba" if valid_rgb_components(&components, true) => Some(CssColor::Rgba),
        "hsl" if valid_hsl_components(&components, false) => Some(CssColor::Hsl),
        "hsla" if valid_hsl_components(&components, true) => Some(CssColor::Hsla),
        _ => None,
    }
}

fn valid_rgb_components(components: &[&str], has_alpha: bool) -> bool {
    if components.len() != if has_alpha { 4 } else { 3 } {
        return false;
    }

    let Some(first_unit) = parse_rgb_channel(components[0]) else {
        return false;
    };
    if !components[1..3]
        .iter()
        .all(|component| parse_rgb_channel(component) == Some(first_unit))
    {
        return false;
    }

    !has_alpha || parse_alpha(components[3])
}

fn valid_hsl_components(components: &[&str], has_alpha: bool) -> bool {
    if components.len() != if has_alpha { 4 } else { 3 } {
        return false;
    }

    parse_hue(components[0])
        && parse_percentage(components[1], 100.0)
        && parse_percentage(components[2], 100.0)
        && (!has_alpha || parse_alpha(components[3]))
}

fn parse_rgb_channel(value: &str) -> Option<ColorChannelUnit> {
    if let Some(percentage) = value.strip_suffix('%') {
        parse_number_in_range(percentage, 0.0, 100.0).then_some(ColorChannelUnit::Percentage)
    } else {
        parse_number_in_range(value, 0.0, 255.0).then_some(ColorChannelUnit::Number)
    }
}

fn parse_hue(value: &str) -> bool {
    let value = if value
        .get(value.len().saturating_sub(3)..)
        .is_some_and(|unit| unit.eq_ignore_ascii_case("deg"))
    {
        &value[..value.len() - 3]
    } else {
        value
    };

    parse_number_in_range(value, 0.0, 360.0)
}

fn parse_percentage(value: &str, maximum: f64) -> bool {
    value
        .strip_suffix('%')
        .is_some_and(|value| parse_number_in_range(value, 0.0, maximum))
}

fn parse_alpha(value: &str) -> bool {
    value.strip_suffix('%').map_or_else(
        || parse_number_in_range(value, 0.0, 1.0),
        |value| parse_number_in_range(value, 0.0, 100.0),
    )
}

fn parse_number_in_range(value: &str, minimum: f64, maximum: f64) -> bool {
    parse_css_number(value).is_some_and(|value| (minimum..=maximum).contains(&value))
}

fn parse_css_number(value: &str) -> Option<f64> {
    let bytes = value.as_bytes();
    let mut index = usize::from(matches!(bytes.first(), Some(b'+' | b'-')));
    let mut integer_digits = 0;

    while bytes.get(index).is_some_and(u8::is_ascii_digit) {
        integer_digits += 1;
        index += 1;
    }

    let mut fractional_digits = 0;
    if bytes.get(index) == Some(&b'.') {
        index += 1;
        while bytes.get(index).is_some_and(u8::is_ascii_digit) {
            fractional_digits += 1;
            index += 1;
        }
        if fractional_digits == 0 {
            return None;
        }
    }

    if index != bytes.len() || integer_digits + fractional_digits == 0 {
        return None;
    }

    value.parse::<f64>().ok().filter(|value| value.is_finite())
}

const COLOR_CODE_STYLE: &str =
    "display:inline-flex;align-items:center;gap:0.35em;white-space:nowrap;vertical-align:middle;";
const COLOR_CHIP_STYLE: &str = concat!(
    "position:relative;",
    "display:inline-block;",
    "flex:0 0 auto;",
    "width:1em;",
    "height:1em;",
    "overflow:hidden;",
    "border:1px solid color-mix(in oklch, var(--color-base-content, #111827) 56%, var(--color-surface, #ffffff));",
    "border-radius:3px;",
    "background-color:var(--color-surface, #ffffff);",
    "background-image:conic-gradient(color-mix(in oklch, var(--color-base-content, #111827) 22%, transparent) 25%, transparent 0 50%, color-mix(in oklch, var(--color-base-content, #111827) 22%, transparent) 0 75%, transparent 0);",
    "background-size:6px 6px;",
    "box-shadow:0 0 0 1px color-mix(in oklch, var(--color-surface, #ffffff) 68%, transparent);",
);
const COLOR_CHIP_SWATCH_STYLE: &str = "position:absolute;inset:0;";

fn render_color_chip(value: &str) -> String {
    let text = escape_html(value);
    let attribute = escape_attribute(value);
    format!(
        "<code class=\"dm-color-code\" style=\"{COLOR_CODE_STYLE}\">{text}<span class=\"dm-color-chip\" role=\"img\" aria-label=\"Color {attribute}\" style=\"{COLOR_CHIP_STYLE}\"><span class=\"dm-color-chip-swatch\" style=\"{COLOR_CHIP_SWATCH_STYLE}background-color:{attribute};\" aria-hidden=\"true\"></span></span></code>"
    )
}

fn coalesce_html_blocks<'a, I>(events: I) -> impl Iterator<Item = Event<'a>>
where
    I: IntoIterator<Item = Event<'a>>,
{
    let mut events = events.into_iter().peekable();

    std::iter::from_fn(move || {
        let event = events.next()?;
        let Event::Html(first) = event else {
            return Some(event);
        };

        if !matches!(events.peek(), Some(Event::Html(_))) {
            return Some(Event::Html(first));
        }

        let mut html = first.into_string();
        while matches!(events.peek(), Some(Event::Html(_))) {
            let Some(Event::Html(next)) = events.next() else {
                unreachable!("peeked HTML event must still be present");
            };
            html.push_str(&next);
        }

        Some(Event::Html(CowStr::Boxed(html.into_boxed_str())))
    })
}

fn sanitize_event<'a>(event: Event<'a>, options: &DmMarkdownOptions) -> Event<'a> {
    match event {
        Event::Html(html) => sanitize_html_event(html, false, options),
        Event::InlineHtml(html) => sanitize_html_event(html, true, options),
        Event::Start(Tag::Link {
            link_type,
            dest_url,
            title,
            id,
        }) => Event::Start(Tag::Link {
            link_type,
            dest_url: if link_type == LinkType::Email {
                safe_url(dest_url)
            } else {
                resolve_url(dest_url, options.base_url.as_deref())
            },
            title,
            id,
        }),
        Event::Start(Tag::Image {
            link_type,
            dest_url,
            title,
            id,
        }) => Event::Start(Tag::Image {
            link_type,
            dest_url: resolve_url(dest_url, options.base_url.as_deref()),
            title,
            id,
        }),
        event => event,
    }
}

fn sanitize_html_event<'a>(
    html: CowStr<'a>,
    inline: bool,
    options: &DmMarkdownOptions,
) -> Event<'a> {
    if !options.allow_html {
        return Event::Text(html);
    }

    let html = escape_disabled_html_tags(&html, options)
        .map(|sanitized| CowStr::Boxed(sanitized.into_boxed_str()))
        .unwrap_or(html);
    let html = options
        .base_url
        .as_deref()
        .and_then(|base_url| resolve_raw_html_urls(&html, base_url))
        .map(|resolved| CowStr::Boxed(resolved.into_boxed_str()))
        .unwrap_or(html);

    if inline {
        Event::InlineHtml(html)
    } else {
        Event::Html(html)
    }
}

struct RawHtmlTag<'a> {
    name: Option<&'a str>,
    attributes_start: usize,
    end: usize,
    closing: bool,
    self_closing: bool,
}

fn escape_disabled_html_tags(html: &str, options: &DmMarkdownOptions) -> Option<String> {
    let mut output = String::new();
    let mut cursor = 0;
    let mut search = 0;
    let mut changed = false;

    while let Some(relative_start) = html[search..].find('<') {
        let start = search + relative_start;
        let Some(tag) = parse_raw_html_tag(html, start) else {
            search = start + 1;
            continue;
        };

        let Some(name) = tag.name else {
            search = tag.end;
            continue;
        };

        if should_escape_html_tag(name, options) {
            let escape_end = if tag.closing || tag.self_closing {
                tag.end
            } else {
                find_closing_html_tag_end(html, name, tag.end).unwrap_or(tag.end)
            };

            output.push_str(&html[cursor..start]);
            output.push_str(&escape_html(&html[start..escape_end]));
            cursor = escape_end;
            search = escape_end;
            changed = true;
        } else if !tag.closing && is_raw_text_html_tag(name) {
            search = raw_text_html_tag_end(html, name, tag.end);
        } else {
            search = tag.end;
        }
    }

    if changed {
        output.push_str(&html[cursor..]);
        Some(output)
    } else {
        None
    }
}

fn parse_raw_html_tag(html: &str, start: usize) -> Option<RawHtmlTag<'_>> {
    let rest = html.get(start + 1..)?;

    if rest.starts_with("!--") {
        let end = rest.find("-->").map(|index| start + 1 + index + 3)?;
        return Some(RawHtmlTag {
            name: None,
            attributes_start: end,
            end,
            closing: false,
            self_closing: false,
        });
    }

    let first = rest.chars().next()?;
    if matches!(first, '!' | '?') {
        let end = find_quoted_html_tag_end(html, start + 1)?;
        return Some(RawHtmlTag {
            name: None,
            attributes_start: end,
            end,
            closing: false,
            self_closing: false,
        });
    }

    let closing = rest.starts_with('/');
    let mut name_start = start + 1 + usize::from(closing);

    while let Some(ch) = html.get(name_start..)?.chars().next() {
        if ch.is_ascii_whitespace() {
            name_start += ch.len_utf8();
        } else {
            break;
        }
    }

    let name_end = html[name_start..]
        .char_indices()
        .find_map(|(index, ch)| (!is_html_tag_name_char(ch)).then_some(name_start + index))
        .unwrap_or(html.len());

    if name_end == name_start {
        return None;
    }

    let end = find_html_tag_end(html, name_end)?;
    let self_closing = html[start..end].trim_end().ends_with("/>");

    Some(RawHtmlTag {
        name: Some(&html[name_start..name_end]),
        attributes_start: name_end,
        end,
        closing,
        self_closing,
    })
}

fn resolve_raw_html_urls(html: &str, base_url: &str) -> Option<String> {
    let mut output = String::new();
    let mut output_cursor = 0;
    let mut search = 0;
    let mut changed = false;

    while let Some(relative_start) = html[search..].find('<') {
        let start = search + relative_start;
        let Some(tag) = parse_raw_html_tag(html, start) else {
            search = start + 1;
            continue;
        };
        let Some(tag_name) = tag.name else {
            search = tag.end;
            continue;
        };
        if tag.closing {
            search = tag.end;
            continue;
        }

        let mut cursor = tag.attributes_start;
        while cursor < tag.end {
            cursor = skip_ascii_whitespace(html, cursor, tag.end);
            let Some(ch) = html[cursor..tag.end].chars().next() else {
                break;
            };
            if matches!(ch, '>' | '/') {
                break;
            }

            let name_start = cursor;
            while let Some(ch) = html[cursor..tag.end].chars().next() {
                if ch.is_ascii_whitespace() || matches!(ch, '=' | '>' | '/') {
                    break;
                }
                cursor += ch.len_utf8();
            }
            let name_end = cursor;

            if name_start == name_end {
                cursor += ch.len_utf8();
                continue;
            }

            cursor = skip_ascii_whitespace(html, cursor, tag.end);
            if html.as_bytes().get(cursor) != Some(&b'=') {
                continue;
            }

            cursor += 1;
            cursor = skip_ascii_whitespace(html, cursor, tag.end);

            let value_with_quotes_start = cursor;
            let quote = html[cursor..tag.end]
                .chars()
                .next()
                .filter(|ch| matches!(ch, '\'' | '"'));
            if quote.is_some() {
                cursor += 1;
            }

            let value_start = cursor;
            if let Some(quote) = quote {
                while let Some(ch) = html[cursor..tag.end].chars().next() {
                    if ch == quote {
                        break;
                    }
                    cursor += ch.len_utf8();
                }
            } else {
                while let Some(ch) = html[cursor..tag.end].chars().next() {
                    if ch.is_ascii_whitespace() || ch == '>' {
                        break;
                    }
                    cursor += ch.len_utf8();
                }
            }
            let value_end = cursor;
            let value_with_quotes_end = if quote.is_some() && cursor < tag.end {
                cursor + 1
            } else {
                cursor
            };

            let name = &html[name_start..name_end];
            if name.eq_ignore_ascii_case("href") || name.eq_ignore_ascii_case("src") {
                let decoded = htmlize::unescape_attribute(&html[value_start..value_end]);
                let resolved = resolve_url(CowStr::Borrowed(&decoded), Some(base_url));
                if resolved.as_ref() != decoded.as_ref() {
                    output.push_str(&html[output_cursor..value_with_quotes_start]);
                    push_raw_html_attribute_value(&mut output, &resolved);
                    output_cursor = value_with_quotes_end;
                    changed = true;
                }
            }

            if quote.is_some() && cursor < tag.end {
                cursor += 1;
            }
        }

        search = if is_raw_text_html_tag(tag_name) {
            raw_text_html_tag_end(html, tag_name, tag.end)
        } else {
            tag.end
        };
    }

    if changed {
        output.push_str(&html[output_cursor..]);
        Some(output)
    } else {
        None
    }
}

fn skip_ascii_whitespace(html: &str, mut cursor: usize, end: usize) -> usize {
    while let Some(ch) = html[cursor..end].chars().next() {
        if !ch.is_ascii_whitespace() {
            break;
        }
        cursor += ch.len_utf8();
    }
    cursor
}

fn push_raw_html_attribute_value(output: &mut String, value: &str) {
    output.push('"');
    output.push_str(&escape_attribute(value));
    output.push('"');
}

fn find_html_tag_end(html: &str, from: usize) -> Option<usize> {
    enum State {
        BetweenAttributes,
        BeforeValue,
        QuotedValue(char),
        UnquotedValue,
    }

    let mut state = State::BetweenAttributes;

    for (offset, ch) in html[from..].char_indices() {
        match state {
            State::QuotedValue(quote) if ch == quote => state = State::BetweenAttributes,
            State::QuotedValue(_) => {}
            State::BeforeValue if ch.is_ascii_whitespace() => {}
            State::BeforeValue if matches!(ch, '"' | '\'') => state = State::QuotedValue(ch),
            State::BeforeValue if ch == '>' => {
                return Some(from + offset + ch.len_utf8());
            }
            State::BeforeValue => state = State::UnquotedValue,
            State::UnquotedValue if ch == '>' => {
                return Some(from + offset + ch.len_utf8());
            }
            State::UnquotedValue if ch.is_ascii_whitespace() => {
                state = State::BetweenAttributes;
            }
            State::UnquotedValue => {}
            State::BetweenAttributes if ch == '=' => state = State::BeforeValue,
            State::BetweenAttributes if ch == '>' => {
                return Some(from + offset + ch.len_utf8());
            }
            State::BetweenAttributes => {}
        }
    }

    None
}

fn find_quoted_html_tag_end(html: &str, from: usize) -> Option<usize> {
    let mut quote = None;

    for (offset, ch) in html[from..].char_indices() {
        match quote {
            Some(quote_ch) if ch == quote_ch => quote = None,
            Some(_) => {}
            None if matches!(ch, '"' | '\'') => quote = Some(ch),
            None if ch == '>' => return Some(from + offset + ch.len_utf8()),
            None => {}
        }
    }

    None
}

fn find_closing_html_tag_end(html: &str, tag_name: &str, from: usize) -> Option<usize> {
    let mut search = from;

    while let Some(relative_start) = html[search..].find('<') {
        let start = search + relative_start;
        let Some(tag) = parse_raw_html_tag(html, start) else {
            search = start + 1;
            continue;
        };

        if tag.closing
            && tag
                .name
                .is_some_and(|name| name.eq_ignore_ascii_case(tag_name))
        {
            return Some(tag.end);
        }

        search = tag.end;
    }

    None
}

fn is_raw_text_html_tag(tag: &str) -> bool {
    matches!(
        tag.to_ascii_lowercase().as_str(),
        "iframe" | "noembed" | "noframes" | "plaintext" | "textarea" | "title" | "xmp"
    )
}

fn raw_text_html_tag_end(html: &str, tag_name: &str, from: usize) -> usize {
    if tag_name.eq_ignore_ascii_case("plaintext") {
        html.len()
    } else {
        find_closing_html_tag_end(html, tag_name, from).unwrap_or(html.len())
    }
}

fn is_unsafe_html_tag(tag: &str) -> bool {
    matches!(
        tag.to_ascii_lowercase().as_str(),
        "object" | "script" | "style"
    )
}

fn should_escape_html_tag(tag: &str, options: &DmMarkdownOptions) -> bool {
    is_unsafe_html_tag(tag)
        || (is_custom_element_tag(tag) && !is_custom_element_enabled(tag, options))
}

fn is_custom_element_tag(tag: &str) -> bool {
    tag.contains('-')
}

fn is_custom_element_enabled(tag: &str, options: &DmMarkdownOptions) -> bool {
    options
        .custom_elements
        .iter()
        .any(|enabled| enabled.eq_ignore_ascii_case(tag))
}

fn is_html_tag_name_char(ch: char) -> bool {
    ch.is_ascii_alphanumeric() || matches!(ch, '-' | ':')
}

struct FencedBlock {
    language: String,
    source: String,
}

fn render_special_blocks<'a>(events: impl IntoIterator<Item = Event<'a>>) -> Vec<Event<'static>> {
    let mut rendered = Vec::new();
    let mut fenced_block: Option<FencedBlock> = None;

    for event in events {
        if let Some(block) = &mut fenced_block {
            match event {
                Event::Text(text) => block.source.push_str(&text),
                Event::SoftBreak | Event::HardBreak => block.source.push('\n'),
                Event::End(TagEnd::CodeBlock) => {
                    rendered.push(Event::Html(CowStr::Boxed(
                        render_fenced_block(&block.language, &block.source).into_boxed_str(),
                    )));
                    fenced_block = None;
                }
                _ => {}
            }
            continue;
        }

        match event {
            Event::Start(Tag::CodeBlock(kind)) => {
                fenced_block = Some(FencedBlock {
                    language: code_block_language(kind),
                    source: String::new(),
                });
            }
            event => rendered.push(event.into_static()),
        }
    }

    if let Some(block) = fenced_block {
        rendered.push(Event::Html(CowStr::Boxed(
            render_fenced_block(&block.language, &block.source).into_boxed_str(),
        )));
    }

    rendered
}

fn code_block_language(kind: CodeBlockKind<'_>) -> String {
    match kind {
        CodeBlockKind::Fenced(info) => info.split_whitespace().next().unwrap_or("").to_owned(),
        CodeBlockKind::Indented => String::new(),
    }
}

fn render_front_matter(source: &str) -> String {
    format!(
        "<div class=\"dm-front-matter\">{}</div>",
        render_fenced_block("yaml", source)
    )
}

fn render_fenced_block(language: &str, source: &str) -> String {
    let language = normalize_language(language);
    if matches!(language, "mermaid" | "mmd") {
        return render_mermaid_chart(source);
    }

    let label = language_label(language);
    let highlighted = highlight_source(language, source);
    let class_language = escape_attribute(language);

    format!(
        "<div class=\"dm-code-block\" data-language=\"{class_language}\"><div class=\"dm-code-block-header\"><span>{label}</span></div><pre><code class=\"language-{class_language}\">{highlighted}</code></pre></div>"
    )
}

fn normalize_language(language: &str) -> &str {
    match language.trim().to_ascii_lowercase().as_str() {
        "ex" | "exs" | "elixir" => "elixir",
        "go" | "golang" => "go",
        "rs" | "rust" => "rust",
        "zig" => "zig",
        "ts" | "tsx" | "typescript" => "typescript",
        "yaml" | "yml" => "yaml",
        "mermaid" | "mmd" => "mermaid",
        _ => "text",
    }
}

fn language_label(language: &str) -> &'static str {
    match language {
        "elixir" => "Elixir",
        "go" => "Go",
        "rust" => "Rust",
        "typescript" => "TypeScript",
        "yaml" => "YAML",
        "zig" => "Zig",
        _ => "Text",
    }
}

fn render_mermaid_chart(source: &str) -> String {
    match detect_mermaid_kind(source) {
        MermaidKind::Flowchart => render_mermaid_flowchart(source),
        MermaidKind::Swimlanes => render_mermaid_swimlanes(source),
        MermaidKind::Sequence => render_mermaid_sequence(source),
        MermaidKind::Class => render_mermaid_class(source),
        MermaidKind::State => render_mermaid_state(source),
        MermaidKind::EntityRelationship => render_mermaid_er(source),
        MermaidKind::Journey => render_mermaid_journey(source),
        MermaidKind::Pie => render_mermaid_pie(source),
        MermaidKind::Gantt => render_mermaid_gantt(source),
        MermaidKind::Requirement => render_mermaid_requirement(source),
        MermaidKind::GitGraph => render_mermaid_git_graph(source),
        MermaidKind::C4 => render_mermaid_c4(source),
        MermaidKind::Timeline => render_mermaid_timeline(source),
        MermaidKind::ZenUml => render_mermaid_zenuml(source),
        MermaidKind::Sankey => render_mermaid_sankey(source),
        MermaidKind::XyChart => render_mermaid_xy_chart(source),
        MermaidKind::Block => render_mermaid_block(source),
        MermaidKind::Packet => render_mermaid_packet(source),
        MermaidKind::Kanban => render_mermaid_kanban(source),
        MermaidKind::Architecture => render_mermaid_architecture(source),
        MermaidKind::Radar => render_mermaid_radar(source),
        MermaidKind::EventModeling => render_mermaid_event_modeling(source),
        MermaidKind::Treemap => render_mermaid_treemap(source),
        MermaidKind::Venn => render_mermaid_venn(source),
        MermaidKind::Ishikawa => render_mermaid_ishikawa(source),
        MermaidKind::Wardley => render_mermaid_wardley(source),
        MermaidKind::Cynefin => render_mermaid_cynefin(source),
        MermaidKind::TreeView => render_mermaid_treeview(source),
        MermaidKind::Mindmap => render_mermaid_mindmap(source),
        MermaidKind::Quadrant => render_mermaid_quadrant(source),
        MermaidKind::Unknown => render_plain_code("mermaid", source),
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum MermaidKind {
    Flowchart,
    Swimlanes,
    Sequence,
    Class,
    State,
    EntityRelationship,
    Journey,
    Gantt,
    Pie,
    GitGraph,
    C4,
    Mindmap,
    Timeline,
    ZenUml,
    Sankey,
    XyChart,
    Block,
    Packet,
    Kanban,
    Architecture,
    Radar,
    EventModeling,
    Treemap,
    Venn,
    Ishikawa,
    Wardley,
    Cynefin,
    TreeView,
    Quadrant,
    Requirement,
    Unknown,
}

fn detect_mermaid_kind(source: &str) -> MermaidKind {
    let first_line = source
        .lines()
        .map(str::trim)
        .find(|line| !line.is_empty() && !line.starts_with("%%"))
        .unwrap_or("");
    let first_line = first_line.to_ascii_lowercase();

    if first_line.starts_with("flowchart ") || first_line.starts_with("graph ") {
        MermaidKind::Flowchart
    } else if first_line.starts_with("swimlanes") || first_line.starts_with("swimlane") {
        MermaidKind::Swimlanes
    } else if first_line.starts_with("sequencediagram") {
        MermaidKind::Sequence
    } else if first_line.starts_with("classdiagram") {
        MermaidKind::Class
    } else if first_line.starts_with("statediagram") {
        MermaidKind::State
    } else if first_line.starts_with("erdiagram") {
        MermaidKind::EntityRelationship
    } else if first_line.starts_with("journey") {
        MermaidKind::Journey
    } else if first_line.starts_with("gantt") {
        MermaidKind::Gantt
    } else if first_line.starts_with("pie") {
        MermaidKind::Pie
    } else if first_line.starts_with("quadrantchart") {
        MermaidKind::Quadrant
    } else if first_line.starts_with("requirementdiagram") {
        MermaidKind::Requirement
    } else if first_line.starts_with("gitgraph") {
        MermaidKind::GitGraph
    } else if first_line.starts_with("c4context")
        || first_line.starts_with("c4container")
        || first_line.starts_with("c4component")
        || first_line.starts_with("c4dynamic")
        || first_line.starts_with("c4deployment")
    {
        MermaidKind::C4
    } else if first_line.starts_with("mindmap") {
        MermaidKind::Mindmap
    } else if first_line.starts_with("timeline") {
        MermaidKind::Timeline
    } else if first_line.starts_with("zenuml") {
        MermaidKind::ZenUml
    } else if first_line.starts_with("sankey") {
        MermaidKind::Sankey
    } else if first_line.starts_with("xychart") {
        MermaidKind::XyChart
    } else if first_line.starts_with("block") {
        MermaidKind::Block
    } else if first_line.starts_with("packet") {
        MermaidKind::Packet
    } else if first_line.starts_with("kanban") {
        MermaidKind::Kanban
    } else if first_line.starts_with("architecture") {
        MermaidKind::Architecture
    } else if first_line.starts_with("radar") {
        MermaidKind::Radar
    } else if first_line.starts_with("eventmodeling") {
        MermaidKind::EventModeling
    } else if first_line.starts_with("treemap") {
        MermaidKind::Treemap
    } else if first_line.starts_with("venn") {
        MermaidKind::Venn
    } else if first_line.starts_with("ishikawa") {
        MermaidKind::Ishikawa
    } else if first_line.starts_with("wardley") {
        MermaidKind::Wardley
    } else if first_line.starts_with("cynefin") {
        MermaidKind::Cynefin
    } else if first_line.starts_with("treeview") {
        MermaidKind::TreeView
    } else {
        MermaidKind::Unknown
    }
}

fn render_mermaid_flowchart(source: &str) -> String {
    let mut nodes: Vec<(String, String)> = Vec::new();
    let mut edges: Vec<(String, String)> = Vec::new();

    for line in source.lines().map(str::trim) {
        if line.is_empty()
            || line.starts_with("graph ")
            || line.starts_with("flowchart ")
            || line.starts_with("sequenceDiagram")
        {
            continue;
        }

        if let Some((from, to)) = line.split_once("-->") {
            let from = parse_mermaid_node(from);
            let to = parse_mermaid_node(to);
            upsert_mermaid_node(&mut nodes, &from);
            upsert_mermaid_node(&mut nodes, &to);
            edges.push((from.0, to.0));
        }
    }

    if nodes.is_empty() {
        return render_plain_code("mermaid", source);
    }

    let count = nodes.len().max(1);
    let step = if count == 1 {
        0.0
    } else {
        480.0 / (count - 1) as f32
    };

    let positions = nodes
        .iter()
        .enumerate()
        .map(|(index, (id, label))| {
            let x = 80.0 + step * index as f32;
            let y = if index % 2 == 0 { 78.0 } else { 150.0 };
            (id.as_str(), label.as_str(), x, y)
        })
        .collect::<Vec<_>>();

    let marker_id = mermaid_marker_id(source);
    let mut svg = format!(
        "<div class=\"dm-mermaid-chart dm-mermaid-flowchart\" role=\"img\" aria-label=\"Rendered Mermaid flowchart\"><div class=\"dm-mermaid-chart-title\">Mermaid flowchart</div><svg viewBox=\"0 0 640 230\" aria-hidden=\"true\"><defs><marker id=\"{marker_id}\" markerWidth=\"10\" markerHeight=\"10\" refX=\"8\" refY=\"3\" orient=\"auto\"><path d=\"M0,0 L0,6 L9,3 z\" class=\"dm-mermaid-arrow\" /></marker></defs>"
    );

    let mut edge_markup = String::new();
    for (from, to) in edges {
        if let (Some((_, _, from_x, from_y)), Some((_, _, to_x, to_y))) = (
            positions.iter().find(|(id, _, _, _)| *id == from),
            positions.iter().find(|(id, _, _, _)| *id == to),
        ) {
            let (start_x, start_y) = rect_edge_point(*from_x, *from_y, *to_x, *to_y, 136.0, 48.0);
            let (end_x, end_y) = rect_edge_point(*to_x, *to_y, *from_x, *from_y, 136.0, 48.0);
            edge_markup.push_str(&format!(
                "<line class=\"dm-mermaid-edge\" x1=\"{start_x:.1}\" y1=\"{start_y:.1}\" x2=\"{end_x:.1}\" y2=\"{end_y:.1}\" marker-end=\"url(#{marker_id})\" />"
            ));
        }
    }

    for (_, label, x, y) in positions {
        let text = escape_html(label);
        svg.push_str(&format!(
            "<g class=\"dm-mermaid-node\"><rect x=\"{:.1}\" y=\"{:.1}\" width=\"136\" height=\"48\" rx=\"8\" /><text x=\"{:.1}\" y=\"{:.1}\" text-anchor=\"middle\">{text}</text></g>",
            x - 68.0,
            y - 24.0,
            x,
            y + 5.0
        ));
    }

    svg.push_str(&edge_markup);
    svg.push_str("</svg></div>");
    svg
}

struct Swimlane {
    label: String,
    nodes: Vec<String>,
}

struct SwimlaneNode {
    id: String,
    label: String,
}

struct SwimlaneEdge {
    from: String,
    to: String,
    label: Option<String>,
}

fn render_mermaid_swimlanes(source: &str) -> String {
    let mut lanes: Vec<Swimlane> = Vec::new();
    let mut nodes: Vec<SwimlaneNode> = Vec::new();
    let mut edges: Vec<SwimlaneEdge> = Vec::new();
    let mut current_lane = None;

    for line in source.lines().map(str::trim) {
        if line.is_empty()
            || line.starts_with("%%")
            || line.starts_with("swimlane")
            || line.starts_with("swimlanes")
        {
            continue;
        }

        if let Some(label) = parse_swimlane_start(line) {
            lanes.push(Swimlane {
                label,
                nodes: Vec::new(),
            });
            current_lane = Some(lanes.len() - 1);
            continue;
        }

        if line == "end" {
            current_lane = None;
            continue;
        }

        if let Some(edge) = parse_swimlane_edge(line) {
            edges.push(edge);
            continue;
        }

        if let Some(lane_index) = current_lane {
            let (id, label) = parse_mermaid_node(line);
            if id.is_empty() {
                continue;
            }

            if !lanes[lane_index].nodes.iter().any(|node| node == &id) {
                lanes[lane_index].nodes.push(id.clone());
            }
            if !nodes.iter().any(|node| node.id == id) {
                nodes.push(SwimlaneNode { id, label });
            }
        }
    }

    if lanes.is_empty() || nodes.is_empty() {
        return render_mermaid_summary("Swimlanes diagram", "swimlanes", source);
    }

    let width = 760.0;
    let lane_label_width = 58.0;
    let lane_height = 118.0;
    let top = 18.0;
    let node_width = 126.0;
    let node_height = 44.0;
    let height = top * 2.0 + lanes.len() as f32 * lane_height;
    let first_node_x = lane_label_width + 76.0;
    let last_node_x = width - 76.0;
    let usable_width = last_node_x - first_node_x;
    let marker_id = mermaid_marker_id(source);
    let mut positions: Vec<(String, String, f32, f32)> = Vec::new();

    for (lane_index, lane) in lanes.iter().enumerate() {
        let count = lane.nodes.len();
        for (node_index, node_id) in lane.nodes.iter().enumerate() {
            let Some(node) = nodes.iter().find(|node| &node.id == node_id) else {
                continue;
            };
            let x = if count <= 1 {
                (first_node_x + last_node_x) / 2.0
            } else {
                first_node_x + usable_width * node_index as f32 / (count - 1) as f32
            };
            let y = top + lane_index as f32 * lane_height + lane_height / 2.0;
            positions.push((node.id.clone(), node.label.clone(), x, y));
        }
    }

    let mut svg = format!(
        "<div class=\"dm-mermaid-chart dm-mermaid-swimlanes\" role=\"img\" aria-label=\"Rendered Mermaid swimlanes diagram\"><div class=\"dm-mermaid-chart-title\">Mermaid Swimlanes diagram</div><svg viewBox=\"0 0 {width:.0} {height:.0}\" aria-hidden=\"true\"><defs><marker id=\"{marker_id}\" markerWidth=\"10\" markerHeight=\"10\" refX=\"8\" refY=\"3\" orient=\"auto\"><path d=\"M0,0 L0,6 L9,3 z\" class=\"dm-mermaid-arrow\" /></marker></defs>"
    );

    for (index, lane) in lanes.iter().enumerate() {
        let y = top + index as f32 * lane_height;
        let label_y = y + lane_height / 2.0;
        svg.push_str(&format!(
            "<g class=\"dm-swimlane-lane\"><rect x=\"0\" y=\"{y:.1}\" width=\"{width:.0}\" height=\"{lane_height:.1}\" /><text x=\"{:.1}\" y=\"{label_y:.1}\" text-anchor=\"middle\" transform=\"rotate(-90 {:.1} {label_y:.1})\">{}</text></g>",
            lane_label_width / 2.0,
            lane_label_width / 2.0,
            escape_html(&lane.label)
        ));
    }

    for edge in &edges {
        let Some((_, _, from_x, from_y)) = positions.iter().find(|(id, _, _, _)| id == &edge.from)
        else {
            continue;
        };
        let Some((_, _, to_x, to_y)) = positions.iter().find(|(id, _, _, _)| id == &edge.to) else {
            continue;
        };
        let (start_x, start_y) =
            rect_edge_point(*from_x, *from_y, *to_x, *to_y, node_width, node_height);
        let (end_x, end_y) =
            rect_edge_point(*to_x, *to_y, *from_x, *from_y, node_width, node_height);
        let label_x = (start_x + end_x) / 2.0;
        let label_y = (start_y + end_y) / 2.0 - 6.0;

        svg.push_str(&format!(
            "<line class=\"dm-mermaid-edge dm-swimlane-edge\" x1=\"{start_x:.1}\" y1=\"{start_y:.1}\" x2=\"{end_x:.1}\" y2=\"{end_y:.1}\" marker-end=\"url(#{marker_id})\" />"
        ));
        if let Some(label) = &edge.label {
            svg.push_str(&format!(
                "<text class=\"dm-swimlane-edge-label\" x=\"{label_x:.1}\" y=\"{label_y:.1}\" text-anchor=\"middle\">{}</text>",
                escape_html(label)
            ));
        }
    }

    for (id, label, x, y) in positions {
        svg.push_str(&format!(
            "<g class=\"dm-swimlane-node\" data-node-id=\"{}\"><rect x=\"{:.1}\" y=\"{:.1}\" width=\"{node_width:.1}\" height=\"{node_height:.1}\" rx=\"2\" /><text x=\"{x:.1}\" y=\"{:.1}\" text-anchor=\"middle\">{}</text></g>",
            escape_attribute(&id),
            x - node_width / 2.0,
            y - node_height / 2.0,
            y + 5.0,
            escape_html(&label)
        ));
    }

    svg.push_str("</svg></div>");
    svg
}

fn parse_swimlane_start(line: &str) -> Option<String> {
    let body = line.strip_prefix("subgraph ")?.trim();
    if body.is_empty() {
        return None;
    }

    if let Some(start) = body.find('[') {
        if let Some(end) = body.rfind(']') {
            return Some(clean_mermaid_label(&body[start + 1..end]));
        }
    }

    Some(clean_mermaid_label(body))
}

fn parse_swimlane_edge(line: &str) -> Option<SwimlaneEdge> {
    let (from, to) = line.trim_end_matches(';').split_once("-->")?;
    let (to, label) = if let Some(labelled) = to.trim().strip_prefix('|') {
        let (label, to) = labelled.split_once('|')?;
        (to, Some(clean_mermaid_label(label)))
    } else {
        (to, None)
    };
    let (from, _) = parse_mermaid_node(from);
    let (to, _) = parse_mermaid_node(to);

    (!from.is_empty() && !to.is_empty()).then_some(SwimlaneEdge { from, to, label })
}

struct MermaidClass {
    id: String,
    label: String,
    namespace: Option<String>,
    members: Vec<String>,
}

struct ClassRelation {
    from: String,
    to: String,
    label: Option<String>,
}

struct ClassGroupLayout {
    namespace: Option<String>,
    x: f32,
    y: f32,
    width: f32,
    height: f32,
}

struct ClassPosition {
    id: String,
    label: String,
    members: Vec<String>,
    x: f32,
    y: f32,
    width: f32,
    height: f32,
}

fn render_mermaid_class(source: &str) -> String {
    let mut classes = Vec::new();
    let mut relations = Vec::new();
    let mut namespaces: Vec<String> = Vec::new();
    let mut current_class: Option<String> = None;

    for line in source.lines().map(str::trim) {
        if line.is_empty()
            || line.starts_with("%%")
            || line.starts_with("classDiagram")
            || line.starts_with("direction ")
        {
            continue;
        }

        if line == "}" {
            if current_class.take().is_none() {
                namespaces.pop();
            }
            continue;
        }

        if let Some(class_id) = current_class.clone() {
            add_class_member(&mut classes, &class_id, line);
            continue;
        }

        if let Some(namespace) = parse_class_namespace_start(line) {
            let namespace = if namespaces.is_empty() || namespace.contains('.') {
                namespace
            } else {
                format!("{}.{}", namespaces.join("."), namespace)
            };
            namespaces.push(namespace);
            continue;
        }

        if let Some(relation) = parse_class_relation(line) {
            ensure_class(&mut classes, &relation.from, None);
            ensure_class(&mut classes, &relation.to, None);
            relations.push(relation);
            continue;
        }

        if let Some((id, label, opens_block)) = parse_class_definition(line) {
            upsert_class(
                &mut classes,
                id.clone(),
                label,
                current_namespace(&namespaces),
            );
            if opens_block {
                current_class = Some(id);
            }
            continue;
        }

        if let Some((class_id, member)) = parse_class_member_line(line) {
            upsert_class(
                &mut classes,
                class_id.clone(),
                class_id.clone(),
                current_namespace(&namespaces),
            );
            add_class_member(&mut classes, &class_id, &member);
        }
    }

    if classes.is_empty() {
        return render_mermaid_summary("Class diagram", "class", source);
    }

    let groups = class_groups(&classes);
    let group_layouts = layout_class_groups(&groups);
    let class_positions = layout_class_positions(&classes, &group_layouts);
    let height = group_layouts
        .iter()
        .map(|group| group.y + group.height)
        .fold(220.0, f32::max)
        + 28.0;
    let marker_id = mermaid_marker_id(source);
    let mut svg = format!(
        "<div class=\"dm-mermaid-chart dm-mermaid-class\" role=\"img\" aria-label=\"Rendered Mermaid class diagram\"><div class=\"dm-mermaid-chart-title\">Mermaid Class diagram</div><svg viewBox=\"0 0 760 {height:.0}\" aria-hidden=\"true\"><defs><marker id=\"{marker_id}\" markerWidth=\"10\" markerHeight=\"10\" refX=\"8\" refY=\"3\" orient=\"auto\"><path d=\"M0,0 L0,6 L9,3 z\" class=\"dm-mermaid-arrow\" /></marker></defs>"
    );

    for group in &group_layouts {
        if let Some(namespace) = &group.namespace {
            svg.push_str(&format!(
                "<g class=\"dm-class-namespace\"><rect x=\"{:.1}\" y=\"{:.1}\" width=\"{:.1}\" height=\"{:.1}\" /><text x=\"{:.1}\" y=\"{:.1}\" text-anchor=\"middle\">{}</text></g>",
                group.x,
                group.y,
                group.width,
                group.height,
                group.x + group.width / 2.0,
                group.y + 28.0,
                escape_html(namespace)
            ));
        }
    }

    for relation in &relations {
        let Some(from) = class_positions
            .iter()
            .find(|position| position.id == relation.from)
        else {
            continue;
        };
        let Some(to) = class_positions
            .iter()
            .find(|position| position.id == relation.to)
        else {
            continue;
        };
        let (start_x, start_y) =
            rect_edge_point(from.x, from.y, to.x, to.y, from.width, from.height);
        let (end_x, end_y) = rect_edge_point(to.x, to.y, from.x, from.y, to.width, to.height);
        let control_offset = ((end_y - start_y).abs() * 0.42).clamp(34.0, 86.0);
        let label_x = (start_x + end_x) / 2.0;
        let label_y = (start_y + end_y) / 2.0 - 8.0;

        svg.push_str(&format!(
            "<path class=\"dm-class-relation\" d=\"M {start_x:.1} {start_y:.1} C {start_x:.1} {:.1}, {end_x:.1} {:.1}, {end_x:.1} {end_y:.1}\" marker-end=\"url(#{marker_id})\" />",
            start_y + control_offset,
            end_y - control_offset
        ));
        if let Some(label) = &relation.label {
            svg.push_str(&format!(
                "<text class=\"dm-class-relation-label\" x=\"{label_x:.1}\" y=\"{label_y:.1}\" text-anchor=\"middle\">{}</text>",
                escape_html(label)
            ));
        }
    }

    for position in class_positions {
        let left = position.x - position.width / 2.0;
        let top = position.y - position.height / 2.0;
        let header_bottom = top + 42.0;
        let members_separator = header_bottom + 28.0;
        let members_top = header_bottom + 54.0;
        svg.push_str(&format!(
            "<g class=\"dm-class-node\" data-class-id=\"{}\"><rect x=\"{left:.1}\" y=\"{top:.1}\" width=\"{:.1}\" height=\"{:.1}\" /><text class=\"dm-class-name\" x=\"{:.1}\" y=\"{:.1}\" text-anchor=\"middle\">{}</text><line x1=\"{left:.1}\" y1=\"{header_bottom:.1}\" x2=\"{:.1}\" y2=\"{header_bottom:.1}\" /><line x1=\"{left:.1}\" y1=\"{:.1}\" x2=\"{:.1}\" y2=\"{:.1}\" />",
            escape_attribute(&position.id),
            position.width,
            position.height,
            position.x,
            top + 27.0,
            escape_html(&position.label),
            left + position.width,
            members_separator,
            left + position.width,
            members_separator
        ));
        for (index, member) in position.members.iter().enumerate() {
            svg.push_str(&format!(
                "<text class=\"dm-class-member\" x=\"{:.1}\" y=\"{:.1}\">{}</text>",
                left + 16.0,
                members_top + index as f32 * 20.0,
                escape_html(member)
            ));
        }
        svg.push_str("</g>");
    }

    svg.push_str("</svg></div>");
    svg
}

fn parse_class_namespace_start(line: &str) -> Option<String> {
    let body = line.strip_prefix("namespace ")?.trim();
    let body = body.strip_suffix('{').unwrap_or(body).trim();
    if body.is_empty() {
        return None;
    }

    if let Some(start) = body.find('[') {
        if let Some(end) = body.rfind(']') {
            return Some(clean_mermaid_label(&body[start + 1..end]));
        }
    }

    Some(clean_mermaid_label(body))
}

fn parse_class_definition(line: &str) -> Option<(String, String, bool)> {
    let body = line.strip_prefix("class ")?.trim();
    let opens_block = body.ends_with('{') || body.contains('{');
    let body = body.split('{').next().unwrap_or(body).trim();
    let (id, label) = parse_class_reference(body);
    (!id.is_empty()).then_some((id, label, opens_block))
}

fn parse_class_member_line(line: &str) -> Option<(String, String)> {
    let (class_id, member) = line.split_once(':')?;
    let (class_id, _) = parse_class_reference(class_id);
    let member = member.trim();
    (!class_id.is_empty() && !member.is_empty()).then(|| (class_id, member.to_owned()))
}

fn parse_class_relation(line: &str) -> Option<ClassRelation> {
    const OPERATORS: &[&str] = &[
        "<|--", "--|>", "*--", "--*", "o--", "--o", "<--", "-->", "<..", "..>", "--", "..",
    ];

    for operator in OPERATORS {
        let Some((from, to)) = line.split_once(operator) else {
            continue;
        };
        let (to, label) = if let Some((to, label)) = to.split_once(':') {
            (to, Some(clean_mermaid_label(label)))
        } else {
            (to, None)
        };
        let (from, _) = parse_class_reference(from);
        let (to, _) = parse_class_reference(to);
        if !from.is_empty() && !to.is_empty() {
            return Some(ClassRelation { from, to, label });
        }
    }

    None
}

fn parse_class_reference(value: &str) -> (String, String) {
    let value = value.trim().trim_end_matches(';').trim();
    if let Some(start) = value.find('[') {
        if let Some(end) = value.rfind(']') {
            let id = clean_class_id(&value[..start]);
            let label = clean_mermaid_label(&value[start + 1..end]);
            return (id, label);
        }
    }

    let id = clean_class_id(value.split_whitespace().next().unwrap_or(value));
    let label = id.rsplit('.').next().unwrap_or(&id).to_owned();
    (id, label)
}

fn clean_class_id(value: &str) -> String {
    value
        .trim()
        .trim_matches('`')
        .trim_matches('"')
        .trim_matches(|ch| matches!(ch, '{' | '}' | '[' | ']'))
        .trim()
        .to_owned()
}

fn current_namespace(namespaces: &[String]) -> Option<String> {
    namespaces.last().cloned()
}

fn upsert_class(
    classes: &mut Vec<MermaidClass>,
    id: String,
    label: String,
    namespace: Option<String>,
) {
    if let Some(class) = classes.iter_mut().find(|class| class.id == id) {
        if class.label == class.id && label != id {
            class.label = label;
        }
        if class.namespace.is_none() && namespace.is_some() {
            class.namespace = namespace;
        }
        return;
    }

    classes.push(MermaidClass {
        id,
        label,
        namespace,
        members: Vec::new(),
    });
}

fn ensure_class(classes: &mut Vec<MermaidClass>, id: &str, namespace: Option<String>) {
    let label = id.rsplit('.').next().unwrap_or(id).to_owned();
    upsert_class(classes, id.to_owned(), label, namespace);
}

fn add_class_member(classes: &mut [MermaidClass], class_id: &str, member: &str) {
    let member = member.trim();
    if member.is_empty() {
        return;
    }

    if let Some(class) = classes.iter_mut().find(|class| class.id == class_id) {
        if !class.members.iter().any(|existing| existing == member) {
            class.members.push(member.to_owned());
        }
    }
}

fn class_groups(classes: &[MermaidClass]) -> Vec<Option<String>> {
    let mut groups = Vec::new();
    for class in classes {
        if !groups.iter().any(|namespace| namespace == &class.namespace) {
            groups.push(class.namespace.clone());
        }
    }
    groups
}

fn layout_class_groups(groups: &[Option<String>]) -> Vec<ClassGroupLayout> {
    match groups.len() {
        0 => Vec::new(),
        1 => vec![ClassGroupLayout {
            namespace: groups[0].clone(),
            x: 70.0,
            y: 26.0,
            width: 620.0,
            height: 180.0,
        }],
        2 => groups
            .iter()
            .enumerate()
            .map(|(index, namespace)| ClassGroupLayout {
                namespace: namespace.clone(),
                x: 42.0 + index as f32 * 378.0,
                y: 42.0,
                width: 320.0,
                height: 190.0,
            })
            .collect(),
        _ => {
            let mut layouts = vec![ClassGroupLayout {
                namespace: groups[0].clone(),
                x: 120.0,
                y: 24.0,
                width: 520.0,
                height: 180.0,
            }];
            for (index, namespace) in groups.iter().enumerate().skip(1) {
                let lane = index - 1;
                layouts.push(ClassGroupLayout {
                    namespace: namespace.clone(),
                    x: 42.0 + (lane % 2) as f32 * 378.0,
                    y: 252.0 + (lane / 2) as f32 * 210.0,
                    width: 320.0,
                    height: 190.0,
                });
            }
            layouts
        }
    }
}

fn layout_class_positions(
    classes: &[MermaidClass],
    groups: &[ClassGroupLayout],
) -> Vec<ClassPosition> {
    let mut positions = Vec::new();

    for group in groups {
        let group_classes = classes
            .iter()
            .filter(|class| class.namespace == group.namespace)
            .collect::<Vec<_>>();
        let count = group_classes.len();
        if count == 0 {
            continue;
        }

        for (index, class) in group_classes.iter().enumerate() {
            let width = 220.0;
            let height = 96.0 + class.members.len().max(1) as f32 * 20.0;
            let x = if count <= 1 {
                group.x + group.width / 2.0
            } else {
                group.x + 82.0 + (group.width - 164.0) * index as f32 / (count - 1) as f32
            };
            let y = group.y + group.height / 2.0 + 14.0;

            positions.push(ClassPosition {
                id: class.id.clone(),
                label: class.label.clone(),
                members: class.members.clone(),
                x,
                y,
                width,
                height,
            });
        }
    }

    positions
}

struct StateTransition {
    from: String,
    to: String,
    label: Option<String>,
}

fn render_mermaid_state(source: &str) -> String {
    let mut states = Vec::new();
    let mut state_labels: Vec<(String, String)> = Vec::new();
    let mut transitions = Vec::new();

    for line in source.lines().map(str::trim) {
        if line.is_empty()
            || line.starts_with("%%")
            || line.starts_with("stateDiagram")
            || line.starts_with("direction ")
        {
            continue;
        }

        if let Some((id, label)) = parse_state_definition(line) {
            upsert_text(&mut states, id.clone());
            if let Some(existing) = state_labels
                .iter_mut()
                .find(|(existing_id, _)| existing_id == &id)
            {
                existing.1 = label;
            } else {
                state_labels.push((id, label));
            }
            continue;
        }

        if let Some(transition) = parse_state_transition(line) {
            if transition.from != "[*]" {
                upsert_text(&mut states, transition.from.clone());
            }
            if transition.to != "[*]" {
                upsert_text(&mut states, transition.to.clone());
            }
            transitions.push(transition);
        }
    }

    if states.is_empty() {
        return render_mermaid_summary("State diagram", "state", source);
    }

    let width: f32 = 420.0;
    let state_width: f32 = 128.0;
    let state_height: f32 = 50.0;
    let x: f32 = 188.0;
    let first_y: f32 = 86.0;
    let step_y: f32 = 92.0;
    let end_y = first_y + states.len() as f32 * step_y + 8.0;
    let height = end_y + 52.0;
    let marker_id = mermaid_marker_id(source);
    let positions = states
        .iter()
        .enumerate()
        .map(|(index, state)| (state.as_str(), x, first_y + index as f32 * step_y))
        .collect::<Vec<_>>();
    let mut svg = format!(
        "<div class=\"dm-mermaid-chart dm-mermaid-state\" role=\"img\" aria-label=\"Rendered Mermaid state diagram\"><div class=\"dm-mermaid-chart-title\">Mermaid State diagram</div><svg viewBox=\"0 0 {width:.0} {height:.0}\" aria-hidden=\"true\"><defs><marker id=\"{marker_id}\" markerWidth=\"10\" markerHeight=\"10\" refX=\"8\" refY=\"3\" orient=\"auto\"><path d=\"M0,0 L0,6 L9,3 z\" class=\"dm-mermaid-arrow\" /></marker></defs>"
    );

    for transition in &transitions {
        let (start_x, start_y) = if transition.from == "[*]" {
            (x, 28.0)
        } else if let Some((_, from_x, from_y)) =
            positions.iter().find(|(id, _, _)| *id == transition.from)
        {
            (*from_x, *from_y + state_height / 2.0)
        } else {
            continue;
        };
        let (end_x, end_y_line) = if transition.to == "[*]" {
            (x, end_y)
        } else if let Some((_, to_x, to_y)) =
            positions.iter().find(|(id, _, _)| *id == transition.to)
        {
            (*to_x, *to_y - state_height / 2.0)
        } else {
            continue;
        };
        let label_x = (start_x + end_x) / 2.0;
        let label_y = (start_y + end_y_line) / 2.0 - 7.0;

        let vertical_gap = (end_y_line - start_y).abs();
        if (start_x - end_x).abs() < 0.1_f32 && start_y < end_y_line && vertical_gap <= step_y {
            svg.push_str(&format!(
                "<line class=\"dm-state-transition\" x1=\"{start_x:.1}\" y1=\"{start_y:.1}\" x2=\"{end_x:.1}\" y2=\"{end_y_line:.1}\" marker-end=\"url(#{marker_id})\" />"
            ));
        } else {
            let offset = if start_y <= end_y_line { -86.0 } else { 86.0 };
            svg.push_str(&format!(
                "<path class=\"dm-state-transition\" d=\"M {start_x:.1} {start_y:.1} C {:.1} {start_y:.1}, {:.1} {end_y_line:.1}, {end_x:.1} {end_y_line:.1}\" marker-end=\"url(#{marker_id})\" />",
                start_x + offset,
                end_x + offset
            ));
        }
        if let Some(label) = &transition.label {
            svg.push_str(&format!(
                "<text class=\"dm-state-label\" x=\"{label_x:.1}\" y=\"{label_y:.1}\" text-anchor=\"middle\">{}</text>",
                escape_html(label)
            ));
        }
    }

    svg.push_str(&format!(
        "<circle class=\"dm-state-start\" cx=\"{x:.1}\" cy=\"28\" r=\"10\" />"
    ));
    for (state, x, y) in &positions {
        let label = state_labels
            .iter()
            .find(|(id, _)| id == state)
            .map(|(_, label)| label.as_str())
            .unwrap_or(state);
        svg.push_str(&format!(
            "<g class=\"dm-state-node\"><rect x=\"{:.1}\" y=\"{:.1}\" width=\"{state_width:.1}\" height=\"{state_height:.1}\" rx=\"8\" /><text x=\"{x:.1}\" y=\"{:.1}\" text-anchor=\"middle\">{}</text></g>",
            x - state_width / 2.0,
            y - state_height / 2.0,
            y + 6.0,
            escape_html(label)
        ));
    }
    svg.push_str(&format!(
        "<g class=\"dm-state-end\"><circle cx=\"{x:.1}\" cy=\"{end_y:.1}\" r=\"13\" /><circle cx=\"{x:.1}\" cy=\"{end_y:.1}\" r=\"7\" /></g>"
    ));
    svg.push_str("</svg></div>");
    svg
}

fn parse_state_definition(line: &str) -> Option<(String, String)> {
    let body = line.strip_prefix("state ")?.trim();
    let (label, id) = body.split_once(" as ")?;
    Some((clean_mermaid_label(id), clean_mermaid_label(label)))
}

fn parse_state_transition(line: &str) -> Option<StateTransition> {
    let (from, to) = line.trim_end_matches(';').split_once("-->")?;
    let (to, label) = if let Some((to, label)) = to.split_once(':') {
        (to, Some(clean_mermaid_label(label)))
    } else {
        (to, None)
    };
    let from = clean_state_id(from);
    let to = clean_state_id(to);
    (!from.is_empty() && !to.is_empty()).then_some(StateTransition { from, to, label })
}

fn clean_state_id(value: &str) -> String {
    let value = value.trim();
    if value == "[*]" {
        "[*]".to_owned()
    } else {
        clean_mermaid_label(value)
    }
}

struct ErEntity {
    id: String,
    attributes: Vec<String>,
}

struct ErRelation {
    from: String,
    to: String,
    operator: String,
    label: String,
}

struct ErPosition {
    id: String,
    attributes: Vec<String>,
    x: f32,
    y: f32,
    width: f32,
    height: f32,
}

fn render_mermaid_er(source: &str) -> String {
    let mut entities = Vec::new();
    let mut relations = Vec::new();
    let mut current_entity: Option<String> = None;

    for line in source.lines().map(str::trim) {
        if line.is_empty() || line.starts_with("%%") || line.starts_with("erDiagram") {
            continue;
        }

        if line == "}" {
            current_entity = None;
            continue;
        }

        if let Some(entity_id) = current_entity.clone() {
            add_er_attribute(&mut entities, &entity_id, line);
            continue;
        }

        if let Some(entity_id) = parse_er_entity_start(line) {
            ensure_er_entity(&mut entities, &entity_id);
            current_entity = Some(entity_id);
            continue;
        }

        if let Some(relation) = parse_er_relation(line) {
            ensure_er_entity(&mut entities, &relation.from);
            ensure_er_entity(&mut entities, &relation.to);
            relations.push(relation);
        }
    }

    if entities.is_empty() {
        return render_mermaid_summary("Entity relationship", "er", source);
    }

    let positions = layout_er_positions(&entities);
    let marker_id = mermaid_marker_id(source);
    let mut svg = format!(
        "<div class=\"dm-mermaid-chart dm-mermaid-er\" role=\"img\" aria-label=\"Rendered Mermaid entity relationship diagram\"><div class=\"dm-mermaid-chart-title\">Mermaid Entity relationship</div><svg viewBox=\"0 0 760 520\" aria-hidden=\"true\"><defs><marker id=\"{marker_id}\" markerWidth=\"10\" markerHeight=\"10\" refX=\"8\" refY=\"3\" orient=\"auto\"><path d=\"M0,0 L0,6 L9,3 z\" class=\"dm-mermaid-arrow\" /></marker></defs>"
    );

    for relation in &relations {
        let Some(from) = positions
            .iter()
            .find(|position| position.id == relation.from)
        else {
            continue;
        };
        let Some(to) = positions.iter().find(|position| position.id == relation.to) else {
            continue;
        };
        let (start_x, start_y) =
            rect_edge_point(from.x, from.y, to.x, to.y, from.width, from.height);
        let (end_x, end_y) = rect_edge_point(to.x, to.y, from.x, from.y, to.width, to.height);
        let label_x = (start_x + end_x) / 2.0;
        let label_y = (start_y + end_y) / 2.0 - 8.0;
        let (from_cardinality, to_cardinality) = split_er_operator(&relation.operator);
        let from_label_x = start_x + (end_x - start_x) * 0.18;
        let from_label_y = start_y + (end_y - start_y) * 0.18 - 8.0;
        let to_label_x = start_x + (end_x - start_x) * 0.82;
        let to_label_y = start_y + (end_y - start_y) * 0.82 - 8.0;

        svg.push_str(&format!(
            "<path class=\"dm-er-relation\" d=\"M {start_x:.1} {start_y:.1} C {start_x:.1} {label_y:.1}, {end_x:.1} {label_y:.1}, {end_x:.1} {end_y:.1}\" marker-end=\"url(#{marker_id})\" />"
        ));
        svg.push_str(&format!(
            "<text class=\"dm-er-cardinality\" x=\"{from_label_x:.1}\" y=\"{from_label_y:.1}\" text-anchor=\"middle\">{}</text><text class=\"dm-er-cardinality\" x=\"{to_label_x:.1}\" y=\"{to_label_y:.1}\" text-anchor=\"middle\">{}</text><text class=\"dm-er-label\" x=\"{label_x:.1}\" y=\"{label_y:.1}\" text-anchor=\"middle\">{}</text>",
            escape_html(from_cardinality),
            escape_html(to_cardinality),
            escape_html(&relation.label)
        ));
    }

    for position in positions {
        let left = position.x - position.width / 2.0;
        let top = position.y - position.height / 2.0;
        svg.push_str(&format!(
            "<g class=\"dm-er-entity\"><rect x=\"{left:.1}\" y=\"{top:.1}\" width=\"{:.1}\" height=\"{:.1}\" /><text class=\"dm-er-name\" x=\"{:.1}\" y=\"{:.1}\" text-anchor=\"middle\">{}</text>",
            position.width,
            position.height,
            position.x,
            top + 34.0,
            escape_html(&position.id)
        ));
        for (index, attribute) in position.attributes.iter().enumerate() {
            svg.push_str(&format!(
                "<text class=\"dm-er-attribute\" x=\"{:.1}\" y=\"{:.1}\">{}</text>",
                left + 16.0,
                top + 64.0 + index as f32 * 18.0,
                escape_html(attribute)
            ));
        }
        svg.push_str("</g>");
    }

    svg.push_str("</svg></div>");
    svg
}

fn parse_er_entity_start(line: &str) -> Option<String> {
    let body = line.strip_suffix('{')?.trim();
    (!body.is_empty()).then(|| clean_er_entity_id(body))
}

fn parse_er_relation(line: &str) -> Option<ErRelation> {
    const OPERATORS: &[&str] = &[
        "||--o{", "||--|{", "}o--||", "}|--||", "}o--o{", "}|--|{", "|o--o|", "|o--||", "||--||",
        "o|--||", "o{--||", "|{--||", "o{--o{", "|{--|{",
    ];

    for operator in OPERATORS {
        let Some((from, to)) = line.split_once(operator) else {
            continue;
        };
        let (to, label) = if let Some((to, label)) = to.split_once(':') {
            (to, clean_mermaid_label(label))
        } else {
            (to, String::new())
        };
        let from = clean_er_entity_id(from);
        let to = clean_er_entity_id(to);
        if !from.is_empty() && !to.is_empty() {
            return Some(ErRelation {
                from,
                to,
                operator: (*operator).to_owned(),
                label,
            });
        }
    }

    None
}

fn clean_er_entity_id(value: &str) -> String {
    value
        .trim()
        .trim_matches('"')
        .trim_matches('`')
        .replace('_', "-")
}

fn ensure_er_entity(entities: &mut Vec<ErEntity>, id: &str) {
    if !entities.iter().any(|entity| entity.id == id) {
        entities.push(ErEntity {
            id: id.to_owned(),
            attributes: Vec::new(),
        });
    }
}

fn add_er_attribute(entities: &mut [ErEntity], entity_id: &str, attribute: &str) {
    let attribute = attribute.trim();
    if attribute.is_empty() {
        return;
    }
    if let Some(entity) = entities.iter_mut().find(|entity| entity.id == entity_id) {
        entity.attributes.push(attribute.to_owned());
    }
}

fn layout_er_positions(entities: &[ErEntity]) -> Vec<ErPosition> {
    let anchors = [
        (380.0, 88.0),
        (150.0, 260.0),
        (150.0, 430.0),
        (575.0, 260.0),
        (575.0, 430.0),
        (380.0, 430.0),
    ];

    entities
        .iter()
        .enumerate()
        .map(|(index, entity)| {
            let (x, y) = anchors[index.min(anchors.len() - 1)];
            let width = (entity.id.len() as f32 * 9.5 + 52.0).clamp(150.0, 250.0);
            let height = 78.0 + entity.attributes.len() as f32 * 18.0;
            ErPosition {
                id: entity.id.clone(),
                attributes: entity.attributes.clone(),
                x,
                y,
                width,
                height,
            }
        })
        .collect()
}

fn split_er_operator(operator: &str) -> (&str, &str) {
    operator.split_once("--").unwrap_or((operator, ""))
}

struct JourneyTask {
    section: String,
    name: String,
    score: u8,
    actors: Vec<String>,
}

fn render_mermaid_journey(source: &str) -> String {
    let mut title = "User journey".to_owned();
    let mut current_section = String::new();
    let mut tasks = Vec::new();
    let mut actors = Vec::new();

    for line in source.lines().map(str::trim) {
        if line.is_empty() || line.starts_with("%%") || line == "journey" {
            continue;
        }

        if let Some(value) = line.strip_prefix("title ") {
            title = clean_mermaid_label(value);
            continue;
        }

        if let Some(value) = line.strip_prefix("section ") {
            current_section = clean_mermaid_label(value);
            continue;
        }

        if let Some(task) = parse_journey_task(line, &current_section) {
            for actor in &task.actors {
                upsert_text(&mut actors, actor.clone());
            }
            tasks.push(task);
        }
    }

    if tasks.is_empty() {
        return render_mermaid_summary("User journey", "journey", source);
    }

    if actors.is_empty() {
        actors.push("User".to_owned());
    }

    let width = 760.0;
    let height = 340.0;
    let first_x = 150.0;
    let last_x = 700.0;
    let span = last_x - first_x;
    let task_width = (span / tasks.len().max(1) as f32 - 16.0).clamp(108.0, 178.0);
    let axis_y = 164.0;
    let mut svg = format!(
        "<div class=\"dm-mermaid-chart dm-mermaid-journey\" role=\"img\" aria-label=\"Rendered Mermaid user journey\"><div class=\"dm-mermaid-chart-title\">Mermaid User journey</div><svg viewBox=\"0 0 {width:.0} {height:.0}\" aria-hidden=\"true\"><text class=\"dm-journey-title\" x=\"150\" y=\"34\">{}</text>",
        escape_html(&title)
    );

    for (index, actor) in actors.iter().take(3).enumerate() {
        let y = 62.0 + index as f32 * 18.0;
        svg.push_str(&format!(
            "<circle class=\"dm-journey-actor-dot dm-journey-actor-{}\" cx=\"42\" cy=\"{y:.1}\" r=\"5\" /><text class=\"dm-journey-actor\" x=\"60\" y=\"{:.1}\">{}</text>",
            index + 1,
            y + 4.0,
            escape_html(actor)
        ));
    }

    svg.push_str(&format!(
        "<line class=\"dm-journey-axis\" x1=\"120\" y1=\"{axis_y:.1}\" x2=\"722\" y2=\"{axis_y:.1}\" /><path class=\"dm-journey-axis-arrow\" d=\"M722 {axis_y:.1} l-14 -6 v12 z\" />"
    ));

    let mut last_section = "";
    for (index, task) in tasks.iter().enumerate() {
        let x = if tasks.len() == 1 {
            (first_x + last_x) / 2.0
        } else {
            first_x + span * index as f32 / (tasks.len() - 1) as f32
        };
        let box_y = 76.0;
        let score_y = 292.0 - task.score as f32 * 28.0;
        if task.section != last_section {
            svg.push_str(&format!(
                "<text class=\"dm-journey-section\" x=\"{:.1}\" y=\"62\">{}</text>",
                x - task_width / 2.0,
                escape_html(&task.section)
            ));
            last_section = &task.section;
        }
        svg.push_str(&format!(
            "<rect class=\"dm-journey-task\" x=\"{:.1}\" y=\"{box_y:.1}\" width=\"{task_width:.1}\" height=\"38\" rx=\"4\" /><text class=\"dm-journey-task-label\" x=\"{x:.1}\" y=\"{:.1}\" text-anchor=\"middle\">{}</text><line class=\"dm-journey-drop\" x1=\"{x:.1}\" y1=\"114\" x2=\"{x:.1}\" y2=\"276\" /><circle class=\"dm-journey-score\" cx=\"{x:.1}\" cy=\"{score_y:.1}\" r=\"13\" /><text class=\"dm-journey-score-text\" x=\"{x:.1}\" y=\"{:.1}\" text-anchor=\"middle\">{}</text>",
            x - task_width / 2.0,
            box_y + 24.0,
            escape_html(&task.name),
            score_y + 4.0,
            task.score
        ));
        for (actor_index, actor) in task.actors.iter().take(3).enumerate() {
            if let Some(color_index) = actors.iter().position(|known| known == actor) {
                svg.push_str(&format!(
                    "<circle class=\"dm-journey-actor-dot dm-journey-actor-{}\" cx=\"{:.1}\" cy=\"76\" r=\"5\" />",
                    color_index + 1,
                    x - task_width / 2.0 + 10.0 + actor_index as f32 * 12.0
                ));
            }
        }
    }

    svg.push_str("</svg></div>");
    svg
}

fn parse_journey_task(line: &str, section: &str) -> Option<JourneyTask> {
    let mut parts = line.split(':').map(str::trim);
    let name = clean_mermaid_label(parts.next()?);
    let score = parts.next()?.parse::<u8>().ok()?.clamp(1, 5);
    let actors = parts
        .next()
        .map(|value| {
            value
                .split(',')
                .map(clean_mermaid_label)
                .filter(|actor| !actor.is_empty())
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();

    Some(JourneyTask {
        section: section.to_owned(),
        name,
        score,
        actors,
    })
}

fn render_mermaid_sequence(source: &str) -> String {
    let mut participants: Vec<String> = Vec::new();
    let mut messages: Vec<(String, String, String)> = Vec::new();

    for line in source.lines().map(str::trim) {
        if line.is_empty() || line.starts_with("sequenceDiagram") {
            continue;
        }

        if let Some(name) = line.strip_prefix("participant ") {
            let participant = name
                .split_once(" as ")
                .map_or(name, |(_, label)| label)
                .trim()
                .to_owned();
            upsert_text(&mut participants, participant);
            continue;
        }

        if let Some((left, label)) = line.split_once(':') {
            let (from, to) = left
                .split_once("-->>")
                .or_else(|| left.split_once("->>"))
                .or_else(|| left.split_once("-->"))
                .or_else(|| left.split_once("->"))
                .unwrap_or(("", ""));
            if !from.trim().is_empty() && !to.trim().is_empty() {
                let from = from.trim().to_owned();
                let to = to.trim().to_owned();
                upsert_text(&mut participants, from.clone());
                upsert_text(&mut participants, to.clone());
                messages.push((from, to, label.trim().to_owned()));
            }
        }
    }

    if participants.is_empty() {
        participants.extend(["Client".to_owned(), "Server".to_owned()]);
    }

    let width = 640.0;
    let lane_gap = if participants.len() <= 1 {
        0.0
    } else {
        480.0 / (participants.len() - 1) as f32
    };
    let height = 110 + messages.len() as i32 * 42;
    let marker_id = mermaid_marker_id(source);
    let mut svg = format!(
        "<div class=\"dm-mermaid-chart dm-mermaid-sequence\" role=\"img\" aria-label=\"Rendered Mermaid sequence diagram\"><div class=\"dm-mermaid-chart-title\">Mermaid sequence diagram</div><svg viewBox=\"0 0 {width:.0} {height}\" aria-hidden=\"true\"><defs><marker id=\"{marker_id}\" markerWidth=\"10\" markerHeight=\"10\" refX=\"8\" refY=\"3\" orient=\"auto\"><path d=\"M0,0 L0,6 L9,3 z\" class=\"dm-mermaid-arrow\" /></marker></defs>"
    );

    for (index, participant) in participants.iter().enumerate() {
        let x = 80.0 + lane_gap * index as f32;
        svg.push_str(&format!(
            "<g class=\"dm-sequence-lane\"><rect x=\"{:.1}\" y=\"18\" width=\"118\" height=\"34\" rx=\"8\" /><text x=\"{:.1}\" y=\"40\" text-anchor=\"middle\">{}</text><line x1=\"{:.1}\" y1=\"54\" x2=\"{:.1}\" y2=\"{}\" /></g>",
            x - 59.0,
            x,
            escape_html(participant),
            x,
            x,
            height - 20
        ));
    }

    for (index, (from, to, label)) in messages.iter().enumerate() {
        let Some(from_index) = participants
            .iter()
            .position(|participant| participant == from)
        else {
            continue;
        };
        let Some(to_index) = participants
            .iter()
            .position(|participant| participant == to)
        else {
            continue;
        };
        let y = 84.0 + index as f32 * 42.0;
        let from_x = 80.0 + lane_gap * from_index as f32;
        let to_x = 80.0 + lane_gap * to_index as f32;
        let label_x = (from_x + to_x) / 2.0;
        svg.push_str(&format!(
            "<g class=\"dm-sequence-message\"><line x1=\"{from_x:.1}\" y1=\"{y:.1}\" x2=\"{to_x:.1}\" y2=\"{y:.1}\" marker-end=\"url(#{marker_id})\" /><text x=\"{label_x:.1}\" y=\"{:.1}\" text-anchor=\"middle\">{}</text></g>",
            y - 8.0,
            escape_html(label)
        ));
    }

    svg.push_str("</svg></div>");
    svg
}

fn render_mermaid_pie(source: &str) -> String {
    let slices = source
        .lines()
        .filter_map(|line| {
            let line = line.trim();
            if line.starts_with("pie") || line.starts_with("title ") {
                return None;
            }
            let (label, value) = line.split_once(':')?;
            let value = value.trim().parse::<f32>().ok()?;
            Some((label.trim().trim_matches('"').to_owned(), value.max(0.0)))
        })
        .collect::<Vec<_>>();

    if slices.is_empty() {
        return render_mermaid_summary("Pie chart", "pie", source);
    }

    let total = slices.iter().map(|(_, value)| value).sum::<f32>().max(1.0);
    let mut start = 0.0;
    let mut gradient_parts = Vec::new();
    for (index, (_, value)) in slices.iter().enumerate() {
        let end = start + (*value / total) * 100.0;
        gradient_parts.push(format!(
            "var(--dm-mermaid-slice-{}) {start:.1}% {end:.1}%",
            index % 6 + 1
        ));
        start = end;
    }

    let mut html = format!(
        "<div class=\"dm-mermaid-chart dm-mermaid-pie\" role=\"img\" aria-label=\"Rendered Mermaid pie chart\"><div class=\"dm-mermaid-chart-title\">Mermaid pie chart</div><div class=\"dm-mermaid-pie-layout\"><div class=\"dm-mermaid-pie-graphic\" style=\"background: conic-gradient({})\"></div><ul>",
        gradient_parts.join(", ")
    );

    for (index, (label, value)) in slices.iter().enumerate() {
        html.push_str(&format!(
            "<li><span class=\"dm-mermaid-swatch dm-mermaid-swatch-{}\"></span><strong>{}</strong><span>{value:.0}</span></li>",
            index % 6 + 1,
            escape_html(label)
        ));
    }

    html.push_str("</ul></div></div>");
    html
}

struct GanttTask {
    section: String,
    name: String,
    start: i32,
    end: i32,
    status: String,
}

fn render_mermaid_gantt(source: &str) -> String {
    let mut title = "Gantt diagram".to_owned();
    let mut current_section = "Schedule".to_owned();
    let mut tasks = Vec::new();

    for line in source.lines().map(str::trim) {
        if line.is_empty() || line.starts_with("%%") || line == "gantt" {
            continue;
        }

        if let Some(value) = line.strip_prefix("title ") {
            title = clean_mermaid_label(value);
            continue;
        }

        if line.starts_with("dateFormat")
            || line.starts_with("axisFormat")
            || line.starts_with("tickInterval")
            || line.starts_with("excludes")
            || line.starts_with("todayMarker")
        {
            continue;
        }

        if let Some(value) = line.strip_prefix("section ") {
            current_section = clean_mermaid_label(value);
            continue;
        }

        if let Some(task) = parse_gantt_task(line, &current_section) {
            tasks.push(task);
        }
    }

    if tasks.is_empty() {
        return render_mermaid_summary("Gantt chart", "gantt", source);
    }

    let min_day = tasks.iter().map(|task| task.start).min().unwrap_or(0);
    let max_day = tasks
        .iter()
        .map(|task| task.end)
        .max()
        .unwrap_or(min_day + 1);
    let total_days = (max_day - min_day).max(1) as f32;
    let row_height = 36.0;
    let top = 54.0;
    let left = 118.0;
    let timeline_width = 590.0;
    let width = 760.0;
    let height = top + 44.0 + tasks.len() as f32 * row_height + 50.0;
    let mut svg = format!(
        "<div class=\"dm-mermaid-chart dm-mermaid-gantt\" role=\"img\" aria-label=\"Rendered Mermaid gantt chart\"><div class=\"dm-mermaid-chart-title\">Mermaid gantt chart</div><svg viewBox=\"0 0 {width:.0} {height:.0}\" aria-hidden=\"true\"><text class=\"dm-gantt-title\" x=\"380\" y=\"28\" text-anchor=\"middle\">{}</text>",
        escape_html(&title)
    );

    let axis_y = top + 20.0;
    svg.push_str(&format!(
        "<line class=\"dm-gantt-axis\" x1=\"{left:.1}\" y1=\"{axis_y:.1}\" x2=\"{:.1}\" y2=\"{axis_y:.1}\" />",
        left + timeline_width
    ));
    for tick in 0..=5 {
        let ratio = tick as f32 / 5.0;
        let day = min_day + ((max_day - min_day) as f32 * ratio).round() as i32;
        let x = left + timeline_width * ratio;
        svg.push_str(&format!(
            "<line class=\"dm-gantt-tick\" x1=\"{x:.1}\" y1=\"{:.1}\" x2=\"{x:.1}\" y2=\"{:.1}\" /><text class=\"dm-gantt-date\" x=\"{x:.1}\" y=\"{:.1}\" text-anchor=\"middle\">{}</text>",
            top + 4.0,
            top + 32.0 + tasks.len() as f32 * row_height,
            top + 48.0 + tasks.len() as f32 * row_height,
            escape_html(&format_ymd(day))
        ));
    }

    let mut last_section = "";
    for (index, task) in tasks.iter().enumerate() {
        let y = top + 42.0 + index as f32 * row_height;
        if task.section != last_section {
            svg.push_str(&format!(
                "<rect class=\"dm-gantt-section-band\" x=\"10\" y=\"{:.1}\" width=\"730\" height=\"{row_height:.1}\" /><text class=\"dm-gantt-section\" x=\"22\" y=\"{:.1}\">{}</text>",
                y - 16.0,
                y + 7.0,
                escape_html(&task.section)
            ));
            last_section = &task.section;
        }

        let start_x = left + ((task.start - min_day) as f32 / total_days) * timeline_width;
        let end_x = left + ((task.end - min_day) as f32 / total_days) * timeline_width;
        let bar_width = (end_x - start_x).max(16.0);
        svg.push_str(&format!(
            "<rect class=\"dm-gantt-bar dm-gantt-bar-{}\" x=\"{start_x:.1}\" y=\"{:.1}\" width=\"{bar_width:.1}\" height=\"14\" rx=\"3\" /><text class=\"dm-gantt-task\" x=\"{:.1}\" y=\"{:.1}\" text-anchor=\"middle\">{}</text>",
            escape_attribute(&task.status),
            y - 7.0,
            start_x + bar_width / 2.0,
            y + 4.0,
            escape_html(&task.name)
        ));
    }

    svg.push_str("</svg></div>");
    svg
}

fn parse_gantt_task(line: &str, section: &str) -> Option<GanttTask> {
    let (name, details) = line.split_once(':')?;
    let parts = details.split(',').map(str::trim).collect::<Vec<_>>();
    let start_index = parts.iter().position(|part| parse_ymd(part).is_some())?;
    let start = parse_ymd(parts[start_index])?;
    let end = parts
        .get(start_index + 1)
        .and_then(|part| parse_ymd(part))
        .or_else(|| {
            parts
                .get(start_index + 1)
                .and_then(|part| parse_duration_days(part).map(|duration| start + duration))
        })
        .unwrap_or(start + 3);
    let status = parts
        .iter()
        .find(|part| matches!(**part, "done" | "active" | "crit" | "milestone"))
        .copied()
        .unwrap_or("default")
        .to_owned();

    Some(GanttTask {
        section: section.to_owned(),
        name: clean_mermaid_label(name),
        start,
        end: end.max(start + 1),
        status,
    })
}

fn parse_duration_days(value: &str) -> Option<i32> {
    let value = value.trim();
    let days = value.strip_suffix('d')?.trim().parse::<i32>().ok()?;
    Some(days.max(1))
}

fn parse_ymd(value: &str) -> Option<i32> {
    let mut parts = value.trim().split('-');
    let year = parts.next()?.parse::<i32>().ok()?;
    let month = parts.next()?.parse::<u32>().ok()?;
    let day = parts.next()?.parse::<u32>().ok()?;
    if parts.next().is_some()
        || !(1..=12).contains(&month)
        || day == 0
        || day > days_in_month(year, month)
    {
        return None;
    }

    Some(days_before_year(year) + days_before_month(year, month) + day as i32 - 1)
}

fn format_ymd(day_index: i32) -> String {
    let mut year = 1970;
    let mut day = day_index;

    while day < 0 {
        year -= 1;
        day += days_in_year(year);
    }
    while day >= days_in_year(year) {
        day -= days_in_year(year);
        year += 1;
    }

    let mut month = 1;
    while day >= days_in_month(year, month) as i32 {
        day -= days_in_month(year, month) as i32;
        month += 1;
    }

    format!("{year:04}-{month:02}-{:02}", day + 1)
}

fn days_before_year(year: i32) -> i32 {
    if year >= 1970 {
        (1970..year).map(days_in_year).sum()
    } else {
        -(year..1970).map(days_in_year).sum::<i32>()
    }
}

fn days_before_month(year: i32, month: u32) -> i32 {
    (1..month)
        .map(|current_month| days_in_month(year, current_month) as i32)
        .sum()
}

fn days_in_year(year: i32) -> i32 {
    if is_leap_year(year) {
        366
    } else {
        365
    }
}

fn days_in_month(year: i32, month: u32) -> u32 {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 if is_leap_year(year) => 29,
        2 => 28,
        _ => 0,
    }
}

fn is_leap_year(year: i32) -> bool {
    (year % 4 == 0 && year % 100 != 0) || year % 400 == 0
}

struct TimelineItem {
    period: String,
    events: Vec<String>,
}

fn render_mermaid_timeline(source: &str) -> String {
    let mut title = "Timeline".to_owned();
    let mut items: Vec<TimelineItem> = Vec::new();

    for line in source.lines().map(str::trim) {
        if line.is_empty() || line.starts_with("%%") || line == "timeline" {
            continue;
        }

        if let Some(value) = line.strip_prefix("title ") {
            title = clean_mermaid_label(value);
            continue;
        }

        if let Some((period, event)) = line.split_once(':') {
            let events = event
                .split(':')
                .map(clean_mermaid_label)
                .filter(|event| !event.is_empty())
                .collect::<Vec<_>>();
            if period.trim().is_empty() {
                if let Some(item) = items.last_mut() {
                    item.events.extend(events);
                }
            } else {
                items.push(TimelineItem {
                    period: clean_mermaid_label(period),
                    events,
                });
            }
        }
    }

    if items.is_empty() {
        return render_mermaid_summary("Timeline", "timeline", source);
    }

    let width: f32 = 760.0;
    let height: f32 = 284.0;
    let start_x: f32 = 86.0;
    let end_x: f32 = 674.0;
    let axis_y: f32 = 144.0;
    let mut svg = format!(
        "<div class=\"dm-mermaid-chart dm-mermaid-timeline\" role=\"img\" aria-label=\"Rendered Mermaid timeline\"><div class=\"dm-mermaid-chart-title\">Mermaid timeline</div><svg viewBox=\"0 0 {width:.0} {height:.0}\" aria-hidden=\"true\"><text class=\"dm-timeline-title\" x=\"380\" y=\"30\" text-anchor=\"middle\">{}</text><line class=\"dm-timeline-axis\" x1=\"{start_x:.1}\" y1=\"{axis_y:.1}\" x2=\"{end_x:.1}\" y2=\"{axis_y:.1}\" />",
        escape_html(&title)
    );

    for (index, item) in items.iter().enumerate() {
        let x = if items.len() == 1 {
            (start_x + end_x) / 2.0
        } else {
            start_x + (end_x - start_x) * index as f32 / (items.len() - 1) as f32
        };
        let event_text = item.events.join(" / ");
        let box_y = if index % 2 == 0 { 54.0 } else { 178.0 };
        let stem_end = if index % 2 == 0 { box_y + 56.0 } else { box_y };
        svg.push_str(&format!(
            "<line class=\"dm-timeline-stem\" x1=\"{x:.1}\" y1=\"{axis_y:.1}\" x2=\"{x:.1}\" y2=\"{stem_end:.1}\" /><circle class=\"dm-timeline-point\" cx=\"{x:.1}\" cy=\"{axis_y:.1}\" r=\"8\" /><rect class=\"dm-timeline-event\" x=\"{:.1}\" y=\"{box_y:.1}\" width=\"170\" height=\"56\" rx=\"6\" /><text class=\"dm-timeline-period\" x=\"{x:.1}\" y=\"{:.1}\" text-anchor=\"middle\">{}</text><text class=\"dm-timeline-event-text\" x=\"{x:.1}\" y=\"{:.1}\" text-anchor=\"middle\">{}</text>",
            x - 85.0,
            box_y + 20.0,
            escape_html(&item.period),
            box_y + 40.0,
            escape_html(&event_text)
        ));
    }

    svg.push_str("</svg></div>");
    svg
}

struct ZenUmlMessage {
    from: String,
    to: String,
    label: String,
}

fn render_mermaid_zenuml(source: &str) -> String {
    let mut participants = Vec::new();
    let mut messages = Vec::new();

    for line in source.lines().map(str::trim) {
        if line.is_empty() || line.starts_with("%%") || line == "zenuml" {
            continue;
        }

        if let Some(message) = parse_zenuml_message(line) {
            upsert_text(&mut participants, message.from.clone());
            upsert_text(&mut participants, message.to.clone());
            messages.push(message);
        }
    }

    if messages.is_empty() || participants.is_empty() {
        return render_mermaid_summary("ZenUML diagram", "zenuml", source);
    }

    let width: f32 = 760.0;
    let lane_top: f32 = 42.0;
    let row_gap: f32 = 52.0;
    let height = 112.0 + messages.len() as f32 * row_gap;
    let marker_id = mermaid_marker_id(source);
    let mut svg = format!(
        "<div class=\"dm-mermaid-chart dm-mermaid-zenuml\" role=\"img\" aria-label=\"Rendered Mermaid ZenUML diagram\"><div class=\"dm-mermaid-chart-title\">Mermaid ZenUML diagram</div><svg viewBox=\"0 0 {width:.0} {height:.0}\" aria-hidden=\"true\"><defs><marker id=\"{marker_id}\" markerWidth=\"10\" markerHeight=\"10\" refX=\"8\" refY=\"3\" orient=\"auto\"><path d=\"M0,0 L0,6 L9,3 z\" class=\"dm-mermaid-arrow\" /></marker></defs>"
    );

    let lane_step = if participants.len() == 1 {
        0.0
    } else {
        560.0 / (participants.len() - 1) as f32
    };
    let positions = participants
        .iter()
        .enumerate()
        .map(|(index, participant)| {
            let x = if participants.len() == 1 {
                width / 2.0
            } else {
                100.0 + index as f32 * lane_step
            };
            (participant.as_str(), x)
        })
        .collect::<Vec<_>>();

    for (participant, x) in &positions {
        svg.push_str(&format!(
            "<g class=\"dm-zenuml-participant\"><rect x=\"{:.1}\" y=\"{lane_top:.1}\" width=\"128\" height=\"38\" rx=\"6\" /><text x=\"{x:.1}\" y=\"{:.1}\" text-anchor=\"middle\">{}</text><line x1=\"{x:.1}\" y1=\"{:.1}\" x2=\"{x:.1}\" y2=\"{:.1}\" /></g>",
            x - 64.0,
            lane_top + 24.0,
            escape_html(participant),
            lane_top + 38.0,
            height - 24.0
        ));
    }

    for (index, message) in messages.iter().enumerate() {
        let y = 116.0 + index as f32 * row_gap;
        let Some((_, from_x)) = positions
            .iter()
            .find(|(participant, _)| *participant == message.from)
        else {
            continue;
        };
        let Some((_, to_x)) = positions
            .iter()
            .find(|(participant, _)| *participant == message.to)
        else {
            continue;
        };
        let direction = if from_x <= to_x { 1.0 } else { -1.0 };
        svg.push_str(&format!(
            "<line class=\"dm-zenuml-message\" x1=\"{:.1}\" y1=\"{y:.1}\" x2=\"{:.1}\" y2=\"{y:.1}\" marker-end=\"url(#{marker_id})\" /><text class=\"dm-zenuml-message-label\" x=\"{:.1}\" y=\"{:.1}\" text-anchor=\"middle\">{}</text>",
            from_x + direction * 64.0,
            to_x - direction * 64.0,
            (from_x + to_x) / 2.0,
            y - 8.0,
            escape_html(&message.label)
        ));
    }

    svg.push_str("</svg></div>");
    svg
}

fn parse_zenuml_message(line: &str) -> Option<ZenUmlMessage> {
    let (from, rest) = line.split_once("->")?;
    let (to, label) = rest.split_once(':')?;
    Some(ZenUmlMessage {
        from: clean_mermaid_label(from),
        to: clean_mermaid_label(to.trim_start_matches('>')),
        label: clean_mermaid_label(label),
    })
}

struct SankeyLink {
    from: String,
    to: String,
    value: f32,
}

fn render_mermaid_sankey(source: &str) -> String {
    let links = source
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty() && !line.starts_with("%%") && !line.starts_with("sankey"))
        .filter_map(parse_sankey_link)
        .collect::<Vec<_>>();

    if links.is_empty() {
        return render_mermaid_summary("Sankey diagram", "sankey", source);
    }

    let mut nodes = Vec::new();
    for link in &links {
        upsert_text(&mut nodes, link.from.clone());
        upsert_text(&mut nodes, link.to.clone());
    }

    let mut layers = vec![0_usize; nodes.len()];
    for _ in 0..nodes.len() {
        for link in &links {
            let Some(from_index) = nodes.iter().position(|node| node == &link.from) else {
                continue;
            };
            let Some(to_index) = nodes.iter().position(|node| node == &link.to) else {
                continue;
            };
            layers[to_index] = layers[to_index].max(layers[from_index] + 1);
        }
    }

    let max_layer = layers.iter().copied().max().unwrap_or(0).max(1);
    let max_value = links
        .iter()
        .map(|link| link.value)
        .fold(0.0_f32, f32::max)
        .max(1.0);
    let width: f32 = 760.0;
    let height: f32 = 330.0;
    let mut positions = Vec::new();
    for (index, node) in nodes.iter().enumerate() {
        let layer = layers[index];
        let layer_nodes = nodes
            .iter()
            .enumerate()
            .filter(|(node_index, _)| layers[*node_index] == layer)
            .map(|(node_index, _)| node_index)
            .collect::<Vec<_>>();
        let slot = layer_nodes
            .iter()
            .position(|node_index| *node_index == index)
            .unwrap_or(0);
        let x = 96.0 + layer as f32 * (568.0 / max_layer as f32);
        let y = 72.0 + (slot + 1) as f32 * (198.0 / (layer_nodes.len() + 1) as f32);
        positions.push((node.as_str(), x, y));
    }

    let mut svg = format!(
        "<div class=\"dm-mermaid-chart dm-mermaid-sankey\" role=\"img\" aria-label=\"Rendered Mermaid Sankey diagram\"><div class=\"dm-mermaid-chart-title\">Mermaid Sankey diagram</div><svg viewBox=\"0 0 {width:.0} {height:.0}\" aria-hidden=\"true\">"
    );

    for (index, link) in links.iter().enumerate() {
        let Some((_, from_x, from_y)) = positions.iter().find(|(node, _, _)| *node == link.from)
        else {
            continue;
        };
        let Some((_, to_x, to_y)) = positions.iter().find(|(node, _, _)| *node == link.to) else {
            continue;
        };
        let stroke_width = 5.0 + (link.value / max_value) * 24.0;
        svg.push_str(&format!(
            "<path class=\"dm-sankey-link dm-sankey-link-{}\" d=\"M {:.1} {from_y:.1} C {:.1} {from_y:.1}, {:.1} {to_y:.1}, {:.1} {to_y:.1}\" style=\"stroke-width: {stroke_width:.1}\" />",
            index % 6 + 1,
            from_x + 64.0,
            from_x + 160.0,
            to_x - 160.0,
            to_x - 64.0
        ));
    }

    for (node, x, y) in positions {
        svg.push_str(&format!(
            "<g class=\"dm-sankey-node\"><rect x=\"{:.1}\" y=\"{:.1}\" width=\"128\" height=\"38\" rx=\"5\" /><text x=\"{x:.1}\" y=\"{:.1}\" text-anchor=\"middle\">{}</text></g>",
            x - 64.0,
            y - 19.0,
            y + 5.0,
            escape_html(node)
        ));
    }

    svg.push_str("</svg></div>");
    svg
}

fn parse_sankey_link(line: &str) -> Option<SankeyLink> {
    let mut parts = line.split(',').map(str::trim);
    let from = clean_mermaid_label(parts.next()?);
    let to = clean_mermaid_label(parts.next()?);
    let value = parts.next()?.parse::<f32>().ok()?;
    Some(SankeyLink { from, to, value })
}

fn render_mermaid_xy_chart(source: &str) -> String {
    let mut title = "XY chart".to_owned();
    let mut labels = Vec::new();
    let mut values = Vec::new();
    let mut y_max = None;

    for line in source.lines().map(str::trim) {
        if line.is_empty() || line.starts_with("%%") || line.starts_with("xychart") {
            continue;
        }

        if let Some(value) = line.strip_prefix("title ") {
            title = clean_mermaid_label(value);
            continue;
        }

        if let Some(value) = line.strip_prefix("x-axis ") {
            labels = parse_bracketed_list(value);
            continue;
        }

        if let Some(value) = line.strip_prefix("y-axis ") {
            if let Some((_, max)) = value.split_once("-->") {
                y_max = max.trim().parse::<f32>().ok();
            }
            continue;
        }

        if let Some(value) = line.strip_prefix("bar ") {
            values = parse_number_list(value);
        }
    }

    if values.is_empty() {
        return render_mermaid_summary("XY chart", "xy-chart", source);
    }

    if labels.is_empty() {
        labels = (1..=values.len()).map(|index| index.to_string()).collect();
    }

    let y_max = y_max
        .unwrap_or_else(|| values.iter().copied().fold(0.0_f32, f32::max))
        .max(1.0);
    let width: f32 = 760.0;
    let height: f32 = 330.0;
    let left: f32 = 84.0;
    let top: f32 = 54.0;
    let chart_width: f32 = 610.0;
    let chart_height: f32 = 210.0;
    let bar_slot = chart_width / values.len().max(1) as f32;
    let bar_width = (bar_slot * 0.56).clamp(24.0, 70.0);
    let mut svg = format!(
        "<div class=\"dm-mermaid-chart dm-mermaid-xy\" role=\"img\" aria-label=\"Rendered Mermaid XY chart\"><div class=\"dm-mermaid-chart-title\">Mermaid XY chart</div><svg viewBox=\"0 0 {width:.0} {height:.0}\" aria-hidden=\"true\"><text class=\"dm-xy-title\" x=\"380\" y=\"28\" text-anchor=\"middle\">{}</text>",
        escape_html(&title)
    );

    for tick in 0..=4 {
        let ratio = tick as f32 / 4.0;
        let y = top + chart_height - chart_height * ratio;
        svg.push_str(&format!(
            "<line class=\"dm-xy-grid\" x1=\"{left:.1}\" y1=\"{y:.1}\" x2=\"{:.1}\" y2=\"{y:.1}\" /><text class=\"dm-xy-tick\" x=\"{:.1}\" y=\"{:.1}\" text-anchor=\"end\">{:.0}</text>",
            left + chart_width,
            left - 10.0,
            y + 4.0,
            y_max * ratio
        ));
    }
    svg.push_str(&format!(
        "<line class=\"dm-xy-axis\" x1=\"{left:.1}\" y1=\"{top:.1}\" x2=\"{left:.1}\" y2=\"{:.1}\" /><line class=\"dm-xy-axis\" x1=\"{left:.1}\" y1=\"{:.1}\" x2=\"{:.1}\" y2=\"{:.1}\" />",
        top + chart_height,
        top + chart_height,
        left + chart_width,
        top + chart_height
    ));

    for (index, value) in values.iter().enumerate() {
        let x = left + bar_slot * index as f32 + bar_slot / 2.0;
        let bar_height = ((*value / y_max).clamp(0.0, 1.0)) * chart_height;
        let y = top + chart_height - bar_height;
        let label = labels.get(index).map(String::as_str).unwrap_or("");
        svg.push_str(&format!(
            "<rect class=\"dm-xy-bar dm-xy-bar-{}\" x=\"{:.1}\" y=\"{y:.1}\" width=\"{bar_width:.1}\" height=\"{bar_height:.1}\" rx=\"4\" /><text class=\"dm-xy-value\" x=\"{x:.1}\" y=\"{:.1}\" text-anchor=\"middle\">{value:.0}</text><text class=\"dm-xy-label\" x=\"{x:.1}\" y=\"{:.1}\" text-anchor=\"middle\">{}</text>",
            index % 6 + 1,
            x - bar_width / 2.0,
            y - 8.0,
            top + chart_height + 24.0,
            escape_html(label)
        ));
    }

    svg.push_str("</svg></div>");
    svg
}

fn parse_bracketed_list(value: &str) -> Vec<String> {
    value
        .trim()
        .strip_prefix('[')
        .and_then(|value| value.strip_suffix(']'))
        .unwrap_or(value)
        .split(',')
        .map(clean_mermaid_label)
        .filter(|item| !item.is_empty())
        .collect()
}

fn parse_number_list(value: &str) -> Vec<f32> {
    value
        .trim()
        .strip_prefix('[')
        .and_then(|value| value.strip_suffix(']'))
        .unwrap_or(value)
        .split(',')
        .filter_map(|item| item.trim().parse::<f32>().ok())
        .collect()
}

struct BlockNode {
    id: String,
    label: String,
}

struct BlockEdge {
    from: String,
    to: String,
}

fn render_mermaid_block(source: &str) -> String {
    let mut nodes = Vec::new();
    let mut edges = Vec::new();
    let mut columns = 3_usize;

    for line in source.lines().map(str::trim) {
        if line.is_empty() || line.starts_with("%%") || line.starts_with("block") {
            continue;
        }

        if let Some(value) = line.strip_prefix("columns ") {
            columns = value.trim().parse::<usize>().unwrap_or(columns).clamp(1, 4);
            continue;
        }

        if let Some((from, to)) = line.split_once("-->") {
            edges.push(BlockEdge {
                from: clean_mermaid_label(from),
                to: clean_mermaid_label(to),
            });
            continue;
        }

        for node in parse_block_nodes(line) {
            if !nodes
                .iter()
                .any(|existing: &BlockNode| existing.id == node.id)
            {
                nodes.push(node);
            }
        }
    }

    if nodes.is_empty() {
        return render_mermaid_summary("Block diagram", "block", source);
    }

    let node_width: f32 = 156.0;
    let node_height: f32 = 62.0;
    let width: f32 = 760.0;
    let rows = nodes.len().div_ceil(columns);
    let height = 80.0 + rows as f32 * 112.0;
    let marker_id = mermaid_marker_id(source);
    let positions = nodes
        .iter()
        .enumerate()
        .map(|(index, node)| {
            let col = index % columns;
            let row = index / columns;
            let x = width * (col + 1) as f32 / (columns + 1) as f32;
            let y = 72.0 + row as f32 * 112.0;
            (node.id.as_str(), x, y)
        })
        .collect::<Vec<_>>();
    let mut svg = format!(
        "<div class=\"dm-mermaid-chart dm-mermaid-block\" role=\"img\" aria-label=\"Rendered Mermaid block diagram\"><div class=\"dm-mermaid-chart-title\">Mermaid Block diagram</div><svg viewBox=\"0 0 {width:.0} {height:.0}\" aria-hidden=\"true\"><defs><marker id=\"{marker_id}\" markerWidth=\"10\" markerHeight=\"10\" refX=\"8\" refY=\"3\" orient=\"auto\"><path d=\"M0,0 L0,6 L9,3 z\" class=\"dm-mermaid-arrow\" /></marker></defs>"
    );

    for edge in &edges {
        let Some((_, from_x, from_y)) = positions.iter().find(|(id, _, _)| *id == edge.from) else {
            continue;
        };
        let Some((_, to_x, to_y)) = positions.iter().find(|(id, _, _)| *id == edge.to) else {
            continue;
        };
        let (start_x, start_y) =
            rect_edge_point(*from_x, *from_y, *to_x, *to_y, node_width, node_height);
        let (end_x, end_y) =
            rect_edge_point(*to_x, *to_y, *from_x, *from_y, node_width, node_height);
        svg.push_str(&format!(
            "<line class=\"dm-block-edge\" x1=\"{start_x:.1}\" y1=\"{start_y:.1}\" x2=\"{end_x:.1}\" y2=\"{end_y:.1}\" marker-end=\"url(#{marker_id})\" />"
        ));
    }

    for (index, node) in nodes.iter().enumerate() {
        let (_, x, y) = positions[index];
        svg.push_str(&format!(
            "<g class=\"dm-block-node\"><rect x=\"{:.1}\" y=\"{:.1}\" width=\"{node_width:.1}\" height=\"{node_height:.1}\" rx=\"7\" /><text x=\"{x:.1}\" y=\"{:.1}\" text-anchor=\"middle\">{}</text></g>",
            x - node_width / 2.0,
            y - node_height / 2.0,
            y + 5.0,
            escape_html(&node.label)
        ));
    }

    svg.push_str("</svg></div>");
    svg
}

fn parse_block_nodes(line: &str) -> Vec<BlockNode> {
    let mut nodes = Vec::new();
    let chars = line.char_indices().collect::<Vec<_>>();
    let mut index = 0;

    while index < chars.len() {
        while index < chars.len() && chars[index].1.is_whitespace() {
            index += 1;
        }
        if index >= chars.len() {
            break;
        }

        let id_start = chars[index].0;
        while index < chars.len() && !chars[index].1.is_whitespace() && chars[index].1 != '[' {
            index += 1;
        }
        let id_end = if index < chars.len() {
            chars[index].0
        } else {
            line.len()
        };
        let id = clean_mermaid_label(&line[id_start..id_end]);

        let mut label = id.clone();
        if index < chars.len() && chars[index].1 == '[' {
            let label_start = chars[index].0 + 1;
            index += 1;
            let mut depth = 1_i32;
            let mut label_end = line.len();
            while index < chars.len() {
                match chars[index].1 {
                    '[' => depth += 1,
                    ']' => {
                        depth -= 1;
                        if depth == 0 {
                            label_end = chars[index].0;
                            index += 1;
                            break;
                        }
                    }
                    _ => {}
                }
                index += 1;
            }
            label = clean_mermaid_label(&line[label_start..label_end]);
        }

        if !id.is_empty() {
            nodes.push(BlockNode { id, label });
        }
    }

    nodes
}

struct PacketField {
    start: u32,
    end: u32,
    label: String,
}

fn render_mermaid_packet(source: &str) -> String {
    let mut title = "Packet diagram".to_owned();
    let mut fields = Vec::new();

    for line in source.lines().map(str::trim) {
        if line.is_empty() || line.starts_with("%%") || line.starts_with("packet") {
            continue;
        }

        if let Some(value) = line.strip_prefix("title ") {
            title = clean_mermaid_label(value);
            continue;
        }

        if let Some(field) = parse_packet_field(line) {
            fields.push(field);
        }
    }

    if fields.is_empty() {
        return render_mermaid_summary("Packet diagram", "packet", source);
    }

    let width: f32 = 760.0;
    let left: f32 = 52.0;
    let top: f32 = 82.0;
    let packet_width: f32 = 656.0;
    let field_height: f32 = 78.0;
    let max_bit = fields
        .iter()
        .map(|field| field.end)
        .max()
        .unwrap_or(1)
        .max(1);
    let mut svg = format!(
        "<div class=\"dm-mermaid-chart dm-mermaid-packet\" role=\"img\" aria-label=\"Rendered Mermaid packet diagram\"><div class=\"dm-mermaid-chart-title\">Mermaid Packet diagram</div><svg viewBox=\"0 0 {width:.0} 210\" aria-hidden=\"true\"><text class=\"dm-packet-title\" x=\"380\" y=\"32\" text-anchor=\"middle\">{}</text>",
        escape_html(&title)
    );

    for field in &fields {
        let x = left + (field.start as f32 / (max_bit + 1) as f32) * packet_width;
        let end_x = left + ((field.end + 1) as f32 / (max_bit + 1) as f32) * packet_width;
        let field_width = (end_x - x).max(38.0);
        svg.push_str(&format!(
            "<g class=\"dm-packet-field\"><rect x=\"{x:.1}\" y=\"{top:.1}\" width=\"{field_width:.1}\" height=\"{field_height:.1}\" rx=\"3\" /><text class=\"dm-packet-range\" x=\"{:.1}\" y=\"{:.1}\" text-anchor=\"middle\">{}-{}</text><text class=\"dm-packet-label\" x=\"{:.1}\" y=\"{:.1}\" text-anchor=\"middle\">{}</text></g>",
            x + field_width / 2.0,
            top - 12.0,
            field.start,
            field.end,
            x + field_width / 2.0,
            top + 44.0,
            escape_html(&field.label)
        ));
    }

    svg.push_str("</svg></div>");
    svg
}

fn parse_packet_field(line: &str) -> Option<PacketField> {
    let (range, label) = line.split_once(':')?;
    let range = range.trim();
    let (start, end) = if let Some((start, end)) = range.split_once('-') {
        (
            start.trim().parse::<u32>().ok()?,
            end.trim().parse::<u32>().ok()?,
        )
    } else {
        let bit = range.parse::<u32>().ok()?;
        (bit, bit)
    };
    Some(PacketField {
        start,
        end: end.max(start),
        label: clean_mermaid_label(label),
    })
}

struct KanbanColumn {
    title: String,
    cards: Vec<String>,
}

fn render_mermaid_kanban(source: &str) -> String {
    let mut columns: Vec<KanbanColumn> = Vec::new();
    let mut current: Option<usize> = None;

    for line in source.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with("%%") || trimmed == "kanban" {
            continue;
        }

        let indent = line.chars().take_while(|ch| ch.is_whitespace()).count();
        if indent <= 2 {
            columns.push(KanbanColumn {
                title: clean_mermaid_label(trimmed),
                cards: Vec::new(),
            });
            current = Some(columns.len() - 1);
        } else if let Some(index) = current {
            columns[index].cards.push(clean_mermaid_label(trimmed));
        }
    }

    if columns.is_empty() {
        return render_mermaid_summary("Kanban board", "kanban", source);
    }

    let width: f32 = 760.0;
    let column_width: f32 = 210.0;
    let gap: f32 = 24.0;
    let max_cards = columns
        .iter()
        .map(|column| column.cards.len())
        .max()
        .unwrap_or(0);
    let height = 92.0 + max_cards as f32 * 58.0;
    let board_width =
        columns.len() as f32 * column_width + (columns.len().saturating_sub(1)) as f32 * gap;
    let start_x = (width - board_width) / 2.0;
    let mut svg = format!(
        "<div class=\"dm-mermaid-chart dm-mermaid-kanban\" role=\"img\" aria-label=\"Rendered Mermaid kanban board\"><div class=\"dm-mermaid-chart-title\">Mermaid Kanban board</div><svg viewBox=\"0 0 {width:.0} {height:.0}\" aria-hidden=\"true\">"
    );

    for (index, column) in columns.iter().enumerate() {
        let x = start_x + index as f32 * (column_width + gap);
        svg.push_str(&format!(
            "<g class=\"dm-kanban-column\"><rect x=\"{x:.1}\" y=\"44\" width=\"{column_width:.1}\" height=\"{:.1}\" rx=\"8\" /><text class=\"dm-kanban-title\" x=\"{:.1}\" y=\"72\">{}</text>",
            height - 58.0,
            x + 14.0,
            escape_html(&column.title)
        ));
        for (card_index, card) in column.cards.iter().enumerate() {
            let y = 92.0 + card_index as f32 * 52.0;
            svg.push_str(&format!(
                "<rect class=\"dm-kanban-card\" x=\"{:.1}\" y=\"{y:.1}\" width=\"{:.1}\" height=\"38\" rx=\"6\" /><text class=\"dm-kanban-card-text\" x=\"{:.1}\" y=\"{:.1}\">{}</text>",
                x + 12.0,
                column_width - 24.0,
                x + 24.0,
                y + 24.0,
                escape_html(card)
            ));
        }
        svg.push_str("</g>");
    }

    svg.push_str("</svg></div>");
    svg
}

struct ArchitectureGroup {
    id: String,
    label: String,
    icon: String,
}

struct ArchitectureService {
    id: String,
    label: String,
    icon: String,
    group: Option<String>,
}

struct ArchitectureEdge {
    from: String,
    to: String,
    label: String,
}

fn render_mermaid_architecture(source: &str) -> String {
    let mut groups = Vec::new();
    let mut services = Vec::new();
    let mut edges = Vec::new();

    for line in source.lines().map(str::trim) {
        if line.is_empty() || line.starts_with("%%") || line.starts_with("architecture") {
            continue;
        }

        if let Some(group) = parse_architecture_group(line) {
            groups.push(group);
            continue;
        }

        if let Some(service) = parse_architecture_service(line) {
            services.push(service);
            continue;
        }

        if let Some(edge) = parse_architecture_edge(line) {
            edges.push(edge);
        }
    }

    if services.is_empty() {
        return render_mermaid_summary("Architecture diagram", "architecture", source);
    }

    let width: f32 = 760.0;
    let height: f32 = 330.0;
    let node_width: f32 = 164.0;
    let node_height: f32 = 76.0;
    let marker_id = mermaid_marker_id(source);
    let positions = services
        .iter()
        .enumerate()
        .map(|(index, service)| {
            let x = if services.len() == 1 {
                width / 2.0
            } else {
                190.0 + index as f32 * (380.0 / (services.len() - 1) as f32)
            };
            (service.id.as_str(), x, 178.0)
        })
        .collect::<Vec<_>>();
    let mut svg = format!(
        "<div class=\"dm-mermaid-chart dm-mermaid-architecture\" role=\"img\" aria-label=\"Rendered Mermaid architecture diagram\"><div class=\"dm-mermaid-chart-title\">Mermaid Architecture diagram</div><svg viewBox=\"0 0 {width:.0} {height:.0}\" aria-hidden=\"true\"><defs><marker id=\"{marker_id}\" markerWidth=\"10\" markerHeight=\"10\" refX=\"8\" refY=\"3\" orient=\"auto\"><path d=\"M0,0 L0,6 L9,3 z\" class=\"dm-mermaid-arrow\" /></marker></defs>"
    );

    if let Some(group) = groups.first() {
        svg.push_str(&format!(
            "<g class=\"dm-architecture-group\"><rect x=\"76\" y=\"48\" width=\"608\" height=\"234\" rx=\"10\" /><text x=\"104\" y=\"78\">{} {}</text></g>",
            escape_html(&architecture_icon(&group.icon)),
            escape_html(&group.label)
        ));
    }

    for edge in &edges {
        let Some((_, from_x, from_y)) = positions.iter().find(|(id, _, _)| *id == edge.from) else {
            continue;
        };
        let Some((_, to_x, to_y)) = positions.iter().find(|(id, _, _)| *id == edge.to) else {
            continue;
        };
        let (start_x, start_y) =
            rect_edge_point(*from_x, *from_y, *to_x, *to_y, node_width, node_height);
        let (end_x, end_y) =
            rect_edge_point(*to_x, *to_y, *from_x, *from_y, node_width, node_height);
        svg.push_str(&format!(
            "<line class=\"dm-architecture-edge\" x1=\"{start_x:.1}\" y1=\"{start_y:.1}\" x2=\"{end_x:.1}\" y2=\"{end_y:.1}\" marker-end=\"url(#{marker_id})\" /><text class=\"dm-architecture-edge-label\" x=\"{:.1}\" y=\"{:.1}\" text-anchor=\"middle\">{}</text>",
            (start_x + end_x) / 2.0,
            (start_y + end_y) / 2.0 - 8.0,
            escape_html(&edge.label)
        ));
    }

    for (index, service) in services.iter().enumerate() {
        let (_, x, y) = positions[index];
        svg.push_str(&format!(
            "<g class=\"dm-architecture-service\"><rect x=\"{:.1}\" y=\"{:.1}\" width=\"{node_width:.1}\" height=\"{node_height:.1}\" rx=\"8\" /><text class=\"dm-architecture-icon\" x=\"{x:.1}\" y=\"{:.1}\" text-anchor=\"middle\">{}</text><text class=\"dm-architecture-label\" x=\"{x:.1}\" y=\"{:.1}\" text-anchor=\"middle\">{}</text><text class=\"dm-architecture-service-kind\" x=\"{x:.1}\" y=\"{:.1}\" text-anchor=\"middle\">{}</text></g>",
            x - node_width / 2.0,
            y - node_height / 2.0,
            y - 12.0,
            escape_html(&architecture_icon(&service.icon)),
            y + 8.0,
            escape_html(&service.label),
            y + 30.0,
            escape_html(service.group.as_deref().unwrap_or("service"))
        ));
    }

    svg.push_str("</svg></div>");
    svg
}

fn parse_architecture_group(line: &str) -> Option<ArchitectureGroup> {
    let value = line.strip_prefix("group ")?;
    let (id, icon, label, _) = parse_architecture_node(value)?;
    Some(ArchitectureGroup { id, icon, label })
}

fn parse_architecture_service(line: &str) -> Option<ArchitectureService> {
    let value = line.strip_prefix("service ")?;
    let (id, icon, label, group) = parse_architecture_node(value)?;
    Some(ArchitectureService {
        id,
        label,
        icon,
        group,
    })
}

fn parse_architecture_node(value: &str) -> Option<(String, String, String, Option<String>)> {
    let open_icon = value.find('(')?;
    let close_icon = value[open_icon + 1..].find(')')? + open_icon + 1;
    let open_label = value[close_icon + 1..].find('[')? + close_icon + 1;
    let close_label = value[open_label + 1..].find(']')? + open_label + 1;
    let id = clean_mermaid_label(&value[..open_icon]);
    let icon = clean_mermaid_label(&value[open_icon + 1..close_icon]);
    let label = clean_mermaid_label(&value[open_label + 1..close_label]);
    let group = value[close_label + 1..]
        .trim()
        .strip_prefix("in ")
        .map(clean_mermaid_label);
    Some((id, icon, label, group))
}

fn parse_architecture_edge(line: &str) -> Option<ArchitectureEdge> {
    let (from, rest) = line.split_once("--")?;
    let mut from_parts = from.split(':');
    let from = clean_mermaid_label(from_parts.next()?);
    let rest = rest.trim();
    let to = if let Some((_, service_id)) = rest.split_once(':') {
        clean_mermaid_label(service_id)
    } else {
        clean_mermaid_label(rest)
    };
    let label = String::new();
    Some(ArchitectureEdge { from, to, label })
}

fn architecture_icon(icon: &str) -> String {
    match icon {
        "cloud" => "cloud".to_owned(),
        "server" => "server".to_owned(),
        "internet" => "net".to_owned(),
        "database" => "db".to_owned(),
        value => value.to_owned(),
    }
}

struct RadarCurve {
    label: String,
    values: Vec<f32>,
}

fn render_mermaid_radar(source: &str) -> String {
    let mut title = "Radar chart".to_owned();
    let mut axes = Vec::new();
    let mut curves = Vec::new();

    for line in source.lines().map(str::trim) {
        if line.is_empty() || line.starts_with("%%") || line.starts_with("radar") {
            continue;
        }

        if let Some(value) = line.strip_prefix("title ") {
            title = clean_mermaid_label(value);
            continue;
        }

        if let Some(value) = line.strip_prefix("axis ") {
            axes = parse_bracketed_list(value);
            continue;
        }

        if let Some(curve) = parse_radar_curve(line) {
            curves.push(curve);
        }
    }

    if axes.is_empty() || curves.is_empty() {
        return render_mermaid_summary("Radar chart", "radar", source);
    }

    let width: f32 = 760.0;
    let height: f32 = 390.0;
    let center_x: f32 = 380.0;
    let center_y: f32 = 210.0;
    let radius: f32 = 120.0;
    let max_value = curves
        .iter()
        .flat_map(|curve| curve.values.iter().copied())
        .fold(0.0_f32, f32::max)
        .max(1.0);
    let mut svg = format!(
        "<div class=\"dm-mermaid-chart dm-mermaid-radar\" role=\"img\" aria-label=\"Rendered Mermaid radar chart\"><div class=\"dm-mermaid-chart-title\">Mermaid Radar chart</div><svg viewBox=\"0 0 {width:.0} {height:.0}\" aria-hidden=\"true\"><text class=\"dm-radar-title\" x=\"380\" y=\"32\" text-anchor=\"middle\">{}</text>",
        escape_html(&title)
    );

    for level in 1..=4 {
        let points = radar_points(axes.len(), center_x, center_y, radius * level as f32 / 4.0);
        svg.push_str(&format!(
            "<polygon class=\"dm-radar-grid\" points=\"{}\" />",
            format_points(&points)
        ));
    }

    for (index, axis) in axes.iter().enumerate() {
        let angle =
            -std::f32::consts::FRAC_PI_2 + index as f32 * std::f32::consts::TAU / axes.len() as f32;
        let x = center_x + angle.cos() * radius;
        let y = center_y + angle.sin() * radius;
        let label_x = center_x + angle.cos() * (radius + 32.0);
        let label_y = center_y + angle.sin() * (radius + 32.0);
        svg.push_str(&format!(
            "<line class=\"dm-radar-axis\" x1=\"{center_x:.1}\" y1=\"{center_y:.1}\" x2=\"{x:.1}\" y2=\"{y:.1}\" /><text class=\"dm-radar-axis-label\" x=\"{label_x:.1}\" y=\"{label_y:.1}\" text-anchor=\"middle\">{}</text>",
            escape_html(axis)
        ));
    }

    for (curve_index, curve) in curves.iter().enumerate() {
        let points = axes
            .iter()
            .enumerate()
            .map(|(index, _)| {
                let value = curve.values.get(index).copied().unwrap_or(0.0);
                let scaled_radius = radius * (value / max_value).clamp(0.0, 1.0);
                let angle = -std::f32::consts::FRAC_PI_2
                    + index as f32 * std::f32::consts::TAU / axes.len() as f32;
                (
                    center_x + angle.cos() * scaled_radius,
                    center_y + angle.sin() * scaled_radius,
                )
            })
            .collect::<Vec<_>>();
        svg.push_str(&format!(
            "<polygon class=\"dm-radar-area dm-radar-area-{}\" points=\"{}\" /><polyline class=\"dm-radar-line dm-radar-line-{}\" points=\"{}\" />",
            curve_index % 6 + 1,
            format_points(&points),
            curve_index % 6 + 1,
            format_points(&points)
        ));
        svg.push_str(&format!(
            "<text class=\"dm-radar-legend\" x=\"612\" y=\"{}\">{}</text>",
            78 + curve_index * 22,
            escape_html(&curve.label)
        ));
    }

    svg.push_str("</svg></div>");
    svg
}

fn parse_radar_curve(line: &str) -> Option<RadarCurve> {
    let value = line.strip_prefix("curve ")?;
    let open = value.find('{')?;
    let close = value.rfind('}')?;
    Some(RadarCurve {
        label: clean_mermaid_label(&value[..open]),
        values: value[open + 1..close]
            .split(',')
            .filter_map(|item| item.trim().parse::<f32>().ok())
            .collect(),
    })
}

fn radar_points(count: usize, center_x: f32, center_y: f32, radius: f32) -> Vec<(f32, f32)> {
    (0..count)
        .map(|index| {
            let angle =
                -std::f32::consts::FRAC_PI_2 + index as f32 * std::f32::consts::TAU / count as f32;
            (
                center_x + angle.cos() * radius,
                center_y + angle.sin() * radius,
            )
        })
        .collect()
}

fn format_points(points: &[(f32, f32)]) -> String {
    points
        .iter()
        .map(|(x, y)| format!("{x:.1},{y:.1}"))
        .collect::<Vec<_>>()
        .join(" ")
}

struct EventModelItem {
    kind: String,
    label: String,
}

fn render_mermaid_event_modeling(source: &str) -> String {
    let mut items = Vec::new();
    for line in source.lines().map(str::trim) {
        if line.is_empty() || line.starts_with("%%") || line.starts_with("eventModeling") {
            continue;
        }
        let mut parts = line.splitn(2, char::is_whitespace);
        let kind = parts.next().unwrap_or_default();
        let label = parts.next().unwrap_or_default();
        if !kind.is_empty() && !label.is_empty() {
            items.push(EventModelItem {
                kind: clean_mermaid_label(kind),
                label: clean_mermaid_label(label),
            });
        }
    }

    if items.is_empty() {
        return render_mermaid_summary("Event modeling diagram", "event-modeling", source);
    }

    let width: f32 = 760.0;
    let height: f32 = 260.0;
    let marker_id = mermaid_marker_id(source);
    let positions = items
        .iter()
        .enumerate()
        .map(|(index, item)| {
            let x = width * (index + 1) as f32 / (items.len() + 1) as f32;
            (item.kind.as_str(), x, 128.0)
        })
        .collect::<Vec<_>>();
    let mut svg = format!(
        "<div class=\"dm-mermaid-chart dm-mermaid-event-modeling\" role=\"img\" aria-label=\"Rendered Mermaid event modeling diagram\"><div class=\"dm-mermaid-chart-title\">Mermaid Event modeling diagram</div><svg viewBox=\"0 0 {width:.0} {height:.0}\" aria-hidden=\"true\"><defs><marker id=\"{marker_id}\" markerWidth=\"10\" markerHeight=\"10\" refX=\"8\" refY=\"3\" orient=\"auto\"><path d=\"M0,0 L0,6 L9,3 z\" class=\"dm-mermaid-arrow\" /></marker></defs>"
    );

    for pair in positions.windows(2) {
        let (_, from_x, from_y) = pair[0];
        let (_, to_x, to_y) = pair[1];
        svg.push_str(&format!(
            "<line class=\"dm-event-modeling-edge\" x1=\"{:.1}\" y1=\"{from_y:.1}\" x2=\"{:.1}\" y2=\"{to_y:.1}\" marker-end=\"url(#{marker_id})\" />",
            from_x + 84.0,
            to_x - 84.0
        ));
    }

    for (index, item) in items.iter().enumerate() {
        let (_, x, y) = positions[index];
        svg.push_str(&format!(
            "<g class=\"dm-event-modeling-node dm-event-modeling-{}\"><rect x=\"{:.1}\" y=\"{:.1}\" width=\"168\" height=\"76\" rx=\"8\" /><text class=\"dm-event-modeling-kind\" x=\"{x:.1}\" y=\"{:.1}\" text-anchor=\"middle\">{}</text><text class=\"dm-event-modeling-label\" x=\"{x:.1}\" y=\"{:.1}\" text-anchor=\"middle\">{}</text></g>",
            escape_attribute(&item.kind),
            x - 84.0,
            y - 38.0,
            y - 10.0,
            escape_html(&item.kind),
            y + 16.0,
            escape_html(&item.label)
        ));
    }

    svg.push_str("</svg></div>");
    svg
}

struct TreemapEntry {
    label: String,
    value: f32,
}

fn render_mermaid_treemap(source: &str) -> String {
    let mut root = "Treemap".to_owned();
    let mut entries = Vec::new();

    for line in source.lines().map(str::trim) {
        if line.is_empty() || line.starts_with("%%") || line.starts_with("treemap") {
            continue;
        }
        if let Some((label, value)) = line.split_once(':') {
            if let Ok(value) = value.trim().parse::<f32>() {
                entries.push(TreemapEntry {
                    label: clean_mermaid_label(label),
                    value: value.max(0.0),
                });
            }
        } else {
            root = clean_mermaid_label(line);
        }
    }

    if entries.is_empty() {
        return render_mermaid_summary("Treemap", "treemap", source);
    }

    let width: f32 = 760.0;
    let height: f32 = 300.0;
    let left: f32 = 52.0;
    let top: f32 = 72.0;
    let map_width: f32 = 656.0;
    let map_height: f32 = 178.0;
    let total = entries
        .iter()
        .map(|entry| entry.value)
        .sum::<f32>()
        .max(1.0);
    let mut cursor = left;
    let mut svg = format!(
        "<div class=\"dm-mermaid-chart dm-mermaid-treemap\" role=\"img\" aria-label=\"Rendered Mermaid treemap\"><div class=\"dm-mermaid-chart-title\">Mermaid Treemap</div><svg viewBox=\"0 0 {width:.0} {height:.0}\" aria-hidden=\"true\"><text class=\"dm-treemap-title\" x=\"380\" y=\"34\" text-anchor=\"middle\">{}</text><rect class=\"dm-treemap-root\" x=\"{left:.1}\" y=\"{top:.1}\" width=\"{map_width:.1}\" height=\"{map_height:.1}\" rx=\"8\" />",
        escape_html(&root)
    );

    for (index, entry) in entries.iter().enumerate() {
        let rect_width = if index == entries.len() - 1 {
            left + map_width - cursor
        } else {
            (map_width * entry.value / total).max(54.0)
        };
        svg.push_str(&format!(
            "<g class=\"dm-treemap-leaf dm-treemap-leaf-{}\"><rect x=\"{cursor:.1}\" y=\"{top:.1}\" width=\"{rect_width:.1}\" height=\"{map_height:.1}\" /><text x=\"{:.1}\" y=\"{:.1}\" text-anchor=\"middle\">{}</text><text x=\"{:.1}\" y=\"{:.1}\" text-anchor=\"middle\">{:.0}</text></g>",
            index % 6 + 1,
            cursor + rect_width / 2.0,
            top + 82.0,
            escape_html(&entry.label),
            cursor + rect_width / 2.0,
            top + 106.0,
            entry.value
        ));
        cursor += rect_width;
    }

    svg.push_str("</svg></div>");
    svg
}

struct VennSet {
    label: String,
    value: f32,
}

fn render_mermaid_venn(source: &str) -> String {
    let mut sets = Vec::new();
    let mut intersection: Option<VennSet> = None;

    for line in source.lines().map(str::trim) {
        if line.is_empty() || line.starts_with("%%") || line.starts_with("venn") {
            continue;
        }
        if let Some((label, value)) = line.split_once(':') {
            let set = VennSet {
                label: clean_mermaid_label(label),
                value: value.trim().parse::<f32>().unwrap_or(0.0),
            };
            if set.label.contains('&') {
                intersection = Some(set);
            } else {
                sets.push(set);
            }
        }
    }

    if sets.len() < 2 {
        return render_mermaid_summary("Venn diagram", "venn", source);
    }

    let width: f32 = 760.0;
    let height: f32 = 320.0;
    let left = &sets[0];
    let right = &sets[1];
    let overlap = intersection.unwrap_or(VennSet {
        label: format!("{} & {}", left.label, right.label),
        value: 0.0,
    });
    let mut svg = format!(
        "<div class=\"dm-mermaid-chart dm-mermaid-venn\" role=\"img\" aria-label=\"Rendered Mermaid venn diagram\"><div class=\"dm-mermaid-chart-title\">Mermaid Venn diagram</div><svg viewBox=\"0 0 {width:.0} {height:.0}\" aria-hidden=\"true\">"
    );
    svg.push_str(
        "<circle class=\"dm-venn-circle dm-venn-left\" cx=\"330\" cy=\"160\" r=\"104\" /><circle class=\"dm-venn-circle dm-venn-right\" cx=\"430\" cy=\"160\" r=\"104\" />",
    );
    svg.push_str(&format!(
        "<text class=\"dm-venn-label\" x=\"280\" y=\"152\" text-anchor=\"middle\">{}</text><text class=\"dm-venn-value\" x=\"280\" y=\"176\" text-anchor=\"middle\">{:.0}</text><text class=\"dm-venn-label\" x=\"480\" y=\"152\" text-anchor=\"middle\">{}</text><text class=\"dm-venn-value\" x=\"480\" y=\"176\" text-anchor=\"middle\">{:.0}</text><text class=\"dm-venn-label\" x=\"380\" y=\"152\" text-anchor=\"middle\">Both</text><text class=\"dm-venn-value\" x=\"380\" y=\"176\" text-anchor=\"middle\">{:.0}</text>",
        escape_html(&left.label),
        left.value,
        escape_html(&right.label),
        right.value,
        overlap.value
    ));
    svg.push_str("</svg></div>");
    svg
}

struct IshikawaBranch {
    label: String,
    causes: Vec<String>,
}

fn render_mermaid_ishikawa(source: &str) -> String {
    let mut root = "Effect".to_owned();
    let mut branches = Vec::new();
    let mut current: Option<usize> = None;

    for line in source.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with("%%") || trimmed == "ishikawa" {
            continue;
        }
        let indent = line.chars().take_while(|ch| ch.is_whitespace()).count();
        if let Some(value) = trimmed.strip_prefix("root") {
            root = clean_mermaid_label(value);
        } else if indent <= 4 {
            branches.push(IshikawaBranch {
                label: clean_mermaid_label(trimmed),
                causes: Vec::new(),
            });
            current = Some(branches.len() - 1);
        } else if let Some(index) = current {
            branches[index].causes.push(clean_mermaid_label(trimmed));
        }
    }

    if branches.is_empty() {
        return render_mermaid_summary("Ishikawa diagram", "ishikawa", source);
    }

    let width: f32 = 760.0;
    let height: f32 = 340.0;
    let spine_y: f32 = 170.0;
    let mut svg = format!(
        "<div class=\"dm-mermaid-chart dm-mermaid-ishikawa\" role=\"img\" aria-label=\"Rendered Mermaid ishikawa diagram\"><div class=\"dm-mermaid-chart-title\">Mermaid Ishikawa diagram</div><svg viewBox=\"0 0 {width:.0} {height:.0}\" aria-hidden=\"true\"><line class=\"dm-ishikawa-spine\" x1=\"96\" y1=\"{spine_y:.1}\" x2=\"592\" y2=\"{spine_y:.1}\" /><polygon class=\"dm-ishikawa-head\" points=\"592,138 704,170 592,202\" /><text class=\"dm-ishikawa-root\" x=\"638\" y=\"175\" text-anchor=\"middle\">{}</text>",
        escape_html(&root)
    );

    for (index, branch) in branches.iter().enumerate() {
        let x = 150.0 + index as f32 * (380.0 / branches.len().max(1) as f32);
        let upward = index % 2 == 0;
        let end_y = if upward { 78.0 } else { 262.0 };
        svg.push_str(&format!(
            "<line class=\"dm-ishikawa-bone\" x1=\"{x:.1}\" y1=\"{spine_y:.1}\" x2=\"{:.1}\" y2=\"{end_y:.1}\" /><text class=\"dm-ishikawa-branch\" x=\"{:.1}\" y=\"{:.1}\" text-anchor=\"middle\">{}</text>",
            x + 82.0,
            x + 96.0,
            if upward { end_y - 10.0 } else { end_y + 22.0 },
            escape_html(&branch.label)
        ));
        for (cause_index, cause) in branch.causes.iter().take(2).enumerate() {
            let cause_y = if upward {
                end_y + 24.0 + cause_index as f32 * 20.0
            } else {
                end_y - 24.0 - cause_index as f32 * 20.0
            };
            svg.push_str(&format!(
                "<text class=\"dm-ishikawa-cause\" x=\"{:.1}\" y=\"{cause_y:.1}\" text-anchor=\"middle\">{}</text>",
                x + 86.0,
                escape_html(cause)
            ));
        }
    }

    svg.push_str("</svg></div>");
    svg
}

struct WardleyPoint {
    id: String,
    label: String,
    x: f32,
    y: f32,
    kind: String,
}

struct WardleyEdge {
    from: String,
    to: String,
}

fn render_mermaid_wardley(source: &str) -> String {
    let mut title = "Wardley map".to_owned();
    let mut points = Vec::new();
    let mut edges = Vec::new();

    for line in source.lines().map(str::trim) {
        if line.is_empty() || line.starts_with("%%") || line == "wardley" {
            continue;
        }
        if let Some(value) = line.strip_prefix("title ") {
            title = clean_mermaid_label(value);
            continue;
        }
        if let Some(point) = parse_wardley_point(line) {
            points.push(point);
            continue;
        }
        if let Some((from, to)) = line.split_once("->") {
            edges.push(WardleyEdge {
                from: clean_mermaid_label(from),
                to: clean_mermaid_label(to),
            });
        }
    }

    if points.is_empty() {
        return render_mermaid_summary("Wardley map", "wardley", source);
    }

    let width: f32 = 760.0;
    let height: f32 = 360.0;
    let left: f32 = 90.0;
    let top: f32 = 64.0;
    let map_width: f32 = 580.0;
    let map_height: f32 = 230.0;
    let marker_id = mermaid_marker_id(source);
    let positions = points
        .iter()
        .map(|point| {
            (
                point.id.as_str(),
                left + point.x.clamp(0.0, 1.0) * map_width,
                top + (1.0 - point.y.clamp(0.0, 1.0)) * map_height,
            )
        })
        .collect::<Vec<_>>();
    let mut svg = format!(
        "<div class=\"dm-mermaid-chart dm-mermaid-wardley\" role=\"img\" aria-label=\"Rendered Mermaid wardley map\"><div class=\"dm-mermaid-chart-title\">Mermaid Wardley map</div><svg viewBox=\"0 0 {width:.0} {height:.0}\" aria-hidden=\"true\"><defs><marker id=\"{marker_id}\" markerWidth=\"10\" markerHeight=\"10\" refX=\"8\" refY=\"3\" orient=\"auto\"><path d=\"M0,0 L0,6 L9,3 z\" class=\"dm-mermaid-arrow\" /></marker></defs><text class=\"dm-wardley-title\" x=\"380\" y=\"30\" text-anchor=\"middle\">{}</text><rect class=\"dm-wardley-plane\" x=\"{left:.1}\" y=\"{top:.1}\" width=\"{map_width:.1}\" height=\"{map_height:.1}\" /><text class=\"dm-wardley-axis\" x=\"{}\" y=\"318\" text-anchor=\"middle\">Genesis</text><text class=\"dm-wardley-axis\" x=\"{}\" y=\"318\" text-anchor=\"middle\">Commodity</text><text class=\"dm-wardley-axis\" x=\"40\" y=\"{}\" text-anchor=\"middle\">Visible</text>",
        escape_html(&title),
        left + 72.0,
        left + map_width - 72.0,
        top + 16.0
    );

    for edge in &edges {
        let Some((_, from_x, from_y)) = positions.iter().find(|(id, _, _)| *id == edge.from) else {
            continue;
        };
        let Some((_, to_x, to_y)) = positions.iter().find(|(id, _, _)| *id == edge.to) else {
            continue;
        };
        svg.push_str(&format!(
            "<line class=\"dm-wardley-edge\" x1=\"{from_x:.1}\" y1=\"{from_y:.1}\" x2=\"{to_x:.1}\" y2=\"{to_y:.1}\" marker-end=\"url(#{marker_id})\" />"
        ));
    }

    for (index, point) in points.iter().enumerate() {
        let (_, x, y) = positions[index];
        svg.push_str(&format!(
            "<g class=\"dm-wardley-point dm-wardley-{}\"><circle cx=\"{x:.1}\" cy=\"{y:.1}\" r=\"8\" /><text x=\"{:.1}\" y=\"{:.1}\">{}</text></g>",
            escape_attribute(&point.kind),
            x + 12.0,
            y - 10.0,
            escape_html(&point.label)
        ));
    }

    svg.push_str("</svg></div>");
    svg
}

fn parse_wardley_point(line: &str) -> Option<WardleyPoint> {
    let kind = if line.starts_with("anchor ") {
        "anchor"
    } else if line.starts_with("component ") {
        "component"
    } else {
        return None;
    };
    let value = line.strip_prefix(kind)?.trim();
    let bracket = value.rfind('[')?;
    let id_label = clean_mermaid_label(&value[..bracket]);
    let coords = value[bracket + 1..].trim_end_matches(']');
    let mut values = coords
        .split(',')
        .filter_map(|item| item.trim().parse::<f32>().ok());
    Some(WardleyPoint {
        id: id_label.clone(),
        label: id_label,
        x: values.next()?,
        y: values.next()?,
        kind: kind.to_owned(),
    })
}

struct CynefinDomain {
    name: String,
    items: Vec<String>,
}

fn render_mermaid_cynefin(source: &str) -> String {
    let mut title = "Cynefin diagram".to_owned();
    let mut domains = Vec::new();

    for line in source.lines().map(str::trim) {
        if line.is_empty() || line.starts_with("%%") || line.starts_with("cynefin") {
            continue;
        }
        if let Some(value) = line.strip_prefix("title ") {
            title = clean_mermaid_label(value);
            continue;
        }
        if let Some((domain, item)) = line.split_once(':') {
            domains.push(CynefinDomain {
                name: clean_mermaid_label(domain),
                items: vec![clean_mermaid_label(item)],
            });
        } else if matches!(
            line,
            "clear" | "simple" | "complicated" | "complex" | "chaotic" | "confusion"
        ) {
            domains.push(CynefinDomain {
                name: clean_mermaid_label(line),
                items: Vec::new(),
            });
        } else if let Some(domain) = domains.last_mut() {
            domain.items.push(clean_mermaid_label(line));
        }
    }

    if domains.is_empty() {
        return render_mermaid_summary("Cynefin diagram", "cynefin", source);
    }

    let width: f32 = 760.0;
    let height: f32 = 360.0;
    let cells = [
        ("Complex", 70.0, 70.0),
        ("Complicated", 390.0, 70.0),
        ("Chaotic", 70.0, 190.0),
        ("Simple", 390.0, 190.0),
    ];
    let mut svg = format!(
        "<div class=\"dm-mermaid-chart dm-mermaid-cynefin\" role=\"img\" aria-label=\"Rendered Mermaid cynefin diagram\"><div class=\"dm-mermaid-chart-title\">Mermaid Cynefin diagram</div><svg viewBox=\"0 0 {width:.0} {height:.0}\" aria-hidden=\"true\"><text class=\"dm-cynefin-title\" x=\"380\" y=\"34\" text-anchor=\"middle\">{}</text>",
        escape_html(&title)
    );
    for (name, x, y) in cells {
        let domain = domains
            .iter()
            .find(|domain| domain.name.eq_ignore_ascii_case(name))
            .or_else(|| {
                if name == "Simple" {
                    domains
                        .iter()
                        .find(|domain| domain.name.eq_ignore_ascii_case("Clear"))
                } else {
                    None
                }
            });
        svg.push_str(&format!(
            "<g class=\"dm-cynefin-domain\"><rect x=\"{x:.1}\" y=\"{y:.1}\" width=\"300\" height=\"96\" rx=\"8\" /><text class=\"dm-cynefin-domain-name\" x=\"{:.1}\" y=\"{:.1}\">{}</text>",
            x + 16.0,
            y + 26.0,
            name
        ));
        if let Some(domain) = domain {
            for (index, item) in domain.items.iter().take(2).enumerate() {
                svg.push_str(&format!(
                    "<text class=\"dm-cynefin-item\" x=\"{:.1}\" y=\"{:.1}\">{}</text>",
                    x + 18.0,
                    y + 54.0 + index as f32 * 20.0,
                    escape_html(item)
                ));
            }
        }
        svg.push_str("</g>");
    }
    svg.push_str("<circle class=\"dm-cynefin-center\" cx=\"380\" cy=\"180\" r=\"34\" /><text class=\"dm-cynefin-center-label\" x=\"380\" y=\"185\" text-anchor=\"middle\">Aporia</text>");
    svg.push_str("</svg></div>");
    svg
}

struct TreeViewNode {
    label: String,
    depth: usize,
    is_dir: bool,
}

fn render_mermaid_treeview(source: &str) -> String {
    let mut parsed = Vec::new();

    for line in source.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with("%%") || trimmed == "treeview" {
            continue;
        }
        parsed.push((
            line.chars().take_while(|ch| ch.is_whitespace()).count() / 2,
            clean_mermaid_label(trimmed.trim_end_matches('/')),
            trimmed.ends_with('/'),
        ));
    }

    if parsed.is_empty() {
        return render_mermaid_summary("TreeView diagram", "treeview", source);
    }

    let min_depth = parsed.iter().map(|(depth, _, _)| *depth).min().unwrap_or(0);
    let nodes = parsed
        .iter()
        .enumerate()
        .map(|(index, (raw_depth, label, explicit_dir))| TreeViewNode {
            label: label.clone(),
            depth: raw_depth.saturating_sub(min_depth),
            is_dir: *explicit_dir
                || parsed
                    .get(index + 1)
                    .is_some_and(|(next_depth, _, _)| next_depth > raw_depth),
        })
        .collect::<Vec<_>>();

    let width: f32 = 760.0;
    let row_height: f32 = 34.0;
    let height = 74.0 + nodes.len() as f32 * row_height;
    let mut svg = format!(
        "<div class=\"dm-mermaid-chart dm-mermaid-treeview\" role=\"img\" aria-label=\"Rendered Mermaid treeview diagram\"><div class=\"dm-mermaid-chart-title\">Mermaid TreeView diagram</div><svg viewBox=\"0 0 {width:.0} {height:.0}\" aria-hidden=\"true\">"
    );
    for (index, node) in nodes.iter().enumerate() {
        let y = 54.0 + index as f32 * row_height;
        let x = 80.0 + node.depth as f32 * 34.0;
        if node.depth > 0 {
            svg.push_str(&format!(
                "<path class=\"dm-treeview-connector\" d=\"M {:.1} {:.1} v17 h24\" />",
                x - 22.0,
                y - 22.0
            ));
        }
        svg.push_str(&format!(
            "<rect class=\"dm-treeview-glyph{}\" x=\"{:.1}\" y=\"{:.1}\" width=\"14\" height=\"14\" rx=\"3\" /><text class=\"dm-treeview-label{}\" x=\"{:.1}\" y=\"{y:.1}\">{}</text>",
            if node.is_dir { " dm-treeview-glyph-dir" } else { "" },
            x,
            y - 12.0,
            if node.is_dir { " dm-treeview-dir" } else { "" },
            x + 24.0,
            escape_html(&node.label)
        ));
    }
    svg.push_str("</svg></div>");
    svg
}

struct MindmapNode {
    label: String,
    parent: Option<usize>,
    depth: usize,
}

fn render_mermaid_mindmap(source: &str) -> String {
    let mut parsed = Vec::new();
    for line in source.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed == "mindmap" || trimmed.starts_with("%%") {
            continue;
        }
        let depth = line.chars().take_while(|ch| ch.is_whitespace()).count() / 2;
        let label = trimmed
            .strip_prefix("root")
            .map(str::trim)
            .unwrap_or(trimmed);
        parsed.push((depth, clean_mindmap_label(label)));
    }

    if parsed.is_empty() {
        return render_mermaid_summary("Mindmap", "mindmap", source);
    }

    let min_depth = parsed.iter().map(|(depth, _)| *depth).min().unwrap_or(0);
    let mut nodes = Vec::new();
    let mut stack: Vec<Option<usize>> = Vec::new();
    for (raw_depth, label) in parsed {
        let depth = raw_depth.saturating_sub(min_depth);
        while stack.len() <= depth {
            stack.push(None);
        }
        let parent = (0..depth)
            .rev()
            .find_map(|index| stack.get(index).copied().flatten());
        let index = nodes.len();
        nodes.push(MindmapNode {
            label,
            parent,
            depth,
        });
        stack[depth] = Some(index);
        for item in stack.iter_mut().skip(depth + 1) {
            *item = None;
        }
    }

    let max_depth = nodes.iter().map(|node| node.depth).max().unwrap_or(0);
    let width: f32 = 760.0;
    let height = 128.0 + max_depth as f32 * 96.0;
    let positions = layout_mindmap_nodes(&nodes, width);
    let mut svg = format!(
        "<div class=\"dm-mermaid-chart dm-mermaid-mindmap\" role=\"img\" aria-label=\"Rendered Mermaid mindmap\"><div class=\"dm-mermaid-chart-title\">Mermaid mindmap</div><svg viewBox=\"0 0 {width:.0} {height:.0}\" aria-hidden=\"true\">"
    );

    for (index, node) in nodes.iter().enumerate().skip(1) {
        let Some(parent) = node.parent else {
            continue;
        };
        let (from_x, from_y, from_w, from_h) = positions[parent];
        let (to_x, to_y, _, to_h) = positions[index];
        svg.push_str(&format!(
            "<path class=\"dm-mindmap-edge\" d=\"M {from_x:.1} {:.1} C {from_x:.1} {:.1}, {to_x:.1} {:.1}, {to_x:.1} {:.1}\" />",
            from_y + from_h / 2.0,
            from_y + from_h / 2.0 + 34.0,
            to_y - to_h / 2.0 - 34.0,
            to_y - to_h / 2.0
        ));
    }

    for (index, node) in nodes.iter().enumerate() {
        let (x, y, node_width, node_height) = positions[index];
        let left = x - node_width / 2.0;
        let top = y - node_height / 2.0;
        let class_name = if index == 0 {
            "dm-mindmap-node dm-mindmap-root"
        } else {
            "dm-mindmap-node"
        };
        svg.push_str(&format!(
            "<g class=\"{class_name}\"><rect x=\"{left:.1}\" y=\"{top:.1}\" width=\"{node_width:.1}\" height=\"{node_height:.1}\" rx=\"{:.1}\" /><text x=\"{x:.1}\" y=\"{:.1}\" text-anchor=\"middle\">{}</text></g>",
            node_height / 2.0,
            y + 5.0,
            escape_html(&node.label)
        ));
    }

    svg.push_str("</svg></div>");
    svg
}

fn layout_mindmap_nodes(nodes: &[MindmapNode], width: f32) -> Vec<(f32, f32, f32, f32)> {
    let mut positions = vec![(width / 2.0, 58.0, 180.0, 54.0); nodes.len()];
    let max_depth = nodes.iter().map(|node| node.depth).max().unwrap_or(0);

    for depth in 0..=max_depth {
        let depth_nodes = nodes
            .iter()
            .enumerate()
            .filter(|(_, node)| node.depth == depth)
            .map(|(index, _)| index)
            .collect::<Vec<_>>();

        if depth == 0 {
            let label_width = (nodes[depth_nodes[0]].label.chars().count() as f32 * 9.5 + 58.0)
                .clamp(156.0, 230.0);
            positions[depth_nodes[0]] = (width / 2.0, 58.0, label_width, 54.0);
            continue;
        }

        for index in depth_nodes {
            let siblings = nodes[index]
                .parent
                .map(|parent| {
                    nodes
                        .iter()
                        .enumerate()
                        .filter(|(_, node)| node.parent == Some(parent))
                        .map(|(sibling_index, _)| sibling_index)
                        .collect::<Vec<_>>()
                })
                .unwrap_or_default();
            let sibling_index = siblings
                .iter()
                .position(|sibling| *sibling == index)
                .unwrap_or(0);
            let parent_x = nodes[index]
                .parent
                .map(|parent| positions[parent].0)
                .unwrap_or(width / 2.0);
            let spread = ((siblings.len().saturating_sub(1)) as f32 * 122.0).min(560.0);
            let x = if siblings.len() <= 1 {
                parent_x
            } else {
                parent_x - spread / 2.0
                    + sibling_index as f32 * (spread / (siblings.len() - 1) as f32)
            }
            .clamp(72.0, width - 72.0);
            let y = 58.0 + depth as f32 * 96.0;
            let node_width =
                (nodes[index].label.chars().count() as f32 * 9.0 + 44.0).clamp(112.0, 190.0);
            positions[index] = (x, y, node_width, 46.0);
        }
    }

    positions
}

fn clean_mindmap_label(label: &str) -> String {
    clean_mermaid_label(label)
}

fn clean_mermaid_label(label: &str) -> String {
    label
        .trim()
        .trim_matches(|ch| matches!(ch, '(' | ')' | '[' | ']' | '{' | '}'))
        .trim_matches('"')
        .trim()
        .to_owned()
}

fn render_mermaid_quadrant(source: &str) -> String {
    let mut points = Vec::new();
    for line in source.lines().map(str::trim) {
        if let Some((label, coords)) = line.split_once(':') {
            if let Some(coords) = coords
                .trim()
                .strip_prefix('[')
                .and_then(|value| value.strip_suffix(']'))
            {
                let mut values = coords
                    .split(',')
                    .filter_map(|value| value.trim().parse::<f32>().ok());
                if let (Some(x), Some(y)) = (values.next(), values.next()) {
                    points.push((
                        label.trim_matches('"').trim().to_owned(),
                        x.clamp(0.0, 1.0),
                        y.clamp(0.0, 1.0),
                    ));
                }
            }
        }
    }

    let mut html = String::from(
        "<div class=\"dm-mermaid-chart dm-mermaid-quadrant\" role=\"img\" aria-label=\"Rendered Mermaid quadrant chart\"><div class=\"dm-mermaid-chart-title\">Mermaid quadrant chart</div><div class=\"dm-mermaid-quadrant-plane\"><span>High impact</span><span>Low effort</span>",
    );
    for (label, x, y) in points {
        html.push_str(&format!(
            "<b style=\"left: {:.1}%; bottom: {:.1}%\">{}</b>",
            x * 100.0,
            y * 100.0,
            escape_html(&label)
        ));
    }
    html.push_str("</div></div>");
    html
}

struct RequirementNode {
    kind: String,
    name: String,
    fields: Vec<(String, String)>,
}

struct RequirementRelation {
    from: String,
    to: String,
    label: String,
}

fn render_mermaid_requirement(source: &str) -> String {
    let mut nodes = Vec::new();
    let mut relations = Vec::new();
    let mut current: Option<RequirementNode> = None;

    for line in source.lines().map(str::trim) {
        if line.is_empty() || line.starts_with("%%") || line == "requirementDiagram" {
            continue;
        }

        if line == "}" {
            if let Some(node) = current.take() {
                nodes.push(node);
            }
            continue;
        }

        if let Some(node) = current.as_mut() {
            if let Some((key, value)) = line.split_once(':') {
                node.fields
                    .push((clean_mermaid_label(key), clean_mermaid_label(value)));
            }
            continue;
        }

        if let Some((head, _)) = line.split_once('{') {
            let mut parts = head.split_whitespace();
            let Some(kind) = parts.next() else {
                continue;
            };
            let Some(name) = parts.next() else {
                continue;
            };
            current = Some(RequirementNode {
                kind: clean_mermaid_label(kind),
                name: clean_mermaid_label(name),
                fields: Vec::new(),
            });
            continue;
        }

        if let Some(relation) = parse_requirement_relation(line) {
            relations.push(relation);
        }
    }

    if let Some(node) = current {
        nodes.push(node);
    }

    if nodes.is_empty() {
        return render_mermaid_summary("Requirement diagram", "requirement", source);
    }

    let node_width: f32 = 244.0;
    let node_height: f32 = 142.0;
    let columns = nodes.len().clamp(1, 3);
    let rows = nodes.len().div_ceil(columns);
    let width: f32 = 760.0;
    let height = 58.0 + rows as f32 * 178.0;
    let marker_id = mermaid_marker_id(source);
    let mut positions = Vec::new();

    for (index, node) in nodes.iter().enumerate() {
        let col = index % columns;
        let row = index / columns;
        let x = width * (col + 1) as f32 / (columns + 1) as f32;
        let y = 92.0 + row as f32 * 178.0;
        positions.push((node.name.as_str(), x, y));
    }

    let mut svg = format!(
        "<div class=\"dm-mermaid-chart dm-mermaid-requirement\" role=\"img\" aria-label=\"Rendered Mermaid requirement diagram\"><div class=\"dm-mermaid-chart-title\">Mermaid Requirement diagram</div><svg viewBox=\"0 0 {width:.0} {height:.0}\" aria-hidden=\"true\"><defs><marker id=\"{marker_id}\" markerWidth=\"10\" markerHeight=\"10\" refX=\"8\" refY=\"3\" orient=\"auto\"><path d=\"M0,0 L0,6 L9,3 z\" class=\"dm-mermaid-arrow\" /></marker></defs>"
    );

    for relation in &relations {
        let Some((_, from_x, from_y)) = positions.iter().find(|(id, _, _)| *id == relation.from)
        else {
            continue;
        };
        let Some((_, to_x, to_y)) = positions.iter().find(|(id, _, _)| *id == relation.to) else {
            continue;
        };
        let (start_x, start_y) =
            rect_edge_point(*from_x, *from_y, *to_x, *to_y, node_width, node_height);
        let (end_x, end_y) =
            rect_edge_point(*to_x, *to_y, *from_x, *from_y, node_width, node_height);
        svg.push_str(&format!(
            "<path class=\"dm-requirement-relation\" d=\"M {start_x:.1} {start_y:.1} C {start_x:.1} {:.1}, {end_x:.1} {:.1}, {end_x:.1} {end_y:.1}\" marker-end=\"url(#{marker_id})\" /><text class=\"dm-requirement-relation-label\" x=\"{:.1}\" y=\"{:.1}\" text-anchor=\"middle\">{}</text>",
            (start_y + end_y) / 2.0,
            (start_y + end_y) / 2.0,
            (start_x + end_x) / 2.0,
            (start_y + end_y) / 2.0 - 8.0,
            escape_html(&relation.label)
        ));
    }

    for (index, node) in nodes.iter().enumerate() {
        let (_, x, y) = positions[index];
        let left = x - node_width / 2.0;
        let top = y - node_height / 2.0;
        svg.push_str(&format!(
            "<g class=\"dm-requirement-node\"><rect x=\"{left:.1}\" y=\"{top:.1}\" width=\"{node_width:.1}\" height=\"{node_height:.1}\" rx=\"4\" /><text class=\"dm-requirement-kind\" x=\"{x:.1}\" y=\"{:.1}\" text-anchor=\"middle\">&lt;&lt;{}&gt;&gt;</text><text class=\"dm-requirement-name\" x=\"{x:.1}\" y=\"{:.1}\" text-anchor=\"middle\">{}</text><line x1=\"{left:.1}\" y1=\"{:.1}\" x2=\"{:.1}\" y2=\"{:.1}\" />",
            top + 24.0,
            escape_html(&node.kind),
            top + 50.0,
            escape_html(&node.name),
            top + 66.0,
            left + node_width,
            top + 66.0
        ));
        for (field_index, (key, value)) in node.fields.iter().take(4).enumerate() {
            svg.push_str(&format!(
                "<text class=\"dm-requirement-field\" x=\"{:.1}\" y=\"{:.1}\"><tspan class=\"dm-requirement-field-key\">{}:</tspan> {}</text>",
                left + 16.0,
                top + 90.0 + field_index as f32 * 18.0,
                escape_html(key),
                escape_html(value)
            ));
        }
        svg.push_str("</g>");
    }

    svg.push_str("</svg></div>");
    svg
}

fn parse_requirement_relation(line: &str) -> Option<RequirementRelation> {
    let (from, rest) = line.split_once("- ")?;
    let (label, to) = rest.split_once(" ->")?;
    Some(RequirementRelation {
        from: clean_mermaid_label(from),
        to: clean_mermaid_label(to),
        label: clean_mermaid_label(label),
    })
}

struct GitGraphCommit {
    branch: String,
    label: String,
    index: usize,
    kind: String,
}

struct GitGraphBranchPoint {
    from_branch: String,
    to_branch: String,
    index: usize,
}

fn render_mermaid_git_graph(source: &str) -> String {
    let mut branches = vec!["main".to_owned()];
    let mut current_branch = "main".to_owned();
    let mut commits = Vec::new();
    let mut branch_points = Vec::new();
    let mut commit_index: usize = 0;

    for line in source.lines().map(str::trim) {
        if line.is_empty()
            || line.starts_with("%%")
            || line.starts_with("gitGraph")
            || line.starts_with("options")
        {
            continue;
        }

        if let Some(branch) = line.strip_prefix("branch ") {
            let branch = clean_mermaid_label(branch);
            if !branches.iter().any(|existing| existing == &branch) {
                branches.push(branch.clone());
            }
            branch_points.push(GitGraphBranchPoint {
                from_branch: current_branch.clone(),
                to_branch: branch,
                index: commit_index.saturating_sub(1),
            });
            continue;
        }

        if let Some(branch) = line.strip_prefix("checkout ") {
            current_branch = clean_mermaid_label(branch);
            if !branches.iter().any(|existing| existing == &current_branch) {
                branches.push(current_branch.clone());
            }
            continue;
        }

        if let Some(branch) = line.strip_prefix("merge ") {
            let branch = clean_mermaid_label(branch);
            commits.push(GitGraphCommit {
                branch: current_branch.clone(),
                label: format!("merge {branch}"),
                index: commit_index,
                kind: "merge".to_owned(),
            });
            commit_index += 1;
            continue;
        }

        if line.starts_with("commit") {
            let label = parse_git_commit_label(line).unwrap_or_else(|| format!("c{commit_index}"));
            let kind = parse_git_commit_kind(line);
            commits.push(GitGraphCommit {
                branch: current_branch.clone(),
                label,
                index: commit_index,
                kind,
            });
            commit_index += 1;
        }
    }

    if commits.is_empty() {
        return render_mermaid_summary("Git graph", "git", source);
    }

    let width: f32 = 760.0;
    let left: f32 = 116.0;
    let step_x: f32 = 150.0;
    let lane_gap: f32 = 82.0;
    let top: f32 = 78.0;
    let height = top + branches.len() as f32 * lane_gap + 44.0;
    let marker_id = mermaid_marker_id(source);
    let mut svg = format!(
        "<div class=\"dm-mermaid-chart dm-mermaid-git\" role=\"img\" aria-label=\"Rendered Mermaid git graph\"><div class=\"dm-mermaid-chart-title\">Mermaid Git graph</div><svg viewBox=\"0 0 {width:.0} {height:.0}\" aria-hidden=\"true\"><defs><marker id=\"{marker_id}\" markerWidth=\"10\" markerHeight=\"10\" refX=\"8\" refY=\"3\" orient=\"auto\"><path d=\"M0,0 L0,6 L9,3 z\" class=\"dm-mermaid-arrow\" /></marker></defs>"
    );

    let max_index = commits.iter().map(|commit| commit.index).max().unwrap_or(0);
    let end_x = (left + max_index as f32 * step_x + 72.0).min(width - 34.0);

    for (branch_index, branch) in branches.iter().enumerate() {
        let y = top + branch_index as f32 * lane_gap;
        svg.push_str(&format!(
            "<text class=\"dm-git-branch-label\" x=\"24\" y=\"{:.1}\">{}</text><line class=\"dm-git-lane dm-git-lane-{}\" x1=\"{left:.1}\" y1=\"{y:.1}\" x2=\"{end_x:.1}\" y2=\"{y:.1}\" />",
            y + 5.0,
            escape_html(branch),
            branch_index % 4
        ));
    }

    for branch_point in &branch_points {
        let Some(from_index) = branches
            .iter()
            .position(|branch| branch == &branch_point.from_branch)
        else {
            continue;
        };
        let Some(to_index) = branches
            .iter()
            .position(|branch| branch == &branch_point.to_branch)
        else {
            continue;
        };
        let x = left + branch_point.index as f32 * step_x;
        let from_y = top + from_index as f32 * lane_gap;
        let to_y = top + to_index as f32 * lane_gap;
        svg.push_str(&format!(
            "<path class=\"dm-git-branch-edge\" d=\"M {x:.1} {from_y:.1} C {:.1} {from_y:.1}, {:.1} {to_y:.1}, {:.1} {to_y:.1}\" />",
            x + 42.0,
            x + 72.0,
            x + 104.0
        ));
    }

    for commit in &commits {
        let Some(branch_index) = branches.iter().position(|branch| branch == &commit.branch) else {
            continue;
        };
        let x = left + commit.index as f32 * step_x;
        let y = top + branch_index as f32 * lane_gap;
        svg.push_str(&format!(
            "<g class=\"dm-git-commit dm-git-commit-{}\"><circle cx=\"{x:.1}\" cy=\"{y:.1}\" r=\"10\" /><text x=\"{x:.1}\" y=\"{:.1}\" text-anchor=\"middle\">{}</text></g>",
            escape_attribute(&commit.kind),
            y - 20.0,
            escape_html(&commit.label)
        ));
    }

    svg.push_str("</svg></div>");
    svg
}

fn parse_git_commit_label(line: &str) -> Option<String> {
    let id_index = line.find("id:")?;
    let value = line[id_index + 3..].trim();
    if let Some(value) = value.strip_prefix('"') {
        let end = value.find('"')?;
        Some(clean_mermaid_label(&value[..end]))
    } else {
        value.split_whitespace().next().map(clean_mermaid_label)
    }
}

fn parse_git_commit_kind(line: &str) -> String {
    if line.contains("type: HIGHLIGHT") {
        "highlight".to_owned()
    } else if line.contains("type: REVERSE") {
        "reverse".to_owned()
    } else {
        "normal".to_owned()
    }
}

struct C4Node {
    id: String,
    kind: String,
    label: String,
    description: String,
}

struct C4Relation {
    from: String,
    to: String,
    label: String,
}

fn render_mermaid_c4(source: &str) -> String {
    let mut title = "C4 diagram".to_owned();
    let mut nodes = Vec::new();
    let mut relations = Vec::new();

    for line in source.lines().map(str::trim) {
        if line.is_empty() || line.starts_with("%%") || line.starts_with("C4") {
            continue;
        }

        if let Some(value) = line.strip_prefix("title ") {
            title = clean_mermaid_label(value);
            continue;
        }

        if let Some(node) = parse_c4_node(line) {
            nodes.push(node);
            continue;
        }

        if let Some(relation) = parse_c4_relation(line) {
            relations.push(relation);
        }
    }

    if nodes.is_empty() {
        return render_mermaid_summary("C4 diagram", "c4", source);
    }

    let width: f32 = 760.0;
    let node_width: f32 = 210.0;
    let node_height: f32 = 92.0;
    let columns = nodes.len().clamp(1, 3);
    let rows = nodes.len().div_ceil(columns);
    let height = 92.0 + rows as f32 * 150.0;
    let marker_id = mermaid_marker_id(source);
    let mut positions = Vec::new();

    for (index, node) in nodes.iter().enumerate() {
        let col = index % columns;
        let row = index / columns;
        let x = width * (col + 1) as f32 / (columns + 1) as f32;
        let y = 112.0 + row as f32 * 150.0;
        positions.push((node.id.as_str(), x, y));
    }

    let mut svg = format!(
        "<div class=\"dm-mermaid-chart dm-mermaid-c4\" role=\"img\" aria-label=\"Rendered Mermaid C4 diagram\"><div class=\"dm-mermaid-chart-title\">Mermaid C4 diagram</div><svg viewBox=\"0 0 {width:.0} {height:.0}\" aria-hidden=\"true\"><defs><marker id=\"{marker_id}\" markerWidth=\"10\" markerHeight=\"10\" refX=\"8\" refY=\"3\" orient=\"auto\"><path d=\"M0,0 L0,6 L9,3 z\" class=\"dm-mermaid-arrow\" /></marker></defs><text class=\"dm-c4-title\" x=\"380\" y=\"30\" text-anchor=\"middle\">{}</text>",
        escape_html(&title)
    );

    for relation in &relations {
        let Some((_, from_x, from_y)) = positions.iter().find(|(id, _, _)| *id == relation.from)
        else {
            continue;
        };
        let Some((_, to_x, to_y)) = positions.iter().find(|(id, _, _)| *id == relation.to) else {
            continue;
        };
        let (start_x, start_y) =
            rect_edge_point(*from_x, *from_y, *to_x, *to_y, node_width, node_height);
        let (end_x, end_y) =
            rect_edge_point(*to_x, *to_y, *from_x, *from_y, node_width, node_height);
        svg.push_str(&format!(
            "<path class=\"dm-c4-relation\" d=\"M {start_x:.1} {start_y:.1} C {:.1} {start_y:.1}, {:.1} {end_y:.1}, {end_x:.1} {end_y:.1}\" marker-end=\"url(#{marker_id})\" /><text class=\"dm-c4-relation-label\" x=\"{:.1}\" y=\"{:.1}\" text-anchor=\"middle\">{}</text>",
            (start_x + end_x) / 2.0,
            (start_x + end_x) / 2.0,
            (start_x + end_x) / 2.0,
            (start_y + end_y) / 2.0 - 8.0,
            escape_html(&relation.label)
        ));
    }

    for (index, node) in nodes.iter().enumerate() {
        let (_, x, y) = positions[index];
        let left = x - node_width / 2.0;
        let top = y - node_height / 2.0;
        svg.push_str(&format!(
            "<g class=\"dm-c4-node dm-c4-node-{}\"><rect x=\"{left:.1}\" y=\"{top:.1}\" width=\"{node_width:.1}\" height=\"{node_height:.1}\" rx=\"6\" /><text class=\"dm-c4-label\" x=\"{x:.1}\" y=\"{:.1}\" text-anchor=\"middle\">{}</text><text class=\"dm-c4-kind\" x=\"{x:.1}\" y=\"{:.1}\" text-anchor=\"middle\">[{}]</text><text class=\"dm-c4-description\" x=\"{x:.1}\" y=\"{:.1}\" text-anchor=\"middle\">{}</text></g>",
            escape_attribute(&node.kind.to_ascii_lowercase()),
            top + 30.0,
            escape_html(&node.label),
            top + 52.0,
            escape_html(&node.kind),
            top + 74.0,
            escape_html(&node.description)
        ));
    }

    svg.push_str("</svg></div>");
    svg
}

fn parse_c4_node(line: &str) -> Option<C4Node> {
    let open = line.find('(')?;
    let close = line.rfind(')')?;
    let kind = clean_mermaid_label(&line[..open]);
    if !matches!(
        kind.as_str(),
        "Person" | "System" | "Container" | "Component" | "System_Ext" | "Person_Ext"
    ) {
        return None;
    }
    let args = split_mermaid_args(&line[open + 1..close]);
    let id = clean_mermaid_label(args.first()?);
    let label = args
        .get(1)
        .map_or_else(|| id.clone(), |value| clean_mermaid_label(value));
    let description = args
        .get(2)
        .map_or_else(String::new, |value| clean_mermaid_label(value));
    Some(C4Node {
        id,
        kind,
        label,
        description,
    })
}

fn parse_c4_relation(line: &str) -> Option<C4Relation> {
    let open = line.find('(')?;
    let close = line.rfind(')')?;
    let kind = clean_mermaid_label(&line[..open]);
    if !kind.starts_with("Rel") {
        return None;
    }
    let args = split_mermaid_args(&line[open + 1..close]);
    Some(C4Relation {
        from: clean_mermaid_label(args.first()?),
        to: clean_mermaid_label(args.get(1)?),
        label: args
            .get(2)
            .map_or_else(String::new, |value| clean_mermaid_label(value)),
    })
}

fn split_mermaid_args(value: &str) -> Vec<String> {
    let mut args = Vec::new();
    let mut current = String::new();
    let mut in_quotes = false;

    for ch in value.chars() {
        match ch {
            '"' => {
                in_quotes = !in_quotes;
                current.push(ch);
            }
            ',' if !in_quotes => {
                args.push(current.trim().to_owned());
                current.clear();
            }
            _ => current.push(ch),
        }
    }

    if !current.trim().is_empty() {
        args.push(current.trim().to_owned());
    }

    args
}

fn render_mermaid_summary(title: &str, class_name: &str, source: &str) -> String {
    let mut items = Vec::new();
    let mut skipped_declaration = false;
    for line in source.lines().map(str::trim) {
        if line.is_empty() || line.starts_with("title ") || line == "{" || line == "}" {
            continue;
        }
        if !skipped_declaration && detect_mermaid_kind(line) != MermaidKind::Unknown {
            skipped_declaration = true;
            continue;
        }
        items.push(line);
        if items.len() == 8 {
            break;
        }
    }

    let mut html = format!(
        "<div class=\"dm-mermaid-chart dm-mermaid-summary dm-mermaid-summary-{class_name}\" role=\"img\" aria-label=\"Rendered Mermaid {class_name} diagram\"><div class=\"dm-mermaid-chart-title\">Mermaid {}</div><div class=\"dm-mermaid-summary-grid\">",
        escape_html(title)
    );
    for item in items {
        html.push_str(&format!("<span>{}</span>", escape_html(item)));
    }
    html.push_str("</div></div>");
    html
}

fn rect_edge_point(
    from_x: f32,
    from_y: f32,
    to_x: f32,
    to_y: f32,
    width: f32,
    height: f32,
) -> (f32, f32) {
    let dx = to_x - from_x;
    let dy = to_y - from_y;
    if dx == 0.0 && dy == 0.0 {
        return (from_x, from_y);
    }

    let scale_x = if dx == 0.0 {
        f32::INFINITY
    } else {
        (width / 2.0) / dx.abs()
    };
    let scale_y = if dy == 0.0 {
        f32::INFINITY
    } else {
        (height / 2.0) / dy.abs()
    };
    let scale = scale_x.min(scale_y);
    (from_x + dx * scale, from_y + dy * scale)
}

fn mermaid_marker_id(source: &str) -> String {
    let mut hash = 0xcbf29ce484222325_u64;
    for byte in source.bytes() {
        hash ^= byte as u64;
        hash = hash.wrapping_mul(0x100000001b3);
    }
    format!("dm-mermaid-arrow-{hash:x}")
}

fn upsert_text(items: &mut Vec<String>, item: String) {
    if !items.iter().any(|existing| existing == &item) {
        items.push(item);
    }
}

fn parse_mermaid_node(value: &str) -> (String, String) {
    let value = value.trim().trim_end_matches(';').trim();
    if let Some(start) = value.find('[') {
        if let Some(end) = value.rfind(']') {
            let id = value[..start].trim().to_owned();
            let label = value[start + 1..end].trim_matches('"').trim().to_owned();
            return (id, label);
        }
    }

    let id = value
        .split_whitespace()
        .next()
        .unwrap_or(value)
        .trim()
        .to_owned();
    (id.clone(), id)
}

fn upsert_mermaid_node(nodes: &mut Vec<(String, String)>, node: &(String, String)) {
    if let Some(existing) = nodes.iter_mut().find(|(id, _)| id == &node.0) {
        if existing.1 == existing.0 && node.1 != node.0 {
            existing.1.clone_from(&node.1);
        }
        return;
    }

    nodes.push(node.clone());
}

fn render_plain_code(language: &str, source: &str) -> String {
    format!(
        "<pre><code class=\"language-{}\">{}</code></pre>",
        escape_attribute(language),
        escape_html(source)
    )
}

fn highlight_source(language: &str, source: &str) -> String {
    source
        .split_inclusive('\n')
        .map(|line| {
            let (line, newline) = line
                .strip_suffix('\n')
                .map_or((line, ""), |line| (line, "\n"));
            format!("{}{}", highlight_line(language, line), newline)
        })
        .collect()
}

fn highlight_line(language: &str, line: &str) -> String {
    let comment = if matches!(language, "elixir" | "yaml") {
        "#"
    } else {
        "//"
    };
    let mut highlighted = String::new();
    let mut index = 0;

    while index < line.len() {
        let rest = &line[index..];

        if rest.starts_with(comment) {
            push_token(&mut highlighted, "comment", rest);
            break;
        }

        let ch = rest.chars().next().expect("line has a char");
        if matches!(ch, '"' | '\'' | '`') {
            let next = consume_string(rest, ch);
            push_token(&mut highlighted, "string", &rest[..next]);
            index += next;
            continue;
        }

        if ch.is_ascii_digit() {
            let next = consume_while(rest, |ch| {
                ch.is_ascii_alphanumeric() || matches!(ch, '_' | '.')
            });
            push_token(&mut highlighted, "number", &rest[..next]);
            index += next;
            continue;
        }

        if is_identifier_start(ch) {
            let next = consume_while(rest, is_identifier_char);
            let word = &rest[..next];
            let next_significant = rest[next..].trim_start().chars().next();
            if let Some(token) = classify_word(language, word, next_significant) {
                push_token(&mut highlighted, token, word);
            } else {
                highlighted.push_str(&escape_html(word));
            }
            index += next;
            continue;
        }

        highlighted.push_str(&escape_html(&ch.to_string()));
        index += ch.len_utf8();
    }

    highlighted
}

fn consume_string(value: &str, quote: char) -> usize {
    let mut escaped = false;
    for (offset, ch) in value.char_indices().skip(1) {
        if escaped {
            escaped = false;
            continue;
        }
        if ch == '\\' {
            escaped = true;
            continue;
        }
        if ch == quote {
            return offset + ch.len_utf8();
        }
    }

    value.len()
}

fn consume_while(value: &str, predicate: impl Fn(char) -> bool) -> usize {
    value
        .char_indices()
        .find_map(|(index, ch)| (!predicate(ch)).then_some(index))
        .unwrap_or(value.len())
}

fn is_identifier_start(ch: char) -> bool {
    matches!(ch, '_' | '@') || ch.is_ascii_alphabetic()
}

fn is_identifier_char(ch: char) -> bool {
    matches!(ch, '_' | '?' | '!' | '@') || ch.is_ascii_alphanumeric()
}

fn is_keyword(language: &str, word: &str) -> bool {
    match language {
        "elixir" => matches!(
            word,
            "def"
                | "defmodule"
                | "defp"
                | "do"
                | "end"
                | "case"
                | "cond"
                | "fn"
                | "if"
                | "else"
                | "with"
                | "use"
                | "alias"
                | "import"
                | "when"
                | "receive"
        ),
        "go" => matches!(
            word,
            "func"
                | "package"
                | "import"
                | "return"
                | "if"
                | "else"
                | "for"
                | "range"
                | "type"
                | "struct"
                | "interface"
                | "go"
                | "defer"
                | "var"
                | "const"
                | "map"
        ),
        "rust" => matches!(
            word,
            "fn" | "let"
                | "mut"
                | "pub"
                | "impl"
                | "struct"
                | "enum"
                | "match"
                | "if"
                | "else"
                | "for"
                | "while"
                | "loop"
                | "use"
                | "mod"
                | "crate"
                | "return"
                | "async"
                | "await"
                | "trait"
        ),
        "typescript" => matches!(
            word,
            "const"
                | "let"
                | "function"
                | "return"
                | "type"
                | "interface"
                | "class"
                | "extends"
                | "implements"
                | "async"
                | "await"
                | "import"
                | "export"
                | "from"
                | "if"
                | "else"
                | "new"
                | "readonly"
        ),
        "zig" => matches!(
            word,
            "const"
                | "var"
                | "fn"
                | "pub"
                | "return"
                | "if"
                | "else"
                | "while"
                | "for"
                | "switch"
                | "defer"
                | "comptime"
                | "try"
                | "catch"
                | "struct"
                | "enum"
                | "error"
        ),
        _ => false,
    }
}

fn classify_word(
    language: &str,
    word: &str,
    next_significant: Option<char>,
) -> Option<&'static str> {
    if is_keyword(language, word) {
        return Some("keyword");
    }

    if is_literal(word) {
        return Some("literal");
    }

    if is_builtin(language, word) {
        return Some("builtin");
    }

    if is_type(language, word) {
        return Some("type");
    }

    if next_significant == Some('(') {
        return Some("function");
    }

    None
}

fn is_literal(word: &str) -> bool {
    matches!(
        word,
        "true" | "false" | "nil" | "null" | "undefined" | "none" | "None"
    )
}

fn is_builtin(language: &str, word: &str) -> bool {
    match language {
        "elixir" => matches!(
            word,
            "IO" | "Kernel" | "Enum" | "Map" | "String" | "Integer" | "List" | "Agent"
        ),
        "go" => matches!(
            word,
            "append"
                | "cap"
                | "close"
                | "copy"
                | "delete"
                | "len"
                | "make"
                | "new"
                | "panic"
                | "print"
                | "println"
        ),
        "rust" => matches!(
            word,
            "format!" | "println!" | "vec!" | "Some" | "Ok" | "Err" | "Box" | "String"
        ),
        "typescript" => matches!(
            word,
            "Array" | "Map" | "Set" | "Promise" | "Record" | "console" | "JSON"
        ),
        "zig" => matches!(
            word,
            "@import" | "@This" | "@TypeOf" | "@compileError" | "std"
        ),
        _ => false,
    }
}

fn is_type(language: &str, word: &str) -> bool {
    if word
        .chars()
        .next()
        .is_some_and(|ch| ch.is_ascii_uppercase())
    {
        return true;
    }

    match language {
        "go" => matches!(
            word,
            "any" | "bool" | "byte" | "error" | "int" | "int64" | "rune" | "string" | "uint"
        ),
        "rust" => matches!(
            word,
            "bool" | "char" | "i32" | "i64" | "str" | "u32" | "u64" | "usize"
        ),
        "typescript" => matches!(
            word,
            "boolean" | "number" | "object" | "string" | "symbol" | "unknown" | "void"
        ),
        "zig" => matches!(
            word,
            "anyerror"
                | "anytype"
                | "bool"
                | "comptime_int"
                | "i32"
                | "type"
                | "u8"
                | "usize"
                | "void"
        ),
        _ => false,
    }
}

fn push_token(output: &mut String, token: &str, value: &str) {
    output.push_str("<span class=\"dm-token-");
    output.push_str(token);
    output.push_str("\">");
    output.push_str(&escape_html(value));
    output.push_str("</span>");
}

fn escape_attribute(value: &str) -> String {
    escape_html(value)
}

fn escape_html(value: &str) -> String {
    let mut escaped = String::with_capacity(value.len());
    for ch in value.chars() {
        match ch {
            '&' => escaped.push_str("&amp;"),
            '<' => escaped.push_str("&lt;"),
            '>' => escaped.push_str("&gt;"),
            '"' => escaped.push_str("&quot;"),
            '\'' => escaped.push_str("&#39;"),
            _ => escaped.push(ch),
        }
    }
    escaped
}

fn resolve_url<'a>(url: CowStr<'a>, base_url: Option<&str>) -> CowStr<'a> {
    let url = safe_url(url);
    let Some(base_url) = base_url.filter(|base_url| !base_url.is_empty()) else {
        return url;
    };

    if url.is_empty() || !is_relative_path_url(&url) {
        return url;
    }

    if !is_resolvable_base_url(base_url) {
        return CowStr::Borrowed("");
    }

    safe_url(CowStr::Boxed(
        join_directory_url(base_url, &url).into_boxed_str(),
    ))
}

fn is_relative_path_url(url: &str) -> bool {
    let normalized = normalize_url_for_safety(url);

    !normalized.is_empty()
        && !normalized.starts_with(['/', '#', '?'])
        && url_scheme_end(&normalized).is_none()
}

fn is_resolvable_base_url(url: &str) -> bool {
    let normalized = normalize_url_for_safety(url);

    !normalized.is_empty()
        && !normalized.starts_with(['#', '?'])
        && (normalized.starts_with('/')
            || normalized.starts_with("http://")
            || normalized.starts_with("https://")
            || url_scheme_end(&normalized).is_none())
}

fn join_directory_url(base_url: &str, relative_url: &str) -> String {
    let base_end = base_url.find(['?', '#']).unwrap_or(base_url.len());
    let base_url = &base_url[..base_end];
    let relative_suffix_start = relative_url.find(['?', '#']).unwrap_or(relative_url.len());
    let (relative_path, relative_suffix) = relative_url.split_at(relative_suffix_start);
    let (prefix, base_path) = split_url_prefix(base_url);

    let mut combined_path = String::with_capacity(base_path.len() + relative_path.len() + 1);
    combined_path.push_str(base_path);
    if !combined_path.is_empty() && !combined_path.ends_with('/') {
        combined_path.push('/');
    }
    combined_path.push_str(relative_path);
    if !prefix.is_empty() && !combined_path.starts_with('/') {
        combined_path.insert(0, '/');
    }

    let path = normalize_url_path(&combined_path);
    format!("{prefix}{path}{relative_suffix}")
}

fn split_url_prefix(url: &str) -> (&str, &str) {
    let authority_start = if url
        .get(..7)
        .is_some_and(|scheme| scheme.eq_ignore_ascii_case("http://"))
    {
        7
    } else if url
        .get(..8)
        .is_some_and(|scheme| scheme.eq_ignore_ascii_case("https://"))
    {
        8
    } else if url.starts_with("//") {
        2
    } else {
        return ("", url);
    };

    let path_start = url[authority_start..]
        .find('/')
        .map_or(url.len(), |offset| authority_start + offset);
    url.split_at(path_start)
}

fn normalize_url_path(path: &str) -> String {
    let absolute = path.starts_with('/');
    let directory = path.ends_with('/')
        || path.ends_with("/.")
        || path.ends_with("/..")
        || matches!(path, "." | "..");
    let path = path.strip_prefix('/').unwrap_or(path);
    let mut segments = Vec::new();

    for segment in path.split('/') {
        match segment {
            "." => {}
            ".." if segments.last().is_some_and(|segment| *segment != "..") => {
                segments.pop();
            }
            ".." if !absolute => segments.push(segment),
            ".." => {}
            _ => segments.push(segment),
        }
    }

    let mut normalized = segments.join("/");
    if absolute {
        normalized.insert(0, '/');
    }
    if directory && !normalized.ends_with('/') {
        normalized.push('/');
    }
    normalized
}

fn safe_url(url: CowStr<'_>) -> CowStr<'_> {
    if is_safe_url(&url) {
        url
    } else {
        CowStr::Borrowed("")
    }
}

fn is_safe_url(url: &str) -> bool {
    let normalized = normalize_url_for_safety(url);

    if normalized.is_empty()
        || normalized.starts_with('#')
        || normalized.starts_with('/')
        || normalized.starts_with("./")
        || normalized.starts_with("../")
        || normalized.starts_with("http://")
        || normalized.starts_with("https://")
        || normalized.starts_with("mailto:")
    {
        return true;
    }

    url_scheme_end(&normalized).is_none()
}

fn normalize_url_for_safety(url: &str) -> String {
    url.trim_start()
        .chars()
        .filter(|ch| !ch.is_ascii_whitespace() && !ch.is_ascii_control())
        .collect::<String>()
        .to_ascii_lowercase()
}

fn url_scheme_end(url: &str) -> Option<usize> {
    url.find([':', '/', '?', '#'])
        .filter(|&index| url.as_bytes()[index] == b':')
}

#[cfg(test)]
mod tests {
    use super::{
        parse_css_color, render_color_chip, render_markdown_to_html,
        render_markdown_to_html_with_options, split_front_matter, DmMarkdownOptions,
        FrontMatterMode,
    };

    #[test]
    fn renders_common_markdown() {
        let html = render_markdown_to_html(
            "# Release\n\n- [x] shipped\n\n| A | B |\n| - | - |\n| 1 | 2 |\n",
        );

        assert!(html.contains("<h1>Release</h1>"));
        assert!(html.contains(r#"<input disabled="" type="checkbox" checked=""/>"#));
        assert!(html.contains("<table>"));
    }

    #[test]
    fn parses_supported_hex_colors() {
        for color in [
            "#fff",
            "#AbC",
            "#ffff",
            "#aBcD",
            "#4c86fc",
            "#4C86FC",
            "#ff000080",
            "#FF000080",
        ] {
            assert!(
                parse_css_color(color).is_some(),
                "expected {color} to parse"
            );
        }
    }

    #[test]
    fn parses_supported_function_colors() {
        for color in [
            "rgb(255, 0, 0)",
            "RGB(100%, 0%, 0%)",
            "rgba(255, 0, 0, 0.5)",
            "rgba(100%, 0%, 0%, 50%)",
            "hsl(210, 100%, 50%)",
            "HSL(210deg, 100%, 50%)",
            "hsla(210, 100%, 50%, 0.5)",
            "hsla(210deg, 100%, 50%, 50%)",
        ] {
            assert!(
                parse_css_color(color).is_some(),
                "expected {color} to parse"
            );
        }
    }

    #[test]
    fn rejects_malformed_and_out_of_range_colors() {
        for color in [
            "#ff",
            "#fffff",
            "#fffffff",
            "#fffffffff",
            "#ggg",
            "#12_",
            "rgb(256, 0, 0)",
            "rgb(101%, 0%, 0%)",
            "rgb(255, 0%, 0)",
            "rgba(255, 0, 0, 1.01)",
            "hsl(361, 100%, 50%)",
            "hsl(210, 101%, 50%)",
            "hsla(210, 100%, 50%, 101%)",
            "rgb(255, 0, 0);color:red",
            "var(--brand-color)",
            "red",
            " #fff",
            "#fff ",
        ] {
            assert!(
                parse_css_color(color).is_none(),
                "expected {color} to be rejected"
            );
        }
    }

    #[test]
    fn renders_inline_color_chips_and_preserves_text() {
        let html = render_markdown_to_html("Primary `#4C86FC` and `rgba(255, 0, 0, 0.5)`.");

        assert_eq!(html.matches(r#"class="dm-color-code""#).count(), 2);
        assert!(html.contains("#4C86FC"));
        assert!(html.contains("rgba(255, 0, 0, 0.5)"));
        assert!(html.contains(r#"class="dm-color-chip" role="img" aria-label="Color #4C86FC""#));
        assert!(html.contains("background-color:#4C86FC;"));
    }

    #[test]
    fn embeds_color_chip_layout_styles() {
        let html = render_markdown_to_html("`#0065FF`");

        assert!(html.contains(
            r#"style="display:inline-flex;align-items:center;gap:0.35em;white-space:nowrap;vertical-align:middle;""#
        ));
        assert!(html.contains(
            r#"style="position:relative;display:inline-block;flex:0 0 auto;width:1em;height:1em;overflow:hidden;"#
        ));
        assert!(html.contains(r#"style="position:absolute;inset:0;background-color:#0065FF;""#));
    }

    #[test]
    fn can_disable_inline_color_chips() {
        let html = render_markdown_to_html_with_options(
            "`#fff`",
            DmMarkdownOptions {
                color_chips: false,
                ..DmMarkdownOptions::default()
            },
        );

        assert_eq!(html, "<p><code>#fff</code></p>\n");
        assert!(!html.contains("dm-color-chip"));
    }

    #[test]
    fn does_not_transform_color_substrings_or_fenced_code() {
        let inline = render_markdown_to_html("`color: #fff`");
        assert_eq!(inline, "<p><code>color: #fff</code></p>\n");

        let fenced = render_markdown_to_html("```css\n#fff\n```");
        assert!(fenced.contains("#fff"));
        assert!(!fenced.contains("dm-color-code"));
        assert!(!fenced.contains("dm-color-chip"));
    }

    #[test]
    fn escapes_generated_color_markup_and_ordinary_inline_code() {
        let generated = render_color_chip("#fff\" onmouseover=\"alert(1)");
        assert!(generated.contains("#fff&quot; onmouseover=&quot;alert(1)"));
        assert!(!generated.contains(r#"onmouseover="alert(1)""#));

        let html = render_markdown_to_html("`<img src=x onerror=alert(1)>`");
        assert!(html.contains("&lt;img src=x onerror=alert(1)&gt;"));
        assert!(!html.contains("<img"));
        assert!(!html.contains("dm-color-chip"));
    }

    #[test]
    fn splits_front_matter_with_dash_or_dot_delimiters() {
        let dashes = split_front_matter("---\ntitle: Example\n---\n# Document").unwrap();
        assert_eq!(dashes.source, "title: Example\n");
        assert_eq!(dashes.body, "# Document");

        let dots = split_front_matter("---\ntitle: Example\n...\n# Document").unwrap();
        assert_eq!(dots.source, "title: Example\n");
        assert_eq!(dots.body, "# Document");
    }

    #[test]
    fn splits_front_matter_with_bom_empty_source_and_crlf() {
        let bom = split_front_matter("\u{feff}---\ntitle: Example\n---\n# Document").unwrap();
        assert_eq!(bom.source, "title: Example\n");
        assert_eq!(bom.body, "# Document");

        let empty = split_front_matter("---\n---\n# Document").unwrap();
        assert_eq!(empty.source, "");
        assert_eq!(empty.body, "# Document");

        let crlf = split_front_matter("---\r\ntitle: Example\r\n---\r\n# Document\r\n").unwrap();
        assert_eq!(crlf.source, "title: Example\r\n");
        assert_eq!(crlf.body, "# Document\r\n");
    }

    #[test]
    fn ignores_unclosed_or_non_initial_front_matter() {
        let unclosed = "---\ntitle: Example\n# Document";
        assert!(split_front_matter(unclosed).is_none());
        let html = render_markdown_to_html(unclosed);
        assert!(!html.contains("dm-front-matter"));
        assert!(html.contains("title: Example"));

        let later_rule = "Introduction\n\n---\ntitle: Not front matter\n---\n";
        assert!(split_front_matter(later_rule).is_none());
        assert!(!render_markdown_to_html(later_rule).contains("dm-front-matter"));
    }

    #[test]
    fn renders_hides_or_disables_front_matter() {
        let markdown = "---\ntitle: Example\n---\n# Document";

        let rendered = render_markdown_to_html(markdown);
        assert!(rendered.starts_with(r#"<div class="dm-front-matter">"#));
        assert!(rendered.contains(r#"data-language="yaml""#));
        assert!(rendered.contains(">YAML</span>"));
        assert!(rendered.contains("Example"));
        assert!(rendered.ends_with("<h1>Document</h1>\n"));

        let hidden = render_markdown_to_html_with_options(
            markdown,
            DmMarkdownOptions {
                front_matter: FrontMatterMode::Hidden,
                ..DmMarkdownOptions::default()
            },
        );
        assert_eq!(hidden, "<h1>Document</h1>\n");

        let disabled = render_markdown_to_html_with_options(
            markdown,
            DmMarkdownOptions {
                front_matter: FrontMatterMode::Disabled,
                ..DmMarkdownOptions::default()
            },
        );
        assert!(!disabled.contains("dm-front-matter"));
        assert!(disabled.contains("title: Example"));
        assert!(disabled.contains("<h1>Document</h1>"));
    }

    #[test]
    fn renders_safe_raw_html_and_escapes_unsafe_tags() {
        let html = render_markdown_to_html(
            "<script>alert(1)</script>\n\nHello <span class=\"name\">world</span>\n\n<div><script>alert(2)</script><span>still safe</span></div>\n\n<style>body { color: red; }</style>\n\n<object data=\"demo.swf\"></object>",
        );

        assert!(!html.contains("<script>"));
        assert!(!html.contains("<style>"));
        assert!(!html.contains("<object"));
        assert!(html.contains("&lt;script&gt;alert(1)&lt;/script&gt;"));
        assert!(html.contains("&lt;style&gt;body { color: red; }&lt;/style&gt;"));
        assert!(html.contains("&lt;object"));
        assert!(html.contains("demo.swf"));
        assert!(html.contains("&lt;/object&gt;"));
        assert!(html.contains(r#"Hello <span class="name">world</span>"#));
        assert!(html
            .contains("<div>&lt;script&gt;alert(2)&lt;/script&gt;<span>still safe</span></div>"));
    }

    #[test]
    fn can_escape_all_raw_html() {
        let html = render_markdown_to_html_with_options(
            "Hello <span>world</span>",
            DmMarkdownOptions {
                allow_html: false,
                ..DmMarkdownOptions::default()
            },
        );

        assert!(!html.contains("<span>world</span>"));
        assert!(html.contains("Hello &lt;span&gt;world&lt;/span&gt;"));
    }

    #[test]
    fn escapes_custom_elements_unless_enabled() {
        let html = render_markdown_to_html(
            "<el-dm-alert><strong>safe</strong></el-dm-alert>\n\n<other-widget>blocked</other-widget>",
        );

        assert!(html.contains("&lt;el-dm-alert&gt;"));
        assert!(html.contains("<strong>safe</strong>"));
        assert!(html.contains("&lt;/el-dm-alert&gt;"));
        assert!(html.contains("&lt;other-widget&gt;blocked&lt;/other-widget&gt;"));

        let html = render_markdown_to_html_with_options(
            "<el-dm-alert><strong>safe</strong></el-dm-alert>\n\n<other-widget>blocked</other-widget>",
            DmMarkdownOptions {
                custom_elements: vec!["el-dm-alert".to_owned()],
                ..DmMarkdownOptions::default()
            },
        );

        assert!(html.contains("<el-dm-alert><strong>safe</strong></el-dm-alert>"));
        assert!(html.contains("&lt;other-widget&gt;blocked&lt;/other-widget&gt;"));
    }

    #[test]
    fn strips_unsafe_link_and_image_urls() {
        let html =
            render_markdown_to_html("[bad](javascript:alert(1))\n\n![bad](data:image/svg+xml,abc)");

        assert!(!html.contains("javascript:"));
        assert!(!html.contains("data:image"));
        assert!(html.contains(r#"<a href="">bad</a>"#));
        assert!(html.contains(r#"<img src="" alt="bad" />"#));
    }

    #[test]
    fn keeps_safe_urls() {
        let html = render_markdown_to_html(
            "[external](https://example.com)\n[relative](docs/start.md)\n![asset](/images/logo.png)",
        );

        assert!(html.contains(r#"<a href="https://example.com">external</a>"#));
        assert!(html.contains(r#"<a href="docs/start.md">relative</a>"#));
        assert!(html.contains(r#"<img src="/images/logo.png" alt="asset" />"#));
    }

    #[test]
    fn resolves_relative_link_and_image_urls_against_a_directory_base() {
        let html = render_markdown_to_html_with_options(
            "[metadata](./meta.json)\n\n![preview](images/preview.png)\n\n[cover](../cover.png?download=1#preview)",
            DmMarkdownOptions {
                base_url: Some("/api/notes/42/attachments/".to_owned()),
                ..DmMarkdownOptions::default()
            },
        );

        assert!(html.contains(r#"href="/api/notes/42/attachments/meta.json""#));
        assert!(html.contains(r#"src="/api/notes/42/attachments/images/preview.png""#));
        assert!(html.contains(r#"href="/api/notes/42/cover.png?download=1#preview""#));
    }

    #[test]
    fn treats_a_base_without_a_trailing_slash_as_a_directory() {
        let html = render_markdown_to_html_with_options(
            "[metadata](meta.json)",
            DmMarkdownOptions {
                base_url: Some("https://files.example.test/notes/42".to_owned()),
                ..DmMarkdownOptions::default()
            },
        );

        assert!(html.contains(r#"href="https://files.example.test/notes/42/meta.json""#));
    }

    #[test]
    fn preserves_directory_and_empty_path_segment_semantics() {
        let html = render_markdown_to_html_with_options(
            "[current](.) [parent](..) [child-current](child/.) [child-parent](child/..) [repeated](images//preview.png)",
            DmMarkdownOptions {
                base_url: Some("/api//notes/42/attachments/".to_owned()),
                ..DmMarkdownOptions::default()
            },
        );

        assert_eq!(
            html.matches(r#"href="/api//notes/42/attachments/""#)
                .count(),
            2
        );
        assert!(html.contains(r#"href="/api//notes/42/""#));
        assert!(html.contains(r#"href="/api//notes/42/attachments/child/""#));
        assert!(html.contains(r#"href="/api//notes/42/attachments/images//preview.png""#));
    }

    #[test]
    fn does_not_treat_a_root_relative_path_fragment_as_an_authority() {
        let html = render_markdown_to_html_with_options(
            "[metadata](meta.json)",
            DmMarkdownOptions {
                base_url: Some("/api/://notes/".to_owned()),
                ..DmMarkdownOptions::default()
            },
        );

        assert!(html.contains(r#"href="/api/://notes/meta.json""#));
    }

    #[test]
    fn leaves_non_path_urls_and_plain_text_unchanged_with_a_base() {
        let html = render_markdown_to_html_with_options(
            "[https](https://example.com/a) [http](http://example.com/b) [mail](mailto:team@example.com) [root](/docs) [cdn](//cdn.example.com/a) [fragment](#section) [query](?view=raw)\n\nPlain ./meta.json",
            DmMarkdownOptions {
                base_url: Some("/api/notes/42/attachments/".to_owned()),
                ..DmMarkdownOptions::default()
            },
        );

        for destination in [
            "https://example.com/a",
            "http://example.com/b",
            "mailto:team@example.com",
            "/docs",
            "//cdn.example.com/a",
            "#section",
            "?view=raw",
        ] {
            assert!(html.contains(&format!(r#"href="{destination}""#)));
        }
        assert!(html.contains("Plain ./meta.json"));
    }

    #[test]
    fn leaves_email_autolinks_unchanged_with_a_base() {
        let html = render_markdown_to_html_with_options(
            "Contact <team@example.com>",
            DmMarkdownOptions {
                base_url: Some("/api/notes/42/attachments/".to_owned()),
                ..DmMarkdownOptions::default()
            },
        );

        assert!(html.contains(r#"href="mailto:team@example.com""#));
    }

    #[test]
    fn resolves_relative_urls_in_inline_and_block_raw_html() {
        let markdown = concat!(
            r#"Inline <a title="1 > 0" HREF = './guide.html' data-href="keep.md">Guide</a>."#,
            "\n\n[Markdown](docs/readme.md)\n\n",
            "<div>\n",
            "<img alt='Cover'\n SRC=images/cover.png data-src=preview.png srcset='small.png 1x'>\n",
            r#"<a href="../index.html" src='extras/icon.svg'>Index</a>"#,
            "\n<a href='./meta.json?x=1&amp;y=2'>Metadata</a>",
            "\n</div>",
        );
        let html = render_markdown_to_html_with_options(
            markdown,
            DmMarkdownOptions {
                base_url: Some("/api/notes/42/attachments/".to_owned()),
                ..DmMarkdownOptions::default()
            },
        );

        assert!(html.contains(r#"title="1 > 0""#));
        assert!(html.contains(r#"HREF = "/api/notes/42/attachments/guide.html""#));
        assert!(html.contains(r#"data-href="keep.md""#));
        assert!(html.contains(r#"href="/api/notes/42/attachments/docs/readme.md""#));
        assert!(html.contains(r#"SRC="/api/notes/42/attachments/images/cover.png""#));
        assert!(html.contains("data-src=preview.png"));
        assert!(html.contains(r#"srcset='small.png 1x'"#));
        assert!(html.contains(r#"href="/api/notes/42/index.html""#));
        assert!(html.contains(r#"src="/api/notes/42/attachments/extras/icon.svg""#));
        assert!(html.contains(r#"href="/api/notes/42/attachments/meta.json?x=1&amp;y=2""#));
    }

    #[test]
    fn preserves_special_raw_html_urls_and_rejects_unsafe_ones() {
        let html = render_markdown_to_html_with_options(
            concat!(
                r#"<a href="https://example.com/a">absolute</a>"#,
                r#"<a href="http://example.com/b">http</a>"#,
                r#"<a href='#section'>fragment</a>"#,
                "<a href=?view=raw>query</a>",
                "<a href=mailto:team@example.com>mail</a>",
                r#"<img src="/images/logo.png">"#,
                r#"<a href="//cdn.example.com/a">cdn</a>"#,
                r#"<a href="java&#x73;cript:alert(1)">unsafe link</a>"#,
                r#"<a href="javascript&#58;alert(1)">encoded colon</a>"#,
                "<img src=data&#58;text/html,unsafe>",
            ),
            DmMarkdownOptions {
                base_url: Some("/api/notes/42/attachments/".to_owned()),
                ..DmMarkdownOptions::default()
            },
        );

        for destination in [
            "https://example.com/a",
            "http://example.com/b",
            "#section",
            "?view=raw",
            "mailto:team@example.com",
            "/images/logo.png",
            "//cdn.example.com/a",
        ] {
            assert!(html.contains(destination));
        }
        assert!(html.contains(r#"href="""#));
        assert!(html.contains(r#"src="""#));
        assert!(!html.to_ascii_lowercase().contains("javascript:"));
        assert!(!html.contains("data:text/html"));
    }

    #[test]
    fn rejects_unsafe_unquoted_raw_html_urls_with_stray_quotes() {
        let html = render_markdown_to_html_with_options(
            "<div>\n<a href=javascript:alert(1)//\" data-label=bad>bad</a>\n</div>",
            DmMarkdownOptions {
                base_url: Some("/api/notes/42/attachments/".to_owned()),
                ..DmMarkdownOptions::default()
            },
        );

        assert!(html.contains(r#"href="" data-label=bad"#));
        assert!(!html.contains("javascript:"));
    }

    #[test]
    fn decodes_unterminated_numeric_entities_before_checking_url_safety() {
        let html = render_markdown_to_html_with_options(
            r#"<a href="javascript&#58alert(1)">bad</a>"#,
            DmMarkdownOptions {
                base_url: Some("./".to_owned()),
                ..DmMarkdownOptions::default()
            },
        );

        assert!(html.contains(r#"href="""#));
        assert!(!html.contains("javascript"));
        assert!(!html.contains("&#58"));
    }

    #[test]
    fn escapes_entities_from_a_raw_html_base_url() {
        let html = render_markdown_to_html_with_options(
            r#"<a href="guide.html">Guide</a>"#,
            DmMarkdownOptions {
                base_url: Some("javascript&colon;".to_owned()),
                ..DmMarkdownOptions::default()
            },
        );

        assert!(html.contains(r#"href="javascript&amp;colon;/guide.html""#));
        assert!(!html.contains(r#"href="javascript&colon;"#));
    }

    #[test]
    fn does_not_resolve_raw_html_when_html_is_disabled() {
        let html = render_markdown_to_html_with_options(
            "<a\n href=guide.html>Guide</a>",
            DmMarkdownOptions {
                allow_html: false,
                base_url: Some("/api/notes/42/attachments/".to_owned()),
                ..DmMarkdownOptions::default()
            },
        );

        assert!(!html.contains("<a"));
        assert!(!html.contains("/api/notes/42/attachments/guide.html"));
        assert!(html.contains("guide.html"));
    }

    #[test]
    fn only_resolves_urls_in_allowed_raw_html_tags() {
        let markdown = concat!(
            "<script\n src=worker.js>\nalert(1)\n</script>\n\n",
            "<blocked-widget href=page.html>blocked</blocked-widget>\n\n",
            "<enabled-widget href=page.html>enabled</enabled-widget>",
        );
        let html = render_markdown_to_html_with_options(
            markdown,
            DmMarkdownOptions {
                base_url: Some("/api/notes/42/attachments/".to_owned()),
                custom_elements: vec!["enabled-widget".to_owned()],
                ..DmMarkdownOptions::default()
            },
        );

        assert!(html.contains("&lt;script"));
        assert!(html.contains("&lt;blocked-widget"));
        assert!(!html.contains("/api/notes/42/attachments/worker.js"));
        assert!(html.contains(
            r#"<enabled-widget href="/api/notes/42/attachments/page.html">enabled</enabled-widget>"#
        ));
    }

    #[test]
    fn does_not_resolve_markup_inside_raw_text_elements() {
        let html = render_markdown_to_html_with_options(
            "<textarea>\n<a href=./inside.html>literal</a>\n</textarea>",
            DmMarkdownOptions {
                base_url: Some("/api/notes/42/attachments/".to_owned()),
                ..DmMarkdownOptions::default()
            },
        );

        assert!(html.contains("href=./inside.html"));
        assert!(!html.contains("/api/notes/42/attachments/inside.html"));
    }

    #[test]
    fn base_url_does_not_bypass_url_safety() {
        let unsafe_destination = render_markdown_to_html_with_options(
            "[bad](javascript:alert(1))",
            DmMarkdownOptions {
                base_url: Some("/api/notes/42/attachments/".to_owned()),
                ..DmMarkdownOptions::default()
            },
        );
        assert!(unsafe_destination.contains(r#"href="""#));
        assert!(!unsafe_destination.contains("javascript:"));

        let unsafe_base = render_markdown_to_html_with_options(
            "[metadata](meta.json)",
            DmMarkdownOptions {
                base_url: Some("javascript:alert(1)".to_owned()),
                ..DmMarkdownOptions::default()
            },
        );
        assert!(unsafe_base.contains(r#"href="""#));
        assert!(!unsafe_base.contains("javascript:"));
    }

    #[test]
    fn highlights_fenced_code_blocks() {
        let html = render_markdown_to_html("```rust\nfn main() {\n    let value = 42;\n}\n```");

        assert!(html.contains(r#"class="dm-code-block""#));
        assert!(html.contains(r#"<span class="dm-token-keyword">fn</span>"#));
        assert!(html.contains(r#"<span class="dm-token-function">main</span>"#));
        assert!(html.contains(r#"<span class="dm-token-number">42</span>"#));
    }

    #[test]
    fn highlights_types_and_builtins() {
        let html = render_markdown_to_html(
            "```zig\nconst std = @import(\"std\");\nfn hello(writer: anytype) void {}\n```",
        );

        assert!(html.contains(r#"<span class="dm-token-builtin">std</span>"#));
        assert!(html.contains(r#"<span class="dm-token-builtin">@import</span>"#));
        assert!(html.contains(r#"<span class="dm-token-type">anytype</span>"#));
        assert!(html.contains(r#"<span class="dm-token-type">void</span>"#));
    }

    #[test]
    fn renders_mermaid_fenced_blocks() {
        let html =
            render_markdown_to_html("```mermaid\nflowchart LR\n  A[Markdown] --> B[HTML]\n```");

        assert!(html.contains("dm-mermaid-chart"));
        assert!(html.contains("dm-mermaid-flowchart"));
        assert!(html.contains("Markdown"));
        assert!(html.contains("HTML"));
        assert!(html.contains("dm-mermaid-edge"));
        assert!(!html.contains(r#"x2="560.0" y2="150.0""#));
    }

    #[test]
    fn renders_mermaid_diagram_family_previews() {
        let diagrams = [
            (
                "swimlane-beta LR\n  subgraph Customer\n    request[Request service]\n  end",
                "Swimlanes diagram",
            ),
            (
                "sequenceDiagram\n  A->>B: Render",
                "Mermaid sequence diagram",
            ),
            ("classDiagram\n  class DmMarkdown", "Class diagram"),
            ("stateDiagram-v2\n  [*] --> Render", "State diagram"),
            (
                "erDiagram\n  DOCUMENT ||--o{ BLOCK : contains",
                "Entity relationship",
            ),
            ("journey\n  section Author\n    Write: 5", "User journey"),
            (
                "gantt\n  title Work\n  Task :2026-07-01, 1d",
                "Mermaid gantt chart",
            ),
            ("pie showData\n  \"Code\" : 42", "Mermaid pie chart"),
            (
                "quadrantChart\n  \"Markdown\": [0.4, 0.7]",
                "Mermaid quadrant chart",
            ),
            (
                "requirementDiagram\n  requirement safe_render",
                "Requirement diagram",
            ),
            ("gitGraph\n  commit", "Git graph"),
            ("C4Context\n  Person(user, \"User\")", "C4 diagram"),
            ("mindmap\n  root((DmMarkdown))", "Mermaid mindmap"),
            ("timeline\n  Source : Markdown", "Mermaid timeline"),
            ("zenuml\n  A->B: Render", "ZenUML diagram"),
            ("sankey-beta\n  A,B,10", "Sankey diagram"),
            ("xychart-beta\n  bar [1, 2]", "XY chart"),
            ("block-beta\n  columns 2", "Block diagram"),
            ("packet-beta\n  0-7: \"type\"", "Packet diagram"),
            ("kanban\n  Todo\n    Add docs", "Kanban board"),
            (
                "architecture-beta\n  service api(server)[API]",
                "Architecture diagram",
            ),
            ("radar-beta\n  axis Safety, Speed", "Radar chart"),
            ("eventModeling\n  event Submitted", "Event modeling diagram"),
            ("treemap-beta\n  \"Markdown\": 40", "Treemap"),
            ("venn\n  Markdown: 40", "Venn diagram"),
            ("ishikawa\n  root((Quality))", "Ishikawa diagram"),
            ("wardley\n  component Markdown [0.6, 0.5]", "Wardley map"),
            ("cynefin\n  Simple: Escape HTML", "Cynefin diagram"),
            ("treeview\n  root\n    markdown", "TreeView diagram"),
        ];

        for (source, title) in diagrams {
            let html = render_markdown_to_html(&format!("```mermaid\n{source}\n```"));
            assert!(html.contains("dm-mermaid-chart"), "{source}");
            assert!(html.contains(title), "{source}");
        }
    }

    #[test]
    fn renders_swimlane_lanes_nodes_and_edges() {
        let html = render_markdown_to_html(
            "```mermaid\nswimlane-beta LR\n  subgraph Customer\n    request[Request service]\n    update[Receive update]\n  end\n  subgraph Support\n    triage[Triage request]\n    answer[Send answer]\n  end\n  request --> triage\n  triage -->|Known issue| answer\n  answer --> update\n```",
        );

        assert!(html.contains("dm-mermaid-swimlanes"));
        assert!(html.contains("dm-swimlane-lane"));
        assert!(html.contains("Customer"));
        assert!(html.contains("Request service"));
        assert!(html.contains("Known issue"));
        assert!(html.contains("dm-swimlane-edge"));
        assert!(!html.contains("dm-mermaid-summary-swimlanes"));
    }

    #[test]
    fn renders_class_namespaces_members_and_relations() {
        let html = render_markdown_to_html(
            "```mermaid\nclassDiagram\n  namespace Company {\n    class CEO {\n      +makeDecisions()\n    }\n  }\n  namespace Company.Engineering.Backend {\n    class Developer {\n      +writeCode()\n    }\n  }\n  CEO --> Developer : oversees\n```",
        );

        assert!(html.contains("dm-mermaid-class"));
        assert!(html.contains("dm-class-namespace"));
        assert!(html.contains("Company.Engineering.Backend"));
        assert!(html.contains("CEO"));
        assert!(html.contains("+makeDecisions()"));
        assert!(html.contains("+writeCode()"));
        assert!(html.contains("oversees"));
        assert!(html.contains("dm-class-relation"));
        assert!(!html.contains("dm-mermaid-summary-class"));
    }

    #[test]
    fn renders_state_er_journey_and_gantt_diagrams() {
        let state = render_markdown_to_html(
            "```mermaid\nstateDiagram-v2\n  [*] --> Still\n  Still --> Moving\n  Moving --> Crash\n  Crash --> [*]\n```",
        );
        assert!(state.contains("dm-mermaid-state"));
        assert!(state.contains("dm-state-node"));
        assert!(state.contains("Still"));
        assert!(!state.contains(">*]<"));
        assert!(!state.contains(">[*<"));
        assert!(!state.contains("dm-mermaid-summary-state"));

        let er = render_markdown_to_html(
            "```mermaid\nerDiagram\n  CUSTOMER ||--o{ ORDER : places\n  ORDER ||--|{ LINE_ITEM : contains\n```",
        );
        assert!(er.contains("dm-mermaid-er"));
        assert!(er.contains("dm-er-entity"));
        assert!(er.contains("CUSTOMER"));
        assert!(er.contains("places"));
        assert!(!er.contains("dm-mermaid-summary-er"));

        let journey = render_markdown_to_html(
            "```mermaid\njourney\n  title My working day\n  section Morning\n    Make tea: 5: Cat, Me\n```",
        );
        assert!(journey.contains("dm-mermaid-journey"));
        assert!(journey.contains("dm-journey-task"));
        assert!(journey.contains("My working day"));
        assert!(!journey.contains("dm-mermaid-summary-journey"));

        let gantt = render_markdown_to_html(
            "```mermaid\ngantt\n  title A Gantt Diagram\n  dateFormat YYYY-MM-DD\n  section Section\n  A task :2014-01-01, 2014-01-31\n```",
        );
        assert!(gantt.contains("dm-mermaid-gantt"));
        assert!(gantt.contains("dm-gantt-bar"));
        assert!(gantt.contains("A Gantt Diagram"));
        assert!(!gantt.contains("dm-mermaid-summary-gantt"));
    }

    #[test]
    fn renders_requirement_git_c4_and_mindmap_diagrams() {
        let requirement = render_markdown_to_html(
            "```mermaid\nrequirementDiagram\n  requirement safe_render {\n    id: DM-1\n    text: Strip unsafe raw HTML\n    risk: high\n    verifymethod: test\n  }\n```",
        );
        assert!(requirement.contains("dm-mermaid-requirement"));
        assert!(requirement.contains("dm-requirement-node"));
        assert!(requirement.contains("safe_render"));
        assert!(!requirement.contains("dm-mermaid-summary-requirement"));

        let git = render_markdown_to_html(
            "```mermaid\ngitGraph\n  commit id: \"markdown\"\n  branch demo\n  checkout demo\n  commit id: \"mermaid\"\n```",
        );
        assert!(git.contains("dm-mermaid-git"));
        assert!(git.contains("dm-git-commit"));
        assert!(git.contains("markdown"));
        assert!(git.contains("mermaid"));
        assert!(!git.contains("dm-mermaid-summary-git"));

        let c4 = render_markdown_to_html(
            "```mermaid\nC4Context\n  title Markdown rendering context\n  Person(author, \"Author\", \"Writes docs\")\n  System(renderer, \"DmMarkdown\", \"Renders safe HTML\")\n  Rel(author, renderer, \"submits markdown\")\n```",
        );
        assert!(c4.contains("dm-mermaid-c4"));
        assert!(c4.contains("dm-c4-node"));
        assert!(c4.contains("submits markdown"));
        assert!(!c4.contains("dm-mermaid-summary-c4"));

        let mindmap = render_markdown_to_html(
            "```mermaid\nmindmap\n  root((DmMarkdown))\n    Markdown\n      Tables\n    Mermaid\n      Flowchart\n```",
        );
        assert!(mindmap.contains("dm-mermaid-mindmap"));
        assert!(mindmap.contains("dm-mindmap-edge"));
        assert!(mindmap.contains("Flowchart"));
        assert!(!mindmap.contains("dm-mermaid-summary-mindmap"));
    }

    #[test]
    fn renders_timeline_zenuml_sankey_and_xy_diagrams() {
        let timeline = render_markdown_to_html(
            "```mermaid\ntimeline\n  title Render path\n  Source : Markdown arrives\n  Parse : Events are sanitized\n  Output : HTML is emitted\n```",
        );
        assert!(timeline.contains("dm-mermaid-timeline"));
        assert!(timeline.contains("dm-timeline-axis"));
        assert!(timeline.contains("Markdown arrives"));
        assert!(!timeline.contains("dm-mermaid-summary-timeline"));

        let zenuml = render_markdown_to_html(
            "```mermaid\nzenuml\n  Author->Renderer: markdown\n  Renderer->Author: html\n```",
        );
        assert!(zenuml.contains("dm-mermaid-zenuml"));
        assert!(zenuml.contains("dm-zenuml-message"));
        assert!(zenuml.contains("markdown"));
        assert!(!zenuml.contains("dm-mermaid-summary-zenuml"));

        let sankey = render_markdown_to_html(
            "```mermaid\nsankey-beta\n  Markdown,Parser,40\n  Parser,Highlighter,22\n  Parser,Mermaid,18\n```",
        );
        assert!(sankey.contains("dm-mermaid-sankey"));
        assert!(sankey.contains("dm-sankey-link"));
        assert!(sankey.contains("Highlighter"));
        assert!(!sankey.contains("dm-mermaid-summary-sankey"));

        let xy = render_markdown_to_html(
            "```mermaid\nxychart-beta\n  title \"Render coverage\"\n  x-axis [Markdown, Code, Mermaid]\n  y-axis \"Coverage\" 0 --> 100\n  bar [92, 88, 76]\n```",
        );
        assert!(xy.contains("dm-mermaid-xy"));
        assert!(xy.contains("dm-xy-bar"));
        assert!(xy.contains("Render coverage"));
        assert!(!xy.contains("dm-mermaid-summary-xy-chart"));
    }

    #[test]
    fn renders_block_packet_kanban_architecture_and_radar_diagrams() {
        let block = render_markdown_to_html(
            "```mermaid\nblock-beta\n  columns 3\n  source[\"Source\"] parser[\"Parser\"] html[\"HTML\"]\n  source --> parser\n  parser --> html\n```",
        );
        assert!(block.contains("dm-mermaid-block"));
        assert!(block.contains("dm-block-node"));
        assert!(block.contains("dm-block-edge"));
        assert!(!block.contains("dm-mermaid-summary-block"));

        let packet = render_markdown_to_html(
            "```mermaid\npacket-beta\n  title Markdown payload\n  0-7: \"type\"\n  8-31: \"content length\"\n```",
        );
        assert!(packet.contains("dm-mermaid-packet"));
        assert!(packet.contains("dm-packet-field"));
        assert!(packet.contains("content length"));
        assert!(!packet.contains("dm-mermaid-summary-packet"));

        let kanban = render_markdown_to_html(
            "```mermaid\nkanban\n  Todo\n    Add docs\n  Doing\n    Render diagrams\n```",
        );
        assert!(kanban.contains("dm-mermaid-kanban"));
        assert!(kanban.contains("dm-kanban-column"));
        assert!(kanban.contains("Render diagrams"));
        assert!(!kanban.contains("dm-mermaid-summary-kanban"));

        let architecture = render_markdown_to_html(
            "```mermaid\narchitecture-beta\n  group app(cloud)[Example app]\n  service markdown(server)[DmMarkdown] in app\n  service browser(internet)[Browser] in app\n  browser:R -- L:markdown\n```",
        );
        assert!(architecture.contains("dm-mermaid-architecture"));
        assert!(architecture.contains("dm-architecture-service"));
        assert!(architecture.contains("DmMarkdown"));
        assert!(!architecture.contains("dm-mermaid-summary-architecture"));

        let radar = render_markdown_to_html(
            "```mermaid\nradar-beta\n  title Renderer qualities\n  axis Safety, Coverage, Readability, Speed\n  curve Demo{90, 80, 88, 74}\n```",
        );
        assert!(radar.contains("dm-mermaid-radar"));
        assert!(radar.contains("dm-radar-area"));
        assert!(radar.contains("Renderer qualities"));
        assert!(!radar.contains("dm-mermaid-summary-radar"));
    }

    #[test]
    fn renders_event_treemap_venn_ishikawa_wardley_cynefin_and_treeview_diagrams() {
        let event_modeling = render_markdown_to_html(
            "```mermaid\neventModeling\n  event MarkdownSubmitted\n  command RenderMarkdown\n  view HtmlPreview\n```",
        );
        assert!(event_modeling.contains("dm-mermaid-event-modeling"));
        assert!(event_modeling.contains("dm-event-modeling-node"));
        assert!(event_modeling.contains("MarkdownSubmitted"));
        assert!(!event_modeling.contains("dm-mermaid-summary-event-modeling"));

        let treemap = render_markdown_to_html(
            "```mermaid\ntreemap-beta\n  \"DmMarkdown\"\n    \"Markdown\": 32\n    \"Code\": 26\n    \"Mermaid\": 42\n```",
        );
        assert!(treemap.contains("dm-mermaid-treemap"));
        assert!(treemap.contains("dm-treemap-leaf"));
        assert!(treemap.contains("DmMarkdown"));
        assert!(!treemap.contains("dm-mermaid-summary-treemap"));

        let venn = render_markdown_to_html(
            "```mermaid\nvenn\n  Markdown: 40\n  Mermaid: 35\n  Markdown & Mermaid: 15\n```",
        );
        assert!(venn.contains("dm-mermaid-venn"));
        assert!(venn.contains("dm-venn-circle"));
        assert!(venn.contains("Mermaid"));
        assert!(!venn.contains("dm-mermaid-summary-venn"));

        let ishikawa = render_markdown_to_html(
            "```mermaid\nishikawa\n  root((Renderer quality))\n    Safety\n      Escaped HTML\n    Readability\n      Highlighted code\n    Coverage\n      Mermaid diagrams\n```",
        );
        assert!(ishikawa.contains("dm-mermaid-ishikawa"));
        assert!(ishikawa.contains("dm-ishikawa-bone"));
        assert!(ishikawa.contains("Renderer quality"));
        assert!(!ishikawa.contains("dm-mermaid-summary-ishikawa"));

        let wardley = render_markdown_to_html(
            "```mermaid\nwardley\n  title Documentation map\n  anchor User [0.95, 0.65]\n  component Markdown [0.65, 0.55]\n  component Mermaid [0.48, 0.44]\n  User->Markdown\n```",
        );
        assert!(wardley.contains("dm-mermaid-wardley"));
        assert!(wardley.contains("dm-wardley-point"));
        assert!(wardley.contains("Documentation map"));
        assert!(!wardley.contains("dm-mermaid-summary-wardley"));

        let cynefin = render_markdown_to_html(
            "```mermaid\ncynefin\n  title Rendering decisions\n  Simple: Escape HTML\n  Complicated: Highlight syntax\n  Complex: Diagram layout\n```",
        );
        assert!(cynefin.contains("dm-mermaid-cynefin"));
        assert!(cynefin.contains("dm-cynefin-domain"));
        assert!(cynefin.contains("Rendering decisions"));
        assert!(!cynefin.contains("dm-mermaid-summary-cynefin"));

        let treeview = render_markdown_to_html(
            "```mermaid\ntreeview\n  root\n    markdown\n      code\n      mermaid\n    html\n```",
        );
        assert!(treeview.contains("dm-mermaid-treeview"));
        assert!(treeview.contains("dm-treeview-connector"));
        assert!(treeview.contains("markdown"));
        assert!(!treeview.contains("dm-mermaid-summary-treeview"));
    }

    #[test]
    fn parses_sequence_return_arrows_and_mindmap_root_labels() {
        let sequence =
            render_markdown_to_html("```mermaid\nsequenceDiagram\n  Renderer-->>Author: Done\n```");
        assert!(sequence.contains("Renderer"));
        assert!(sequence.contains("Author"));
        assert!(!sequence.contains("Renderer-"));

        let mindmap = render_markdown_to_html("```mermaid\nmindmap\n  root((DmMarkdown))\n```");
        assert!(mindmap.contains(">DmMarkdown<"));
        assert!(!mindmap.contains("root((DmMarkdown"));
    }
}
