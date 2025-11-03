use std::sync::Arc;

use tree_sitter_highlight::HighlightConfiguration;
#[cfg(feature = "ocaml")]
use tree_sitter_languages::ocaml;
#[cfg(feature = "php")]
use tree_sitter_languages::php;
#[cfg(feature = "typescript")]
use tree_sitter_languages::typescript;

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

    pub(crate) fn default_langs(&mut self) {
        macro_rules! add_lang {
            ([$name:literal$(,$alias:literal)*], $lang:expr, $hl:expr, $inj:expr, $loc:expr) => {
                self.add_language(
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
            ($name:ident$(,$alias:literal)*, inj, loc) => {
                let name = stringify!($name);
                self.add_language(
                    &[name$(,$alias)*],
                    tree_sitter_highlight::HighlightConfiguration::new(
                        tree_sitter_languages::$name::GRAMMAR.into(),
                        name,
                        tree_sitter_languages::$name::HIGHLIGHTS,
                        tree_sitter_languages::$name::INJECTIONS,
                        tree_sitter_languages::$name::LOCALS,
                    ).unwrap()
                )
            };
            ($name:ident$(,$alias:literal)*, inj) => {
                let name = stringify!($name);
                self.add_language(
                    &[name$(,$alias)*],
                    tree_sitter_highlight::HighlightConfiguration::new(
                        tree_sitter_languages::$name::GRAMMAR.into(),
                        name,
                        tree_sitter_languages::$name::HIGHLIGHTS,
                        tree_sitter_languages::$name::INJECTIONS,
                        "",
                    ).unwrap()
                )
            };
            ($name:ident$(,$alias:literal)*, loc) => {
                let name = stringify!($name);
                self.add_language(
                    &[name$(,$alias)*],
                    tree_sitter_highlight::HighlightConfiguration::new(
                        tree_sitter_languages::$name::GRAMMAR.into(),
                        name,
                        tree_sitter_languages::$name::HIGHLIGHTS,
                        "",
                        tree_sitter_languages::$name::LOCALS,
                    ).unwrap()
                )
            };
            ($name:ident$(,$alias:literal)*) => {
                let name = stringify!($name);
                self.add_language(
                    &[name$(,$alias)*],
                    tree_sitter_highlight::HighlightConfiguration::new(
                        tree_sitter_languages::$name::GRAMMAR.into(),
                        name,
                        tree_sitter_languages::$name::HIGHLIGHTS,
                        "",
                        "",
                    ).unwrap()
                )
            };
        }

        #[cfg(feature = "agda")]
        add_lang!(agda);
        #[cfg(feature = "bash")]
        add_lang!(bash, "shellscript", "shell", "zsh", "sh");
        #[cfg(feature = "c")]
        add_lang!(c);
        #[cfg(feature = "cpp")]
        add_lang!(cpp, "c++", inj);
        #[cfg(feature = "css")]
        add_lang!(css);
        #[cfg(feature = "go")]
        add_lang!(go);
        #[cfg(feature = "haskell")]
        add_lang!(haskell, "hs", inj, loc);
        #[cfg(feature = "html")]
        add_lang!(html, inj);
        #[cfg(feature = "java")]
        add_lang!(java);
        #[cfg(feature = "javascript")]
        add_lang!(javascript, "js", "jsx", inj, loc);
        #[cfg(feature = "jsdoc")]
        add_lang!(jsdoc);
        #[cfg(feature = "json")]
        add_lang!(json);
        #[cfg(feature = "julia")]
        add_lang!(julia, "jl", loc);
        #[cfg(feature = "ocaml")]
        add_lang!(
            ["ocaml"],
            ocaml::GRAMMAR_OCAML,
            ocaml::HIGHLIGHTS,
            "",
            ocaml::LOCALS
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
            ["php"],
            php::GRAMMAR_PHP,
            php::HIGHLIGHTS,
            php::INJECTIONS_PHP,
            ""
        );
        #[cfg(feature = "php")]
        add_lang!(
            ["php_only"],
            php::GRAMMAR_PHP_ONLY,
            php::HIGHLIGHTS,
            php::INJECTIONS_PHP_ONLY,
            ""
        );
        #[cfg(feature = "python")]
        add_lang!(python, "py");
        #[cfg(feature = "regex")]
        add_lang!(regex, "regexp");
        #[cfg(feature = "ruby")]
        add_lang!(ruby, "rb", loc);
        #[cfg(feature = "rust")]
        add_lang!(rust, "rs", inj);
        #[cfg(feature = "scala")]
        add_lang!(scala, loc);
        #[cfg(feature = "typescript")]
        add_lang!(
            ["typescript", "ts"],
            typescript::GRAMMAR_TYPESCRIPT,
            typescript::HIGHLIGHTS_TYPESCRIPT,
            typescript::INJECTIONS,
            typescript::LOCALS
        );
        #[cfg(feature = "typescript")]
        add_lang!(
            ["tsx"],
            typescript::GRAMMAR_TSX,
            typescript::HIGHLIGHTS_TSX,
            typescript::INJECTIONS,
            typescript::LOCALS
        );
    }
}
