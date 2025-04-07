# Znak

A parser for a Markdown-like markup language that supports a smaller subset of Markdown syntax, and is stricter and more opinionated. It has features like syntax highlighting, LaTeX, and heading IDs built-in.

You can read the syntax [here](./docs/syntax.md).

## Usage

Add it as a dependency to your project:

```sh
go get github.com/noClaps/znak
```

Then you can use it in your code:

```go
package main

import (
	"github.com/noClaps/znak"
	"github.com/noClaps/znak/highlight"
)

func main() {
	themeFile, err := os.ReadFile("path/to/theme.json")
	codeTheme, err := highlight.NewTheme(string(themeFile))

	inputFile, err := os.ReadFile("path/to/file.md")
	input := string(inputFile)

	renderedHtml, err := znak.Render(input, codeTheme)
	headings := znak.Headings(input)
	frontmatter, err := znak.Frontmatter(input)
}
```
