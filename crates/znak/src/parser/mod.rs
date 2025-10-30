use highlight::Highlight;
use math::{MathDisplay, render_math};
use regex::Regex;

pub(crate) use crate::parser::renderer::renderer;
use crate::{
    headings::Slugger,
    parser::{
        containers::containers,
        inline_formatting::inline_formatting,
        list_items::{ListType, list_items},
        tables::tables,
        types::Node,
    },
};

mod containers;
mod inline_formatting;
mod list_items;
mod renderer;
mod tables;
mod types;

#[cfg(test)]
mod tests;

fn count_chars<S: Into<String>>(input: S, char: char) -> usize {
    input.into().chars().filter(|&c| c == char).count()
}

pub(crate) fn parse(input: String, hl: &Highlight) -> Vec<Node> {
    let mut slugger = Slugger::new();
    let lines = input.lines().collect::<Vec<&str>>();
    let mut tokens = vec![];

    let mut line_cursor = 0;
    while line_cursor < lines.len() {
        let line = lines[line_cursor];

        // Headings
        if Regex::new("^#{1,6} .+").unwrap().is_match(line) {
            let mut cursor = 0;
            while &line[cursor..cursor + 1] == "#" {
                cursor += 1;
            }
            let level = cursor;
            let heading = &line[cursor..].trim();

            let slug = slugger.slug(heading.to_string(), level);
            let children = inline_formatting(heading.to_string());
            tokens.push(Node::element(
                format!("h{level}"),
                vec![("id".to_string(), slug)],
                children,
            ));
            line_cursor += 1;
            continue;
        }

        // Blockquotes
        if line.starts_with(">") {
            let mut blockquote_lines = String::new();
            while line_cursor < lines.len() && lines[line_cursor].starts_with(">") {
                blockquote_lines +=
                    format!("{}\n", lines[line_cursor].trim_start_matches(">").trim()).as_str();
                line_cursor += 1;
            }
            let children = parse(blockquote_lines, hl);
            tokens.push(Node::element("blockquote", vec![], children));
            line_cursor += 1;
            continue;
        }

        // Horizontal rule
        if count_chars(line, '-') == line.len() && line.len() >= 3 {
            tokens.push(Node::element("hr", vec![], vec![]));
            line_cursor += 1;
            continue;
        }

        // Images
        if line.starts_with("![") && line.contains("](") && line.ends_with(")") {
            // safe to unwrap as checked in if condition
            let image_split = line.rfind("](").unwrap();
            let image_title = &line[2..image_split];
            let image_url = &line[image_split + 2..line.len() - 1];

            let children = inline_formatting(image_title.to_string());
            tokens.push(Node::element(
                "figure",
                vec![],
                vec![
                    Node::element(
                        "img",
                        vec![("src", image_url), ("alt", image_title)],
                        vec![],
                    ),
                    Node::element("figcaption", vec![], children),
                ],
            ));
            line_cursor += 1;
            continue;
        }

        // Code blocks
        if line.starts_with("```")
            && lines[line_cursor + 1..].contains(&"`".repeat(count_chars(line, '`')).as_str())
        {
            let backtick_count = count_chars(line, '`');
            let language = &line[backtick_count..];

            let mut code_buffer = String::new();
            line_cursor += 1; // Move inside code block
            while line_cursor < lines.len() && lines[line_cursor] != "`".repeat(backtick_count) {
                code_buffer += format!("{}\n", lines[line_cursor]).as_str();
                line_cursor += 1;
            }

            let highlighted_text = match language {
                "" => hl.highlight(code_buffer, "plaintext".to_string()),
                _ => hl.highlight(code_buffer, language.to_string()),
            };
            tokens.push(Node::text(highlighted_text));
            line_cursor += 1;
            continue;
        }

        // Ordered list (1., 3 space indentation)
        // can unwrap as known safe regex
        if Regex::new(r#"^\d+\. "#).unwrap().is_match(line) {
            let mut buffer = String::new();
            while line_cursor < lines.len()
                && (Regex::new(r#"(^\d+\. |   )"#)
                    .unwrap()
                    .is_match(lines[line_cursor])
                    || lines[line_cursor] == "")
            {
                buffer += format!("{}\n", lines[line_cursor]).as_str();
                line_cursor += 1;
            }

            let children = list_items(buffer, hl, ListType::Ordered);
            tokens.push(Node::element("ol", vec![], children));
            continue;
        }

        // Unordered list (-, 2 space indentation)
        if line.starts_with("- ") {
            let mut buffer = String::new();
            while line_cursor < lines.len()
                && (lines[line_cursor].starts_with("- ")
                    || lines[line_cursor].starts_with("  ")
                    || lines[line_cursor] == "")
            {
                buffer += format!("{}\n", lines[line_cursor]).as_str();
                line_cursor += 1;
            }

            let children = list_items(buffer, hl, ListType::Unordered);
            tokens.push(Node::element("ul", vec![], children));
            continue;
        }

        // Tables
        if line.starts_with("| ") {
            let mut buffer = String::new();
            while line_cursor < lines.len() && lines[line_cursor].starts_with("| ") {
                buffer += format!("{}\n", lines[line_cursor]).as_str();
                line_cursor += 1;
            }

            let tables = tables(buffer);
            tokens.push(tables);
            line_cursor += 1;
            continue;
        }

        // HTML elements
        if line.starts_with("<") {
            let t = line[1..]
                .split("")
                .position(|c| c == ">" || c == " ")
                .expect(&format!("Unterminated HTML element found: {}", line));
            let tag_name = &line[1..t];
            let mut depth = 0;
            let mut buffer = String::new();
            while line_cursor < lines.len()
                && (!lines[line_cursor].contains(&format!("</{}>", tag_name)) || depth > 0)
            {
                if lines[line_cursor].contains(&format!("<{}", tag_name)) {
                    depth += 1;
                }
                buffer += &format!("{}\n", lines[line_cursor]);
                line_cursor += 1;
            }
            tokens.push(Node::text(buffer.trim()));
        }

        // Math block
        if line == "$$" && lines[line_cursor + 1..].contains(&"$$") {
            let mut buffer = String::new();
            line_cursor += 1;
            while line_cursor < lines.len() && lines[line_cursor] != "$$" {
                buffer += format!("{}\n", lines[line_cursor]).as_str();
                line_cursor += 1;
            }

            let math = render_math(buffer, MathDisplay::Block);
            tokens.push(Node::text(math));
            line_cursor += 1;
            continue;
        }

        // Container
        if line.starts_with(":::")
            && line.split_once(" ").is_some_and(|split| {
                split.1 != ""
                    && lines[line_cursor + 1..]
                        .iter()
                        .find(|&&future_line| future_line == ":".repeat(count_chars(split.0, ':')))
                        .is_some()
            })
        {
            // safe to unwrap as checked in if condition
            let colon_count = count_chars(line.split_once(" ").unwrap().0, ':');
            let mut buffer = format!("{}\n", line);
            line_cursor += 1; // Move inside container
            while line_cursor < lines.len() && lines[line_cursor] != ":".repeat(colon_count) {
                buffer += format!("{}\n", lines[line_cursor]).as_str();
                line_cursor += 1;
            }

            let container = containers(buffer, hl);
            tokens.push(container);
            line_cursor += 1;
            continue;
        }

        let mut buffer = String::new();
        while line_cursor < lines.len() && lines[line_cursor] != "" {
            buffer += lines[line_cursor];
            line_cursor += 1;
        }
        if buffer != "" {
            let children = inline_formatting(buffer);
            tokens.push(Node::element("p", vec![], children));
        }
        line_cursor += 1;
    }

    tokens
}
