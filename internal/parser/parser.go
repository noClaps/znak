package parser

import (
	"fmt"
	"log"
	"regexp"
	"slices"
	"strings"

	"github.com/noclaps/znak/highlight"
)

func Parse(input string, codeTheme highlight.Theme) ([]node, error) {
	slugger := NewSlugger()
	lines := strings.Split(input, "\n")
	tokens := []node{}

	lineCursor := 0
	for ; lineCursor < len(lines); lineCursor++ {
		line := lines[lineCursor]

		// Headings
		if regexp.MustCompile("(?m)^#{1,6} .+").Match([]byte(line)) {
			cursor := 0
			for line[cursor] == '#' {
				cursor++
			}
			level := cursor
			heading := line[cursor+1:]

			slug := slugger.Slug(heading, uint(level))
			children, err := inlineFormatting(heading)
			if err != nil {
				return nil, err
			}
			tokens = append(tokens, element{fmt.Sprintf("h%d", level), map[string]string{"id": slug}, children})
			continue
		}

		// Blockquotes
		if strings.HasPrefix(line, ">") {
			blockquoteLines := ""
			for lineCursor < len(lines) && strings.HasPrefix(lines[lineCursor], ">") {
				blockquoteLines += strings.TrimSpace(strings.TrimPrefix(lines[lineCursor], ">")) + "\n"
				lineCursor++
			}
			children, err := Parse(blockquoteLines, codeTheme)
			if err != nil {
				return nil, err
			}
			tokens = append(tokens, element{TagName: "blockquote", Children: children})
			continue
		}

		// Horizontal rule
		if strings.Count(line, "-") == len(line) && len(line) >= 3 {
			tokens = append(tokens, element{TagName: "hr"})
			continue
		}

		// Images
		if strings.HasPrefix(line, "![") && strings.Contains(line, "](") && strings.HasSuffix(line, ")") {
			imageSplit := strings.LastIndex(line, "](")
			imageTitle := line[2:imageSplit]
			imageUrl := line[imageSplit+2 : len(line)-1]

			children, err := inlineFormatting(imageTitle)
			if err != nil {
				return nil, err
			}
			tokens = append(tokens, element{TagName: "figure", Children: []node{
				element{TagName: "img", Properties: map[string]string{"src": imageUrl, "alt": imageTitle}},
				element{TagName: "figcaption", Children: children},
			}})
			continue
		}

		// Code blocks
		if strings.HasPrefix(line, "```") && slices.Contains(lines[lineCursor+1:], strings.Repeat("`", strings.Count(line, "`"))) {
			backtickCount := strings.Count(line, "`")
			language := line[backtickCount:]

			codeBuffer := ""
			lineCursor++ // Move inside code block
			for lineCursor < len(lines) && lines[lineCursor] != strings.Repeat("`", backtickCount) {
				codeBuffer += fmt.Sprintln(lines[lineCursor])
				lineCursor++
			}

			if language == "" {
				highlightedText, err := highlightSyntax(codeBuffer, "plaintext", codeTheme)
				if err != nil {
					return nil, err
				}
				tokens = append(tokens, highlightedText)
			} else {
				highlightedText, err := highlightSyntax(codeBuffer, language, codeTheme)
				if err != nil {
					log.Println("[WARN]", err)
					highlightedText, err = highlightSyntax(codeBuffer, "plaintext", codeTheme)
					if err != nil {
						return nil, err
					}
				}
				tokens = append(tokens, highlightedText)
			}
			continue
		}

		// Ordered list (1., 3 space indentation)
		if regexp.MustCompile(`^\d+\. `).Match([]byte(line)) {
			buffer := ""
			for lineCursor < len(lines) && (regexp.MustCompile(`^(\d+\. |   )`).Match([]byte(lines[lineCursor])) || lines[lineCursor] == "") {
				buffer += fmt.Sprintln(lines[lineCursor])
				lineCursor++
			}

			children, err := listItems(buffer, codeTheme, orderedList)
			if err != nil {
				return nil, err
			}
			tokens = append(tokens, element{TagName: "ol", Children: children})
			lineCursor--
			continue
		}

		// Unordered list (-, 2 space indentation)
		if strings.HasPrefix(line, "- ") {
			buffer := ""
			for lineCursor < len(lines) && (strings.HasPrefix(lines[lineCursor], "- ") || strings.HasPrefix(lines[lineCursor], "  ") || lines[lineCursor] == "") {
				buffer += fmt.Sprintln(lines[lineCursor])
				lineCursor++
			}

			children, err := listItems(buffer, codeTheme, unorderedList)
			if err != nil {
				return nil, err
			}
			tokens = append(tokens, element{TagName: "ul", Children: children})
			lineCursor--
			continue
		}

		// Tables
		if strings.HasPrefix(line, "| ") {
			buffer := ""
			for lineCursor < len(lines) && strings.HasPrefix(lines[lineCursor], "| ") {
				buffer += lines[lineCursor] + "\n"
				lineCursor++
			}

			tables, err := tables(buffer)
			if err != nil {
				return nil, err
			}
			tokens = append(tokens, tables)
			continue
		}

		// HTML elements
		if strings.HasPrefix(line, "<") {
			buffer := fmt.Sprintln(line)

			if slices.IndexFunc(lines[lineCursor:], func(line string) bool {
				return strings.Contains(line, "</")
			}) == -1 {
				tokens = append(tokens, text{strings.TrimSpace(buffer)})
				continue
			}

			for lineCursor < len(lines) && !strings.Contains(lines[lineCursor], "</") {
				lineCursor++
				buffer += fmt.Sprintln(lines[lineCursor])
			}
			tokens = append(tokens, text{strings.TrimSpace(buffer)})
			continue
		}

		// Math block
		if line == "$$" && slices.Contains(lines[lineCursor+1:], "$$") {
			buffer := ""
			lineCursor++
			for lineCursor < len(lines) && lines[lineCursor] != "$$" {
				buffer += fmt.Sprintln(lines[lineCursor])
				lineCursor++
			}

			math, err := renderMath(buffer, true)
			if err != nil {
				return nil, err
			}
			tokens = append(tokens, math)
			continue
		}

		// Container
		if strings.HasPrefix(line, ":::") && len(strings.SplitN(line, " ", 2)) > 1 && strings.SplitN(line, " ", 2)[1] != "" && slices.IndexFunc(lines[lineCursor+1:], func(futureLine string) bool {
			return futureLine == strings.Repeat(":", strings.Count(strings.Split(line, " ")[0], ":"))
		}) != -1 {
			colonCount := strings.Count(strings.SplitN(line, " ", 2)[0], ":")

			buffer := fmt.Sprintln(line)
			lineCursor++ // Move inside container
			for lineCursor < len(lines) && lines[lineCursor] != strings.Repeat(":", colonCount) {
				buffer += fmt.Sprintln(lines[lineCursor])
				lineCursor++
			}

			container, err := containers(buffer, codeTheme)
			if err != nil {
				return nil, err
			}
			tokens = append(tokens, container)
			continue
		}

		// Paragraph
		buffer := ""
		for lineCursor < len(lines) && lines[lineCursor] != "" {
			buffer += lines[lineCursor]
			lineCursor++
		}
		if buffer != "" {
			children, err := inlineFormatting(buffer)
			if err != nil {
				return nil, err
			}
			tokens = append(tokens, element{TagName: "p", Children: children})
		}
	}

	return tokens, nil
}
