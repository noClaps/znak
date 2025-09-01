# Highlight

A syntax highlighting library that uses [Tree-sitter](https://tree-sitter.github.io/tree-sitter/) for incredibly quick parsing and highlighting.

## Usage

You can use the `highlight` module from the `github.com/noclaps/znak` package:

```go
import (
	"os"

	"github.com/noClaps/znak/highlight"
)

func main() {
	themeFile, err := os.ReadFile("path/to/theme.css")
	theme, err := highlight.NewTheme(themeFile)

	code = `fmt.Println("Hello world")`
	language = "go" // You can use any language supported by Highlight
	highlightedHtml, err := highlight.Highlight(code, language, theme)

	fmt.Println(highlightedHtml) // <pre class="ts-highlight" ... </pre>
}
```

You can use `highlight.NewTheme()` to create a new theme from a CSS string, or create a theme using `highlight.Theme{}`.

## Themes

A theme is a CSS file, supporting a very basic CSS syntax.

- You can define global styles on the `:root` pseudo-element. These will be applied to the parent `<pre>` element, and used by the content inside if no styles are present for that syntax.

  ```css
  :root {
      color: #fff;
      background-color: #111;
      /* any other css property */
  }
  ```

- You can configure line numbers by using the `:line-numbers` pseudo-element. If you want to set the number of spaces between the line numbers and the code, you can use `margin-right` with **only a number without any units**. Other than that special case, all other CSS properties are allowed.

  ```css
  :line-numbers {
      margin-right: 2; /* no units allowed here */
      color: #888;
  }
  ```

- You can configure highlights by defining your syntax type as the selector, and apply styles to that selector. Syntax types with dots in them are allowed, as well as using multiple selectors for the same styles, are allowed.

  ```css
  type {
      color: #5ac8f5;
      font-weight: 500;
      font-style: normal;
      background-color: #111;
  }

  /* an example of using multiple selectors and types with dots */
  comment,
  comment.doc {
      color: #9198a1;
  }
  ```

Note that advanced CSS features, like nesting, combinators, other pseudo-elements, media queries, etc., are not supported. Everything inside the `{}` braces will be used as-is for the inline style, so only properties will work inside.

You can look at [`theme.css`](https://github.com/noClaps/znak/blob/main/theme.css) for an example theme.
