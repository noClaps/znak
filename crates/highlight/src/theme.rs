use std::{collections::HashMap, error::Error, fmt::Display};

#[derive(Debug)]
pub struct ThemeError {
    cause: String,
}
impl ThemeError {
    fn new<S: Into<String>>(cause: S) -> Self {
        Self {
            cause: cause.into(),
        }
    }
}
impl Display for ThemeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error parsing theme: {}", self.cause)
    }
}
impl Error for ThemeError {}

#[derive(Debug, PartialEq, Clone)]
pub(crate) struct LineNumbers {
    pub(crate) margin_right: Option<usize>,
    pub(crate) styles: String,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Theme {
    pub(crate) root: String,
    pub(crate) line_numbers: Option<LineNumbers>,
    pub(crate) highlights: HashMap<String, String>,
}

impl Theme {
    pub fn new<S: Into<String>>(css: S) -> Result<Theme, ThemeError> {
        let mut theme = Theme {
            root: String::new(),
            line_numbers: None,
            highlights: HashMap::new(),
        };

        let css = css
            .into()
            .replace('\n', "")
            .replace(' ', "")
            .replace('\t', "");
        let mut minified_css = String::new();
        let mut i = 0;
        while i < css.len() {
            if css[i..].starts_with("/*") {
                let comment_close = match css[i..].find("*/") {
                    Some(close) => close,
                    None => {
                        return Err(ThemeError::new(format!(
                            "Unterminated comment found: {}",
                            &css[i..i + 10]
                        )));
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
                None => return Err(ThemeError::new("No opening braces found in CSS")),
            };
            let selectors = css[i..i + open_brace].split(',');
            i = i + open_brace + 1;

            // parsing styles
            let close_brace = match css[i..].find('}') {
                Some(brace) => brace,
                None => {
                    return Err(ThemeError::new(format!(
                        "Mismatched opening and closing braces, closing brace not found: {}",
                        &css[i..i + 10]
                    )));
                }
            };
            let styles = &css[i..i + close_brace];
            i = i + close_brace;

            for selector in selectors {
                match selector {
                    ":root" => theme.root = styles.to_string(),
                    ":line-numbers" => {
                        let mut line_numbers = None;
                        let mut line_number_styles = String::new();

                        for style in styles.split(';') {
                            if style == "" {
                                continue;
                            }
                            let (key, value) = match style.split_once(':') {
                                Some(parts) => parts,
                                None => {
                                    return Err(ThemeError::new(format!(
                                        "Invalid CSS syntax: {}",
                                        &css[i - 10..i + 10]
                                    )));
                                }
                            };
                            if key == "margin-right" {
                                line_numbers = match line_numbers {
                                    None => Some(LineNumbers {
                                        margin_right: value.parse().ok(),
                                        styles: String::new(),
                                    }),
                                    Some(mut line_numbers) => {
                                        line_numbers.margin_right = value.parse().ok();
                                        Some(line_numbers)
                                    }
                                };
                                continue;
                            }
                            line_number_styles += style;
                        }

                        line_numbers = match line_numbers {
                            None if line_number_styles != "" => Some(LineNumbers {
                                margin_right: None,
                                styles: line_number_styles,
                            }),
                            Some(mut line_numbers) if line_number_styles != "" => {
                                line_numbers.styles = line_number_styles;
                                Some(line_numbers)
                            }
                            _ => line_numbers,
                        };

                        theme.line_numbers = line_numbers;
                    }
                    _ => {
                        theme
                            .highlights
                            .insert(selector.to_string(), styles.to_string());
                    }
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
        let blank = Theme::new("").unwrap();
        assert_eq!(
            blank,
            Theme {
                root: String::new(),
                line_numbers: None,
                highlights: HashMap::new()
            }
        )
    }

    #[test]
    fn css_theme() {
        let css = include_str!("../../../theme.css");
        Theme::new(css).unwrap();
    }
}
