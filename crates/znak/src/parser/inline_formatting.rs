use std::mem;

use highlight::escape_html;
use math::{MathDisplay, render_math};

use crate::parser::types::Node;

pub(crate) fn inline_formatting(line: String) -> Vec<Node> {
    let mut contents = vec![];
    let mut buffer = String::new();
    let line = line.split("").filter(|&s| s != "").collect::<Vec<&str>>();

    let mut cursor = 0;
    while cursor < line.len() {
        // Escape characters
        if line[cursor] == "\\" {
            cursor += 1;
            buffer += line[cursor];
            cursor += 1;
            continue;
        }

        // Bold (**)
        if formatting(
            &line,
            &mut cursor,
            "**",
            &mut buffer,
            &mut contents,
            "strong",
        ) {
            continue;
        }

        // Underline (__)
        if formatting(&line, &mut cursor, "__", &mut buffer, &mut contents, "u") {
            continue;
        }

        // Strikethrough (~~)
        if formatting(&line, &mut cursor, "~~", &mut buffer, &mut contents, "s") {
            continue;
        }

        // Highlight (==)
        if formatting(&line, &mut cursor, "==", &mut buffer, &mut contents, "mark") {
            continue;
        }

        // Inline math ($$)
        if line[cursor..].join("").starts_with("$$") && line[cursor + 2..].join("").contains("$$") {
            if !buffer.is_empty() {
                contents.push(Node::text(mem::take(&mut buffer)));
            }
            // safe to unwrap as checked in if condition
            let next_index = cursor + 2 + line[cursor + 2..].join("").find("$$").unwrap();
            let temp_buf = line[cursor + 2..next_index].join("");

            if temp_buf.is_empty() {
                contents.push(Node::text("$$$$"));
            } else {
                let math = render_math(temp_buf.to_string(), MathDisplay::Inline);
                contents.push(Node::text(math));
            }
            cursor = next_index + 2;
            continue;
        }

        // Italics (_)
        if formatting(&line, &mut cursor, "_", &mut buffer, &mut contents, "em") {
            continue;
        }

        // Code (`)
        if line[cursor] == "`" && line[cursor + 1..].join("").contains('`') {
            if !buffer.is_empty() {
                contents.push(Node::text(mem::take(&mut buffer)));
            }
            // safe to unwrap as checked in if condition
            let next_index = cursor + 1 + line[cursor + 1..].join("").find("`").unwrap();
            let temp_buf = line[cursor + 1..next_index].join("");

            if temp_buf.is_empty() {
                contents.push(Node::text("``"));
            } else {
                let code = escape_html(temp_buf.to_string());
                contents.push(Node::element("code", vec![], vec![Node::text(code)]));
            }
            cursor = next_index + 1;
            continue;
        }

        // Subscript (~)
        if formatting(&line, &mut cursor, "~", &mut buffer, &mut contents, "sub") {
            continue;
        }

        // Superscript (^)
        if formatting(&line, &mut cursor, "^", &mut buffer, &mut contents, "sup") {
            continue;
        }

        // Links
        if line[cursor] == "["
            && line[cursor + 1..].join("").contains("](")
            && line[cursor + 1..].join("").contains(")")
        {
            if !buffer.is_empty() {
                contents.push(Node::text(mem::take(&mut buffer)));
            }

            let mut link_title = String::new();
            let mut nest_level = 0;
            cursor += 1; // Move inside link title
            while line[cursor] != "]" || nest_level != 0 {
                if line[cursor] == "[" {
                    nest_level += 1;
                }
                if line[cursor] == "]" {
                    nest_level -= 1;
                }
                link_title += line[cursor];
                cursor += 1;
            }

            let mut link_url = String::new();
            let mut is_inside_link = false;
            cursor += 2; // Move inside link URL
            while line[cursor] != ")" || is_inside_link {
                if line[cursor] == "<" {
                    is_inside_link = true;
                    cursor += 1;
                    continue;
                }
                if line[cursor] == ">" {
                    is_inside_link = false;
                    cursor += 1;
                    continue;
                }
                link_url += line[cursor];
                cursor += 1;
            }

            if link_title.is_empty() || link_url.is_empty() {
                contents.push(Node::text(format!("[{}]({})", link_title, link_url)));
            } else {
                let children = inline_formatting(link_title);
                contents.push(Node::element(
                    "a",
                    vec![("href", link_url.as_str())],
                    children,
                ));
            }

            cursor += 1;
            continue;
        }

        // Default
        buffer += line[cursor];
        cursor += 1;
    }

    if !buffer.is_empty() {
        contents.push(Node::text(buffer));
    }

    contents
}

fn formatting(
    line: &Vec<&str>,
    cursor: &mut usize,
    pat: &'static str,
    buffer: &mut String,
    contents: &mut Vec<Node>,
    tag_name: &'static str,
) -> bool {
    match pat.len() {
        1 if line[*cursor] == pat && line[*cursor + 1..].contains(&pat) => {
            if !buffer.is_empty() {
                contents.push(Node::text(mem::take(buffer)));
            }
            // safe to unwrap as checked in if condition
            let next_index =
                *cursor + 1 + line[*cursor + 1..].iter().position(|&s| s == pat).unwrap();
            let temp_buf = line[*cursor + 1..next_index].join("");

            if temp_buf.is_empty() {
                contents.push(Node::text(pat.repeat(2)));
            } else {
                let children = inline_formatting(temp_buf);
                contents.push(Node::element(tag_name, vec![], children));
            }
            *cursor = next_index + 1;
            return true;
        }
        2 if line[*cursor..].join("").starts_with(pat)
            && line[*cursor + 2..].join("").contains(&pat) =>
        {
            if !buffer.is_empty() {
                contents.push(Node::text(mem::take(buffer)));
            }
            // safe to unwrap as checked in if condition
            let next_index = *cursor + 2 + line[*cursor + 2..].join("").find(pat).unwrap();
            let temp_buf = line[*cursor + 2..next_index].join("");

            if temp_buf.is_empty() {
                contents.push(Node::text(pat.repeat(2)));
            } else {
                let children = inline_formatting(temp_buf);
                contents.push(Node::element(tag_name, vec![], children));
            }
            *cursor = next_index + 2;
            return true;
        }
        _ => false,
    }
}
