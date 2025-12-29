#![doc = include_str!("../README.md")]

mod languages;

use std::{collections::HashMap, rc::Rc};

use tree_sitter_highlight::{HighlightConfiguration, Highlighter, HtmlRenderer};

use crate::languages::default_languages;

/// A highlighter configuration object. As it can be expensive to create Highlight objects, mainly
/// due to creating HighlightConfigurations, you should initialise it once and reuse it everywhere
/// you need, instead of creating multiple.
pub struct Highlight(HashMap<String, Rc<HighlightConfiguration>>);

impl Highlight {
    /// Creates a new [Highlight] object.
    ///
    /// # Usage
    ///
    /// ```rust
    /// use highlight::Highlight;
    ///
    /// let hl = Highlight::new();
    /// ```
    pub fn new() -> Self {
        Self(default_languages())
    }

    /// Used to highlight the given source code as the given language.
    ///
    /// # Parameters
    ///
    /// - `code`: The source code to highlight.
    /// - `language`: The language to highlight the source code as.
    ///
    /// # Usage
    ///
    /// ```rust
    /// use highlight::Highlight;
    ///
    /// let hl = Highlight::new();
    ///
    /// let code = r#"
    /// def main():
    ///     print("Hello world")
    ///
    /// main()
    /// "#;
    /// let language = "python";
    ///
    /// let highlighted = hl.highlight(code, language);
    /// ```
    pub fn highlight(&self, code: &str, language: &str) -> String {
        let config = self.0.get(language).unwrap();

        let mut highlighter = Highlighter::new();
        let highlights = highlighter
            .highlight(&config, code.as_bytes(), None, |capture| {
                Some(self.0.get(capture)?.as_ref())
            })
            .unwrap();

        let mut html_renderer = HtmlRenderer::new();
        html_renderer
            .render(highlights, code.as_bytes(), &|h, output| {
                output.extend(
                    format!("class=\"{}\"", RECOGNISED_NAMES[h.0])
                        .replace('.', "-")
                        .as_bytes(),
                )
            })
            .unwrap();
        let highlighted_text: String = html_renderer.lines().collect();

        highlighted_text.trim().to_string()
    }
}

pub(crate) const RECOGNISED_NAMES: &[&str] = &[
    "attribute",
    "boolean",
    "comment",
    "comment.doc",
    "constant",
    "constructor",
    "embedded",
    "emphasis",
    "emphasis.strong",
    "enum",
    "function",
    "hint",
    "keyword",
    "label",
    "link_text",
    "link_uri",
    "number",
    "operator",
    "predictive",
    "preproc",
    "primary",
    "property",
    "punctuation",
    "punctuation.bracket",
    "punctuation.delimiter",
    "punctuation.list_marker",
    "punctuation.special",
    "string",
    "string.escape",
    "string.regex",
    "string.special",
    "string.special.symbol",
    "tag",
    "tag.doctype",
    "text.literal",
    "title",
    "type",
    "variable",
    "variable.special",
    "variant",
];
