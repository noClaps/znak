#![doc = include_str!("../README.md")]

/// Tree-sitter grammar and queries for Agda
#[cfg(feature = "agda")]
pub mod agda {
    use tree_sitter_language::LanguageFn;

    /// The Tree-sitter grammar
    pub const GRAMMAR: LanguageFn = tree_sitter_agda::LANGUAGE;
    /// The syntax highlighting query
    pub const HIGHLIGHTS: &str = tree_sitter_agda::HIGHLIGHTS_QUERY;
}
/// Tree-sitter grammar and queries for Bash
#[cfg(feature = "bash")]
pub mod bash {
    use tree_sitter_language::LanguageFn;

    /// The Tree-sitter grammar
    pub const GRAMMAR: LanguageFn = tree_sitter_bash::LANGUAGE;
    /// The syntax highlighting query
    pub const HIGHLIGHTS: &str = tree_sitter_bash::HIGHLIGHT_QUERY;
}
/// Tree-sitter grammar and queries for C
#[cfg(feature = "c")]
pub mod c {
    use tree_sitter_language::LanguageFn;

    /// The Tree-sitter grammar
    pub const GRAMMAR: LanguageFn = tree_sitter_c::LANGUAGE;
    /// The syntax highlighting query
    pub const HIGHLIGHTS: &str = tree_sitter_c::HIGHLIGHT_QUERY;
}
/// Tree-sitter grammar and queries for C++
#[cfg(feature = "cpp")]
pub mod cpp {
    use constcat::concat;
    use tree_sitter_language::LanguageFn;

    /// The Tree-sitter grammar
    pub const GRAMMAR: LanguageFn = tree_sitter_cpp::LANGUAGE;
    /// The syntax highlighting query
    pub const HIGHLIGHTS: &str = concat!(
        tree_sitter_c::HIGHLIGHT_QUERY,
        "\n",
        tree_sitter_cpp::HIGHLIGHT_QUERY
    );
    /// The syntax highlighting query for injected languages
    pub const INJECTIONS: &str = include_str!("../queries/cpp/injections.scm");
}
/// Tree-sitter grammar and queries for CSS
#[cfg(feature = "css")]
pub mod css {
    use tree_sitter_language::LanguageFn;

    /// The Tree-sitter grammar
    pub const GRAMMAR: LanguageFn = tree_sitter_css::LANGUAGE;
    /// The syntax highlighting query
    pub const HIGHLIGHTS: &str = tree_sitter_css::HIGHLIGHTS_QUERY;
}
/// Tree-sitter grammar and queries for Go
#[cfg(feature = "go")]
pub mod go {
    use tree_sitter_language::LanguageFn;

    /// The Tree-sitter grammar
    pub const GRAMMAR: LanguageFn = tree_sitter_go::LANGUAGE;
    /// The syntax highlighting query
    pub const HIGHLIGHTS: &str = tree_sitter_go::HIGHLIGHTS_QUERY;
}
/// Tree-sitter grammar and queries for Haskell
#[cfg(feature = "haskell")]
pub mod haskell {
    use tree_sitter_language::LanguageFn;

    /// The Tree-sitter grammar
    pub const GRAMMAR: LanguageFn = tree_sitter_haskell::LANGUAGE;
    /// The syntax highlighting query
    pub const HIGHLIGHTS: &str = tree_sitter_haskell::HIGHLIGHTS_QUERY;
    /// The syntax highlighting query for injected languages
    pub const INJECTIONS: &str = tree_sitter_haskell::INJECTIONS_QUERY;
    /// The local-variable syntax highlighting query
    pub const LOCALS: &str = tree_sitter_haskell::LOCALS_QUERY;
}
/// Tree-sitter grammar and queries for HTML
#[cfg(feature = "html")]
pub mod html {
    use tree_sitter_language::LanguageFn;

    /// The Tree-sitter grammar
    pub const GRAMMAR: LanguageFn = tree_sitter_html::LANGUAGE;
    /// The syntax highlighting query
    pub const HIGHLIGHTS: &str = tree_sitter_html::HIGHLIGHTS_QUERY;
    /// The syntax highlighting query for injected languages
    pub const INJECTIONS: &str = tree_sitter_html::INJECTIONS_QUERY;
}
/// Tree-sitter grammar and queries for Java
#[cfg(feature = "java")]
pub mod java {
    use tree_sitter_language::LanguageFn;

    /// The Tree-sitter grammar
    pub const GRAMMAR: LanguageFn = tree_sitter_java::LANGUAGE;
    /// The syntax highlighting query
    pub const HIGHLIGHTS: &str = tree_sitter_java::HIGHLIGHTS_QUERY;
}
/// Tree-sitter grammar and queries for JavaScript and JSX
#[cfg(feature = "javascript")]
pub mod javascript {
    use constcat::concat;
    use tree_sitter_language::LanguageFn;

    /// The Tree-sitter grammar
    pub const GRAMMAR: LanguageFn = tree_sitter_javascript::LANGUAGE;
    /// The syntax highlighting query
    pub const HIGHLIGHTS: &str = concat!(
        tree_sitter_javascript::HIGHLIGHT_QUERY,
        "\n",
        tree_sitter_javascript::JSX_HIGHLIGHT_QUERY,
        "\n",
        include_str!("../queries/javascript/highlights-params.scm")
    );
    /// The syntax highlighting query for injected languages
    pub const INJECTIONS: &str = tree_sitter_javascript::INJECTIONS_QUERY;
    /// The local-variable syntax highlighting query
    pub const LOCALS: &str = tree_sitter_javascript::LOCALS_QUERY;
}
/// Tree-sitter grammar and queries for JSDoc
#[cfg(feature = "jsdoc")]
pub mod jsdoc {
    use tree_sitter_language::LanguageFn;

    /// The Tree-sitter grammar
    pub const GRAMMAR: LanguageFn = tree_sitter_jsdoc::LANGUAGE;
    /// The syntax highlighting query
    pub const HIGHLIGHTS: &str = tree_sitter_jsdoc::HIGHLIGHTS_QUERY;
}
/// Tree-sitter grammar and queries for JSON
#[cfg(feature = "json")]
pub mod json {
    use tree_sitter_language::LanguageFn;

    /// The Tree-sitter grammar
    pub const GRAMMAR: LanguageFn = tree_sitter_json::LANGUAGE;
    /// The syntax highlighting query
    pub const HIGHLIGHTS: &str = tree_sitter_json::HIGHLIGHTS_QUERY;
}
/// Tree-sitter grammar and queries for Julia
#[cfg(feature = "julia")]
pub mod julia {
    use tree_sitter_language::LanguageFn;

    /// The Tree-sitter grammar
    pub const GRAMMAR: LanguageFn = tree_sitter_julia::LANGUAGE;
    /// The syntax highlighting query
    pub const HIGHLIGHTS: &str = include_str!("../queries/julia/highlights.scm");
    /// The local-variable syntax highlighting query
    pub const LOCALS: &str = include_str!("../queries/julia/locals.scm");
}
/// Tree-sitter grammar and queries for OCaml, OCaml Interface, and OCaml Type
#[cfg(feature = "ocaml")]
pub mod ocaml {
    use tree_sitter_language::LanguageFn;

    /// The Tree-sitter grammar
    pub const GRAMMAR_OCAML: LanguageFn = tree_sitter_ocaml::LANGUAGE_OCAML;
    /// The Tree-sitter grammar
    pub const GRAMMAR_OCAML_INTERFACE: LanguageFn = tree_sitter_ocaml::LANGUAGE_OCAML_INTERFACE;
    /// The Tree-sitter grammar
    pub const GRAMMAR_OCAML_TYPE: LanguageFn = tree_sitter_ocaml::LANGUAGE_OCAML_TYPE;
    /// The syntax highlighting query
    pub const HIGHLIGHTS: &str = tree_sitter_ocaml::HIGHLIGHTS_QUERY;
    /// The local-variable syntax highlighting query
    pub const LOCALS: &str = tree_sitter_ocaml::LOCALS_QUERY;
}
/// Tree-sitter grammar and queries for PHP
#[cfg(feature = "php")]
pub mod php {
    use constcat::concat;
    use tree_sitter_language::LanguageFn;

    /// The Tree-sitter grammar
    pub const GRAMMAR_PHP: LanguageFn = tree_sitter_php::LANGUAGE_PHP;
    /// The Tree-sitter grammar
    pub const GRAMMAR_PHP_ONLY: LanguageFn = tree_sitter_php::LANGUAGE_PHP_ONLY;
    /// The syntax highlighting query
    pub const HIGHLIGHTS: &str = tree_sitter_php::HIGHLIGHTS_QUERY;
    /// The syntax highlighting query for injected languages
    pub const INJECTIONS_PHP: &str = concat!(
        tree_sitter_php::INJECTIONS_QUERY,
        "\n",
        include_str!("../queries/php/injections-text.scm")
    );
    /// The syntax highlighting query for injected languages
    pub const INJECTIONS_PHP_ONLY: &str = tree_sitter_php::INJECTIONS_QUERY;
}
/// Tree-sitter grammar and queries for Python
#[cfg(feature = "python")]
pub mod python {
    use tree_sitter_language::LanguageFn;

    /// The Tree-sitter grammar
    pub const GRAMMAR: LanguageFn = tree_sitter_python::LANGUAGE;
    /// The syntax highlighting query
    pub const HIGHLIGHTS: &str = tree_sitter_python::HIGHLIGHTS_QUERY;
}
/// Tree-sitter grammar and queries for Regex
#[cfg(feature = "regex")]
pub mod regex {
    use tree_sitter_language::LanguageFn;

    /// The Tree-sitter grammar
    pub const GRAMMAR: LanguageFn = tree_sitter_regex::LANGUAGE;
    /// The syntax highlighting query
    pub const HIGHLIGHTS: &str = tree_sitter_regex::HIGHLIGHTS_QUERY;
}
/// Tree-sitter grammar and queries for Ruby
#[cfg(feature = "ruby")]
pub mod ruby {
    use tree_sitter_language::LanguageFn;

    /// The Tree-sitter grammar
    pub const GRAMMAR: LanguageFn = tree_sitter_ruby::LANGUAGE;
    /// The syntax highlighting query
    pub const HIGHLIGHTS: &str = tree_sitter_ruby::HIGHLIGHTS_QUERY;
    /// The local-variable syntax highlighting query
    pub const LOCALS: &str = tree_sitter_ruby::LOCALS_QUERY;
}
/// Tree-sitter grammar and queries for Rust
#[cfg(feature = "rust")]
pub mod rust {
    use tree_sitter_language::LanguageFn;

    /// The Tree-sitter grammar
    pub const GRAMMAR: LanguageFn = tree_sitter_rust::LANGUAGE;
    /// The syntax highlighting query
    pub const HIGHLIGHTS: &str = tree_sitter_rust::HIGHLIGHTS_QUERY;
    /// The syntax highlighting query for injected languages
    pub const INJECTIONS: &str = tree_sitter_rust::INJECTIONS_QUERY;
}
/// Tree-sitter grammar and queries for Scala
#[cfg(feature = "scala")]
pub mod scala {
    use tree_sitter_language::LanguageFn;

    /// The Tree-sitter grammar
    pub const GRAMMAR: LanguageFn = tree_sitter_scala::LANGUAGE;
    /// The syntax highlighting query
    pub const HIGHLIGHTS: &str = tree_sitter_scala::HIGHLIGHTS_QUERY;
    /// The local-variable syntax highlighting query
    pub const LOCALS: &str = tree_sitter_scala::LOCALS_QUERY;
}
/// Tree-sitter grammar and queries for TypeScript and TSX
#[cfg(feature = "typescript")]
pub mod typescript {
    use constcat::concat;
    use tree_sitter_language::LanguageFn;

    /// The Tree-sitter grammar
    pub const GRAMMAR_TYPESCRIPT: LanguageFn = tree_sitter_typescript::LANGUAGE_TYPESCRIPT;
    /// The Tree-sitter grammar
    pub const GRAMMAR_TSX: LanguageFn = tree_sitter_typescript::LANGUAGE_TSX;
    /// The syntax highlighting query
    pub const HIGHLIGHTS_TYPESCRIPT: &str = concat!(
        tree_sitter_typescript::HIGHLIGHTS_QUERY,
        "\n",
        tree_sitter_javascript::HIGHLIGHT_QUERY
    );
    /// The syntax highlighting query
    pub const HIGHLIGHTS_TSX: &str = concat!(
        tree_sitter_typescript::HIGHLIGHTS_QUERY,
        "\n",
        tree_sitter_javascript::JSX_HIGHLIGHT_QUERY,
        "\n",
        tree_sitter_javascript::HIGHLIGHT_QUERY
    );
    /// The syntax highlighting query for injected languages
    pub const INJECTIONS: &str = tree_sitter_javascript::INJECTIONS_QUERY;
    /// The local-variable syntax highlighting query
    pub const LOCALS: &str = concat!(
        tree_sitter_typescript::LOCALS_QUERY,
        "\n",
        tree_sitter_javascript::LOCALS_QUERY
    );
}
