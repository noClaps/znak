use std::{collections::HashMap, str::FromStr};

/// A theme object used to apply styles for syntax highlighting.
///
/// A theme is a CSS file, supporting a very basic CSS syntax.
///
/// - You can define global styles on the `:root` pseudo-element. These will be
///   applied to the parent `<pre>` element, and used by the content inside if
///   no styles are present for that syntax.
///
///   ```css
///   :root {
///       color: #fff;
///       background-color: #111;
///       /* any other css property */
///   }
///   ```
///
/// - You can configure highlights by defining your syntax type as the
///   selector, and apply styles to that selector. Syntax types with dots in
///   them are allowed, as well as using multiple selectors for the same
///   styles, are allowed.
///
///   ```css
///   type {
///       color: #5ac8f5;
///       font-weight: 500;
///       font-style: normal;
///       background-color: #111;
///   }
///
///   /* an example of using multiple selectors and types with dots */
///   comment,
///   comment.doc {
///       color: #9198a1;
///   }
///   ```
///
/// Note that advanced CSS features, like nesting, combinators, other
/// pseudo-elements, media queries, etc., are not supported. Everything inside
/// the `{}` braces will be used as-is for the inline style, so only properties
/// will work inside.
///
/// You can look at
/// [`theme.css`](https://github.com/noClaps/znak/blob/main/theme.css) for an
/// example theme.
///
/// # Usage
///
/// You can create a new theme using the `.parse()` method on a string, or the
/// [Theme::from_str] method.
///
/// ```rust
/// use highlight::Theme;
///
/// let css = include_str!("../../../theme.css");
/// let theme: Theme = css.parse().unwrap();
/// ```
#[derive(Debug, PartialEq, Clone)]
pub struct Theme {
    pub(crate) root: String,
    pub(crate) highlights: HashMap<String, String>,
}

impl FromStr for Theme {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut theme = Theme {
            root: String::new(),
            highlights: HashMap::new(),
        };

        let css = s.replace('\n', "").replace(' ', "").replace('\t', "");
        let mut minified_css = String::new();
        let mut i = 0;
        while i < css.len() {
            if css[i..].starts_with("/*") {
                let comment_close = match css[i..].find("*/") {
                    Some(close) => close,
                    None => {
                        return Err(format!("Unterminated comment found: {}", &css[i..i + 10]));
                    }
                };
                i = i + comment_close + 2;
            }
            minified_css += &css[i..i + 1];
            i += 1;
        }

        let css = minified_css;
        let mut i = 0;
        while i < css.len() {
            // parsing selectors
            let open_brace = match css[i..].find('{') {
                Some(brace) => brace,
                None => return Err("No opening braces found in CSS".to_string()),
            };
            let selectors = css[i..i + open_brace].split(',');
            i = i + open_brace + 1;

            // parsing styles
            let close_brace = match css[i..].find('}') {
                Some(brace) => brace,
                None => {
                    return Err(format!(
                        "Mismatched opening and closing braces, closing brace not found: {}",
                        &css[i..i + 10]
                    ));
                }
            };
            let styles = &css[i..i + close_brace];
            i = i + close_brace;

            for selector in selectors {
                match selector {
                    ":root" => theme.root += styles,
                    _ => match theme.highlights.get_mut(selector) {
                        None => {
                            theme
                                .highlights
                                .insert(selector.to_string(), styles.to_string());
                        }
                        Some(h) => h.push_str(styles),
                    },
                }
            }

            i += 1;
        }

        Ok(theme)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn blank_theme() {
        let blank: Theme = "".parse().unwrap();
        assert_eq!(
            blank,
            Theme {
                root: String::new(),
                highlights: HashMap::new()
            }
        )
    }

    #[test]
    fn css_theme() {
        let css = include_str!("../../../theme.css");
        css.parse::<Theme>().unwrap();
    }
}
