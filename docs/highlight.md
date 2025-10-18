# Highlight

A syntax highlighting library that uses [Tree-sitter](https://tree-sitter.github.io/tree-sitter/) for incredibly quick parsing and highlighting.

## Usage

You can use the `highlight` crate in this repository by adding it to your project:

```sh
cargo add --git https://github.com/noClaps/znak highlight
```

and then using it in your code:

```rust
use highlight::{Theme, highlight};

fn main() {
    let css = include_str!("path/to/theme.css");
    let theme: Theme = Theme::new(css).unwrap()

    let code = "Your code here".to_string();
    let lang = "plaintext".to_string();
    let highlighted_text: String = highlight(code, lang, theme);
}
```

You can use `Theme::new()` to create a new theme from a CSS string. If you'd like to make a blank theme, use `Theme::new("")`.

## Languages

Highlight has a number of languages built in, but none are enabled by default to keep the bundle size small. You can enable the languages you'd like by enabling the respective features on the crate:

```toml
# Cargo.toml

[dependencies]
highlight = { git = "https://github.com/noClaps/znak", version = "0.21.0", features = [
  "bash",
  "c",
  "python",
  "typescript"
] }
```

If you'd like to enable all languages, you can do so by enabling the `all` feature:

```toml
# Cargo.toml

[dependencies]
highlight = { git = "https://github.com/noClaps/znak", version = "0.21.0", features = ["all"] }
```

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
