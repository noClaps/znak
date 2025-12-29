/// Unescapes unsafe characters in HTML.
///
/// - `&amp;` becomes `&`
/// - `&quot;` becomes `"`
/// - `&#x27;` becomes `'`
/// - `&lt;` becomes `<`
/// - `&gt;` becomes `>`
///
/// # Parameters
///
/// - `input`: Text to escape.
///
/// # Usage
///
/// ```rust
/// use html::unescape_html;
///
/// let text = "&lt;div&gt;This text has HTML in it&lt;/div&gt;";
/// let escaped = unescape_html!(text);
/// assert_eq!(escaped, "<div>This text has HTML in it</div>")
/// ```
#[macro_export]
macro_rules! unescape_html {
    ($input:expr) => {
        $input
            .replace("&amp;", "&")
            .replace("&quot;", "\"")
            .replace("&#x27;", "'")
            .replace("&lt;", "<")
            .replace("&gt;", ">")
    };
}
