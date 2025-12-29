use std::rc::Rc;

use tree_sitter_highlight::HighlightConfiguration;

use crate::Highlight;

impl Highlight {
    pub(crate) fn get_language(&self, name: &str) -> Option<&HighlightConfiguration> {
        Some(self.languages.get(name)?.as_ref())
    }

    pub(crate) fn add_language(
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

        let config = Rc::new(config);
        for &name in names.as_ref() {
            self.languages.insert(name.into(), config.clone());
        }
    }

    pub(crate) fn default_languages(&mut self) {
        self.add_language(
            &["bash", "sh"],
            tree_sitter_bash::LANGUAGE.into(),
            include_str!("../queries/bash/highlights.scm"),
            "",
            "",
        );
        self.add_language(
            &["c"],
            tree_sitter_c::LANGUAGE.into(),
            include_str!("../queries/c/highlights.scm"),
            include_str!("../queries/c/injections.scm"),
            "",
        );
        self.add_language(
            &["html"],
            tree_sitter_html::LANGUAGE.into(),
            include_str!("../queries/html/highlights.scm"),
            include_str!("../queries/html/injections.scm"),
            "",
        );
        self.add_language(
            &["json"],
            tree_sitter_json::LANGUAGE.into(),
            include_str!("../queries/json/highlights.scm"),
            "",
            "",
        );
        self.add_language(
            &["python", "py"],
            tree_sitter_python::LANGUAGE.into(),
            include_str!("../queries/python/highlights.scm"),
            include_str!("../queries/python/injections.scm"),
            "",
        );
        self.add_language(
            &["typescript", "ts"],
            tree_sitter_typescript::LANGUAGE_TYPESCRIPT.into(),
            include_str!("../queries/typescript/highlights.scm"),
            include_str!("../queries/typescript/injections.scm"),
            "",
        );
    }
}
