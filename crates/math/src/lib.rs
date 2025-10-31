pub use math_core::MathDisplay;
use math_core::{LatexToMathML, MathCoreConfig};

/// Renders the input LaTeX to MathML.
///
/// This function will panic if invalid LaTeX is passed in.
///
/// # Parameters
///
/// - `math`: The LaTeX to convert to MathML.
/// - `display`: Whether to display the output in block or inline mode. Use
///   [MathDisplay::Block] for block mode, and [MathDisplay::Inline] for inline
///   mode.
///
/// # Usage
///
/// ```rust
/// use math::{MathDisplay, render_math};
///
/// let latex = "a^2 + b^2 = c^2";
/// let mathml_block = render_math(latex, MathDisplay::Block);
/// let mathml_inline = render_math(latex, MathDisplay::Inline);
/// ```
// TODO: replace with Typst when their MathML export is ready
pub fn render_math(math: impl AsRef<str>, display: MathDisplay) -> String {
    let config = MathCoreConfig::default();
    let converter = LatexToMathML::new(&config).unwrap(); // safe to unwrap as no macros
    let mathml = converter
        .convert_with_local_counter(math.as_ref(), display)
        .unwrap(); // good to unwrap as it should panic if invalid LaTeX is passed in
    mathml
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn math_block() {
        let output = render_math("a^2 + b^2 = c^2", MathDisplay::Block);
        let check = r#"<math display="block"><msup><mi>a</mi><mn>2</mn></msup><mo>+</mo><msup><mi>b</mi><mn>2</mn></msup><mo>=</mo><msup><mi>c</mi><mn>2</mn></msup></math>"#;
        assert_eq!(output, check)
    }

    #[test]
    fn math_inline() {
        let output = render_math("x+y", MathDisplay::Inline);
        let check = r#"<math><mi>x</mi><mo>+</mo><mi>y</mi></math>"#;
        assert_eq!(output, check);
    }
}
