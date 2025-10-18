# Znak

A parser for a Markdown-like markup language that supports a smaller subset of Markdown syntax, and is stricter and more opinionated. It has features like syntax highlighting, $\LaTeX$, and heading IDs built-in.

You can read the syntax [here](./docs/syntax.md). You can also read the documentation for Highlight (the syntax highlighter in Znak) [here](./docs/highlight.md).

## Usage

Add it as a dependency to your project:

```sh
cargo add --git https://github.com/noClaps/znak znak
```

Then you can use it in your code:

```rust
use znak::{Theme, Heading, render, parse_headings, parse_frontmatter};

fn main() {
    let css = include_str!("path/to/theme.css");
    let theme: Theme = Theme::new(css).unwrap();

    let input_files = include_str!("path/to/file.md");
    let rendered_html: String = render(input, theme);

    let headings: Vec<Heading> = parse_headings(input);
    let frontmatter: HashMap<String, String> = parse_frontmatter(input).unwrap();
}
```

## Acknowledgements

$\LaTeX$ is rendered to MathML using [tmke8/math-core](https://github.com/tmke8/math-core).

The example code theme in `theme.css` is taken from [PyaeSoneAungRgn/github-zed-theme](https://github.com/PyaeSoneAungRgn/github-zed-theme).
