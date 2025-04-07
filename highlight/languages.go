package highlight

import (
	"embed"
	"fmt"

	tree_sitter "github.com/tree-sitter/go-tree-sitter"
	tree_sitter_agda "github.com/tree-sitter/tree-sitter-agda/bindings/go"
	tree_sitter_bash "github.com/tree-sitter/tree-sitter-bash/bindings/go"
	tree_sitter_c "github.com/tree-sitter/tree-sitter-c/bindings/go"
	tree_sitter_cpp "github.com/tree-sitter/tree-sitter-cpp/bindings/go"
	tree_sitter_css "github.com/tree-sitter/tree-sitter-css/bindings/go"
	tree_sitter_go "github.com/tree-sitter/tree-sitter-go/bindings/go"
	tree_sitter_haskell "github.com/tree-sitter/tree-sitter-haskell/bindings/go"
	tree_sitter_html "github.com/tree-sitter/tree-sitter-html/bindings/go"
	tree_sitter_java "github.com/tree-sitter/tree-sitter-java/bindings/go"
	tree_sitter_javascript "github.com/tree-sitter/tree-sitter-javascript/bindings/go"
	tree_sitter_jsdoc "github.com/tree-sitter/tree-sitter-jsdoc/bindings/go"
	tree_sitter_json "github.com/tree-sitter/tree-sitter-json/bindings/go"
	tree_sitter_ocaml "github.com/tree-sitter/tree-sitter-ocaml/bindings/go"
	tree_sitter_php "github.com/tree-sitter/tree-sitter-php/bindings/go"
	tree_sitter_python "github.com/tree-sitter/tree-sitter-python/bindings/go"
	tree_sitter_regex "github.com/tree-sitter/tree-sitter-regex/bindings/go"
	tree_sitter_ruby "github.com/tree-sitter/tree-sitter-ruby/bindings/go"
	tree_sitter_rust "github.com/tree-sitter/tree-sitter-rust/bindings/go"
	tree_sitter_scala "github.com/tree-sitter/tree-sitter-scala/bindings/go"
	tree_sitter_typescript "github.com/tree-sitter/tree-sitter-typescript/bindings/go"
)

type language struct {
	name            string
	highlightsQuery []byte
	injectionQuery  []byte
	localsQuery     []byte
	lang            *tree_sitter.Language
}

//go:embed queries
var queries embed.FS

func parseLanguage(lang string) (language, error) {
	switch lang {
	case "agda":
		highlights, err := queries.ReadFile("queries/agda/highlights.scm")
		if err != nil {
			return language{}, err
		}
		return language{
			name:            "agda",
			highlightsQuery: highlights,
			lang:            tree_sitter.NewLanguage(tree_sitter_agda.Language()),
		}, nil
	case "bash", "shellscript", "shell", "zsh", "sh":
		highlights, err := queries.ReadFile("queries/bash/highlights.scm")
		if err != nil {
			return language{}, err
		}
		return language{
			name:            "bash",
			highlightsQuery: highlights,
			lang:            tree_sitter.NewLanguage(tree_sitter_bash.Language()),
		}, nil
	case "c":
		highlights, err := queries.ReadFile("queries/c/highlights.scm")
		if err != nil {
			return language{}, err
		}
		injections, err := queries.ReadFile("queries/c/injections.scm")
		if err != nil {
			return language{}, err
		}
		return language{
			name:            "c",
			highlightsQuery: highlights,
			injectionQuery:  injections,
			lang:            tree_sitter.NewLanguage(tree_sitter_c.Language()),
		}, nil
	case "cpp", "c++":
		highlights, err := queries.ReadFile("queries/cpp/highlights.scm")
		if err != nil {
			return language{}, err
		}
		injections, err := queries.ReadFile("queries/cpp/injections.scm")
		if err != nil {
			return language{}, err
		}
		return language{
			name:            "cpp",
			highlightsQuery: highlights,
			injectionQuery:  injections,
			lang:            tree_sitter.NewLanguage(tree_sitter_cpp.Language()),
		}, nil
	case "css":
		highlights, err := queries.ReadFile("queries/css/highlights.scm")
		if err != nil {
			return language{}, err
		}
		return language{
			name:            "css",
			highlightsQuery: highlights,
			lang:            tree_sitter.NewLanguage(tree_sitter_css.Language()),
		}, nil
	case "go":
		highlights, err := queries.ReadFile("queries/go/highlights.scm")
		if err != nil {
			return language{}, err
		}
		injections, err := queries.ReadFile("queries/go/injections.scm")
		if err != nil {
			return language{}, err
		}
		return language{
			name:            "go",
			highlightsQuery: highlights,
			injectionQuery:  injections,
			lang:            tree_sitter.NewLanguage(tree_sitter_go.Language()),
		}, nil
	case "haskell", "hs":
		highlights, err := queries.ReadFile("queries/haskell/highlights.scm")
		if err != nil {
			return language{}, err
		}
		return language{
			name:            "haskell",
			highlightsQuery: highlights,
			lang:            tree_sitter.NewLanguage(tree_sitter_haskell.Language()),
		}, nil
	case "html":
		highlights, err := queries.ReadFile("queries/html/highlights.scm")
		if err != nil {
			return language{}, err
		}
		injections, err := queries.ReadFile("queries/html/injections.scm")
		if err != nil {
			return language{}, err
		}
		return language{
			name:            "html",
			highlightsQuery: highlights,
			injectionQuery:  injections,
			lang:            tree_sitter.NewLanguage(tree_sitter_html.Language()),
		}, nil
	case "java":
		highlights, err := queries.ReadFile("queries/java/highlights.scm")
		if err != nil {
			return language{}, err
		}
		injections, err := queries.ReadFile("queries/java/injections.scm")
		if err != nil {
			return language{}, err
		}
		locals, err := queries.ReadFile("queries/java/locals.scm")
		if err != nil {
			return language{}, err
		}
		return language{
			name:            "java",
			highlightsQuery: highlights,
			injectionQuery:  injections,
			localsQuery:     locals,
			lang:            tree_sitter.NewLanguage(tree_sitter_java.Language()),
		}, nil
	case "javascript", "js", "jsx":
		highlights, err := queries.ReadFile("queries/javascript/highlights.scm")
		if err != nil {
			return language{}, err
		}
		injections, err := queries.ReadFile("queries/javascript/injections.scm")
		if err != nil {
			return language{}, err
		}
		return language{
			name:            "javascript",
			highlightsQuery: highlights,
			injectionQuery:  injections,
			lang:            tree_sitter.NewLanguage(tree_sitter_javascript.Language()),
		}, nil
	case "jsdoc":
		highlights, err := queries.ReadFile("queries/jsdoc/highlights.scm")
		if err != nil {
			return language{}, err
		}
		return language{
			name:            "jsdoc",
			highlightsQuery: highlights,
			lang:            tree_sitter.NewLanguage(tree_sitter_jsdoc.Language()),
		}, nil
	case "json":
		highlights, err := queries.ReadFile("queries/json/highlights.scm")
		if err != nil {
			return language{}, err
		}
		return language{
			name:            "json",
			highlightsQuery: highlights,
			lang:            tree_sitter.NewLanguage(tree_sitter_json.Language()),
		}, nil
	case "ocaml":
		highlights, err := queries.ReadFile("queries/ocaml/highlights.scm")
		if err != nil {
			return language{}, err
		}
		return language{
			name:            "ocaml",
			highlightsQuery: highlights,
			lang:            tree_sitter.NewLanguage(tree_sitter_ocaml.LanguageOCaml()),
		}, nil
	case "ocaml_interface":
		highlights, err := queries.ReadFile("queries/ocaml/highlights.scm")
		if err != nil {
			return language{}, err
		}
		return language{
			name:            "ocaml_interface",
			highlightsQuery: highlights,
			lang:            tree_sitter.NewLanguage(tree_sitter_ocaml.LanguageOCamlInterface()),
		}, nil
	case "ocaml_type":
		highlights, err := queries.ReadFile("queries/ocaml/highlights.scm")
		if err != nil {
			return language{}, err
		}
		return language{
			name:            "ocaml_type",
			highlightsQuery: highlights,
			lang:            tree_sitter.NewLanguage(tree_sitter_ocaml.LanguageOCamlType()),
		}, nil
	case "php":
		highlights, err := queries.ReadFile("queries/php/highlights.scm")
		if err != nil {
			return language{}, err
		}
		injections, err := queries.ReadFile("queries/php/injections.scm")
		if err != nil {
			return language{}, err
		}
		return language{
			name:            "php",
			highlightsQuery: highlights,
			injectionQuery:  injections,
			lang:            tree_sitter.NewLanguage(tree_sitter_php.LanguagePHP()),
		}, nil
	case "php_only":
		highlights, err := queries.ReadFile("queries/php/highlights.scm")
		if err != nil {
			return language{}, err
		}
		injections, err := queries.ReadFile("queries/php/injections.scm")
		if err != nil {
			return language{}, err
		}
		return language{
			name:            "php_only",
			highlightsQuery: highlights,
			injectionQuery:  injections,
			lang:            tree_sitter.NewLanguage(tree_sitter_php.LanguagePHPOnly()),
		}, nil
	case "python", "py":
		highlights, err := queries.ReadFile("queries/python/highlights.scm")
		if err != nil {
			return language{}, err
		}
		return language{
			name:            "python",
			highlightsQuery: highlights,
			lang:            tree_sitter.NewLanguage(tree_sitter_python.Language()),
		}, nil
	case "regexp", "regex":
		highlights, err := queries.ReadFile("queries/regex/highlights.scm")
		if err != nil {
			return language{}, err
		}
		return language{
			name:            "regex",
			highlightsQuery: highlights,
			lang:            tree_sitter.NewLanguage(tree_sitter_regex.Language()),
		}, nil
	case "ruby", "rb":
		highlights, err := queries.ReadFile("queries/ruby/highlights.scm")
		if err != nil {
			return language{}, err
		}
		injections, err := queries.ReadFile("queries/ruby/injections.scm")
		if err != nil {
			return language{}, err
		}
		return language{
			name:            "ruby",
			highlightsQuery: highlights,
			injectionQuery:  injections,
			lang:            tree_sitter.NewLanguage(tree_sitter_ruby.Language()),
		}, nil
	case "rust", "rs":
		highlights, err := queries.ReadFile("queries/rust/highlights.scm")
		if err != nil {
			return language{}, err
		}
		injections, err := queries.ReadFile("queries/rust/injections.scm")
		if err != nil {
			return language{}, err
		}
		return language{
			name:            "rust",
			highlightsQuery: highlights,
			injectionQuery:  injections,
			lang:            tree_sitter.NewLanguage(tree_sitter_rust.Language()),
		}, nil
	case "scala":
		highlights, err := queries.ReadFile("queries/scala/highlights.scm")
		if err != nil {
			return language{}, err
		}
		injections, err := queries.ReadFile("queries/scala/injections.scm")
		if err != nil {
			return language{}, err
		}
		return language{
			name:            "scala",
			highlightsQuery: highlights,
			injectionQuery:  injections,
			lang:            tree_sitter.NewLanguage(tree_sitter_scala.Language()),
		}, nil
	case "typescript", "ts":
		highlights, err := queries.ReadFile("queries/typescript/highlights.scm")
		if err != nil {
			return language{}, err
		}
		injections, err := queries.ReadFile("queries/typescript/injections.scm")
		if err != nil {
			return language{}, err
		}
		return language{
			name:            "typescript",
			highlightsQuery: highlights,
			injectionQuery:  injections,
			lang:            tree_sitter.NewLanguage(tree_sitter_typescript.LanguageTypescript()),
		}, nil
	case "tsx":
		highlights, err := queries.ReadFile("queries/tsx/highlights.scm")
		if err != nil {
			return language{}, err
		}
		injections, err := queries.ReadFile("queries/tsx/injections.scm")
		if err != nil {
			return language{}, err
		}
		return language{
			name:            "tsx",
			highlightsQuery: highlights,
			injectionQuery:  injections,
			lang:            tree_sitter.NewLanguage(tree_sitter_typescript.LanguageTSX()),
		}, nil
	}

	return language{}, fmt.Errorf("Language not found: %s", lang)
}
