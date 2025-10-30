use std::sync::Arc;

use tree_sitter_highlight::HighlightConfiguration;

use crate::Highlight;

impl Highlight {
    pub(crate) fn get_language(&self, name: &str) -> Option<&HighlightConfiguration> {
        Some(self.languages.get(name)?.as_ref())
    }

    /// Add your own language to the highlighter.
    ///
    /// ```rust
    /// use highlight::{Highlight, HighlightConfiguration, Theme};
    ///
    /// let theme = Theme::new(include_str!("../../../theme.css")).unwrap();
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
        for &name in names {
            self.languages.insert(name.to_string(), config.clone());
        }
    }

    pub(crate) fn default_langs(&mut self) {
        #[cfg(feature = "agda")]
        self.add_language(
            &["agda"],
            HighlightConfiguration::new(
                tree_sitter_agda::LANGUAGE.into(),
                "agda",
                tree_sitter_agda::HIGHLIGHTS_QUERY,
                "",
                "",
            )
            .unwrap(),
        );
        #[cfg(feature = "bash")]
        self.add_language(
            &["bash", "shellscript", "shell", "zsh", "sh"],
            HighlightConfiguration::new(
                tree_sitter_bash::LANGUAGE.into(),
                "bash",
                tree_sitter_bash::HIGHLIGHT_QUERY,
                "",
                "",
            )
            .unwrap(),
        );
        #[cfg(feature = "c")]
        self.add_language(
            &["c"],
            HighlightConfiguration::new(
                tree_sitter_c::LANGUAGE.into(),
                "c",
                tree_sitter_c::HIGHLIGHT_QUERY,
                "",
                "",
            )
            .unwrap(),
        );
        #[cfg(feature = "cpp")]
        self.add_language(
            &["cpp", "c++"],
            HighlightConfiguration::new(
                tree_sitter_cpp::LANGUAGE.into(),
                "cpp",
                &[
                    tree_sitter_c::HIGHLIGHT_QUERY,
                    tree_sitter_cpp::HIGHLIGHT_QUERY,
                ]
                .join("\n"),
                include_str!("../queries/cpp/injections.scm"),
                "",
            )
            .unwrap(),
        );
        #[cfg(feature = "css")]
        self.add_language(
            &["css"],
            HighlightConfiguration::new(
                tree_sitter_css::LANGUAGE.into(),
                "css",
                tree_sitter_css::HIGHLIGHTS_QUERY,
                "",
                "",
            )
            .unwrap(),
        );
        #[cfg(feature = "go")]
        self.add_language(
            &["go"],
            HighlightConfiguration::new(
                tree_sitter_go::LANGUAGE.into(),
                "go",
                tree_sitter_go::HIGHLIGHTS_QUERY,
                "",
                "",
            )
            .unwrap(),
        );
        #[cfg(feature = "haskell")]
        self.add_language(
            &["haskell", "hs"],
            HighlightConfiguration::new(
                tree_sitter_haskell::LANGUAGE.into(),
                "haskell",
                tree_sitter_haskell::HIGHLIGHTS_QUERY,
                tree_sitter_haskell::INJECTIONS_QUERY,
                tree_sitter_haskell::LOCALS_QUERY,
            )
            .unwrap(),
        );
        #[cfg(feature = "html")]
        self.add_language(
            &["html"],
            HighlightConfiguration::new(
                tree_sitter_html::LANGUAGE.into(),
                "html",
                tree_sitter_html::HIGHLIGHTS_QUERY,
                tree_sitter_html::INJECTIONS_QUERY,
                "",
            )
            .unwrap(),
        );
        #[cfg(feature = "java")]
        self.add_language(
            &["java"],
            HighlightConfiguration::new(
                tree_sitter_java::LANGUAGE.into(),
                "java",
                tree_sitter_java::HIGHLIGHTS_QUERY,
                "",
                "",
            )
            .unwrap(),
        );
        #[cfg(feature = "javascript")]
        self.add_language(
            &["javascript", "js", "jsx"],
            HighlightConfiguration::new(
                tree_sitter_javascript::LANGUAGE.into(),
                "javascript",
                &[
                    tree_sitter_javascript::HIGHLIGHT_QUERY,
                    tree_sitter_javascript::JSX_HIGHLIGHT_QUERY,
                    include_str!("../queries/javascript/highlights-params.scm"),
                ]
                .join("\n"),
                tree_sitter_javascript::INJECTIONS_QUERY,
                tree_sitter_javascript::LOCALS_QUERY,
            )
            .unwrap(),
        );
        #[cfg(feature = "jsdoc")]
        self.add_language(
            &["jsdoc"],
            HighlightConfiguration::new(
                tree_sitter_jsdoc::LANGUAGE.into(),
                "jsdoc",
                tree_sitter_jsdoc::HIGHLIGHTS_QUERY,
                "",
                "",
            )
            .unwrap(),
        );
        #[cfg(feature = "json")]
        self.add_language(
            &["json"],
            HighlightConfiguration::new(
                tree_sitter_json::LANGUAGE.into(),
                "json",
                tree_sitter_json::HIGHLIGHTS_QUERY,
                "",
                "",
            )
            .unwrap(),
        );
        #[cfg(feature = "julia")]
        self.add_language(
            &["julia", "jl"],
            HighlightConfiguration::new(
                tree_sitter_julia::LANGUAGE.into(),
                "julia",
                include_str!("../queries/julia/highlights.scm"),
                "",
                include_str!("../queries/julia/locals.scm"),
            )
            .unwrap(),
        );
        #[cfg(feature = "ocaml")]
        self.add_language(
            &["ocaml"],
            HighlightConfiguration::new(
                tree_sitter_ocaml::LANGUAGE_OCAML.into(),
                "ocaml",
                tree_sitter_ocaml::HIGHLIGHTS_QUERY,
                "",
                tree_sitter_ocaml::LOCALS_QUERY,
            )
            .unwrap(),
        );
        // #[cfg(feature = "ocaml")]
        // self.add_language(
        //     &["ocaml_interface"],
        //     HighlightConfiguration::new(
        //         tree_sitter_ocaml::LANGUAGE_OCAML_INTERFACE.into(),
        //         "ocaml_interface",
        //         tree_sitter_ocaml::HIGHLIGHTS_QUERY,
        //         "",
        //         tree_sitter_ocaml::LOCALS_QUERY,
        //     )
        //     .unwrap(),
        // );
        // #[cfg(feature = "ocaml")]
        // self.add_language(
        //     &["ocaml_type"],
        //     HighlightConfiguration::new(
        //         tree_sitter_ocaml::LANGUAGE_OCAML_TYPE.into(),
        //         "ocaml_type",
        //         tree_sitter_ocaml::HIGHLIGHTS_QUERY,
        //         "",
        //         tree_sitter_ocaml::LOCALS_QUERY,
        //     )
        //     .unwrap(),
        // );
        #[cfg(feature = "php")]
        self.add_language(
            &["php"],
            HighlightConfiguration::new(
                tree_sitter_php::LANGUAGE_PHP.into(),
                "php",
                tree_sitter_php::HIGHLIGHTS_QUERY,
                &[
                    tree_sitter_php::INJECTIONS_QUERY,
                    include_str!("../queries/php/injections-text.scm"),
                ]
                .join("\n"),
                "",
            )
            .unwrap(),
        );
        #[cfg(feature = "php")]
        self.add_language(
            &["php_only"],
            HighlightConfiguration::new(
                tree_sitter_php::LANGUAGE_PHP_ONLY.into(),
                "php_only",
                tree_sitter_php::HIGHLIGHTS_QUERY,
                tree_sitter_php::INJECTIONS_QUERY,
                "",
            )
            .unwrap(),
        );
        #[cfg(feature = "python")]
        self.add_language(
            &["python", "py"],
            HighlightConfiguration::new(
                tree_sitter_python::LANGUAGE.into(),
                "python",
                tree_sitter_python::HIGHLIGHTS_QUERY,
                "",
                "",
            )
            .unwrap(),
        );
        #[cfg(feature = "regex")]
        self.add_language(
            &["regexp", "regex"],
            HighlightConfiguration::new(
                tree_sitter_regex::LANGUAGE.into(),
                "regex",
                tree_sitter_regex::HIGHLIGHTS_QUERY,
                "",
                "",
            )
            .unwrap(),
        );
        #[cfg(feature = "ruby")]
        self.add_language(
            &["ruby", "rb"],
            HighlightConfiguration::new(
                tree_sitter_ruby::LANGUAGE.into(),
                "ruby",
                tree_sitter_ruby::HIGHLIGHTS_QUERY,
                "",
                tree_sitter_ruby::LOCALS_QUERY,
            )
            .unwrap(),
        );
        #[cfg(feature = "rust")]
        self.add_language(
            &["rust", "rs"],
            HighlightConfiguration::new(
                tree_sitter_rust::LANGUAGE.into(),
                "rust",
                tree_sitter_rust::HIGHLIGHTS_QUERY,
                tree_sitter_rust::INJECTIONS_QUERY,
                "",
            )
            .unwrap(),
        );
        #[cfg(feature = "scala")]
        self.add_language(
            &["scala"],
            HighlightConfiguration::new(
                tree_sitter_scala::LANGUAGE.into(),
                "scala",
                tree_sitter_scala::HIGHLIGHTS_QUERY,
                "",
                tree_sitter_scala::LOCALS_QUERY,
            )
            .unwrap(),
        );
        #[cfg(feature = "typescript")]
        self.add_language(
            &["typescript", "ts"],
            HighlightConfiguration::new(
                tree_sitter_typescript::LANGUAGE_TYPESCRIPT.into(),
                "typescript",
                &[
                    tree_sitter_typescript::HIGHLIGHTS_QUERY,
                    tree_sitter_javascript::HIGHLIGHT_QUERY,
                ]
                .join("\n"),
                tree_sitter_javascript::INJECTIONS_QUERY,
                &[
                    tree_sitter_typescript::LOCALS_QUERY,
                    tree_sitter_javascript::LOCALS_QUERY,
                ]
                .join("\n"),
            )
            .unwrap(),
        );
        #[cfg(feature = "typescript")]
        self.add_language(
            &["tsx"],
            HighlightConfiguration::new(
                tree_sitter_typescript::LANGUAGE_TSX.into(),
                "tsx",
                &[
                    tree_sitter_typescript::HIGHLIGHTS_QUERY,
                    tree_sitter_javascript::JSX_HIGHLIGHT_QUERY,
                    tree_sitter_javascript::HIGHLIGHT_QUERY,
                ]
                .join("\n"),
                tree_sitter_javascript::INJECTIONS_QUERY,
                &[
                    tree_sitter_typescript::LOCALS_QUERY,
                    tree_sitter_javascript::LOCALS_QUERY,
                ]
                .join("\n"),
            )
            .unwrap(),
        );
    }
}
