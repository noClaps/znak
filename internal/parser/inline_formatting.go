package parser

import (
	"fmt"
	"html"
	"strings"
)

func inlineFormatting(line string) ([]node, error) {
	contents := []node{}
	buffer := ""

	for cursor := 0; cursor < len(line); cursor++ {
		if line[cursor] == '\\' {
			cursor++
			buffer += line[cursor : cursor+1]
			continue
		}

		// Bold (**)
		if strings.HasPrefix(line[cursor:], "**") && strings.Contains(line[cursor+2:], "**") {
			if buffer != "" {
				contents = append(contents, text{buffer})
				buffer = ""
			}
			nextIndex := cursor + 2 + strings.Index(line[cursor+2:], "**")
			tempBuf := line[cursor+2 : nextIndex]

			if tempBuf == "" {
				contents = append(contents, text{"****"})
			} else {
				children, err := inlineFormatting(tempBuf)
				if err != nil {
					return nil, err
				}
				contents = append(contents, element{TagName: "strong", Children: children})
			}
			cursor = nextIndex + 1
			continue
		}

		// Underline (__)
		if strings.HasPrefix(line[cursor:], "__") && strings.Contains(line[cursor+2:], "__") {
			if buffer != "" {
				contents = append(contents, text{buffer})
				buffer = ""
			}
			nextIndex := cursor + 2 + strings.Index(line[cursor+2:], "__")
			tempBuf := line[cursor+2 : nextIndex]

			if tempBuf == "" {
				contents = append(contents, text{"____"})
			} else {
				children, err := inlineFormatting(tempBuf)
				if err != nil {
					return nil, err
				}
				contents = append(contents, element{TagName: "u", Children: children})
			}
			cursor = nextIndex + 1
			continue
		}

		// Strikethrough (~~)
		if strings.HasPrefix(line[cursor:], "~~") && strings.Contains(line[cursor+2:], "~~") {
			if buffer != "" {
				contents = append(contents, text{buffer})
				buffer = ""
			}
			nextIndex := cursor + 2 + strings.Index(line[cursor+2:], "~~")
			tempBuf := line[cursor+2 : nextIndex]

			if tempBuf == "" {
				contents = append(contents, text{"~~~~"})
			} else {
				children, err := inlineFormatting(tempBuf)
				if err != nil {
					return nil, err
				}
				contents = append(contents, element{TagName: "s", Children: children})
			}
			cursor = nextIndex + 1
			continue
		}

		// Highlight (==)
		if strings.HasPrefix(line[cursor:], "==") && strings.Contains(line[cursor+2:], "==") {
			if buffer != "" {
				contents = append(contents, text{buffer})
				buffer = ""
			}
			nextIndex := cursor + 2 + strings.Index(line[cursor+2:], "==")
			tempBuf := line[cursor+2 : nextIndex]

			if tempBuf == "" {
				contents = append(contents, text{"===="})
			} else {
				children, err := inlineFormatting(tempBuf)
				if err != nil {
					return nil, err
				}
				contents = append(contents, element{TagName: "mark", Children: children})
			}
			cursor = nextIndex + 1
			continue
		}

		// Inline math ($$)
		if strings.HasPrefix(line[cursor:], "$$") && strings.Contains(line[cursor+2:], "$$") {
			if buffer != "" {
				contents = append(contents, text{buffer})
				buffer = ""
			}
			nextIndex := cursor + 2 + strings.Index(line[cursor+2:], "$$")
			tempBuf := line[cursor+2 : nextIndex]
			if tempBuf == "" {
				contents = append(contents, text{"$$$$"})
			} else {
				math, err := renderMath(tempBuf, false)
				if err != nil {
					return nil, err
				}
				contents = append(contents, math)
			}
			cursor = nextIndex + 1
			continue
		}

		// Italics (_)
		if line[cursor] == '_' && strings.Contains(line[cursor+1:], "_") {
			if buffer != "" {
				contents = append(contents, text{buffer})
				buffer = ""
			}
			nextIndex := cursor + 1 + strings.Index(line[cursor+1:], "_")
			tempBuf := line[cursor+1 : nextIndex]
			if tempBuf == "" {
				contents = append(contents, text{"__"})
			} else {
				children, err := inlineFormatting(tempBuf)
				if err != nil {
					return nil, err
				}
				contents = append(contents, element{TagName: "em", Children: children})
			}
			cursor = nextIndex
			continue
		}

		// Code (`)
		if cursor+1 < len(line) && line[cursor] == '`' && strings.Contains(line[cursor+1:], "`") {
			if buffer != "" {
				contents = append(contents, text{buffer})
				buffer = ""
			}
			nextIndex := cursor + 1 + strings.Index(line[cursor+1:], "`")
			tempBuf := line[cursor+1 : nextIndex]
			if tempBuf == "" {
				contents = append(contents, text{"``"})
			} else {
				contents = append(contents, element{TagName: "code", Children: []node{
					text{html.EscapeString(tempBuf)},
				}})
			}

			cursor = nextIndex
			continue
		}

		// Subscript (~)
		if line[cursor] == '~' && strings.Contains(line[cursor+1:], "~") {
			if buffer != "" {
				contents = append(contents, text{buffer})
				buffer = ""
			}
			nextIndex := cursor + 1 + strings.Index(line[cursor+1:], "~")
			tempBuf := line[cursor+1 : nextIndex]
			if tempBuf == "" {
				contents = append(contents, text{"~~"})
			} else {
				children, err := inlineFormatting(tempBuf)
				if err != nil {
					return nil, err
				}
				contents = append(contents, element{TagName: "sub", Children: children})
			}
			cursor = nextIndex
			continue
		}

		// Superscript (^)
		if line[cursor] == '^' && strings.Contains(line[cursor+1:], "^") {
			if buffer != "" {
				contents = append(contents, text{buffer})
				buffer = ""
			}
			nextIndex := cursor + 1 + strings.Index(line[cursor+1:], "^")
			tempBuf := line[cursor+1 : nextIndex]
			if tempBuf == "" {
				contents = append(contents, text{"^^"})
			} else {
				children, err := inlineFormatting(tempBuf)
				if err != nil {
					return nil, err
				}
				contents = append(contents, element{TagName: "sup", Children: children})
			}
			cursor = nextIndex
			continue
		}

		// Links
		if line[cursor] == '[' && strings.Contains(line[cursor+1:], "](") && strings.Contains(line[cursor+1:], ")") {
			if buffer != "" {
				contents = append(contents, text{buffer})
				buffer = ""
			}
			linkTitle := ""
			isInsideNested := false
			cursor++ // Move inside link title
			for line[cursor] != ']' || isInsideNested {
				if line[cursor] == '[' {
					isInsideNested = true
				}
				if line[cursor] == ']' {
					isInsideNested = false
				}
				linkTitle += line[cursor : cursor+1]
				cursor++
			}

			linkUrl := ""
			isInsideLink := false
			cursor += 2 // Move inside link url
			for line[cursor] != ')' || isInsideLink {
				if line[cursor] == '<' {
					isInsideLink = true
					cursor++
					continue
				}
				if line[cursor] == '>' {
					isInsideLink = false
					cursor++
					continue
				}
				linkUrl += line[cursor : cursor+1]
				cursor++
			}

			if linkTitle == "" || linkUrl == "" {
				contents = append(contents, text{fmt.Sprintf("[%s](%s)", linkTitle, linkUrl)})
			} else {
				children, err := inlineFormatting(linkTitle)
				if err != nil {
					return nil, err
				}
				contents = append(contents, element{"a", map[string]string{"href": linkUrl}, children})
			}

			continue
		}

		// Default
		buffer += line[cursor : cursor+1]
	}

	if buffer != "" {
		contents = append(contents, text{buffer})
	}

	return contents, nil
}
