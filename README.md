# Znak

A parser for a Markdown-like markup language that supports a smaller subset of Markdown syntax, and is stricter and more opinionated. It has features like syntax highlighting, LaTeX, and heading IDs built-in.

You can read the syntax [here](./docs/syntax.md).

## Usage

Add it as a dependency to your project:

```sh
cargo add --git https://github.com/noClaps/znak.git
```

Then you can use it in your code:

```rs
use znak::{render, headings, Theme, Heading};
use std::collections::HashMap;

fn main() {
    let theme = match Theme::new(include_str!("path/to/theme.toml").to_string()) {
        Ok(theme) => theme,
        Err(err) => panic!("Handle error here: {err}")
    };

    let input = include_str!("path/to/file.md").to_string();
    let rendered_html: String = render(input, theme);
    let headings: Vec<Heading> = headings(input);
    let frontmatter: Option<HashMap<String, String>> = frontmatter(input);
}
```
