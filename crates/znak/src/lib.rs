#![doc = include_str!("../README.md")]

mod frontmatter;
mod headings;
mod parser;

pub use frontmatter::parse_frontmatter;
pub use headings::{Heading, parse_headings};
pub use highlight::{Highlight, Theme};

use crate::parser::{parse, renderer};

/// A function that renders the input text to HTML.
///
/// # Parameters
///
/// - `input`: The input text to be converted to HTML. This can be from a
///   Markdown file as long as the syntax is supported by Znak. See the
///   [documentation](index.html#syntax) for the supported syntax.
///
/// - `hl`: The highlighter for code blocks. You can read the docs for
///   [Highlight] to learn more about how to set this up.
///
/// # Usage
///
/// ```rust
/// use znak::{Highlight, render};
///
/// let css = include_str!("../../../theme.css");
/// let theme = css.parse().unwrap();
/// let hl = Highlight::new(theme);
///
/// let input = include_str!("../demo.md");
/// let rendered_html = render(input, &hl);
/// ```
pub fn render(input: impl Into<String>, hl: &Highlight) -> String {
    let input = input.into();
    let lines = input.trim().lines().collect::<Vec<&str>>();
    let mut cur = 0;

    if lines[cur] == "---" {
        match lines[cur + 1..].into_iter().position(|&line| line == "---") {
            Some(line_idx) => cur = line_idx + 2,
            None => (),
        }
    }

    let input = lines[cur..].join("\n");
    let parser_output = parse(input, hl);

    let mut output = String::new();
    for node in parser_output {
        output += renderer(node).as_str();
    }
    output
}
