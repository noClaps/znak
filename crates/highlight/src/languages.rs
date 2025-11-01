use std::sync::Arc;

use tree_sitter_highlight::HighlightConfiguration;

use crate::Highlight;

macro_rules! add_lang {
    ($self:expr, [$name:expr$(,$alias:expr)*], $lang:expr, $hl:expr, $inj:expr, $loc:expr) => {
        $self.add_language(
            &[$name$(,$alias)*],
            tree_sitter_highlight::HighlightConfiguration::new(
                $lang.into(),
                $name,
                $hl,
                $inj,
                $loc,
            ).unwrap()
        )
    };
    ($self:expr, [$name:expr$(,$alias:expr)*], $lang:expr, $hl:expr, inj=$inj:expr) => {
        $self.add_language(
            &[$name$(,$alias)*],
            tree_sitter_highlight::HighlightConfiguration::new(
                $lang.into(),
                $name,
                $hl,
                $inj,
                "",
            ).unwrap()
        )
    };
    ($self:expr, [$name:expr$(,$alias:expr)*], $lang:expr, $hl:expr, loc=$loc:expr) => {
        $self.add_language(
            &[$name$(,$alias)*],
            tree_sitter_highlight::HighlightConfiguration::new(
                $lang.into(),
                $name,
                $hl,
                "",
                $loc,
            ).unwrap()
        )
    };
    ($self:expr, [$name:expr$(,$alias:expr)*], $lang:expr, $hl:expr) => {
        $self.add_language(
            &[$name$(,$alias)*],
            tree_sitter_highlight::HighlightConfiguration::new(
                $lang.into(),
                $name,
                $hl,
                "",
                "",
            ).unwrap()
        )
    };
}

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
        for &name in names.as_ref() {
            self.languages.insert(name.into(), config.clone());
        }
    }

    pub(crate) fn default_langs(&mut self) {
        #[cfg(feature = "agda")]
        add_lang!(
            self,
            ["agda"],
            tree_sitter_agda::LANGUAGE,
            tree_sitter_agda::HIGHLIGHTS_QUERY
        );
        #[cfg(feature = "bash")]
        add_lang!(
            self,
            ["bash", "shellscript", "shell", "zsh", "sh"],
            tree_sitter_bash::LANGUAGE,
            tree_sitter_bash::HIGHLIGHT_QUERY
        );
        #[cfg(feature = "c")]
        add_lang!(
            self,
            ["c"],
            tree_sitter_c::LANGUAGE,
            tree_sitter_c::HIGHLIGHT_QUERY
        );
        #[cfg(feature = "cpp")]
        add_lang!(
            self,
            ["cpp", "c++"],
            tree_sitter_cpp::LANGUAGE,
            &[
                tree_sitter_c::HIGHLIGHT_QUERY,
                tree_sitter_cpp::HIGHLIGHT_QUERY
            ]
            .join("\n"),
            inj = include_str!("../queries/cpp/injections.scm")
        );
        #[cfg(feature = "css")]
        add_lang!(
            self,
            ["css"],
            tree_sitter_css::LANGUAGE,
            tree_sitter_css::HIGHLIGHTS_QUERY
        );
        #[cfg(feature = "go")]
        add_lang!(
            self,
            ["go"],
            tree_sitter_go::LANGUAGE,
            tree_sitter_go::HIGHLIGHTS_QUERY
        );
        #[cfg(feature = "haskell")]
        add_lang!(
            self,
            ["haskell", "hs"],
            tree_sitter_haskell::LANGUAGE,
            tree_sitter_haskell::HIGHLIGHTS_QUERY,
            tree_sitter_haskell::INJECTIONS_QUERY,
            tree_sitter_haskell::LOCALS_QUERY
        );
        #[cfg(feature = "html")]
        add_lang!(
            self,
            ["html"],
            tree_sitter_html::LANGUAGE,
            tree_sitter_html::HIGHLIGHTS_QUERY,
            inj = tree_sitter_html::INJECTIONS_QUERY
        );
        #[cfg(feature = "java")]
        add_lang!(
            self,
            ["java"],
            tree_sitter_java::LANGUAGE,
            tree_sitter_java::HIGHLIGHTS_QUERY
        );
        #[cfg(feature = "javascript")]
        add_lang!(
            self,
            ["javascript", "js", "jsx"],
            tree_sitter_javascript::LANGUAGE,
            &[
                tree_sitter_javascript::HIGHLIGHT_QUERY,
                tree_sitter_javascript::JSX_HIGHLIGHT_QUERY,
                include_str!("../queries/javascript/highlights-params.scm")
            ]
            .join("\n"),
            tree_sitter_javascript::INJECTIONS_QUERY,
            tree_sitter_javascript::LOCALS_QUERY
        );
        #[cfg(feature = "jsdoc")]
        add_lang!(
            self,
            ["jsdoc"],
            tree_sitter_jsdoc::LANGUAGE,
            tree_sitter_jsdoc::HIGHLIGHTS_QUERY
        );
        #[cfg(feature = "json")]
        add_lang!(
            self,
            ["json"],
            tree_sitter_json::LANGUAGE,
            tree_sitter_json::HIGHLIGHTS_QUERY
        );
        #[cfg(feature = "julia")]
        add_lang!(
            self,
            ["julia", "jl"],
            tree_sitter_julia::LANGUAGE,
            include_str!("../queries/julia/highlights.scm"),
            loc = include_str!("../queries/julia/locals.scm")
        );
        #[cfg(feature = "ocaml")]
        add_lang!(
            self,
            ["ocaml"],
            tree_sitter_ocaml::LANGUAGE_OCAML,
            tree_sitter_ocaml::HIGHLIGHTS_QUERY,
            loc = tree_sitter_ocaml::LOCALS_QUERY
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
        add_lang!(
            self,
            ["php"],
            tree_sitter_php::LANGUAGE_PHP,
            tree_sitter_php::HIGHLIGHTS_QUERY,
            inj = &[
                tree_sitter_php::INJECTIONS_QUERY,
                include_str!("../queries/php/injections-text.scm")
            ]
            .join("\n")
        );
        #[cfg(feature = "php")]
        add_lang!(
            self,
            ["php_only"],
            tree_sitter_php::LANGUAGE_PHP_ONLY,
            tree_sitter_php::HIGHLIGHTS_QUERY,
            inj = tree_sitter_php::INJECTIONS_QUERY
        );
        #[cfg(feature = "python")]
        add_lang!(
            self,
            ["python", "py"],
            tree_sitter_python::LANGUAGE,
            tree_sitter_python::HIGHLIGHTS_QUERY
        );
        #[cfg(feature = "regex")]
        add_lang!(
            self,
            ["regex", "regexp"],
            tree_sitter_regex::LANGUAGE,
            tree_sitter_regex::HIGHLIGHTS_QUERY
        );
        #[cfg(feature = "ruby")]
        add_lang!(
            self,
            ["ruby", "rb"],
            tree_sitter_ruby::LANGUAGE,
            tree_sitter_ruby::HIGHLIGHTS_QUERY,
            loc = tree_sitter_ruby::LOCALS_QUERY
        );
        #[cfg(feature = "rust")]
        add_lang!(
            self,
            ["rust", "rs"],
            tree_sitter_rust::LANGUAGE,
            tree_sitter_rust::HIGHLIGHTS_QUERY,
            inj = tree_sitter_rust::INJECTIONS_QUERY
        );
        #[cfg(feature = "scala")]
        add_lang!(
            self,
            ["scala"],
            tree_sitter_scala::LANGUAGE,
            tree_sitter_scala::HIGHLIGHTS_QUERY,
            loc = tree_sitter_scala::LOCALS_QUERY
        );
        #[cfg(feature = "typescript")]
        add_lang!(
            self,
            ["typescript", "ts"],
            tree_sitter_typescript::LANGUAGE_TYPESCRIPT,
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
            .join("\n")
        );
        #[cfg(feature = "typescript")]
        add_lang!(
            self,
            ["tsx"],
            tree_sitter_typescript::LANGUAGE_TSX,
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
            .join("\n")
        );
    }
}
