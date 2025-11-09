#![doc = include_str!("../README.md")]

#[cfg(feature = "agda")]
pub mod agda {
    use tree_sitter_highlight::HighlightConfiguration;
    use tree_sitter_language::LanguageFn;

    pub const GRAMMAR: LanguageFn = tree_sitter_agda::LANGUAGE;
    pub const HIGHLIGHTS: &str = include_str!("../queries/agda/highlights.scm");
    pub fn highlight_configuration() -> HighlightConfiguration {
        HighlightConfiguration::new(GRAMMAR.into(), "agda", HIGHLIGHTS, "", "").unwrap()
    }
}
#[cfg(feature = "bash")]
pub mod bash {
    use tree_sitter_highlight::HighlightConfiguration;
    use tree_sitter_language::LanguageFn;

    pub const GRAMMAR: LanguageFn = tree_sitter_bash::LANGUAGE;
    pub const HIGHLIGHTS: &str = include_str!("../queries/bash/highlights.scm");
    pub const INJECTIONS: &str = include_str!("../queries/bash/injections.scm");
    pub fn highlight_configuration() -> HighlightConfiguration {
        HighlightConfiguration::new(GRAMMAR.into(), "bash", HIGHLIGHTS, INJECTIONS, "").unwrap()
    }
}
#[cfg(feature = "c")]
pub mod c {
    use tree_sitter_highlight::HighlightConfiguration;
    use tree_sitter_language::LanguageFn;

    pub const GRAMMAR: LanguageFn = tree_sitter_c::LANGUAGE;
    pub const HIGHLIGHTS: &str = include_str!("../queries/c/highlights.scm");
    pub const INJECTIONS: &str = include_str!("../queries/c/injections.scm");
    pub const LOCALS: &str = include_str!("../queries/c/locals.scm");
    pub fn highlight_configuration() -> HighlightConfiguration {
        HighlightConfiguration::new(GRAMMAR.into(), "c", HIGHLIGHTS, INJECTIONS, LOCALS).unwrap()
    }
}
#[cfg(feature = "cpp")]
pub mod cpp {
    use tree_sitter_highlight::HighlightConfiguration;
    use tree_sitter_language::LanguageFn;

    pub const GRAMMAR: LanguageFn = tree_sitter_cpp::LANGUAGE;
    pub const HIGHLIGHTS: &str = include_str!("../queries/cpp/highlights.scm");
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
    pub const HIGHLIGHTS: &str = include_str!("../queries/css/highlights.scm");
    pub const INJECTIONS: &str = include_str!("../queries/css/injections.scm");
    pub fn highlight_configuration() -> HighlightConfiguration {
        HighlightConfiguration::new(GRAMMAR.into(), "css", HIGHLIGHTS, INJECTIONS, "").unwrap()
    }
}
#[cfg(feature = "go")]
pub mod go {
    use tree_sitter_highlight::HighlightConfiguration;
    use tree_sitter_language::LanguageFn;

    pub const GRAMMAR: LanguageFn = tree_sitter_go::LANGUAGE;
    pub const HIGHLIGHTS: &str = include_str!("../queries/go/highlights.scm");
    pub const INJECTIONS: &str = include_str!("../queries/go/injections.scm");
    pub const LOCALS: &str = include_str!("../queries/go/locals.scm");
    pub fn highlight_configuration() -> HighlightConfiguration {
        HighlightConfiguration::new(GRAMMAR.into(), "go", HIGHLIGHTS, INJECTIONS, LOCALS).unwrap()
    }
}
#[cfg(feature = "haskell")]
pub mod haskell {
    use tree_sitter_highlight::HighlightConfiguration;
    use tree_sitter_language::LanguageFn;

    pub const GRAMMAR: LanguageFn = tree_sitter_haskell::LANGUAGE;
    pub const HIGHLIGHTS: &str = include_str!("../queries/haskell/highlights.scm");
    pub const INJECTIONS: &str = include_str!("../queries/haskell/injections.scm");
    pub const LOCALS: &str = include_str!("../queries/haskell/locals.scm");
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
    pub const HIGHLIGHTS: &str = include_str!("../queries/html/highlights.scm");
    pub const INJECTIONS: &str = include_str!("../queries/html/injections.scm");
    pub fn highlight_configuration() -> HighlightConfiguration {
        HighlightConfiguration::new(GRAMMAR.into(), "html", HIGHLIGHTS, INJECTIONS, "").unwrap()
    }
}
#[cfg(feature = "java")]
pub mod java {
    use tree_sitter_highlight::HighlightConfiguration;
    use tree_sitter_language::LanguageFn;

    pub const GRAMMAR: LanguageFn = tree_sitter_java::LANGUAGE;
    pub const HIGHLIGHTS: &str = include_str!("../queries/java/highlights.scm");
    pub const INJECTIONS: &str = include_str!("../queries/java/injections.scm");
    pub fn highlight_configuration() -> HighlightConfiguration {
        HighlightConfiguration::new(GRAMMAR.into(), "java", HIGHLIGHTS, INJECTIONS, "").unwrap()
    }
}
#[cfg(feature = "javascript")]
pub mod javascript {
    use tree_sitter_highlight::HighlightConfiguration;
    use tree_sitter_language::LanguageFn;

    pub const GRAMMAR: LanguageFn = tree_sitter_javascript::LANGUAGE;

    pub const HIGHLIGHTS_JAVASCRIPT: &str = include_str!("../queries/javascript/highlights.scm");
    pub const INJECTIONS_JAVASCRIPT: &str = include_str!("../queries/javascript/injections.scm");
    pub const LOCALS_JAVASCRIPT: &str = include_str!("../queries/javascript/locals.scm");
    pub fn highlight_configuration_javascript() -> HighlightConfiguration {
        HighlightConfiguration::new(
            GRAMMAR.into(),
            "javascript",
            HIGHLIGHTS_JAVASCRIPT,
            INJECTIONS_JAVASCRIPT,
            LOCALS_JAVASCRIPT,
        )
        .unwrap()
    }

    pub const HIGHLIGHTS_JSX: &str = include_str!("../queries/jsx/highlights.scm");
    pub const INJECTIONS_JSX: &str = include_str!("../queries/jsx/injections.scm");
    pub const LOCALS_JSX: &str = include_str!("../queries/jsx/locals.scm");
    pub fn highlight_configuration_jsx() -> HighlightConfiguration {
        HighlightConfiguration::new(
            GRAMMAR.into(),
            "jsx",
            HIGHLIGHTS_JSX,
            INJECTIONS_JSX,
            LOCALS_JSX,
        )
        .unwrap()
    }
}
#[cfg(feature = "jsdoc")]
pub mod jsdoc {
    use tree_sitter_highlight::HighlightConfiguration;
    use tree_sitter_language::LanguageFn;

    pub const GRAMMAR: LanguageFn = tree_sitter_jsdoc::LANGUAGE;
    pub const HIGHLIGHTS: &str = include_str!("../queries/jsdoc/highlights.scm");
    pub const INJECTIONS: &str = include_str!("../queries/jsdoc/injections.scm");
    pub fn highlight_configuration() -> HighlightConfiguration {
        HighlightConfiguration::new(GRAMMAR.into(), "jsdoc", HIGHLIGHTS, INJECTIONS, "").unwrap()
    }
}
#[cfg(feature = "json")]
pub mod json {
    use tree_sitter_highlight::HighlightConfiguration;
    use tree_sitter_language::LanguageFn;

    pub const GRAMMAR: LanguageFn = tree_sitter_json::LANGUAGE;
    pub const HIGHLIGHTS: &str = include_str!("../queries/json/highlights.scm");
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
    pub const INJECTIONS: &str = include_str!("../queries/julia/injections.scm");
    pub const LOCALS: &str = include_str!("../queries/julia/locals.scm");
    pub fn highlight_configuration() -> HighlightConfiguration {
        HighlightConfiguration::new(GRAMMAR.into(), "julia", HIGHLIGHTS, INJECTIONS, LOCALS)
            .unwrap()
    }
}
#[cfg(feature = "ocaml")]
pub mod ocaml {
    use tree_sitter_highlight::HighlightConfiguration;
    use tree_sitter_language::LanguageFn;

    pub const GRAMMAR_OCAML: LanguageFn = tree_sitter_ocaml::LANGUAGE_OCAML;
    pub const HIGHLIGHTS_OCAML: &str = include_str!("../queries/ocaml/highlights.scm");
    pub const INJECTIONS_OCAML: &str = include_str!("../queries/ocaml/injections.scm");
    pub const LOCALS_OCAML: &str = include_str!("../queries/ocaml/locals.scm");
    pub fn highlight_configuration_ocaml() -> HighlightConfiguration {
        HighlightConfiguration::new(
            GRAMMAR_OCAML.into(),
            "ocaml",
            HIGHLIGHTS_OCAML,
            INJECTIONS_OCAML,
            LOCALS_OCAML,
        )
        .unwrap()
    }

    pub const GRAMMAR_OCAML_INTERFACE: LanguageFn = tree_sitter_ocaml::LANGUAGE_OCAML_INTERFACE;
    pub const HIGHLIGHTS_OCAML_INTERFACE: &str =
        include_str!("../queries/ocaml-interface/highlights.scm");
    pub const INJECTIONS_OCAML_INTERFACE: &str =
        include_str!("../queries/ocaml-interface/injections.scm");
    pub fn highlight_configuration_ocaml_interface() -> HighlightConfiguration {
        HighlightConfiguration::new(
            GRAMMAR_OCAML_INTERFACE.into(),
            "ocaml_interface",
            HIGHLIGHTS_OCAML_INTERFACE,
            INJECTIONS_OCAML_INTERFACE,
            "",
        )
        .unwrap()
    }
}
#[cfg(feature = "php")]
pub mod php {
    use tree_sitter_highlight::HighlightConfiguration;
    use tree_sitter_language::LanguageFn;

    pub const GRAMMAR_PHP: LanguageFn = tree_sitter_php::LANGUAGE_PHP;
    pub const HIGHLIGHTS_PHP: &str = include_str!("../queries/php/highlights.scm");
    pub const INJECTIONS_PHP: &str = include_str!("../queries/php/injections.scm");
    pub fn highlight_configuration_php() -> HighlightConfiguration {
        HighlightConfiguration::new(
            GRAMMAR_PHP.into(),
            "php",
            HIGHLIGHTS_PHP,
            INJECTIONS_PHP,
            "",
        )
        .unwrap()
    }

    pub const GRAMMAR_PHP_ONLY: LanguageFn = tree_sitter_php::LANGUAGE_PHP_ONLY;
    pub const HIGHLIGHTS_PHP_ONLY: &str = include_str!("../queries/php-only/highlights.scm");
    pub const INJECTIONS_PHP_ONLY: &str = include_str!("../queries/php-only/injections.scm");
    pub fn highlight_configuration_php_only() -> HighlightConfiguration {
        HighlightConfiguration::new(
            GRAMMAR_PHP_ONLY.into(),
            "php_only",
            HIGHLIGHTS_PHP_ONLY,
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
    pub const HIGHLIGHTS: &str = include_str!("../queries/python/highlights.scm");
    pub const INJECTIONS: &str = include_str!("../queries/python/injections.scm");
    pub const LOCALS: &str = include_str!("../queries/python/locals.scm");
    pub fn highlight_configuration() -> HighlightConfiguration {
        HighlightConfiguration::new(GRAMMAR.into(), "python", HIGHLIGHTS, INJECTIONS, LOCALS)
            .unwrap()
    }
}
#[cfg(feature = "regex")]
pub mod regex {
    use tree_sitter_highlight::HighlightConfiguration;
    use tree_sitter_language::LanguageFn;

    pub const GRAMMAR: LanguageFn = tree_sitter_regex::LANGUAGE;
    pub const HIGHLIGHTS: &str = include_str!("../queries/regex/highlights.scm");
    pub fn highlight_configuration() -> HighlightConfiguration {
        HighlightConfiguration::new(GRAMMAR.into(), "regex", HIGHLIGHTS, "", "").unwrap()
    }
}
#[cfg(feature = "ruby")]
pub mod ruby {
    use tree_sitter_highlight::HighlightConfiguration;
    use tree_sitter_language::LanguageFn;

    pub const GRAMMAR: LanguageFn = tree_sitter_ruby::LANGUAGE;
    pub const HIGHLIGHTS: &str = include_str!("../queries/ruby/highlights.scm");
    pub const INJECTIONS: &str = include_str!("../queries/ruby/injections.scm");
    pub const LOCALS: &str = include_str!("../queries/ruby/locals.scm");
    pub fn highlight_configuration() -> HighlightConfiguration {
        HighlightConfiguration::new(GRAMMAR.into(), "ruby", HIGHLIGHTS, INJECTIONS, LOCALS).unwrap()
    }
}
#[cfg(feature = "rust")]
pub mod rust {
    use tree_sitter_highlight::HighlightConfiguration;
    use tree_sitter_language::LanguageFn;

    pub const GRAMMAR: LanguageFn = tree_sitter_rust::LANGUAGE;
    pub const HIGHLIGHTS: &str = include_str!("../queries/rust/highlights.scm");
    pub const INJECTIONS: &str = include_str!("../queries/rust/injections.scm");
    pub const LOCALS: &str = include_str!("../queries/rust/locals.scm");
    pub fn highlight_configuration() -> HighlightConfiguration {
        HighlightConfiguration::new(GRAMMAR.into(), "rust", HIGHLIGHTS, INJECTIONS, LOCALS).unwrap()
    }
}
#[cfg(feature = "scala")]
pub mod scala {
    use tree_sitter_highlight::HighlightConfiguration;
    use tree_sitter_language::LanguageFn;

    pub const GRAMMAR: LanguageFn = tree_sitter_scala::LANGUAGE;
    pub const HIGHLIGHTS: &str = include_str!("../queries/scala/highlights.scm");
    pub const INJECTIONS: &str = include_str!("../queries/scala/injections.scm");
    pub const LOCALS: &str = include_str!("../queries/scala/locals.scm");
    pub fn highlight_configuration() -> HighlightConfiguration {
        HighlightConfiguration::new(GRAMMAR.into(), "scala", HIGHLIGHTS, INJECTIONS, LOCALS)
            .unwrap()
    }
}
#[cfg(feature = "typescript")]
pub mod typescript {
    use tree_sitter_highlight::HighlightConfiguration;
    use tree_sitter_language::LanguageFn;

    pub const GRAMMAR_TYPESCRIPT: LanguageFn = tree_sitter_typescript::LANGUAGE_TYPESCRIPT;
    pub const HIGHLIGHTS_TYPESCRIPT: &str = include_str!("../queries/typescript/highlights.scm");
    pub const INJECTIONS_TYPESCRIPT: &str = include_str!("../queries/typescript/injections.scm");
    pub const LOCALS_TYPESCRIPT: &str = include_str!("../queries/typescript/locals.scm");
    pub fn highlight_configuration_typescript() -> HighlightConfiguration {
        HighlightConfiguration::new(
            GRAMMAR_TYPESCRIPT.into(),
            "typescript",
            HIGHLIGHTS_TYPESCRIPT,
            INJECTIONS_TYPESCRIPT,
            LOCALS_TYPESCRIPT,
        )
        .unwrap()
    }

    pub const GRAMMAR_TSX: LanguageFn = tree_sitter_typescript::LANGUAGE_TSX;
    pub const HIGHLIGHTS_TSX: &str = include_str!("../queries/tsx/highlights.scm");
    pub const INJECTIONS_TSX: &str = include_str!("../queries/tsx/injections.scm");
    pub const LOCALS_TSX: &str = include_str!("../queries/tsx/locals.scm");
    pub fn highlight_configuration_tsx() -> HighlightConfiguration {
        HighlightConfiguration::new(
            GRAMMAR_TSX.into(),
            "typescript",
            HIGHLIGHTS_TSX,
            INJECTIONS_TSX,
            LOCALS_TSX,
        )
        .unwrap()
    }
}
