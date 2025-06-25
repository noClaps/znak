# Highlight

A syntax highlighting library that uses [Tree-sitter](https://tree-sitter.github.io/tree-sitter/) for incredibly quick parsing and highlighting.

## Usage

You can use the `highlight` module from the `github.com/noClaps/znak` package:

```go
import (
	"os"

	"github.com/noClaps/znak/highlight"
)

func main() {
	themeFile, err := os.ReadFile("path/to/theme.json")
	theme, err := highlight.NewTheme(themeFile)

	code = `fmt.Println("Hello world")`
	language = "go" // You can use any language supported by Highlight
	highlightedHtml, err := highlight.Highlight(code, language, theme)

	fmt.Println(highlightedHtml) // <pre class="ts-highlight" ... </pre>
}
```

You can use `highlight.NewTheme()` to create a new theme from a JSON string, or create a theme using `highlight.Theme{}`.

## Themes

A theme is a JSON file with the following properties:

- `color`: The default text color. This is set on the parent `<pre>` element and is used if no valid highlight is present, or no highlight color is provided for that syntax.

  ```json
  {
    "color": "#fff"
    // "color": <CSS color>
  }
  ```

- `backgroundColor`: The default background color. This is the background color of the code block.

  ```json
  {
    "backgroundColor": "#111"
    // "backgroundColor": <CSS color>
  }
  ```

- `lineNumbers`: An object to configure line numbers. If this is left out, no line numbers will be present. It has the following properties:

  - `color`: The text color of the line numbers. Required.

    ```json
    {
      "lineNumbers": {
        "color": "#888"
        // "color": <CSS color>
      }
    }
    ```

  - `rightSpace`: The number of spaces between the line numbers and the code, in units of `ch`. Default is 1.

    ```json
    {
      "lineNumbers": {
        "rightSpace": 2
        // "rightSpace": <number>
      }
    }
    ```

- `highlights`: An object to configure highlights. This is a map, with the keys being the syntax types, and the values being the configuration object. If you don't want to have inline styles, you can have the keys be the syntax types you want to select, and the configuration object empty for each one. Each configuration object has the following properties:

  - `color`: The text color of the syntax type.

    ```json
    {
      "highlights": {
        "type": { "color": "#5ac8f5" }
        // <syntax name>: { "color": <CSS color> }
      }
    }
    ```

  - `fontWeight`: The font weight for the syntax type. This should be a number between 1 and 1000.

    ```json
    {
      "highlights": {
        "type": { "fontWeight": 500 }
        // <syntax name>: { "fontWeight": <number> }
      }
    }
    ```

  - `fontStyle`: The font style for the syntax type. One of `italic`, `normal`, and `oblique`. Default is `normal`.

    ```json
    {
      "highlights": {
        "type": { "fontStyle": "normal" }
        // <syntax name>: { "fontStyle": <"italic" | "normal" | "oblique"> }
      }
    }
    ```

  - `backgroundColor`: The background color for the syntax type.

    ```json
    {
      "highlights": {
        "type": { "backgroundColor": "#111" }
        // <syntax name>: { "backgroundColor": <CSS color> }
      }
    }
    ```

All of the color values are CSS colors, so you can use hex (`#rrggbbaa`), OKLCH (`oklch(lightness# chroma hue / alpha)`), etc.

You can look at [`theme.json`](https://github.com/noClaps/znak/blob/main/theme.json) for an example theme. You can also add:

```json
{
  "$schema": "https://noclaps.github.io/znak/theme-schema.json"
}
```

to the top of your theme file for completions and descriptions of the different properties.

## Acknowledgements

The Tree-sitter bindings are provided by [gopad-dev/go-tree-sitter-highlight](https://github.com/gopad-dev/go-tree-sitter-highlight).
