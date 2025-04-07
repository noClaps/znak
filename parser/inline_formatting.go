package parser

import (
	"fmt"
	"html"
	"slices"
	"strings"
)

func inlineFormatting(input string) ([]node, error) {
	contents := []node{}
	buffer := ""
	cursor := 0
	line := []rune(input)

	for cursor < len(line) {
		if line[cursor] == '\\' {
			cursor++
			buffer += string(line[cursor])
			cursor++
			continue
		}

		// Bold (**)
		if cursor+2 < len(line) && string(line[cursor:cursor+2]) == "**" && strings.Contains(string(line[cursor+2:]), "**") {
			if buffer != "" {
				contents = append(contents, text{buffer})
				buffer = ""
			}
			nextIndex := cursor + 2 + strings.Index(string(line[cursor+2:]), "**")
			tempBuf := string(line[cursor+2 : nextIndex])

			if tempBuf == "" {
				contents = append(contents, text{"****"})
			} else {
				children, err := inlineFormatting(tempBuf)
				if err != nil {
					return nil, err
				}
				contents = append(contents, element{TagName: "strong", Children: children})
			}
			cursor = nextIndex + 2
			continue
		}

		// Underline (__)
		if cursor+2 < len(line) && string(line[cursor:cursor+2]) == "__" && strings.Contains(string(line[cursor+2:]), "__") {
			if buffer != "" {
				contents = append(contents, text{buffer})
				buffer = ""
			}
			nextIndex := cursor + 2 + strings.Index(string(line[cursor+2:]), "__")
			tempBuf := string(line[cursor+2 : nextIndex])

			if tempBuf == "" {
				contents = append(contents, text{"____"})
			} else {
				children, err := inlineFormatting(tempBuf)
				if err != nil {
					return nil, err
				}
				contents = append(contents, element{TagName: "u", Children: children})
			}
			cursor = nextIndex + 2
			continue
		}

		// Strikethrough (~~)
		if cursor+2 < len(line) && string(line[cursor:cursor+2]) == "~~" && strings.Contains(string(line[cursor+2:]), "~~") {
			if buffer != "" {
				contents = append(contents, text{buffer})
				buffer = ""
			}
			nextIndex := cursor + 2 + strings.Index(string(line[cursor+2:]), "~~")
			tempBuf := string(line[cursor+2 : nextIndex])

			if tempBuf == "" {
				contents = append(contents, text{"~~~~"})
			} else {
				children, err := inlineFormatting(tempBuf)
				if err != nil {
					return nil, err
				}
				contents = append(contents, element{TagName: "s", Children: children})
			}
			cursor = nextIndex + 2
			continue
		}

		// Highlight (==)
		if cursor+2 < len(line) && string(line[cursor:cursor+2]) == "==" && strings.Contains(string(line[cursor+2:]), "==") {
			if buffer != "" {
				contents = append(contents, text{buffer})
				buffer = ""
			}
			nextIndex := cursor + 2 + strings.Index(string(line[cursor+2:]), "==")
			tempBuf := string(line[cursor+2 : nextIndex])

			if tempBuf == "" {
				contents = append(contents, text{"===="})
			} else {
				children, err := inlineFormatting(tempBuf)
				if err != nil {
					return nil, err
				}
				contents = append(contents, element{TagName: "mark", Children: children})
			}
			cursor = nextIndex + 2
			continue
		}

		// Inline math ($$)
		if cursor+2 < len(line) && string(line[cursor:cursor+2]) == "$$" && strings.Contains(string(line[cursor+2:]), "$$") {
			if buffer != "" {
				contents = append(contents, text{buffer})
				buffer = ""
			}
			nextIndex := cursor + 2 + strings.Index(string(line[cursor+2:]), "$$")
			tempBuf := string(line[cursor+2 : nextIndex])
			if tempBuf == "" {
				contents = append(contents, text{"$$$$"})
			} else {
				math, err := renderMath(tempBuf, false)
				if err != nil {
					return nil, err
				}
				contents = append(contents, math)
			}
			cursor = nextIndex + 2
			continue
		}

		// Italics (_)
		if line[cursor] == '_' && slices.Contains(line[cursor+1:], '_') {
			if buffer != "" {
				contents = append(contents, text{buffer})
				buffer = ""
			}
			nextIndex := cursor + 1 + slices.Index(line[cursor+1:], '_')
			tempBuf := string(line[cursor+1 : nextIndex])
			if tempBuf == "" {
				contents = append(contents, text{"__"})
			} else {
				children, err := inlineFormatting(tempBuf)
				if err != nil {
					return nil, err
				}
				contents = append(contents, element{TagName: "em", Children: children})
			}
			cursor = nextIndex + 1
			continue
		}

		// Code (`)
		if cursor+1 < len(line) && line[cursor] == '`' && slices.Contains(line[cursor+1:], '`') {
			if buffer != "" {
				contents = append(contents, text{buffer})
				buffer = ""
			}
			nextIndex := cursor + 1 + slices.Index(line[cursor+1:], '`')
			tempBuf := string(line[cursor+1 : nextIndex])
			if tempBuf == "" {
				contents = append(contents, text{"``"})
			} else {
				contents = append(contents, element{TagName: "code", Children: []node{
					text{html.EscapeString(tempBuf)},
				}})
			}

			cursor = nextIndex + 1
			continue
		}

		// Subscript (~)
		if line[cursor] == '~' && slices.Contains(line[cursor+1:], '~') {
			if buffer != "" {
				contents = append(contents, text{buffer})
				buffer = ""
			}
			nextIndex := cursor + 1 + slices.Index(line[cursor+1:], '~')
			tempBuf := string(line[cursor+1 : nextIndex])
			if tempBuf == "" {
				contents = append(contents, text{"~~"})
			} else {
				children, err := inlineFormatting(tempBuf)
				if err != nil {
					return nil, err
				}
				contents = append(contents, element{TagName: "sub", Children: children})
			}
			cursor = nextIndex + 1
			continue
		}

		// Superscript (^)
		if line[cursor] == '^' && slices.Contains(line[cursor+1:], '^') {
			if buffer != "" {
				contents = append(contents, text{buffer})
				buffer = ""
			}
			nextIndex := cursor + 1 + slices.Index(line[cursor+1:], '^')
			tempBuf := string(line[cursor+1 : nextIndex])
			if tempBuf == "" {
				contents = append(contents, text{"^^"})
			} else {
				children, err := inlineFormatting(tempBuf)
				if err != nil {
					return nil, err
				}
				contents = append(contents, element{TagName: "sup", Children: children})
			}
			cursor = nextIndex + 1
			continue
		}

		// Links
		if line[cursor] == '[' && strings.Contains(string(line[cursor+1:]), "](") && slices.Contains(line[cursor+1:], ')') {
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
				linkTitle += string(line[cursor])
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
				linkUrl += string(line[cursor])
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

			cursor++
			continue
		}

		// Default
		buffer += string(line[cursor])
		cursor++
	}

	if buffer != "" {
		contents = append(contents, text{buffer})
	}

	return contents, nil
}
