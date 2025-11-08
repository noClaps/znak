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
    pub fn highlight_configuration() -> HighlightConfiguration {
        HighlightConfiguration::new(GRAMMAR.into(), "c", HIGHLIGHTS, INJECTIONS, "").unwrap()
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
    pub fn highlight_configuration() -> HighlightConfiguration {
        HighlightConfiguration::new(GRAMMAR.into(), "go", HIGHLIGHTS, INJECTIONS, "").unwrap()
    }
}
#[cfg(feature = "haskell")]
pub mod haskell {
    use tree_sitter_highlight::HighlightConfiguration;
    use tree_sitter_language::LanguageFn;

    pub const GRAMMAR: LanguageFn = tree_sitter_haskell::LANGUAGE;
    pub const HIGHLIGHTS: &str = include_str!("../queries/haskell/highlights.scm");
    pub const INJECTIONS: &str = include_str!("../queries/haskell/injections.scm");
    pub fn highlight_configuration() -> HighlightConfiguration {
        HighlightConfiguration::new(GRAMMAR.into(), "haskell", HIGHLIGHTS, INJECTIONS, "").unwrap()
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
    pub const LOCALS: &str = include_str!("../queries/java/locals.scm");
    pub fn highlight_configuration() -> HighlightConfiguration {
        HighlightConfiguration::new(GRAMMAR.into(), "java", HIGHLIGHTS, INJECTIONS, LOCALS).unwrap()
    }
}
#[cfg(feature = "javascript")]
pub mod javascript {
    use tree_sitter_highlight::HighlightConfiguration;
    use tree_sitter_language::LanguageFn;

    pub const GRAMMAR: LanguageFn = tree_sitter_javascript::LANGUAGE;
    pub const HIGHLIGHTS: &str = include_str!("../queries/javascript/highlights.scm");
    pub const INJECTIONS: &str = include_str!("../queries/javascript/injections.scm");
    pub fn highlight_configuration() -> HighlightConfiguration {
        HighlightConfiguration::new(GRAMMAR.into(), "javascript", HIGHLIGHTS, INJECTIONS, "")
            .unwrap()
    }
}
#[cfg(feature = "jsdoc")]
pub mod jsdoc {
    use tree_sitter_highlight::HighlightConfiguration;
    use tree_sitter_language::LanguageFn;

    pub const GRAMMAR: LanguageFn = tree_sitter_jsdoc::LANGUAGE;
    pub const HIGHLIGHTS: &str = include_str!("../queries/jsdoc/highlights.scm");
    pub fn highlight_configuration() -> HighlightConfiguration {
        HighlightConfiguration::new(GRAMMAR.into(), "jsdoc", HIGHLIGHTS, "", "").unwrap()
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
    pub fn highlight_configuration() -> HighlightConfiguration {
        HighlightConfiguration::new(GRAMMAR.into(), "julia", HIGHLIGHTS, INJECTIONS, "").unwrap()
    }
}
#[cfg(feature = "ocaml")]
pub mod ocaml {
    use tree_sitter_highlight::HighlightConfiguration;
    use tree_sitter_language::LanguageFn;

    pub const GRAMMAR_OCAML: LanguageFn = tree_sitter_ocaml::LANGUAGE_OCAML;
    pub const GRAMMAR_OCAML_INTERFACE: LanguageFn = tree_sitter_ocaml::LANGUAGE_OCAML_INTERFACE;
    pub const HIGHLIGHTS: &str = include_str!("../queries/ocaml/highlights.scm");
    pub fn highlight_configuration_ocaml() -> HighlightConfiguration {
        HighlightConfiguration::new(GRAMMAR_OCAML.into(), "ocaml", HIGHLIGHTS, "", "").unwrap()
    }
    pub fn highlight_configuration_ocaml_interface() -> HighlightConfiguration {
        HighlightConfiguration::new(
            GRAMMAR_OCAML_INTERFACE.into(),
            "ocaml_interface",
            HIGHLIGHTS,
            "",
            "",
        )
        .unwrap()
    }
}
#[cfg(feature = "php")]
pub mod php {
    use tree_sitter_highlight::HighlightConfiguration;
    use tree_sitter_language::LanguageFn;

    pub const GRAMMAR: LanguageFn = tree_sitter_php::LANGUAGE_PHP;
    pub const HIGHLIGHTS: &str = include_str!("../queries/php/highlights.scm");
    pub const INJECTIONS: &str = include_str!("../queries/php/injections.scm");
    pub fn highlight_configuration() -> HighlightConfiguration {
        HighlightConfiguration::new(GRAMMAR.into(), "php", HIGHLIGHTS, INJECTIONS, "").unwrap()
    }
}
#[cfg(feature = "python")]
pub mod python {
    use tree_sitter_highlight::HighlightConfiguration;
    use tree_sitter_language::LanguageFn;

    pub const GRAMMAR: LanguageFn = tree_sitter_python::LANGUAGE;
    pub const HIGHLIGHTS: &str = include_str!("../queries/python/highlights.scm");
    pub const INJECTIONS: &str = include_str!("../queries/python/injections.scm");
    pub fn highlight_configuration() -> HighlightConfiguration {
        HighlightConfiguration::new(GRAMMAR.into(), "python", HIGHLIGHTS, INJECTIONS, "").unwrap()
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
    pub fn highlight_configuration() -> HighlightConfiguration {
        HighlightConfiguration::new(GRAMMAR.into(), "ruby", HIGHLIGHTS, INJECTIONS, "").unwrap()
    }
}
#[cfg(feature = "rust")]
pub mod rust {
    use tree_sitter_highlight::HighlightConfiguration;
    use tree_sitter_language::LanguageFn;

    pub const GRAMMAR: LanguageFn = tree_sitter_rust::LANGUAGE;
    pub const HIGHLIGHTS: &str = include_str!("../queries/rust/highlights.scm");
    pub const INJECTIONS: &str = include_str!("../queries/rust/injections.scm");
    pub fn highlight_configuration() -> HighlightConfiguration {
        HighlightConfiguration::new(GRAMMAR.into(), "rust", HIGHLIGHTS, INJECTIONS, "").unwrap()
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
    pub const GRAMMAR_TSX: LanguageFn = tree_sitter_typescript::LANGUAGE_TSX;
    pub const HIGHLIGHTS_TYPESCRIPT: &str = include_str!("../queries/typescript/highlights.scm");
    pub const HIGHLIGHTS_TSX: &str = include_str!("../queries/tsx/highlights.scm");
    pub const INJECTIONS_TYPESCRIPT: &str = include_str!("../queries/typescript/injections.scm");
    pub const INJECTIONS_TSX: &str = include_str!("../queries/tsx/injections.scm");
    pub fn highlight_configuration_typescript() -> HighlightConfiguration {
        HighlightConfiguration::new(
            GRAMMAR_TYPESCRIPT.into(),
            "typescript",
            HIGHLIGHTS_TYPESCRIPT,
            INJECTIONS_TYPESCRIPT,
            "",
        )
        .unwrap()
    }
    pub fn highlight_configuration_tsx() -> HighlightConfiguration {
        HighlightConfiguration::new(
            GRAMMAR_TSX.into(),
            "typescript",
            HIGHLIGHTS_TSX,
            INJECTIONS_TSX,
            "",
        )
        .unwrap()
    }
}
