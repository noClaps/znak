#![doc = include_str!("../README.md")]

mod languages;
mod theme;

use std::{collections::HashMap, rc::Rc};

use html::escape_html;
use tree_sitter_highlight::{HighlightConfiguration, Highlighter, HtmlRenderer};

pub use crate::theme::Theme;

/// A highlighter configuration object. As it can be expensive to create Highlight objects, mainly
/// due to creating HighlightConfigurations, you should initialise it once and reuse it everywhere
/// you need, instead of creating multiple.
pub struct Highlight {
    theme: Theme,
    recognised_names: Vec<String>,
    languages: HashMap<String, Rc<HighlightConfiguration>>,
}

impl Highlight {
    /// Creates a new [Highlight] object.
    ///
    /// # Parameters
    ///
    /// - `theme`: The [Theme] you want to use for syntax highlighting.
    ///
    /// # Usage
    ///
    /// ```rust
    /// use highlight::Highlight;
    ///
    /// let theme = include_str!("../../../theme.css").parse().unwrap();
    /// let hl = Highlight::new(theme);
    /// ```
    pub fn new(theme: Theme) -> Self {
        let recognised_names = theme
            .highlights
            .keys()
            .map(|k| k.to_owned())
            .collect::<Vec<String>>();
        let mut hl = Self {
            theme,
            recognised_names,
            languages: HashMap::with_capacity(9),
        };
        hl.default_languages();

        hl
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
    /// let theme = include_str!("../../../theme.css").parse().unwrap();
    /// let mut hl = Highlight::new(theme);
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
        let global_style = self.theme.root.clone();

        if language == "plaintext" || language == "plain" || language == "text" || language == "txt"
        {
            return format!(
                "<pre class=\"znak-highlight\" style=\"{}\"><code>{}</code></pre>",
                global_style,
                escape_html!(code)
            );
        }

        let mut highlighter = Highlighter::new();
        let Some(config) = self.get_language(&language) else {
            eprintln!("Language not supported: {language}, continuing as plaintext");
            return format!(
                "<pre class=\"znak-highlight\" style=\"{}\"><code>{}</code></pre>",
                global_style,
                escape_html!(code)
            );
        };

        let highlights = match highlighter.highlight(&config, code.as_bytes(), None, |capture| {
            self.get_language(capture)
        }) {
            Ok(highlights) => highlights,
            Err(err) => {
                eprintln!("Error while highlighting: {err}. Continuing as plaintext");
                return format!(
                    "<pre class=\"znak-highlight\" style=\"{}\"><code>{}</code></pre>",
                    global_style,
                    escape_html!(code)
                );
            }
        };

        let mut html_renderer = HtmlRenderer::new();
        match html_renderer.render(highlights, code.as_bytes(), &|h, output| {
            let Some(style) = self.theme.highlights.get(&self.recognised_names[h.0]) else {
                unreachable!()
            };
            let attr = format!(
                "class=\"{}\" style=\"{}\"",
                self.recognised_names[h.0], style
            );
            output.extend(attr.as_bytes())
        }) {
            Ok(()) => (),
            Err(err) => {
                eprintln!(
                    "Error rendering highlighted text to HTML: {err}. Continuing as plaintext",
                );
                return format!(
                    "<pre class=\"znak-highlight\" style=\"{}\"><code>{}</code></pre>",
                    global_style,
                    escape_html!(code)
                );
            }
        };
        let highlighted_text: String = html_renderer.lines().collect();

        format!(
            "<pre class=\"znak-highlight\" style=\"{}\"><code>{}</code></pre>",
            global_style,
            highlighted_text.trim()
        )
    }
}
