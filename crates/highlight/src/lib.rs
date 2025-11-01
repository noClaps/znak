//! A syntax highlighting library that uses
//! [Tree-sitter](https://tree-sitter.github.io/tree-sitter/) for incredibly
//! quick parsing and highlighting.
//!
//! You can add this library to your project with:
//!
//! ```bash
//! cargo add --git https://github.com/noClaps/znak highlight
//! ```
//!
//! ## Languages
//!
//! Highlight has a number of languages built in, but none are enabled by
//! default to keep the bundle size small. You can enable the languages you'd
//! like by enabling the respective features on the crate:
//!
//! ```toml
//! # Cargo.toml
//!
//! [dependencies]
//! highlight = { git = "https://github.com/noClaps/znak", version = "0.21.0", features = [
//!   "bash",
//!   "c",
//!   "python",
//!   "typescript"
//! ] }
//! ```
//!
//! If you'd like to enable all languages, you can do so by enabling the `all` feature:
//!
//! ```toml
//! # Cargo.toml
//!
//! [dependencies]
//! highlight = { git = "https://github.com/noClaps/znak", version = "0.21.0", features = ["all"] }
//! ```

mod languages;
mod theme;

use std::{collections::HashMap, sync::Arc};

use html::escape_html;
pub use tree_sitter_highlight::HighlightConfiguration;
use tree_sitter_highlight::{Highlighter, HtmlRenderer};

pub use crate::theme::Theme;

/// A highlighter configuration object.
pub struct Highlight {
    theme: Theme,
    recognised_names: Vec<String>,
    languages: HashMap<String, Arc<HighlightConfiguration>>,
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
    /// use highlight::{Highlight, Theme};
    ///
    /// let theme = Theme::new(include_str!("../../../theme.css")).unwrap();
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
            languages: HashMap::new(),
        };
        hl.default_langs();

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
    /// use highlight::{Highlight, Theme};
    ///
    /// let theme = Theme::new(include_str!("../../../theme.css")).unwrap();
    /// let hl = Highlight::new(theme);
    ///
    /// let code = r#"
    /// main :: IO()
    /// main = putStrLn "Hello World"
    /// "#.to_string();
    /// let language = "haskell".to_string();
    ///
    /// let highlighted = hl.highlight(code, language);
    /// ```
    pub fn highlight(&self, code: String, language: String) -> String {
        let global_style = self.theme.root.clone();

        if language == "plaintext" || language == "plain" || language == "text" || language == "txt"
        {
            return format!(
                "<pre class=\"ts-highlight\" style=\"{}\"><code>{}</code></pre>",
                global_style,
                escape_html!(code)
            );
        }

        let mut highlighter = Highlighter::new();
        let config = match self.get_language(&language) {
            Some(config) => config,
            None => {
                eprintln!("Language not supported: {language}, continuing as plaintext");
                return format!(
                    "<pre class=\"ts-highlight\" style=\"{}\"><code>{}</code></pre>",
                    global_style,
                    escape_html!(code)
                );
            }
        };

        let highlights = match highlighter.highlight(&config, code.as_bytes(), None, |capture| {
            self.get_language(capture)
        }) {
            Ok(highlights) => highlights,
            Err(err) => {
                eprintln!("Error while highlighting: {err}. Continuing as plaintext");
                return format!(
                    "<pre class=\"ts-highlight\" style=\"{}\"><code>{}</code></pre>",
                    global_style,
                    escape_html!(code)
                );
            }
        };

        let mut html_renderer = HtmlRenderer::new();
        match html_renderer.render(highlights, code.as_bytes(), &|h, output| {
            let attr = match self.theme.highlights.get(&self.recognised_names[h.0]) {
                Some(style) => format!(
                    "class=\"{}\" style=\"{}\"",
                    self.recognised_names[h.0], style
                ),
                None => format!("class=\"{}\"", self.recognised_names[h.0]),
            };
            return output.extend(attr.as_bytes());
        }) {
            Ok(()) => (),
            Err(err) => {
                eprintln!(
                    "Error rendering highlighted text to HTML: {err}. Continuing as plaintext",
                );
                return format!(
                    "<pre class=\"ts-highlight\" style=\"{}\"><code>{}</code></pre>",
                    global_style,
                    escape_html!(code)
                );
            }
        };
        let highlighted_text = html_renderer.lines().collect::<Vec<&str>>().join("\n");

        format!(
            "<pre class=\"ts-highlight\" style=\"{}\"><code>{}</code></pre>",
            global_style,
            highlighted_text.trim()
        )
    }
}
