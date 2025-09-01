package highlight

import (
	"bytes"
	"fmt"
)

type Theme struct {
	Root        string
	LineNumbers struct {
		MarginRight string
		Styles      string
	}
	Highlights map[string]string
}

func NewTheme(css []byte) (Theme, error) {
	theme := Theme{}

	css = bytes.ReplaceAll(css, []byte("\n"), []byte(""))
	css = bytes.ReplaceAll(css, []byte(" "), []byte(""))
	css = bytes.ReplaceAll(css, []byte("\t"), []byte(""))

	// remove all comments
	minifiedCss := []byte{}
	for i := 0; i < len(css); i++ {
		if bytes.HasPrefix(css[i:], []byte("/*")) {
			commentClose := bytes.Index(css[i:], []byte("*/"))
			if commentClose == -1 {
				return Theme{}, fmt.Errorf("Unterminated comment found")
			}
			i = i + commentClose + 2
		}
		minifiedCss = append(minifiedCss, css[i])
	}
	css = minifiedCss

	for i := 0; i < len(css); i++ {
		// parsing selectors
		openBrace := bytes.IndexByte(css[i:], '{')
		if openBrace == -1 {
			return Theme{}, fmt.Errorf("No opening brace found in theme")
		}
		selectors := bytes.SplitSeq(css[i:i+openBrace], []byte(","))
		i = i + openBrace + 1

		// parsing styles
		closeBrace := bytes.IndexByte(css[i:], '}')
		if closeBrace == -1 {
			return Theme{}, fmt.Errorf("Mismatched opening and closing braces, closing brace not found")
		}
		styles := css[i : i+closeBrace]
		i = i + closeBrace

		for selector := range selectors {
			switch string(selector) {
			case ":root":
				theme.Root = string(styles)
			case ":line-numbers":
				lineNumberStyles := ""
				for style := range bytes.SplitSeq(styles, []byte(";")) {
					if string(style) == "" {
						continue
					}
					parts := bytes.SplitN(style, []byte(":"), 2)
					if string(parts[0]) == "margin-right" {
						theme.LineNumbers.MarginRight = string(parts[1])
						continue
					}
					lineNumberStyles += string(style) + ";"
				}
				theme.LineNumbers.Styles = lineNumberStyles
			default:
				if theme.Highlights == nil {
					theme.Highlights = make(map[string]string)
				}
				theme.Highlights[string(selector)] = string(styles)
			}
		}
	}

	return theme, nil
}
