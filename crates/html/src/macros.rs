/// Compares two HTML strings for equality.
///
/// Attribute order is ignored.
///
/// # Parameters
///
/// The two HTML strings to be compared. These can be [&str], [String], or any
/// other type that implements [ToString].
///
/// # Usage
///
/// ```rust
/// use html::assert_html_eq;
///
/// let source1 = "<html data-theme=\"light\" lang=\"en\"><head></head></html>";
/// let source2 = "<html lang=\"en\" data-theme=\"light\"><head></head></html>";
/// assert_html_eq!(source1, source2);
/// ```
#[macro_export]
macro_rules! assert_html_eq {
    ($expr1:expr, $expr2:expr) => {
        assert_eq!(
            $expr1.parse::<html::Node>().unwrap(),
            $expr2.parse::<html::Node>().unwrap()
        )
    };
}

/// Escapes unsafe characters in HTML.
///
/// - `&` becomes `&amp;`
/// - `"` becomes `&quot;`
/// - `'` becomes `&#x27;`
/// - `<` becomes `&lt;`
/// - `>` becomes `&gt;`
///
/// # Parameters
///
/// - `input`: Text to escape.
///
/// # Usage
///
/// ```rust
/// use html::escape_html;
///
/// let text = "<div>This text has HTML in it</div>";
/// let escaped = escape_html!(text);
/// assert_eq!(escaped, "&lt;div&gt;This text has HTML in it&lt;/div&gt;")
/// ```
#[macro_export]
macro_rules! escape_html {
    ($input:expr) => {
        $input
            .replace('&', "&amp;")
            .replace('"', "&quot;")
            .replace("'", "&#x27;")
            .replace('<', "&lt;")
            .replace('>', "&gt;")
    };
}
