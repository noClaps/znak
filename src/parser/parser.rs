use crate::{
    types::Node,
    utils::{math::render_math, slugger::Slugger, syntax_highlighting::highlight_syntax},
};
use highlight::Theme;
use pulldown_latex::config::DisplayMode;
use regex::Regex;

use super::{
    containers::containers,
    inline_formatting::inline_formatting,
    list_items::{list_items, ListType},
    tables::tables,
};

pub fn parse(input: String, code_theme: Theme) -> Vec<Node> {
    let mut slugger = Slugger::new();
    let lines = input.lines().collect::<Vec<&str>>();
    let mut tokens: Vec<Node> = vec![];
    let mut buffer = String::new();

    let mut line_cursor = 0;
    while line_cursor < lines.len() {
        // Headings
        if line_cursor != lines.len()
            && Regex::new(r"^#{1,6} .+")
                .unwrap()
                .is_match(lines[line_cursor])
        {
            let mut level = 0;
            let mut cursor = 0;
            while &lines[line_cursor][cursor..cursor + 1] == "#" {
                level += 1;
                cursor += 1;
            }

            let heading = &lines[line_cursor][cursor + 1..];

            let slug = slugger.slug(heading.to_string(), level as u8);
            tokens.push(Node::element(
                &format!("h{}", level),
                Node::props(vec![["id", &slug]]),
                inline_formatting(heading.to_string()),
            ));

            line_cursor += 1;
            continue;
        }

        // Blockquotes
        if line_cursor != lines.len() && lines[line_cursor].starts_with(">") {
            while line_cursor != lines.len()
                && !lines[line_cursor].is_empty()
                && lines[line_cursor].starts_with(">")
            {
                buffer += &format!("{}\n", lines[line_cursor]);
                line_cursor += 1
            }
            let blockquote_lines = buffer
                .lines()
                .map(|line| line[1..].trim())
                .collect::<Vec<&str>>()
                .join("\n");

            tokens.push(Node::element(
                "blockquote",
                None,
                parse(blockquote_lines, code_theme.clone()),
            ));
            buffer.clear();

            line_cursor += 1;
            continue;
        }

        // Horizontal rule
        if line_cursor != lines.len()
            && lines[line_cursor].chars().all(|c| c == '-')
            && lines[line_cursor].len() >= 3
        {
            tokens.push(Node::element("hr", None, vec![]));
            buffer.clear();

            line_cursor += 1;
            continue;
        }

        // Images
        if line_cursor != lines.len()
            && lines[line_cursor].starts_with("![")
            && lines[line_cursor].ends_with(")")
        {
            let line = lines[line_cursor];
            let image_split = line.rfind("](").unwrap();
            let image_title = &line[2..image_split];
            let image_url = &line[image_split + 2..line.len() - 1];

            tokens.push(Node::element(
                "figure",
                None,
                vec![
                    Node::element(
                        "img",
                        Node::props(vec![["src", image_url], ["alt", image_title]]),
                        vec![],
                    ),
                    Node::element(
                        "figcaption",
                        None,
                        inline_formatting(image_title.to_string()),
                    ),
                ],
            ));

            line_cursor += 1;
            continue;
        }

        // Code block
        if line_cursor != lines.len()
            && lines[line_cursor].starts_with("```")
            && lines[line_cursor + 1..]
                .to_vec()
                .into_iter()
                .find(|line| line == &"`".repeat(lines[line_cursor].matches('`').count()).as_str())
                .is_some_and(|line| line.ends_with('`'))
        {
            let backtick_count = lines[line_cursor].matches('`').count();
            let mut code_buffer: Vec<String> = vec![];
            let language = lines[line_cursor].replace('`', "");

            // Move inside code block
            line_cursor += 1;
            while line_cursor != lines.len()
                && !lines[line_cursor].ends_with(&"`".repeat(backtick_count))
            {
                code_buffer.push(lines[line_cursor].to_string());
                line_cursor += 1;
            }

            if language.is_empty() {
                tokens.push(highlight_syntax(
                    code_buffer.join("\n").to_string(),
                    "plaintext".to_string(),
                    code_theme.clone(),
                ));
            } else {
                tokens.push(highlight_syntax(
                    code_buffer.join("\n").to_string(),
                    language,
                    code_theme.clone(),
                ));
            }

            line_cursor += 1;
            continue;
        }

        // Ordered list (1., 3 space indentation)
        if line_cursor != lines.len()
            && Regex::new(r"^\d+\. ").unwrap().is_match(lines[line_cursor])
        {
            while line_cursor != lines.len()
                && (Regex::new(r"^(\d+\. |   |\t)")
                    .unwrap()
                    .is_match(lines[line_cursor])
                    || lines[line_cursor].is_empty())
            {
                buffer += &format!("{}\n", lines[line_cursor]);
                line_cursor += 1;
            }

            tokens.push(Node::element(
                "ol",
                None,
                list_items(buffer.clone(), code_theme.clone(), ListType::Ordered),
            ));

            buffer.clear();
            continue;
        }

        // Unordered list (-, 2 space indentation)
        if line_cursor != lines.len() && lines[line_cursor].starts_with("- ") {
            while line_cursor != lines.len()
                && (Regex::new("^(- |  |\t)")
                    .unwrap()
                    .is_match(lines[line_cursor])
                    || lines[line_cursor].is_empty())
            {
                buffer += &format!("{}\n", lines[line_cursor]);
                line_cursor += 1;
            }

            tokens.push(Node::element(
                "ul",
                None,
                list_items(buffer.clone(), code_theme.clone(), ListType::Unordered),
            ));

            buffer.clear();
            continue;
        }

        // Tables
        if line_cursor != lines.len() && lines[line_cursor].starts_with("| ") {
            while line_cursor != lines.len() && lines[line_cursor].starts_with("| ") {
                buffer += &format!("{}\n", lines[line_cursor]);
                line_cursor += 1;
            }

            tokens.push(tables(buffer.clone()));

            buffer.clear();
            line_cursor += 1;
            continue;
        }

        // HTML elements
        if line_cursor != lines.len() && lines[line_cursor].starts_with("<") {
            buffer += &format!("{}\n", lines[line_cursor]);

            if lines[line_cursor..]
                .into_iter()
                .find(|line| line.contains("</"))
                .is_none()
            {
                tokens.push(Node::text(buffer.trim()));
                line_cursor += 1;
                continue;
            }

            while line_cursor != lines.len() && !lines[line_cursor].contains("</") {
                line_cursor += 1;
                buffer += &format!("{}\n", lines[line_cursor]);
            }

            tokens.push(Node::text(buffer.trim()));

            buffer.clear();
            line_cursor += 1;
            continue;
        }

        // Math block
        if line_cursor != lines.len()
            && lines[line_cursor] == "$$"
            && lines[line_cursor + 1..].contains(&"$$")
        {
            line_cursor += 1;
            while line_cursor != lines.len() && lines[line_cursor] != "$$" {
                buffer += lines[line_cursor];
                line_cursor += 1;
            }

            tokens.push(render_math(buffer.clone(), DisplayMode::Block));

            buffer.clear();
            line_cursor += 1;
            continue;
        }

        // Container
        if line_cursor != lines.len()
            && lines[line_cursor].starts_with(":::")
            && lines[line_cursor].split_once(" ").is_some()
            && lines[line_cursor + 1..]
                .to_vec()
                .into_iter()
                .find(|line| {
                    line == &":".repeat(
                        lines[line_cursor]
                            .split_once(" ")
                            .unwrap()
                            .0
                            .matches(':')
                            .count(),
                    )
                })
                .is_some_and(|line| line.ends_with(':'))
        {
            let colon_count = lines[line_cursor]
                .split_once(" ")
                .unwrap()
                .0
                .matches(':')
                .count();

            buffer += &format!("{}\n", lines[line_cursor]);
            // Move inside container
            line_cursor += 1;
            while line_cursor != lines.len() && lines[line_cursor] != ":".repeat(colon_count) {
                buffer += &format!("{}\n", lines[line_cursor]);
                line_cursor += 1;
            }

            tokens.push(containers(buffer.clone(), code_theme.clone()));
            line_cursor += 1;
            continue;
        }

        // Paragraph
        while line_cursor != lines.len() && !lines[line_cursor].is_empty() {
            buffer += lines[line_cursor];
            line_cursor += 1;
        }
        if !buffer.is_empty() {
            tokens.push(Node::element("p", None, inline_formatting(buffer.clone())));
        }
        buffer.clear();

        line_cursor += 1;
    }

    return tokens;
}
