package highlight

import (
	"fmt"
	"html"
	"log"
	"maps"
	"slices"
	"strconv"
	"strings"

	tsh "github.com/noclaps/go-tree-sitter-highlight"
	tsh_types "github.com/noclaps/go-tree-sitter-highlight/types"
)

func Highlight(code string, language string, theme Theme) (string, error) {
	globalStyle := theme.Root

	if language == "plaintext" || language == "plain" || language == "text" || language == "txt" {
		return fmt.Sprintf(
			`<pre class="ts-highlight" style="%s"><code>%s</code></pre>`,
			globalStyle, html.EscapeString(strings.TrimSpace(code)),
		), nil
	}

	highlightNames := []string{}
	if theme.Highlights != nil {
		highlightNames = slices.Collect(maps.Keys(theme.Highlights))
	}

	lang, err := parseLanguage(language)
	if err != nil {
		return "", err
	}

	config, err := tsh.NewConfiguration(lang, highlightNames)
	if err != nil {
		return "", err
	}

	attributes := []string{}
	for _, key := range highlightNames {
		attribute := fmt.Sprintf(`class="%s"`, key)

		if theme.Highlights != nil {
			style, ok := theme.Highlights[key]
			if ok {
				attribute += fmt.Sprintf(` style="%s"`, style)
			}
		}

		attributes = append(attributes, attribute)
	}

	var injectionCallback tsh_types.InjectionCallback = func(languageName string) *tsh_types.Configuration {
		lang, err := parseLanguage(languageName)
		if err != nil {
			log.Println(err) // This error shouldn't crash the program, only skip injection highlighting
			return nil
		}
		config, err := tsh.NewConfiguration(lang, highlightNames)
		if err != nil {
			log.Println(err)
			return nil
		}
		return config
	}
	var attributeCallback tsh_types.AttributeCallback = func(h tsh_types.CaptureIndex, languageName string) string {
		return attributes[h]
	}
	highlightedText, err := tsh.Highlight(*config, code, injectionCallback, attributeCallback)
	if err != nil {
		return "", err
	}

	if theme.LineNumbers != struct {
		MarginRight string
		Styles      string
	}{} {
		lines := slices.Collect(strings.Lines(highlightedText))
		maxLineNum := len(fmt.Sprint(len(lines) + 1))
		rightSpace := 1

		if theme.LineNumbers.MarginRight != "" {
			rsVal, err := strconv.ParseInt(theme.LineNumbers.MarginRight, 10, 64)
			if err != nil {
				log.Println(err)
			}
			rightSpace = int(rsVal)
		}
		style := theme.LineNumbers.Styles

		linesWithLineNos := []string{}
		for i, line := range lines {
			linesWithLineNos = append(linesWithLineNos, fmt.Sprintf(
				`<span class="line-number" style="%s;margin-right:%dch\">%d</span>%s`,
				style,
				maxLineNum+rightSpace-len(fmt.Sprint(i+1)),
				i+1,
				line,
			))
		}
		highlightedText = strings.Join(linesWithLineNos, "\n")
	}

	return fmt.Sprintf(
		`<pre class="ts-highlight" style="%s"><code>%s</code></pre>`,
		globalStyle, strings.TrimSpace(highlightedText),
	), nil
}
