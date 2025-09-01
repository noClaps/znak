package znak

import (
	"fmt"
	"regexp"
	"slices"
	"strings"

	"github.com/noclaps/znak/highlight"
	"github.com/noclaps/znak/internal/parser"
	"github.com/noclaps/znak/types"
)

// A function that renders the input text to HTML.
//
// `input`: The input text to be converted to HTML. This can be from a Markdown
// file as long as the syntax is supported by Znak. See the [documentation] for
// the supported syntax.
//
// `codeTheme`: The theme for code blocks. There is no theme set by default,
// and you must bring your own theme. An example theme can be found in
// [theme.css]. You can create a theme without any syntax
// highlighting using:
//
//	import "github.com/noclaps/znak/highlight"
//	highlight.Theme{}
//
// [documentation]: https://github.com/noClaps/znak/blob/main/docs/syntax.md
// [theme.css]: https://github.com/noClaps/znak/blob/main/theme.css
func Render(input string, codeTheme highlight.Theme) (string, error) {
	lines := strings.Split(strings.TrimSpace(input), "\n")
	cur := 0
	if lines[cur] == "---" && slices.Contains(lines[cur+1:], "---") {
		cur++
		for lines[cur] != "---" {
			cur++
		}
		cur++
	}

	input = strings.Join(lines[cur:], "\n")
	parserOutput, err := parser.Parse(input, codeTheme)
	if err != nil {
		return "", err
	}

	output := ""
	for _, node := range parserOutput {
		output += parser.Renderer(node)
	}
	return output, nil
}

// A function that returns the headings in the given input text.
//
// # Arguments
//
// `input`: The input text to extract the headings from. This can be from a
// Markdown file as long as the syntax is supported by Znak. See the
// [documentation] for the supported syntax.
//
// # Returns
//
// Returns a list of [types.Heading] structs.
//
// [documentation]: https://github.com/noClaps/znak/blob/main/docs/syntax.md
func ParseHeadings(input string) []types.Heading {
	slugger := parser.NewSlugger()
	re := regexp.MustCompile("(?m)^(#{1,6}) (.+)")

	for _, match := range re.FindAllStringSubmatch(input, -1) {
		level := uint(len(match[1]))
		heading := match[2]
		slugger.Slug(heading, level)
	}

	return slugger.Headings
}

// A function that returns the frontmatter in the given input text.
//
// # Arguments
//
// `input`: The input text to extract the frontmatter from. This can be from
// a Markdown file as long as the syntax is supported by Znak. See the
// [documentation] for the supported syntax.
//
// # Returns
//
// Returns a map of frontmatter keys and values.
//
// [documentation]: https://github.com/noClaps/znak/blob/main/docs/syntax.md
func ParseFrontmatter(input string) (map[string]string, error) {
	fmVals := make(map[string]string)
	lines := strings.Split(strings.TrimSpace(input), "\n")
	if lines[0] != "---" {
		return nil, fmt.Errorf("No frontmatter found: %s", input)
	}

	for _, line := range lines[1:] {
		if line == "---" {
			break
		}
		key, val, found := strings.Cut(line, ":")
		if !found {
			return nil, fmt.Errorf("Line is formatted incorrectly, missing `:` between key and value: %s", line)
		}
		fmVals[key] = strings.TrimSpace(val)
	}

	return fmVals, nil
}
