#![doc = include_str!("../README.md")]

mod languages;
mod theme;

use std::{collections::HashMap, rc::Rc};

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
            .0
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
        let config = self.get_language(&language).unwrap();

        let mut highlighter = Highlighter::new();
        let highlights = highlighter
            .highlight(&config, code.as_bytes(), None, |capture| {
                self.get_language(capture)
            })
            .unwrap();

        let mut html_renderer = HtmlRenderer::new();
        html_renderer
            .render(highlights, code.as_bytes(), &|h, output| {
                let style = self.theme.0.get(&self.recognised_names[h.0]).unwrap();
                output.extend(format!("style=\"{}\"", style).as_bytes())
            })
            .unwrap();
        let highlighted_text: String = html_renderer.lines().collect();

        highlighted_text.trim().to_string()
    }
}
