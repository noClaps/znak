pub use highlight::Theme;

use parser::{headings::parse_headings, parser::parse};
use renderer::renderer;
use utils::slugger::Heading;

mod parser;
mod renderer;
mod types;
mod utils;

#[cfg(test)]
mod test;

/// A function that renders the input text to HTML.
///
/// # Arguments
///
/// - `input`: The input text to be converted to HTML. This can be from a
/// Markdown file as long as the syntax is supported by Znak. See the
/// [documentation] for the supported
/// syntax.
/// - `code_theme` The theme for code blocks. There is no theme set by default,
/// and you must bring your own theme. You can create a theme without any syntax
/// highlighting using:
///   ```rs
///   use znak_lang::Theme;
///
///   Theme::new("[highlights]".to_string()).unwrap();
///   ```
///   An example of the GitHub Dark theme can be found in [`theme.toml`].
///
/// [documentation]: https://gitlab.com/noClaps/znak-lang/-/blob/main/docs/syntax.md
pub fn render(input: String, code_theme: Theme) -> String {
    let parser_output = parse(input, code_theme);
    parser_output
        .into_iter()
        .map(|node| renderer(node))
        .collect()
}

/// A function that returns the headings in the given input text.
///
/// # Arguments
///
/// - `input`: The input text to extract the headings from. This can be from a
/// Markdown file as long as the syntax is supported by Znak. See the [documentation]
/// for the supported syntax.
///
/// [documentation]: https://gitlab.com/noClaps/znak-lang/-/blob/main/docs/syntax.md
pub fn headings(input: String) -> Vec<Heading> {
    parse_headings(input)
}
