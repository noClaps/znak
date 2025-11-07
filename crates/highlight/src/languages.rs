use std::sync::Arc;

use tree_sitter_highlight::HighlightConfiguration;

use crate::Highlight;

impl Highlight {
    pub(crate) fn get_language(&self, name: &str) -> Option<&HighlightConfiguration> {
        Some(self.languages.get(name)?.as_ref())
    }

    /// Add your own language to the highlighter.
    ///
    /// # Parameters
    ///
    /// - `names`: The names of the language.
    /// - `config`: The [HighlightConfiguration] for the language.
    ///
    /// # Usage
    ///
    /// ```rust
    /// use highlight::{Highlight, HighlightConfiguration};
    ///
    /// let theme = include_str!("../../../theme.css").parse().unwrap();
    /// let mut hl = Highlight::new(theme);
    /// let config = HighlightConfiguration::new(
    ///     tree_sitter_haskell::LANGUAGE.into(),
    ///     "haskell",
    ///     tree_sitter_haskell::HIGHLIGHTS_QUERY,
    ///     tree_sitter_haskell::INJECTIONS_QUERY,
    ///     tree_sitter_haskell::LOCALS_QUERY,
    /// ).unwrap();
    /// let language_names = &["haskell", "hs"];
    /// hl.add_language(language_names, config);
    /// ```
    pub fn add_language(&mut self, names: &[&str], mut config: HighlightConfiguration) {
        config.configure(&self.recognised_names);
        let config = Arc::new(config);
        for &name in names.as_ref() {
            self.languages.insert(name.into(), config.clone());
        }
    }
}
