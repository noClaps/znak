package highlight

import (
	"embed"
	"fmt"

	"github.com/noclaps/go-tree-sitter-highlight/language"
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

//go:embed queries
var queries embed.FS

func parseLanguage(lang string) (language.Language, error) {
	switch lang {
	case "agda":
		highlights, err := queries.ReadFile("queries/agda/highlights.scm")
		if err != nil {
			return language.Language{}, err
		}
		return language.NewLanguage("agda", tree_sitter_agda.Language(), highlights, nil, nil), nil
	case "bash", "shellscript", "shell", "zsh", "sh":
		highlights, err := queries.ReadFile("queries/bash/highlights.scm")
		if err != nil {
			return language.Language{}, err
		}
		return language.NewLanguage("bash", tree_sitter_bash.Language(), highlights, nil, nil), nil
	case "c":
		highlights, err := queries.ReadFile("queries/c/highlights.scm")
		if err != nil {
			return language.Language{}, err
		}
		injections, err := queries.ReadFile("queries/c/injections.scm")
		if err != nil {
			return language.Language{}, err
		}
		return language.NewLanguage("c", tree_sitter_c.Language(), highlights, injections, nil), nil
	case "cpp", "c++":
		highlights, err := queries.ReadFile("queries/cpp/highlights.scm")
		if err != nil {
			return language.Language{}, err
		}
		injections, err := queries.ReadFile("queries/cpp/injections.scm")
		if err != nil {
			return language.Language{}, err
		}
		return language.NewLanguage("cpp", tree_sitter_cpp.Language(), highlights, injections, nil), nil
	case "css":
		highlights, err := queries.ReadFile("queries/css/highlights.scm")
		if err != nil {
			return language.Language{}, err
		}
		return language.NewLanguage("css", tree_sitter_css.Language(), highlights, nil, nil), nil
	case "go":
		highlights, err := queries.ReadFile("queries/go/highlights.scm")
		if err != nil {
			return language.Language{}, err
		}
		injections, err := queries.ReadFile("queries/go/injections.scm")
		if err != nil {
			return language.Language{}, err
		}
		return language.NewLanguage("go", tree_sitter_go.Language(), highlights, injections, nil), nil
	case "haskell", "hs":
		highlights, err := queries.ReadFile("queries/haskell/highlights.scm")
		if err != nil {
			return language.Language{}, err
		}
		return language.NewLanguage("haskell", tree_sitter_haskell.Language(), highlights, nil, nil), nil
	case "html":
		highlights, err := queries.ReadFile("queries/html/highlights.scm")
		if err != nil {
			return language.Language{}, err
		}
		injections, err := queries.ReadFile("queries/html/injections.scm")
		if err != nil {
			return language.Language{}, err
		}
		return language.NewLanguage("html", tree_sitter_html.Language(), highlights, injections, nil), nil
	case "java":
		highlights, err := queries.ReadFile("queries/java/highlights.scm")
		if err != nil {
			return language.Language{}, err
		}
		injections, err := queries.ReadFile("queries/java/injections.scm")
		if err != nil {
			return language.Language{}, err
		}
		locals, err := queries.ReadFile("queries/java/locals.scm")
		if err != nil {
			return language.Language{}, err
		}
		return language.NewLanguage("java", tree_sitter_java.Language(), highlights, injections, locals), nil
	case "javascript", "js", "jsx":
		highlights, err := queries.ReadFile("queries/javascript/highlights.scm")
		if err != nil {
			return language.Language{}, err
		}
		injections, err := queries.ReadFile("queries/javascript/injections.scm")
		if err != nil {
			return language.Language{}, err
		}
		return language.NewLanguage("javascript", tree_sitter_javascript.Language(), highlights, injections, nil), nil
	case "jsdoc":
		highlights, err := queries.ReadFile("queries/jsdoc/highlights.scm")
		if err != nil {
			return language.Language{}, err
		}
		return language.NewLanguage("jsdoc", tree_sitter_jsdoc.Language(), highlights, nil, nil), nil
	case "json":
		highlights, err := queries.ReadFile("queries/json/highlights.scm")
		if err != nil {
			return language.Language{}, err
		}
		return language.NewLanguage("json", tree_sitter_json.Language(), highlights, nil, nil), nil
	case "ocaml":
		highlights, err := queries.ReadFile("queries/ocaml/highlights.scm")
		if err != nil {
			return language.Language{}, err
		}
		return language.NewLanguage("ocaml", tree_sitter_ocaml.LanguageOCaml(), highlights, nil, nil), nil
	case "ocaml_interface":
		highlights, err := queries.ReadFile("queries/ocaml/highlights.scm")
		if err != nil {
			return language.Language{}, err
		}
		return language.NewLanguage("ocaml_interface", tree_sitter_ocaml.LanguageOCamlInterface(), highlights, nil, nil), nil
	case "ocaml_type":
		highlights, err := queries.ReadFile("queries/ocaml/highlights.scm")
		if err != nil {
			return language.Language{}, err
		}
		return language.NewLanguage("ocaml_type", tree_sitter_ocaml.LanguageOCamlType(), highlights, nil, nil), nil
	case "php":
		highlights, err := queries.ReadFile("queries/php/highlights.scm")
		if err != nil {
			return language.Language{}, err
		}
		injections, err := queries.ReadFile("queries/php/injections.scm")
		if err != nil {
			return language.Language{}, err
		}
		return language.NewLanguage("php", tree_sitter_php.LanguagePHP(), highlights, injections, nil), nil
	case "php_only":
		highlights, err := queries.ReadFile("queries/php/highlights.scm")
		if err != nil {
			return language.Language{}, err
		}
		injections, err := queries.ReadFile("queries/php/injections.scm")
		if err != nil {
			return language.Language{}, err
		}
		return language.NewLanguage("php_only", tree_sitter_php.LanguagePHPOnly(), highlights, injections, nil), nil
	case "python", "py":
		highlights, err := queries.ReadFile("queries/python/highlights.scm")
		if err != nil {
			return language.Language{}, err
		}
		return language.NewLanguage("python", tree_sitter_python.Language(), highlights, nil, nil), nil
	case "regexp", "regex":
		highlights, err := queries.ReadFile("queries/regex/highlights.scm")
		if err != nil {
			return language.Language{}, err
		}
		return language.NewLanguage("regex", tree_sitter_regex.Language(), highlights, nil, nil), nil
	case "ruby", "rb":
		highlights, err := queries.ReadFile("queries/ruby/highlights.scm")
		if err != nil {
			return language.Language{}, err
		}
		injections, err := queries.ReadFile("queries/ruby/injections.scm")
		if err != nil {
			return language.Language{}, err
		}
		return language.NewLanguage("ruby", tree_sitter_ruby.Language(), highlights, injections, nil), nil
	case "rust", "rs":
		highlights, err := queries.ReadFile("queries/rust/highlights.scm")
		if err != nil {
			return language.Language{}, err
		}
		injections, err := queries.ReadFile("queries/rust/injections.scm")
		if err != nil {
			return language.Language{}, err
		}
		return language.NewLanguage("rust", tree_sitter_rust.Language(), highlights, injections, nil), nil
	case "scala":
		highlights, err := queries.ReadFile("queries/scala/highlights.scm")
		if err != nil {
			return language.Language{}, err
		}
		injections, err := queries.ReadFile("queries/scala/injections.scm")
		if err != nil {
			return language.Language{}, err
		}
		return language.NewLanguage("scala", tree_sitter_scala.Language(), highlights, injections, nil), nil
	case "typescript", "ts":
		highlights, err := queries.ReadFile("queries/typescript/highlights.scm")
		if err != nil {
			return language.Language{}, err
		}
		injections, err := queries.ReadFile("queries/typescript/injections.scm")
		if err != nil {
			return language.Language{}, err
		}
		return language.NewLanguage("typescript", tree_sitter_typescript.LanguageTypescript(), highlights, injections, nil), nil
	case "tsx":
		highlights, err := queries.ReadFile("queries/tsx/highlights.scm")
		if err != nil {
			return language.Language{}, err
		}
		injections, err := queries.ReadFile("queries/tsx/injections.scm")
		if err != nil {
			return language.Language{}, err
		}
		return language.NewLanguage("tsx", tree_sitter_typescript.LanguageTSX(), highlights, injections, nil), nil
	}

	return language.Language{}, fmt.Errorf("Language not found: %s", lang)
}
