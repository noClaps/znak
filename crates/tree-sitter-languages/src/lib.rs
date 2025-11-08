#![doc = include_str!("../README.md")]

#[cfg(feature = "agda")]
pub mod agda {
    use tree_sitter_highlight::HighlightConfiguration;
    use tree_sitter_language::LanguageFn;

    pub const GRAMMAR: LanguageFn = tree_sitter_agda::LANGUAGE;
    pub const HIGHLIGHTS: &str = tree_sitter_agda::HIGHLIGHTS_QUERY;
    pub fn highlight_configuration() -> HighlightConfiguration {
        HighlightConfiguration::new(GRAMMAR.into(), "agda", HIGHLIGHTS, "", "").unwrap()
    }
}
#[cfg(feature = "bash")]
pub mod bash {
    use tree_sitter_highlight::HighlightConfiguration;
    use tree_sitter_language::LanguageFn;

    pub const GRAMMAR: LanguageFn = tree_sitter_bash::LANGUAGE;
    pub const HIGHLIGHTS: &str = tree_sitter_bash::HIGHLIGHT_QUERY;
    pub fn highlight_configuration() -> HighlightConfiguration {
        HighlightConfiguration::new(GRAMMAR.into(), "bash", HIGHLIGHTS, "", "").unwrap()
    }
}
#[cfg(feature = "c")]
pub mod c {
    use tree_sitter_highlight::HighlightConfiguration;
    use tree_sitter_language::LanguageFn;

    pub const GRAMMAR: LanguageFn = tree_sitter_c::LANGUAGE;
    pub const HIGHLIGHTS: &str = tree_sitter_c::HIGHLIGHT_QUERY;
    pub fn highlight_configuration() -> HighlightConfiguration {
        HighlightConfiguration::new(GRAMMAR.into(), "c", HIGHLIGHTS, "", "").unwrap()
    }
}
#[cfg(feature = "cpp")]
pub mod cpp {
    use constcat::concat;
    use tree_sitter_highlight::HighlightConfiguration;
    use tree_sitter_language::LanguageFn;

    pub const GRAMMAR: LanguageFn = tree_sitter_cpp::LANGUAGE;
    pub const HIGHLIGHTS: &str = concat!(
        tree_sitter_c::HIGHLIGHT_QUERY,
        "\n",
        tree_sitter_cpp::HIGHLIGHT_QUERY
    );
    pub const INJECTIONS: &str = include_str!("../queries/cpp/injections.scm");
    pub fn highlight_configuration() -> HighlightConfiguration {
        HighlightConfiguration::new(GRAMMAR.into(), "cpp", HIGHLIGHTS, INJECTIONS, "").unwrap()
    }
}
#[cfg(feature = "css")]
pub mod css {
    use tree_sitter_highlight::HighlightConfiguration;
    use tree_sitter_language::LanguageFn;

    pub const GRAMMAR: LanguageFn = tree_sitter_css::LANGUAGE;
    pub const HIGHLIGHTS: &str = tree_sitter_css::HIGHLIGHTS_QUERY;
    pub fn highlight_configuration() -> HighlightConfiguration {
        HighlightConfiguration::new(GRAMMAR.into(), "css", HIGHLIGHTS, "", "").unwrap()
    }
}
#[cfg(feature = "go")]
pub mod go {
    use tree_sitter_highlight::HighlightConfiguration;
    use tree_sitter_language::LanguageFn;

    pub const GRAMMAR: LanguageFn = tree_sitter_go::LANGUAGE;
    pub const HIGHLIGHTS: &str = tree_sitter_go::HIGHLIGHTS_QUERY;
    pub fn highlight_configuration() -> HighlightConfiguration {
        HighlightConfiguration::new(GRAMMAR.into(), "go", HIGHLIGHTS, "", "").unwrap()
    }
}
#[cfg(feature = "haskell")]
pub mod haskell {
    use tree_sitter_highlight::HighlightConfiguration;
    use tree_sitter_language::LanguageFn;

    pub const GRAMMAR: LanguageFn = tree_sitter_haskell::LANGUAGE;
    pub const HIGHLIGHTS: &str = tree_sitter_haskell::HIGHLIGHTS_QUERY;
    pub const INJECTIONS: &str = tree_sitter_haskell::INJECTIONS_QUERY;
    pub const LOCALS: &str = tree_sitter_haskell::LOCALS_QUERY;
    pub fn highlight_configuration() -> HighlightConfiguration {
        HighlightConfiguration::new(GRAMMAR.into(), "haskell", HIGHLIGHTS, INJECTIONS, LOCALS)
            .unwrap()
    }
}
#[cfg(feature = "html")]
pub mod html {
    use tree_sitter_highlight::HighlightConfiguration;
    use tree_sitter_language::LanguageFn;

    pub const GRAMMAR: LanguageFn = tree_sitter_html::LANGUAGE;
    pub const HIGHLIGHTS: &str = tree_sitter_html::HIGHLIGHTS_QUERY;
    pub const INJECTIONS: &str = tree_sitter_html::INJECTIONS_QUERY;
    pub fn highlight_configuration() -> HighlightConfiguration {
        HighlightConfiguration::new(GRAMMAR.into(), "html", HIGHLIGHTS, INJECTIONS, "").unwrap()
    }
}
#[cfg(feature = "java")]
pub mod java {
    use tree_sitter_highlight::HighlightConfiguration;
    use tree_sitter_language::LanguageFn;

    pub const GRAMMAR: LanguageFn = tree_sitter_java::LANGUAGE;
    pub const HIGHLIGHTS: &str = tree_sitter_java::HIGHLIGHTS_QUERY;
    pub fn highlight_configuration() -> HighlightConfiguration {
        HighlightConfiguration::new(GRAMMAR.into(), "java", HIGHLIGHTS, "", "").unwrap()
    }
}
#[cfg(feature = "javascript")]
pub mod javascript {
    use constcat::concat;
    use tree_sitter_highlight::HighlightConfiguration;
    use tree_sitter_language::LanguageFn;

    pub const GRAMMAR: LanguageFn = tree_sitter_javascript::LANGUAGE;
    pub const HIGHLIGHTS: &str = concat!(
        tree_sitter_javascript::HIGHLIGHT_QUERY,
        "\n",
        tree_sitter_javascript::JSX_HIGHLIGHT_QUERY,
        "\n",
        include_str!("../queries/javascript/highlights-params.scm")
    );
    pub const INJECTIONS: &str = tree_sitter_javascript::INJECTIONS_QUERY;
    pub const LOCALS: &str = tree_sitter_javascript::LOCALS_QUERY;
    pub fn highlight_configuration() -> HighlightConfiguration {
        HighlightConfiguration::new(GRAMMAR.into(), "javascript", HIGHLIGHTS, INJECTIONS, LOCALS)
            .unwrap()
    }
}
#[cfg(feature = "jsdoc")]
pub mod jsdoc {
    use tree_sitter_highlight::HighlightConfiguration;
    use tree_sitter_language::LanguageFn;

    pub const GRAMMAR: LanguageFn = tree_sitter_jsdoc::LANGUAGE;
    pub const HIGHLIGHTS: &str = tree_sitter_jsdoc::HIGHLIGHTS_QUERY;
    pub fn highlight_configuration() -> HighlightConfiguration {
        HighlightConfiguration::new(GRAMMAR.into(), "jsdoc", HIGHLIGHTS, "", "").unwrap()
    }
}
#[cfg(feature = "json")]
pub mod json {
    use tree_sitter_highlight::HighlightConfiguration;
    use tree_sitter_language::LanguageFn;

    pub const GRAMMAR: LanguageFn = tree_sitter_json::LANGUAGE;
    pub const HIGHLIGHTS: &str = tree_sitter_json::HIGHLIGHTS_QUERY;
    pub fn highlight_configuration() -> HighlightConfiguration {
        HighlightConfiguration::new(GRAMMAR.into(), "json", HIGHLIGHTS, "", "").unwrap()
    }
}
#[cfg(feature = "julia")]
pub mod julia {
    use tree_sitter_highlight::HighlightConfiguration;
    use tree_sitter_language::LanguageFn;

    pub const GRAMMAR: LanguageFn = tree_sitter_julia::LANGUAGE;
    pub const HIGHLIGHTS: &str = include_str!("../queries/julia/highlights.scm");
    pub const LOCALS: &str = include_str!("../queries/julia/locals.scm");
    pub fn highlight_configuration() -> HighlightConfiguration {
        HighlightConfiguration::new(GRAMMAR.into(), "julia", HIGHLIGHTS, "", LOCALS).unwrap()
    }
}
#[cfg(feature = "ocaml")]
pub mod ocaml {
    use tree_sitter_highlight::HighlightConfiguration;
    use tree_sitter_language::LanguageFn;

    pub const GRAMMAR_OCAML: LanguageFn = tree_sitter_ocaml::LANGUAGE_OCAML;
    pub const GRAMMAR_OCAML_INTERFACE: LanguageFn = tree_sitter_ocaml::LANGUAGE_OCAML_INTERFACE;
    pub const GRAMMAR_OCAML_TYPE: LanguageFn = tree_sitter_ocaml::LANGUAGE_OCAML_TYPE;
    pub const HIGHLIGHTS: &str = tree_sitter_ocaml::HIGHLIGHTS_QUERY;
    pub const LOCALS: &str = tree_sitter_ocaml::LOCALS_QUERY;
    pub fn highlight_configuration_ocaml() -> HighlightConfiguration {
        HighlightConfiguration::new(GRAMMAR_OCAML.into(), "ocaml", HIGHLIGHTS, "", LOCALS).unwrap()
    }
    pub fn highlight_configuration_ocaml_interface() -> HighlightConfiguration {
        HighlightConfiguration::new(
            GRAMMAR_OCAML_INTERFACE.into(),
            "ocaml_interface",
            HIGHLIGHTS,
            "",
            LOCALS,
        )
        .unwrap()
    }
    pub fn highlight_configuration_ocaml_type() -> HighlightConfiguration {
        HighlightConfiguration::new(
            GRAMMAR_OCAML_TYPE.into(),
            "ocaml_type",
            HIGHLIGHTS,
            "",
            LOCALS,
        )
        .unwrap()
    }
}
#[cfg(feature = "php")]
pub mod php {
    use constcat::concat;
    use tree_sitter_highlight::HighlightConfiguration;
    use tree_sitter_language::LanguageFn;

    pub const GRAMMAR_PHP: LanguageFn = tree_sitter_php::LANGUAGE_PHP;
    pub const GRAMMAR_PHP_ONLY: LanguageFn = tree_sitter_php::LANGUAGE_PHP_ONLY;
    pub const HIGHLIGHTS: &str = tree_sitter_php::HIGHLIGHTS_QUERY;
    pub const INJECTIONS_PHP: &str = concat!(
        tree_sitter_php::INJECTIONS_QUERY,
        "\n",
        include_str!("../queries/php/injections-text.scm")
    );
    pub const INJECTIONS_PHP_ONLY: &str = tree_sitter_php::INJECTIONS_QUERY;
    pub fn highlight_configuration_php() -> HighlightConfiguration {
        HighlightConfiguration::new(GRAMMAR_PHP.into(), "php", HIGHLIGHTS, INJECTIONS_PHP, "")
            .unwrap()
    }
    pub fn highlight_configuration_php_only() -> HighlightConfiguration {
        HighlightConfiguration::new(
            GRAMMAR_PHP_ONLY.into(),
            "php_only",
            HIGHLIGHTS,
            INJECTIONS_PHP_ONLY,
            "",
        )
        .unwrap()
    }
}
#[cfg(feature = "python")]
pub mod python {
    use tree_sitter_highlight::HighlightConfiguration;
    use tree_sitter_language::LanguageFn;

    pub const GRAMMAR: LanguageFn = tree_sitter_python::LANGUAGE;
    pub const HIGHLIGHTS: &str = tree_sitter_python::HIGHLIGHTS_QUERY;
    pub fn highlight_configuration() -> HighlightConfiguration {
        HighlightConfiguration::new(GRAMMAR.into(), "python", HIGHLIGHTS, "", "").unwrap()
    }
}
#[cfg(feature = "regex")]
pub mod regex {
    use tree_sitter_highlight::HighlightConfiguration;
    use tree_sitter_language::LanguageFn;

    pub const GRAMMAR: LanguageFn = tree_sitter_regex::LANGUAGE;
    pub const HIGHLIGHTS: &str = tree_sitter_regex::HIGHLIGHTS_QUERY;
    pub fn highlight_configuration() -> HighlightConfiguration {
        HighlightConfiguration::new(GRAMMAR.into(), "regex", HIGHLIGHTS, "", "").unwrap()
    }
}
#[cfg(feature = "ruby")]
pub mod ruby {
    use tree_sitter_highlight::HighlightConfiguration;
    use tree_sitter_language::LanguageFn;

    pub const GRAMMAR: LanguageFn = tree_sitter_ruby::LANGUAGE;
    pub const HIGHLIGHTS: &str = tree_sitter_ruby::HIGHLIGHTS_QUERY;
    pub const LOCALS: &str = tree_sitter_ruby::LOCALS_QUERY;
    pub fn highlight_configuration() -> HighlightConfiguration {
        HighlightConfiguration::new(GRAMMAR.into(), "ruby", HIGHLIGHTS, "", LOCALS).unwrap()
    }
}
#[cfg(feature = "rust")]
pub mod rust {
    use tree_sitter_highlight::HighlightConfiguration;
    use tree_sitter_language::LanguageFn;

    pub const GRAMMAR: LanguageFn = tree_sitter_rust::LANGUAGE;
    pub const HIGHLIGHTS: &str = tree_sitter_rust::HIGHLIGHTS_QUERY;
    pub const INJECTIONS: &str = tree_sitter_rust::INJECTIONS_QUERY;
    pub fn highlight_configuration() -> HighlightConfiguration {
        HighlightConfiguration::new(GRAMMAR.into(), "rust", HIGHLIGHTS, INJECTIONS, "").unwrap()
    }
}
#[cfg(feature = "scala")]
pub mod scala {
    use tree_sitter_highlight::HighlightConfiguration;
    use tree_sitter_language::LanguageFn;

    pub const GRAMMAR: LanguageFn = tree_sitter_scala::LANGUAGE;
    pub const HIGHLIGHTS: &str = tree_sitter_scala::HIGHLIGHTS_QUERY;
    pub const LOCALS: &str = tree_sitter_scala::LOCALS_QUERY;
    pub fn highlight_configuration() -> HighlightConfiguration {
        HighlightConfiguration::new(GRAMMAR.into(), "scala", HIGHLIGHTS, "", LOCALS).unwrap()
    }
}
#[cfg(feature = "typescript")]
pub mod typescript {
    use constcat::concat;
    use tree_sitter_highlight::HighlightConfiguration;
    use tree_sitter_language::LanguageFn;

    pub const GRAMMAR_TYPESCRIPT: LanguageFn = tree_sitter_typescript::LANGUAGE_TYPESCRIPT;
    pub const GRAMMAR_TSX: LanguageFn = tree_sitter_typescript::LANGUAGE_TSX;
    pub const HIGHLIGHTS_TYPESCRIPT: &str = concat!(
        tree_sitter_typescript::HIGHLIGHTS_QUERY,
        "\n",
        tree_sitter_javascript::HIGHLIGHT_QUERY
    );
    pub const HIGHLIGHTS_TSX: &str = concat!(
        tree_sitter_typescript::HIGHLIGHTS_QUERY,
        "\n",
        tree_sitter_javascript::JSX_HIGHLIGHT_QUERY,
        "\n",
        tree_sitter_javascript::HIGHLIGHT_QUERY
    );
    pub const INJECTIONS: &str = tree_sitter_javascript::INJECTIONS_QUERY;
    pub const LOCALS: &str = concat!(
        tree_sitter_typescript::LOCALS_QUERY,
        "\n",
        tree_sitter_javascript::LOCALS_QUERY
    );
    pub fn highlight_configuration_typescript() -> HighlightConfiguration {
        HighlightConfiguration::new(
            GRAMMAR_TYPESCRIPT.into(),
            "typescript",
            HIGHLIGHTS_TYPESCRIPT,
            INJECTIONS,
            LOCALS,
        )
        .unwrap()
    }
    pub fn highlight_configuration_tsx() -> HighlightConfiguration {
        HighlightConfiguration::new(
            GRAMMAR_TSX.into(),
            "typescript",
            HIGHLIGHTS_TSX,
            INJECTIONS,
            LOCALS,
        )
        .unwrap()
    }
}
