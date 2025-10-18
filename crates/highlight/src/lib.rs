mod languages;
mod theme;

use tree_sitter_highlight::{Highlighter, HtmlRenderer};

pub use crate::theme::Theme;

pub fn escape_html<S: Into<String>>(input: S) -> String {
    input
        .into()
        .replace('&', "&amp")
        .replace('"', "&quot;")
        .replace("'", "&#x27;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
}

pub fn highlight(code: String, language: String, theme: Theme) -> String {
    let global_style = theme.root.clone();

    if language == "plaintext" || language == "plain" || language == "text" || language == "txt" {
        return format!(
            "<pre class=\"ts-highlight\" style=\"{}\"><code>{}</code></pre>",
            global_style,
            escape_html(code)
        );
    }

    let highlight_names = theme
        .highlights
        .keys()
        .map(|k| k.to_owned())
        .collect::<Vec<String>>();

    let mut highlighter = Highlighter::new();
    let mut config = match languages::new(&language) {
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
    config.configure(&highlight_names);

    let highlights = match highlighter.highlight(&config, code.as_bytes(), None, |capture| {
        let mut config = languages::new(&capture.to_string())?;
        config.configure(&highlight_names);
        Some(Box::leak(Box::new(config)))
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
        let attr = match theme.highlights.get(&highlight_names[h.0]) {
            Some(style) => format!("class=\"{}\" style=\"{}\"", highlight_names[h.0], style),
            None => format!("class=\"{}\"", highlight_names[h.0]),
        };
        return output.extend(attr.as_bytes());
    }) {
        Ok(()) => (),
        Err(err) => {
            eprintln!("Error rendering highlighted text to HTML: {err}. Continuing as plaintext",);
            return format!(
                "<pre class=\"ts-highlight\" style=\"{}\"><code>{}</code></pre>",
                global_style,
                escape_html(code)
            );
        }
    };
    let mut highlighted_text = html_renderer.lines().collect::<Vec<&str>>().join("\n");

    match theme.line_numbers {
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
