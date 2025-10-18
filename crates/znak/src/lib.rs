mod frontmatter;
mod headings;
mod parser;

pub use frontmatter::parse_frontmatter;
pub use headings::{Heading, parse_headings};
pub use highlight::Theme;

use crate::parser::{parse, renderer};

/// A function that renders the input text to HTML.
///
/// `input`: The input text to be converted to HTML. This can be from a Markdown
/// file as long as the syntax is supported by Znak. See the
/// [documentation](https://github.com/noClaps/znak/blob/main/docs/syntax.md)
/// for the supported syntax.
///
/// `codeTheme`: The theme for code blocks. There is no theme set by default,
/// and you must bring your own theme. An example theme can be found in
/// [theme.css](https://github.com/noClaps/znak/blob/main/theme.css). You can
/// create a theme without any syntax highlighting using:
///
/// ```rust
///	use znak::Theme;
///	Theme::new("").unwrap();
/// ```
pub fn render<S: Into<String>>(input: S, code_theme: Theme) -> String {
    let input = input.into();
    let lines = input.trim().lines().collect::<Vec<&str>>();
    let mut cur = 0;

    if lines[cur] == "---" {
        match lines[cur + 1..].into_iter().position(|&line| line == "---") {
            Some(line_idx) => cur = line_idx + 2,
            None => (),
        }
    }

    let input = lines[cur..].join("\n");
    let parser_output = parse(input, code_theme);

    let mut output = String::new();
    for node in parser_output {
        output += renderer(node).as_str();
    }
    output
}
