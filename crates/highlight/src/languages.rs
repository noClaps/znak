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
    /// - `language`: The Tree-sitter Language object.
    /// - `highlights_query`: The highlights query for the language. If this is empty, there will
    ///   be no syntax highlighting.
    /// - `injection_query`: The injections query for the language.
    /// - `locals_query`: The locals query for the language.
    ///
    /// # Usage
    ///
    /// ```rust
    /// use highlight::Highlight;
    ///
    /// let theme = include_str!("../../../theme.css").parse().unwrap();
    /// let mut hl = Highlight::new(theme);
    /// let language_names = &["python", "py"];
    /// hl.add_language(
    ///     language_names,
    ///     tree_sitter_python::LANGUAGE.into(),
    ///     tree_sitter_python::HIGHLIGHTS_QUERY,
    ///     "",
    ///     "",
    /// );
    /// ```
    pub fn add_language(
        &mut self,
        names: &[&str],
        language: tree_sitter::Language,
        highlights_query: &str,
        injection_query: &str,
        locals_query: &str,
    ) {
        let mut config = HighlightConfiguration::new(
            language,
            names[0],
            highlights_query,
            injection_query,
            locals_query,
        )
        .unwrap();

        config.configure(&self.recognised_names);
        let config = Arc::new(config);
        for &name in names.as_ref() {
            self.languages.insert(name.into(), config.clone());
        }
    }
}
