use highlight::escape_html;
use math::{MathDisplay, render_math};

use crate::parser::{types::Node, utils::char_string::CharString};

pub(crate) fn inline_formatting(line: String) -> Vec<Node> {
    let mut contents = vec![];
    let mut buffer = String::new();
    let line = CharString::from(line);

    let mut cursor = 0;
    while cursor < line.len() {
        // Escape characters
        if line[cursor] == '\\' {
            cursor += 1;
            buffer.push(line[cursor]);
            cursor += 1;
            continue;
        }

        // Bold (**)
        if line[cursor..].starts_with("**") && line[cursor + 2..].contains("**") {
            if buffer != "" {
                contents.push(Node::text(buffer));
                buffer = String::new();
            }
            // safe to unwrap as checked in if condition
            let next_index = cursor + 2 + line[cursor + 2..].find("**").unwrap();
            let temp_buf = line[cursor + 2..next_index].clone();

            if temp_buf == "" {
                contents.push(Node::text("****"));
            } else {
                let children = inline_formatting(temp_buf.to_string());
                contents.push(Node::element("strong", vec![], children));
            }
            cursor = next_index + 2;
            continue;
        }

        // Underline (__)
        if line[cursor..].starts_with("__") && line[cursor + 2..].contains("__") {
            if buffer != "" {
                contents.push(Node::text(buffer));
                buffer = String::new();
            }
            // safe to unwrap as checked in if condition
            let next_index = cursor + 2 + line[cursor + 2..].find("__").unwrap();
            let temp_buf = line[cursor + 2..next_index].clone();

            if temp_buf == "" {
                contents.push(Node::text("____"));
            } else {
                let children = inline_formatting(temp_buf.to_string());
                contents.push(Node::element("u", vec![], children));
            }
            cursor = next_index + 2;
            continue;
        }

        // Strikethrough (~~)
        if line[cursor..].starts_with("~~") && line[cursor + 2..].contains("~~") {
            if buffer != "" {
                contents.push(Node::text(buffer));
                buffer = String::new();
            }
            // safe to unwrap as checked in if condition
            let next_index = cursor + 2 + line[cursor + 2..].find("~~").unwrap();
            let temp_buf = line[cursor + 2..next_index].clone();

            if temp_buf == "" {
                contents.push(Node::text("~~~~"));
            } else {
                let children = inline_formatting(temp_buf.to_string());
                contents.push(Node::element("s", vec![], children));
            }
            cursor = next_index + 2;
            continue;
        }

        // Highlight (==)
        if line[cursor..].starts_with("==") && line[cursor + 2..].contains("==") {
            if buffer != "" {
                contents.push(Node::text(buffer));
                buffer = String::new();
            }
            // safe to unwrap as checked in if condition
            let next_index = cursor + 2 + line[cursor + 2..].find("==").unwrap();
            let temp_buf = line[cursor + 2..next_index].clone();

            if temp_buf == "" {
                contents.push(Node::text("===="));
            } else {
                let children = inline_formatting(temp_buf.to_string());
                contents.push(Node::element("mark", vec![], children));
            }
            cursor = next_index + 2;
            continue;
        }

        // Inline math (**)
        if line[cursor..].starts_with("$$") && line[cursor + 2..].contains("$$") {
            if buffer != "" {
                contents.push(Node::text(buffer));
                buffer = String::new();
            }
            // safe to unwrap as checked in if condition
            let next_index = cursor + 2 + line[cursor + 2..].find("$$").unwrap();
            let temp_buf = line[cursor + 2..next_index].clone();

            if temp_buf == "" {
                contents.push(Node::text("$$$$"));
            } else {
                let math = render_math(temp_buf.to_string(), MathDisplay::Inline);
                contents.push(Node::text(math));
            }
            cursor = next_index + 2;
            continue;
        }

        // Italics (_)
        if line[cursor..].starts_with("_") && line[cursor + 1..].contains("_") {
            if buffer != "" {
                contents.push(Node::text(buffer));
                buffer = String::new();
            }
            // safe to unwrap as checked in if condition
            let next_index = cursor + 1 + line[cursor + 1..].find("_").unwrap();
            let temp_buf = line[cursor + 1..next_index].clone();

            if temp_buf == "" {
                contents.push(Node::text("__"));
            } else {
                let children = inline_formatting(temp_buf.to_string());
                contents.push(Node::element("em", vec![], children));
            }
            cursor = next_index + 1;
            continue;
        }

        // Code (`)
        if line[cursor..].starts_with("`") && line[cursor + 1..].contains("`") {
            if buffer != "" {
                contents.push(Node::text(buffer));
                buffer = String::new();
            }
            // safe to unwrap as checked in if condition
            let next_index = cursor + 1 + line[cursor + 1..].find("`").unwrap();
            let temp_buf = line[cursor + 1..next_index].clone();

            if temp_buf == "" {
                contents.push(Node::text("``"));
            } else {
                let code = escape_html(temp_buf.to_string());
                contents.push(Node::element("code", vec![], vec![Node::text(code)]));
            }
            cursor = next_index + 1;
            continue;
        }

        // Subscript (~)
        if line[cursor..].starts_with("~") && line[cursor + 1..].contains("~") {
            if buffer != "" {
                contents.push(Node::text(buffer));
                buffer = String::new();
            }
            // safe to unwrap as checked in if condition
            let next_index = cursor + 1 + line[cursor + 1..].find("~").unwrap();
            let temp_buf = line[cursor + 1..next_index].clone();

            if temp_buf == "" {
                contents.push(Node::text("~~"));
            } else {
                let children = inline_formatting(temp_buf.to_string());
                contents.push(Node::element("sub", vec![], children));
            }
            cursor = next_index + 1;
            continue;
        }

        // Superscript (^)
        if line[cursor..].starts_with("^") && line[cursor + 1..].contains("^") {
            if buffer != "" {
                contents.push(Node::text(buffer));
                buffer = String::new();
            }
            // safe to unwrap as checked in if condition
            let next_index = cursor + 1 + line[cursor + 1..].find("^").unwrap();
            let temp_buf = line[cursor + 1..next_index].clone();

            if temp_buf == "" {
                contents.push(Node::text("^^"));
            } else {
                let children = inline_formatting(temp_buf.to_string());
                contents.push(Node::element("sup", vec![], children));
            }
            cursor = next_index + 1;
            continue;
        }

        // Links
        if line[cursor] == '['
            && line[cursor + 1..].contains("](")
            && line[cursor + 1..].contains(")")
        {
            if buffer != "" {
                contents.push(Node::text(buffer));
                buffer = String::new();
            }

            let mut link_title = String::new();
            let mut nest_level = 0;
            cursor += 1; // Move inside link title
            while line[cursor] != ']' || nest_level != 0 {
                if line[cursor] == '[' {
                    nest_level += 1;
                }
                if line[cursor] == ']' {
                    nest_level -= 1;
                }
                link_title.push(line[cursor]);
                cursor += 1;
            }

            let mut link_url = String::new();
            let mut is_inside_link = false;
            cursor += 2; // Move inside link URL
            while line[cursor] != ')' || is_inside_link {
                if line[cursor] == '<' {
                    is_inside_link = true;
                    cursor += 1;
                    continue;
                }
                if line[cursor] == '>' {
                    is_inside_link = false;
                    cursor += 1;
                    continue;
                }
                link_url.push(line[cursor]);
                cursor += 1;
            }

            if link_title == "" || link_url == "" {
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
        buffer.push(line[cursor]);
        cursor += 1;
    }

    if buffer != "" {
        contents.push(Node::text(buffer));
    }

    contents
}
