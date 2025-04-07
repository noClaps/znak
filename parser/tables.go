package parser

import (
	"strings"
)

func tables(input string) (node, error) {
	lines := strings.Split(strings.TrimSpace(input), "\n")
	headRow := lines[0]
	alignRow := lines[1]
	tbody := lines[2:]

	thead := strings.Split(headRow[1:len(headRow)-1], "|")
	for i := range thead {
		thead[i] = strings.TrimSpace(thead[i])
	}

	alignments := strings.Split(alignRow[1:len(alignRow)-1], "|")
	for i := range alignments {
		col := strings.TrimSpace(alignments[i])
		switch {
		case col[0] == ':' && col[len(col)-1] == ':':
			alignments[i] = "center"
		case col[len(col)-1] == ':':
			alignments[i] = "right"
		case col[0] == ':':
			alignments[i] = "left"
		default:
			alignments[i] = ""
		}
	}

	theadNodes := []node{}
	for i, th := range thead {
		children, err := inlineFormatting(th)
		if err != nil {
			return nil, err
		}
		theadNodes = append(theadNodes, element{"th", map[string]string{"align": alignments[i]}, children})
	}

	tbodyNodes := []node{}
	for _, line := range tbody {
		trNodes := []node{}
		for i, col := range strings.Split(line[1:len(line)-1], "|") {
			children, err := inlineFormatting(strings.TrimSpace(col))
			if err != nil {
				return nil, err
			}
			trNodes = append(trNodes, element{"td", map[string]string{"align": alignments[i]}, children})
		}
		tbodyNodes = append(tbodyNodes, element{TagName: "tr", Children: trNodes})
	}

	return element{TagName: "table", Children: []node{
		element{TagName: "thead", Children: []node{
			element{TagName: "tr", Children: theadNodes},
		}},
		element{TagName: "tbody", Children: tbodyNodes},
	}}, nil
}
