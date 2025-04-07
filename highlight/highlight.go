package highlight

import (
	"bytes"
	"context"
	"fmt"
	"html"
	"log"
	"maps"
	"slices"
	"strings"

	tsHighlight "go.gopad.dev/go-tree-sitter-highlight"
)

func highlightCode(highlightNames []string, language string, code string) (string, error) {
	highlighter := tsHighlight.New()

	lang, err := parseLanguage(language)
	if err != nil {
		return "", err
	}

	config, err := tsHighlight.NewConfiguration(lang.lang, lang.name, lang.highlightsQuery, lang.injectionQuery, lang.localsQuery)
	if err != nil {
		return "", err
	}
	config.Configure(highlightNames)

	highlightClasses := []string{}
	for _, name := range highlightNames {
		highlightClasses = append(highlightClasses, fmt.Sprintf(`class="%s"`, name))
	}

	highlights := highlighter.Highlight(context.Background(), *config, []byte(code), func(languageName string) *tsHighlight.Configuration {
		captureLang, err := parseLanguage(languageName)
		if err != nil {
			log.Println(err) // This error shouldn't crash the program, only skip injection highlighting
			return nil
		}
		nestedConfig, err := tsHighlight.NewConfiguration(captureLang.lang, captureLang.name, captureLang.highlightsQuery, captureLang.injectionQuery, captureLang.localsQuery)
		if err != nil {
			log.Println(err)
			return nil
		}
		nestedConfig.Configure(highlightNames)
		return nestedConfig
	})

	buf := new(bytes.Buffer)
	htmlRenderer := tsHighlight.NewHTMLRender()
	htmlRenderer.Render(buf, highlights, []byte(code), func(h tsHighlight.Highlight, languageName string) []byte {
		return []byte(highlightClasses[h])
	})

	return buf.String(), nil
}

func Highlight(code string, language string, theme Theme) (string, error) {
	globalStyle := ""
	if theme.BackgroundColor != nil {
		globalStyle += fmt.Sprintf(`background-color:%s;`, *theme.BackgroundColor)
	}
	if theme.Color != nil {
		globalStyle += fmt.Sprintf(`color:%s;`, *theme.Color)
	}

	if language == "plaintext" || language == "plain" || language == "text" || language == "txt" {
		return fmt.Sprintf(
			`<pre class="ts-highlight" style="%s"><code>%s</code></pre>`,
			globalStyle, html.EscapeString(strings.TrimSpace(code)),
		), nil
	}

	highlightNames := []string{}
	if theme.Highlights != nil {
		highlightNames = slices.Collect(maps.Keys(*theme.Highlights))
	}
	highlightedText, err := highlightCode(highlightNames, language, code)
	if err != nil {
		return "", err
	}

	if theme.Highlights != nil {
		for key, val := range *theme.Highlights {
			style := ""
			if val.Color != nil {
				style += fmt.Sprintf(`color:%s;`, *val.Color)
			}
			if val.FontWeight != nil {
				style += fmt.Sprintf(`font-weight:%d;`, *val.FontWeight)
			}
			if val.FontStyle != nil {
				style += fmt.Sprintf(`font-style:%s;`, *val.FontStyle)
			}
			if val.BackgroundColor != nil {
				style += fmt.Sprintf(`background-color:%s;`, *val.BackgroundColor)
			}

			highlightedText = strings.ReplaceAll(
				highlightedText,
				fmt.Sprintf(`<span class="%s"`, key),
				fmt.Sprintf(`<span class="%s" style="%s"`, key, style),
			)
		}
	}

	if theme.LineNumbers != nil {
		lines := slices.Collect(strings.Lines(highlightedText))
		maxLineNum := len(fmt.Sprint(len(lines) + 1))
		var rightSpace uint = 1
		if theme.LineNumbers.RightSpace != nil {
			rightSpace = *theme.LineNumbers.RightSpace
		}

		linesWithLineNos := []string{}
		for i, line := range lines {
			linesWithLineNos = append(linesWithLineNos, fmt.Sprintf(
				`<span class="line-number" style="color:%s;margin-right:%dch\">%d</span>%s`,
				theme.LineNumbers.Color,
				maxLineNum+int(rightSpace)-len(fmt.Sprint(i+1)),
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
