package parser

import (
	"github.com/noclaps/znak/highlight"
)

func highlightSyntax(code string, language string, theme highlight.Theme) (node, error) {
	highlightedText, err := highlight.Highlight(code, language, theme)
	if err != nil {
		return nil, err
	}

	return text{highlightedText}, nil
}
