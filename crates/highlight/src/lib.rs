mod languages;
mod theme;

use std::{collections::HashMap, sync::Arc};

pub use tree_sitter_highlight::HighlightConfiguration;
use tree_sitter_highlight::{Highlighter, HtmlRenderer};

pub use crate::theme::Theme;

pub struct Highlight {
    theme: Theme,
    recognised_names: Vec<String>,
    languages: HashMap<String, Arc<HighlightConfiguration>>,
}

impl Highlight {
    pub fn new(theme: Theme) -> Self {
        let recognised_names = theme
            .highlights
            .keys()
            .map(|k| k.to_owned())
            .collect::<Vec<String>>();
        let mut hl = Self {
            theme,
            recognised_names,
            languages: HashMap::new(),
        };
        hl.default_langs();

        hl
    }

    pub fn highlight(&self, code: String, language: String) -> String {
        let global_style = self.theme.root.clone();

        if language == "plaintext" || language == "plain" || language == "text" || language == "txt"
        {
            return format!(
                "<pre class=\"ts-highlight\" style=\"{}\"><code>{}</code></pre>",
                global_style,
                escape_html(code)
            );
        }

        let mut highlighter = Highlighter::new();
        let config = match self.get_language(&language) {
            Some(config) => config,
            None => {
                eprintln!("Language not supported: {language}, continuing as plaintext");
                return format!(
                    "<pre class=\"ts-highlight\" style=\"{}\"><code>{}</code></pre>",
                    global_style,
                    escape_html(code)
                );
            }
        };

        let highlights = match highlighter.highlight(&config, code.as_bytes(), None, |capture| {
            self.get_language(capture)
        }) {
            Ok(highlights) => highlights,
            Err(err) => {
                eprintln!("Error while highlighting: {err}. Continuing as plaintext");
                return format!(
                    "<pre class=\"ts-highlight\" style=\"{}\"><code>{}</code></pre>",
                    global_style,
                    escape_html(code.clone())
                );
            }
        };

        let mut html_renderer = HtmlRenderer::new();
        match html_renderer.render(highlights, code.as_bytes(), &|h, output| {
            let attr = match self.theme.highlights.get(&self.recognised_names[h.0]) {
                Some(style) => format!(
                    "class=\"{}\" style=\"{}\"",
                    self.recognised_names[h.0], style
                ),
                None => format!("class=\"{}\"", self.recognised_names[h.0]),
            };
            return output.extend(attr.as_bytes());
        }) {
            Ok(()) => (),
            Err(err) => {
                eprintln!(
                    "Error rendering highlighted text to HTML: {err}. Continuing as plaintext",
                );
                return format!(
                    "<pre class=\"ts-highlight\" style=\"{}\"><code>{}</code></pre>",
                    global_style,
                    escape_html(code)
                );
            }
        };
        let mut highlighted_text = html_renderer.lines().collect::<Vec<&str>>().join("\n");

        match self.theme.line_numbers.clone() {
            None => (),
            Some(line_numbers) => {
                let max_line_num = (highlighted_text.lines().count() + 1).to_string().len();
                let right_space = match line_numbers.margin_right {
                    Some(r) => r,
                    None => 1,
                };

                highlighted_text = highlighted_text
                    .lines()
                    .enumerate()
                    .map(|(index, line)| {
                        format!(
                            "<span class=\"line-number\" style=\"margin-right:{}ch;{}\">{}</span>{}",
                            max_line_num + right_space - (index + 1).to_string().len(),
                            line_numbers.styles,
                            index + 1,
                            line
                        )
                    })
                    .collect::<Vec<String>>()
                    .join("\n");
            }
        };

        format!(
            "<pre class=\"ts-highlight\" style=\"{}\"><code>{}</code></pre>",
            global_style,
            highlighted_text.trim()
        )
    }
}

pub fn escape_html<S: Into<String>>(input: S) -> String {
    input
        .into()
        .replace('&', "&amp")
        .replace('"', "&quot;")
        .replace("'", "&#x27;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
}
