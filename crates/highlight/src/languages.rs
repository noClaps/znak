use tree_sitter_highlight::HighlightConfiguration;

pub(crate) fn new(lang: &String) -> Option<HighlightConfiguration> {
    match lang.as_str() {
        #[cfg(feature = "agda")]
        "agda" => HighlightConfiguration::new(
            tree_sitter_agda::LANGUAGE.into(),
            "agda",
            tree_sitter_agda::HIGHLIGHTS_QUERY,
            "",
            "",
        )
        .ok(),
        #[cfg(feature = "bash")]
        "bash" | "shellscript" | "shell" | "zsh" | "sh" => HighlightConfiguration::new(
            tree_sitter_bash::LANGUAGE.into(),
            "bash",
            tree_sitter_bash::HIGHLIGHT_QUERY,
            "",
            "",
        )
        .ok(),
        #[cfg(feature = "c")]
        "c" => HighlightConfiguration::new(
            tree_sitter_c::LANGUAGE.into(),
            "c",
            tree_sitter_c::HIGHLIGHT_QUERY,
            "",
            "",
        )
        .ok(),
        #[cfg(feature = "cpp")]
        "cpp" | "c++" => HighlightConfiguration::new(
            tree_sitter_cpp::LANGUAGE.into(),
            "cpp",
            tree_sitter_cpp::HIGHLIGHT_QUERY,
            "",
            "",
        )
        .ok(),
        #[cfg(feature = "css")]
        "css" => HighlightConfiguration::new(
            tree_sitter_css::LANGUAGE.into(),
            "css",
            tree_sitter_css::HIGHLIGHTS_QUERY,
            "",
            "",
        )
        .ok(),
        #[cfg(feature = "go")]
        "go" => HighlightConfiguration::new(
            tree_sitter_go::LANGUAGE.into(),
            "go",
            tree_sitter_go::HIGHLIGHTS_QUERY,
            "",
            "",
        )
        .ok(),
        #[cfg(feature = "haskell")]
        "haskell" | "hs" => HighlightConfiguration::new(
            tree_sitter_haskell::LANGUAGE.into(),
            "haskell",
            tree_sitter_haskell::HIGHLIGHTS_QUERY,
            tree_sitter_haskell::INJECTIONS_QUERY,
            tree_sitter_haskell::LOCALS_QUERY,
        )
        .ok(),
        #[cfg(feature = "html")]
        "html" => HighlightConfiguration::new(
            tree_sitter_html::LANGUAGE.into(),
            "html",
            tree_sitter_html::HIGHLIGHTS_QUERY,
            tree_sitter_html::INJECTIONS_QUERY,
            "",
        )
        .ok(),
        #[cfg(feature = "java")]
        "java" => HighlightConfiguration::new(
            tree_sitter_java::LANGUAGE.into(),
            "java",
            tree_sitter_java::HIGHLIGHTS_QUERY,
            "",
            "",
        )
        .ok(),
        #[cfg(feature = "javascript")]
        "javascript" | "js" => HighlightConfiguration::new(
            tree_sitter_javascript::LANGUAGE.into(),
            "javascript",
            tree_sitter_javascript::HIGHLIGHT_QUERY,
            tree_sitter_javascript::INJECTIONS_QUERY,
            tree_sitter_javascript::LOCALS_QUERY,
        )
        .ok(),
        #[cfg(feature = "javascript")]
        "jsx" => HighlightConfiguration::new(
            tree_sitter_javascript::LANGUAGE.into(),
            "jsx",
            tree_sitter_javascript::JSX_HIGHLIGHT_QUERY,
            tree_sitter_javascript::INJECTIONS_QUERY,
            tree_sitter_javascript::LOCALS_QUERY,
        )
        .ok(),
        #[cfg(feature = "jsdoc")]
        "jsdoc" => HighlightConfiguration::new(
            tree_sitter_jsdoc::LANGUAGE.into(),
            "jsdoc",
            tree_sitter_jsdoc::HIGHLIGHTS_QUERY,
            "",
            "",
        )
        .ok(),
        #[cfg(feature = "json")]
        "json" => HighlightConfiguration::new(
            tree_sitter_json::LANGUAGE.into(),
            "json",
            tree_sitter_json::HIGHLIGHTS_QUERY,
            "",
            "",
        )
        .ok(),
        #[cfg(feature = "ocaml")]
        "ocaml" => HighlightConfiguration::new(
            tree_sitter_ocaml::LANGUAGE_OCAML.into(),
            "ocaml",
            tree_sitter_ocaml::HIGHLIGHTS_QUERY,
            "",
            tree_sitter_ocaml::LOCALS_QUERY,
        )
        .ok(),
        #[cfg(feature = "ocaml")]
        "ocaml_interface" => HighlightConfiguration::new(
            tree_sitter_ocaml::LANGUAGE_OCAML_INTERFACE.into(),
            "ocaml_interface",
            tree_sitter_ocaml::HIGHLIGHTS_QUERY,
            "",
            tree_sitter_ocaml::LOCALS_QUERY,
        )
        .ok(),
        #[cfg(feature = "ocaml")]
        "ocaml_type" => HighlightConfiguration::new(
            tree_sitter_ocaml::LANGUAGE_OCAML_TYPE.into(),
            "ocaml_type",
            tree_sitter_ocaml::HIGHLIGHTS_QUERY,
            "",
            tree_sitter_ocaml::LOCALS_QUERY,
        )
        .ok(),
        #[cfg(feature = "php")]
        "php" => HighlightConfiguration::new(
            tree_sitter_php::LANGUAGE_PHP.into(),
            "php",
            tree_sitter_php::HIGHLIGHTS_QUERY,
            tree_sitter_php::INJECTIONS_QUERY,
            "",
        )
        .ok(),
        #[cfg(feature = "php")]
        "php_only" => HighlightConfiguration::new(
            tree_sitter_php::LANGUAGE_PHP_ONLY.into(),
            "php_only",
            tree_sitter_php::HIGHLIGHTS_QUERY,
            tree_sitter_php::INJECTIONS_QUERY,
            "",
        )
        .ok(),
        #[cfg(feature = "python")]
        "python" | "py" => HighlightConfiguration::new(
            tree_sitter_python::LANGUAGE.into(),
            "python",
            tree_sitter_python::HIGHLIGHTS_QUERY,
            "",
            "",
        )
        .ok(),
        #[cfg(feature = "regex")]
        "regexp" | "regex" => HighlightConfiguration::new(
            tree_sitter_regex::LANGUAGE.into(),
            "regex",
            tree_sitter_regex::HIGHLIGHTS_QUERY,
            "",
            "",
        )
        .ok(),
        #[cfg(feature = "ruby")]
        "ruby" | "rb" => HighlightConfiguration::new(
            tree_sitter_ruby::LANGUAGE.into(),
            "ruby",
            tree_sitter_ruby::HIGHLIGHTS_QUERY,
            "",
            tree_sitter_ruby::LOCALS_QUERY,
        )
        .ok(),
        #[cfg(feature = "rust")]
        "rust" | "rs" => HighlightConfiguration::new(
            tree_sitter_rust::LANGUAGE.into(),
            "rust",
            tree_sitter_rust::HIGHLIGHTS_QUERY,
            tree_sitter_rust::INJECTIONS_QUERY,
            "",
        )
        .ok(),
        #[cfg(feature = "scala")]
        "scala" => HighlightConfiguration::new(
            tree_sitter_scala::LANGUAGE.into(),
            "scala",
            tree_sitter_scala::HIGHLIGHTS_QUERY,
            "",
            tree_sitter_scala::LOCALS_QUERY,
        )
        .ok(),
        #[cfg(feature = "typescript")]
        "typescript" | "ts" => HighlightConfiguration::new(
            tree_sitter_typescript::LANGUAGE_TYPESCRIPT.into(),
            "typescript",
            tree_sitter_typescript::HIGHLIGHTS_QUERY,
            "",
            tree_sitter_typescript::LOCALS_QUERY,
        )
        .ok(),
        #[cfg(feature = "typescript")]
        "tsx" => HighlightConfiguration::new(
            tree_sitter_typescript::LANGUAGE_TSX.into(),
            "tsx",
            tree_sitter_typescript::HIGHLIGHTS_QUERY,
            "",
            tree_sitter_typescript::LOCALS_QUERY,
        )
        .ok(),
        _ => None,
    }
}
