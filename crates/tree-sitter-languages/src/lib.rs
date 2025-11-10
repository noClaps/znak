#![doc = include_str!("../README.md")]

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
#[cfg(feature = "comment")]
pub mod comment {
    use tree_sitter_highlight::HighlightConfiguration;
    use tree_sitter_language::LanguageFn;

    pub const GRAMMAR: LanguageFn = tree_sitter_comment::LANGUAGE;
    pub const HIGHLIGHTS: &str = include_str!("../queries/comment/highlights.scm");
    pub fn highlight_configuration() -> HighlightConfiguration {
        HighlightConfiguration::new(GRAMMAR.into(), "comment", HIGHLIGHTS, "", "").unwrap()
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
#[cfg(feature = "git")]
pub mod git {
    use tree_sitter_highlight::HighlightConfiguration;
    use tree_sitter_language::LanguageFn;

    // gitattributes
    pub const GRAMMAR_ATTRIBUTES: LanguageFn = tree_sitter_gitattributes::LANGUAGE;
    pub const HIGHLIGHTS_ATTRIBUTES: &str = include_str!("../queries/gitattributes/highlights.scm");
    pub fn highlight_configuration_attributes() -> HighlightConfiguration {
        HighlightConfiguration::new(
            GRAMMAR_ATTRIBUTES.into(),
            "gitattributes",
            HIGHLIGHTS_ATTRIBUTES,
            "",
            "",
        )
        .unwrap()
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

    // json
    pub const GRAMMAR_JSON: LanguageFn = tree_sitter_json::LANGUAGE;
    pub const HIGHLIGHTS_JSON: &str = include_str!("../queries/json/highlights.scm");
    pub fn highlight_configuration_json() -> HighlightConfiguration {
        HighlightConfiguration::new(GRAMMAR_JSON.into(), "json", HIGHLIGHTS_JSON, "", "").unwrap()
    }

    // jsonc
    pub const GRAMMAR_JSONC: LanguageFn = tree_sitter_json::LANGUAGE;
    pub const HIGHLIGHTS_JSONC: &str = include_str!("../queries/jsonc/highlights.scm");
    pub const INJECTIONS_JSONC: &str = include_str!("../queries/jsonc/injections.scm");
    pub fn highlight_configuration_jsonc() -> HighlightConfiguration {
        HighlightConfiguration::new(
            GRAMMAR_JSONC.into(),
            "jsonc",
            HIGHLIGHTS_JSONC,
            INJECTIONS_JSONC,
            "",
        )
        .unwrap()
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
#[cfg(feature = "lua")]
pub mod lua {
    use tree_sitter_highlight::HighlightConfiguration;
    use tree_sitter_language::LanguageFn;

    pub const GRAMMAR: LanguageFn = tree_sitter_lua::LANGUAGE;
    pub const HIGHLIGHTS: &str = include_str!("../queries/lua/highlights.scm");
    pub fn highlight_configuration() -> HighlightConfiguration {
        HighlightConfiguration::new(GRAMMAR.into(), "lua", HIGHLIGHTS, "", "").unwrap()
    }
}
#[cfg(feature = "make")]
pub mod make {
    use tree_sitter_highlight::HighlightConfiguration;
    use tree_sitter_language::LanguageFn;

    pub const GRAMMAR: LanguageFn = tree_sitter_make::LANGUAGE;
    pub const HIGHLIGHTS: &str = include_str!("../queries/make/highlights.scm");
    pub const INJECTIONS: &str = include_str!("../queries/make/injections.scm");
    pub fn highlight_configuration() -> HighlightConfiguration {
        HighlightConfiguration::new(GRAMMAR.into(), "make", HIGHLIGHTS, INJECTIONS, "").unwrap()
    }
}
#[cfg(feature = "markdown")]
pub mod markdown {
    use tree_sitter_highlight::HighlightConfiguration;
    use tree_sitter_language::LanguageFn;

    // markdown
    pub const GRAMMAR_MARKDOWN: LanguageFn = tree_sitter_md::LANGUAGE;
    pub const HIGHLIGHTS_MARKDOWN: &str = include_str!("../queries/markdown/highlights.scm");
    pub const INJECTIONS_MARKDOWN: &str = include_str!("../queries/markdown/injections.scm");
    pub fn highlight_configuration_markdown() -> HighlightConfiguration {
        HighlightConfiguration::new(
            GRAMMAR_MARKDOWN.into(),
            "markdown",
            HIGHLIGHTS_MARKDOWN,
            INJECTIONS_MARKDOWN,
            "",
        )
        .unwrap()
    }

    // markdown-inline
    pub const GRAMMAR_MARKDOWN_INLINE: LanguageFn = tree_sitter_md::INLINE_LANGUAGE;
    pub const HIGHLIGHTS_MARKDOWN_INLINE: &str =
        include_str!("../queries/markdown-inline/highlights.scm");
    pub fn highlight_configuration_markdown_inline() -> HighlightConfiguration {
        HighlightConfiguration::new(
            GRAMMAR_MARKDOWN_INLINE.into(),
            "markdown-inline",
            HIGHLIGHTS_MARKDOWN_INLINE,
            "",
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
#[cfg(feature = "scheme")]
pub mod scheme {
    use tree_sitter_highlight::HighlightConfiguration;
    use tree_sitter_language::LanguageFn;

    pub const GRAMMAR: LanguageFn = tree_sitter_scheme::LANGUAGE;
    pub const HIGHLIGHTS: &str = include_str!("../queries/scheme/highlights.scm");
    pub const INJECTIONS: &str = include_str!("../queries/scheme/injections.scm");
    pub fn highlight_configuration() -> HighlightConfiguration {
        HighlightConfiguration::new(GRAMMAR.into(), "scheme", HIGHLIGHTS, INJECTIONS, "").unwrap()
    }
}
#[cfg(feature = "sql")]
pub mod sql {
    use tree_sitter_highlight::HighlightConfiguration;
    use tree_sitter_language::LanguageFn;

    pub const GRAMMAR: LanguageFn = tree_sitter_sequel::LANGUAGE;
    pub const HIGHLIGHTS: &str = include_str!("../queries/sql/highlights.scm");
    pub fn highlight_configuration() -> HighlightConfiguration {
        HighlightConfiguration::new(GRAMMAR.into(), "sql", HIGHLIGHTS, "", "").unwrap()
    }
}
#[cfg(feature = "swift")]
pub mod swift {
    use tree_sitter_highlight::HighlightConfiguration;
    use tree_sitter_language::LanguageFn;

    pub const GRAMMAR: LanguageFn = tree_sitter_swift::LANGUAGE;
    pub const HIGHLIGHTS: &str = include_str!("../queries/swift/highlights.scm");
    pub const INJECTIONS: &str = include_str!("../queries/swift/injections.scm");
    pub const LOCALS: &str = include_str!("../queries/swift/locals.scm");
    pub fn highlight_configuration() -> HighlightConfiguration {
        HighlightConfiguration::new(GRAMMAR.into(), "swift", HIGHLIGHTS, INJECTIONS, LOCALS)
            .unwrap()
    }
}
#[cfg(feature = "toml")]
pub mod toml {
    use tree_sitter_highlight::HighlightConfiguration;
    use tree_sitter_language::LanguageFn;

    pub const GRAMMAR: LanguageFn = tree_sitter_toml_ng::LANGUAGE;
    pub const HIGHLIGHTS: &str = include_str!("../queries/toml/highlights.scm");
    pub const INJECTIONS: &str = include_str!("../queries/toml/injections.scm");
    pub fn highlight_configuration() -> HighlightConfiguration {
        HighlightConfiguration::new(GRAMMAR.into(), "toml", HIGHLIGHTS, INJECTIONS, "").unwrap()
    }
}
#[cfg(feature = "typescript")]
pub mod typescript {
    use tree_sitter_highlight::HighlightConfiguration;
    use tree_sitter_language::LanguageFn;

    // typescript
    pub const GRAMMAR_TYPESCRIPT: LanguageFn = tree_sitter_typescript::LANGUAGE_TYPESCRIPT;
    pub const HIGHLIGHTS_TYPESCRIPT: &str = include_str!("../queries/typescript/highlights.scm");
    pub const INJECTIONS_TYPESCRIPT: &str = include_str!("../queries/typescript/injections.scm");
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

    // tsx
    pub const GRAMMAR_TSX: LanguageFn = tree_sitter_typescript::LANGUAGE_TSX;
    pub const HIGHLIGHTS_TSX: &str = include_str!("../queries/tsx/highlights.scm");
    pub const INJECTIONS_TSX: &str = include_str!("../queries/tsx/injections.scm");
    pub fn highlight_configuration_tsx() -> HighlightConfiguration {
        HighlightConfiguration::new(
            GRAMMAR_TSX.into(),
            "tsx",
            HIGHLIGHTS_TSX,
            INJECTIONS_TSX,
            "",
        )
        .unwrap()
    }
}
#[cfg(feature = "xml")]
pub mod xml {
    use tree_sitter_highlight::HighlightConfiguration;
    use tree_sitter_language::LanguageFn;

    pub const GRAMMAR: LanguageFn = tree_sitter_xml::LANGUAGE_XML;
    pub const HIGHLIGHTS: &str = include_str!("../queries/xml/highlights.scm");
    pub fn highlight_configuration() -> HighlightConfiguration {
        HighlightConfiguration::new(GRAMMAR.into(), "xml", HIGHLIGHTS, "", "").unwrap()
    }
}
#[cfg(feature = "yaml")]
pub mod yaml {
    use tree_sitter_highlight::HighlightConfiguration;
    use tree_sitter_language::LanguageFn;

    pub const GRAMMAR: LanguageFn = tree_sitter_yaml::LANGUAGE;
    pub const HIGHLIGHTS: &str = include_str!("../queries/yaml/highlights.scm");
    pub const INJECTIONS: &str = include_str!("../queries/yaml/injections.scm");
    pub fn highlight_configuration() -> HighlightConfiguration {
        HighlightConfiguration::new(GRAMMAR.into(), "yaml", HIGHLIGHTS, INJECTIONS, "").unwrap()
    }
}
