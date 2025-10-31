//! # Highlight
//!
//! A syntax highlighting library that uses
//! [Tree-sitter](https://tree-sitter.github.io/tree-sitter/) for incredibly
//! quick parsing and highlighting.
//!
//! ## Usage
//!
//! You can use this crate by adding it to your project:
//!
//! ```sh
//! cargo add --git https://github.com/noClaps/znak highlight
//! ```
//!
//! and then using it in your code:
//!
//! ```rust
//! use highlight::{Highlight, Theme};
//!
//! let css = include_str!("../../../theme.css");
//! let theme: Theme = Theme::new(css).unwrap();
//! let hl = Highlight::new(theme);
//!
//! let code = "Your code here".to_string();
//! let lang = "plaintext".to_string();
//! let highlighted_text: String = hl.highlight(code, lang);
//! ```
//!
//! You can use [Theme::new] to create a new theme from a CSS string. If
//! you'd like to make a blank theme, use `Theme::new("")`.
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
                escape_html(code)
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
                    escape_html(code)
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
                    escape_html(code.clone())
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
                    escape_html(code)
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

/// Escapes unsafe characters in HTML.
///
/// - `&` becomes `&amp;`
/// - `"` becomes `&quot;`
/// - `'` becomes `&#x27;`
/// - `<` becomes `&lt;`
/// - `>` becomes `&gt;`
///
/// # Parameters
///
/// - `input`: Text to escape.
///
/// # Usage
///
/// ```rust
/// use highlight::escape_html;
///
/// let text = "<div>This text has HTML in it</div>";
/// let escaped = escape_html(text);
/// assert_eq!(escaped, "&lt;div&gt;This text has HTML in it&lt;/div&gt;")
/// ```
pub fn escape_html(input: impl Into<String>) -> String {
    input
        .into()
        .replace('&', "&amp;")
        .replace('"', "&quot;")
        .replace("'", "&#x27;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
}
