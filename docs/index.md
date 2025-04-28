# Znak

A parser for a Markdown-like markup language that supports a smaller subset of Markdown syntax, and is stricter and more opinionated. It has features like syntax highlighting, LaTeX, and heading IDs built-in.

You can read the syntax [here](./syntax.md). You can also read the documentation for Highlight (the syntax highlighter in Znak) [here](./highlight.md).

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

## Acknowledgements

LaTeX is rendered to MathML using [TreeBlood](https://github.com/Wyatt915/treeblood).

The Tree-sitter bindings for syntax highlighting are provided by [go-tree-sitter-highlight](https://github.com/gopad-dev/go-tree-sitter-highlight).
