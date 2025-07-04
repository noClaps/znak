package parser

import (
	"fmt"
	"regexp"
	"strings"

	"github.com/noclaps/znak/highlight"
)

type listType int

const (
	orderedList listType = iota
	unorderedList
)

func listItems(input string, codeTheme highlight.Theme, listType listType) ([]node, error) {
	var re *regexp.Regexp
	var segMatch string
	switch listType {
	case orderedList:
		re = regexp.MustCompile(`(?m)^\d+\. `)
		segMatch = "   "
	case unorderedList:
		re = regexp.MustCompile(`(?m)^- `)
		segMatch = "  "
	}

	reSplitInput := re.Split(input, -1)
	lines := []string{}
	for _, line := range reSplitInput {
		if line == "" {
			continue
		}
		lines = append(lines, strings.TrimSpace(line))
	}

	tokens := []node{}
	for _, line := range lines {
		segments := strings.Split(line, "\n")
		if len(segments) == 1 {
			children, err := Parse(line, codeTheme)
			if err != nil {
				return nil, err
			}
			tokens = append(tokens, element{TagName: "li", Children: children})
		} else {
			remainingLines := ""
			for _, line := range segments[1:] {
				remainingLines += fmt.Sprintln(strings.Replace(line, segMatch, "", 1))
			}
			input := fmt.Sprintf("%s\n\n%s", segments[0], remainingLines)
			children, err := Parse(input, codeTheme)
			if err != nil {
				return nil, err
			}
			tokens = append(tokens, element{TagName: "li", Children: children})
		}
	}

	return tokens, nil
}
