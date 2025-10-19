use tree_sitter_highlight::HighlightConfiguration;

pub(crate) fn new(lang: &String) -> Option<HighlightConfiguration> {
    match lang.as_str() {
        #[cfg(feature = "agda")]
        "agda" => HighlightConfiguration::new(
            tree_sitter_agda::LANGUAGE.into(),
            "agda",
            include_str!("../queries/agda/highlights.scm"),
            "",
            "",
        )
        .ok(),
        #[cfg(feature = "bash")]
        "bash" | "shellscript" | "shell" | "zsh" | "sh" => HighlightConfiguration::new(
            tree_sitter_bash::LANGUAGE.into(),
            "bash",
            include_str!("../queries/bash/highlights.scm"),
            "",
            "",
        )
        .ok(),
        #[cfg(feature = "c")]
        "c" => HighlightConfiguration::new(
            tree_sitter_c::LANGUAGE.into(),
            "c",
            include_str!("../queries/c/highlights.scm"),
            include_str!("../queries/c/injections.scm"),
            "",
        )
        .ok(),
        #[cfg(feature = "cpp")]
        "cpp" | "c++" => HighlightConfiguration::new(
            tree_sitter_cpp::LANGUAGE.into(),
            "cpp",
            include_str!("../queries/cpp/highlights.scm"),
            include_str!("../queries/cpp/injections.scm"),
            "",
        )
        .ok(),
        #[cfg(feature = "css")]
        "css" => HighlightConfiguration::new(
            tree_sitter_css::LANGUAGE.into(),
            "css",
            include_str!("../queries/css/highlights.scm"),
            "",
            "",
        )
        .ok(),
        #[cfg(feature = "go")]
        "go" => HighlightConfiguration::new(
            tree_sitter_go::LANGUAGE.into(),
            "go",
            include_str!("../queries/go/highlights.scm"),
            include_str!("../queries/go/injections.scm"),
            "",
        )
        .ok(),
        #[cfg(feature = "haskell")]
        "haskell" | "hs" => HighlightConfiguration::new(
            tree_sitter_haskell::LANGUAGE.into(),
            "haskell",
            include_str!("../queries/haskell/highlights.scm"),
            "",
            "",
        )
        .ok(),
        #[cfg(feature = "html")]
        "html" => HighlightConfiguration::new(
            tree_sitter_html::LANGUAGE.into(),
            "html",
            include_str!("../queries/html/highlights.scm"),
            include_str!("../queries/html/injections.scm"),
            "",
        )
        .ok(),
        #[cfg(feature = "java")]
        "java" => HighlightConfiguration::new(
            tree_sitter_java::LANGUAGE.into(),
            "java",
            include_str!("../queries/java/highlights.scm"),
            include_str!("../queries/java/injections.scm"),
            include_str!("../queries/java/locals.scm"),
        )
        .ok(),
        #[cfg(feature = "javascript")]
        "javascript" | "js" => HighlightConfiguration::new(
            tree_sitter_javascript::LANGUAGE.into(),
            "javascript",
            include_str!("../queries/javascript/highlights.scm"),
            include_str!("../queries/javascript/highlights.scm"),
            "",
        )
        .ok(),
        #[cfg(feature = "javascript")]
        "jsx" => HighlightConfiguration::new(
            tree_sitter_javascript::LANGUAGE.into(),
            "jsx",
            include_str!("../queries/javascript/highlights.scm"),
            include_str!("../queries/javascript/highlights.scm"),
            "",
        )
        .ok(),
        #[cfg(feature = "jsdoc")]
        "jsdoc" => HighlightConfiguration::new(
            tree_sitter_jsdoc::LANGUAGE.into(),
            "jsdoc",
            include_str!("../queries/jsdoc/highlights.scm"),
            "",
            "",
        )
        .ok(),
        #[cfg(feature = "json")]
        "json" => HighlightConfiguration::new(
            tree_sitter_json::LANGUAGE.into(),
            "json",
            include_str!("../queries/json/highlights.scm"),
            "",
            "",
        )
        .ok(),
        #[cfg(feature = "ocaml")]
        "ocaml" => HighlightConfiguration::new(
            tree_sitter_ocaml::LANGUAGE_OCAML.into(),
            "ocaml",
            include_str!("../queries/ocaml/highlights.scm"),
            "",
            "",
        )
        .ok(),
        #[cfg(feature = "ocaml")]
        "ocaml_interface" => HighlightConfiguration::new(
            tree_sitter_ocaml::LANGUAGE_OCAML_INTERFACE.into(),
            "ocaml_interface",
            include_str!("../queries/ocaml/highlights.scm"),
            "",
            "",
        )
        .ok(),
        #[cfg(feature = "ocaml")]
        "ocaml_type" => HighlightConfiguration::new(
            tree_sitter_ocaml::LANGUAGE_OCAML_TYPE.into(),
            "ocaml_type",
            include_str!("../queries/ocaml/highlights.scm"),
            "",
            "",
        )
        .ok(),
        #[cfg(feature = "php")]
        "php" => HighlightConfiguration::new(
            tree_sitter_php::LANGUAGE_PHP.into(),
            "php",
            include_str!("../queries/php/highlights.scm"),
            include_str!("../queries/php/injections.scm"),
            "",
        )
        .ok(),
        #[cfg(feature = "php")]
        "php_only" => HighlightConfiguration::new(
            tree_sitter_php::LANGUAGE_PHP_ONLY.into(),
            "php_only",
            include_str!("../queries/php/highlights.scm"),
            include_str!("../queries/php/injections.scm"),
            "",
        )
        .ok(),
        #[cfg(feature = "python")]
        "python" | "py" => HighlightConfiguration::new(
            tree_sitter_python::LANGUAGE.into(),
            "python",
            include_str!("../queries/python/highlights.scm"),
            "",
            "",
        )
        .ok(),
        #[cfg(feature = "regex")]
        "regexp" | "regex" => HighlightConfiguration::new(
            tree_sitter_regex::LANGUAGE.into(),
            "regex",
            include_str!("../queries/regex/highlights.scm"),
            "",
            "",
        )
        .ok(),
        #[cfg(feature = "ruby")]
        "ruby" | "rb" => HighlightConfiguration::new(
            tree_sitter_ruby::LANGUAGE.into(),
            "ruby",
            include_str!("../queries/ruby/highlights.scm"),
            include_str!("../queries/ruby/injections.scm"),
            "",
        )
        .ok(),
        #[cfg(feature = "rust")]
        "rust" | "rs" => HighlightConfiguration::new(
            tree_sitter_rust::LANGUAGE.into(),
            "rust",
            include_str!("../queries/rust/highlights.scm"),
            include_str!("../queries/rust/injections.scm"),
            "",
        )
        .ok(),
        #[cfg(feature = "scala")]
        "scala" => HighlightConfiguration::new(
            tree_sitter_scala::LANGUAGE.into(),
            "scala",
            include_str!("../queries/scala/highlights.scm"),
            include_str!("../queries/scala/injections.scm"),
            include_str!("../queries/scala/locals.scm"),
        )
        .ok(),
        #[cfg(feature = "typescript")]
        "typescript" | "ts" => HighlightConfiguration::new(
            tree_sitter_typescript::LANGUAGE_TYPESCRIPT.into(),
            "typescript",
            include_str!("../queries/typescript/highlights.scm"),
            include_str!("../queries/typescript/injections.scm"),
            "",
        )
        .ok(),
        #[cfg(feature = "typescript")]
        "tsx" => HighlightConfiguration::new(
            tree_sitter_typescript::LANGUAGE_TSX.into(),
            "tsx",
            include_str!("../queries/typescript/highlights.scm"),
            include_str!("../queries/typescript/injections.scm"),
            "",
        )
        .ok(),
        _ => None,
    }
}
