use crate::{
    types::Node,
    utils::{escape_html::escape_html, math::render_math},
};
use pulldown_latex::config::DisplayMode;

fn find_char(line: &String, init: usize, pat: char) -> usize {
    init + line[init..].find(pat).unwrap_or_default()
}

fn find_str(line: &String, init: usize, pat: &str) -> usize {
    init + line[init..].find(pat).unwrap_or_default()
}

pub fn inline_formatting(line: String) -> Vec<Node> {
    let mut contents: Vec<Node> = vec![];

    let mut buffer = String::new();

    let mut cursor = 0;

    let cur = |cursor, amount: usize| {
        let mut temp_cur = cursor;

        if temp_cur + amount > line.len() {
            return temp_cur;
        }

        for _ in 0..amount {
            temp_cur += 1;
            while !line.is_char_boundary(temp_cur) {
                temp_cur += 1;
            }
        }
        temp_cur
    };

    while cursor < line.len() {
        // Escape characters
        if &line[cursor..cur(cursor, 1)] == "\\" {
            cursor = cur(cursor, 1);
            buffer += &line[cursor..cursor + 1];
            cursor = cur(cursor, 1);
            continue;
        }

        // Bold (**)
        if &line[cursor..cur(cursor, 2)] == "**" && line[cur(cursor, 2)..].contains("**") {
            // Push existing buffer and reset buffer
            if !buffer.is_empty() {
                contents.push(Node::text(&buffer));
                buffer.clear();
            }

            let next = find_str(&line, cur(cursor, 2), "**");
            let temp_buf = line[cur(cursor, 2)..next].to_string();

            if temp_buf.is_empty() {
                contents.push(Node::text("****"));
            } else {
                contents.push(Node::element("strong", None, inline_formatting(temp_buf)));
            }

            cursor = next;
            cursor = cur(cursor, 2);
            continue;
        }

        // Underline (__)
        if &line[cursor..cur(cursor, 2)] == "__" && line[cur(cursor, 2)..].contains("__") {
            // Push existing buffer and reset buffer
            if !buffer.is_empty() {
                contents.push(Node::text(&buffer));
                buffer.clear();
            }

            let next = find_str(&line, cur(cursor, 2), "__");
            let temp_buf = line[cur(cursor, 2)..next].to_string();

            if temp_buf.is_empty() {
                contents.push(Node::text("____"));
            } else {
                contents.push(Node::element("u", None, inline_formatting(temp_buf)));
            }

            cursor = next;
            cursor = cur(cursor, 2);
            continue;
        }

        // Strikethrough (~~)
        if &line[cursor..cur(cursor, 2)] == "~~" && line[cur(cursor, 2)..].contains("~~") {
            // Push existing buffer and reset buffer
            if !buffer.is_empty() {
                contents.push(Node::text(&buffer));
                buffer.clear();
            }

            let next = find_str(&line, cur(cursor, 2), "~~");
            let temp_buf = line[cur(cursor, 2)..next].to_string();

            if temp_buf.is_empty() {
                contents.push(Node::text("~~~~"));
            } else {
                contents.push(Node::element("s", None, inline_formatting(temp_buf)));
            }

            cursor = next;
            cursor = cur(cursor, 2);
            continue;
        }

        // Highlight (==)
        if &line[cursor..cur(cursor, 2)] == "==" && line[cur(cursor, 2)..].contains("==") {
            // Push existing buffer and reset buffer
            if !buffer.is_empty() {
                contents.push(Node::text(&buffer));
                buffer.clear();
            }

            let next = find_str(&line, cur(cursor, 2), "==");
            let temp_buf = line[cursor + 2..next].to_string();

            if temp_buf.is_empty() {
                contents.push(Node::text("===="));
            } else {
                contents.push(Node::element("mark", None, inline_formatting(temp_buf)));
            }

            cursor = next;
            cursor = cur(cursor, 2);
            continue;
        }

        // Inline math ($$)
        if &line[cursor..cur(cursor, 2)] == "$$" && line[cur(cursor, 2)..].contains("$$") {
            // Push existing buffer and reset buffer
            if !buffer.is_empty() {
                contents.push(Node::text(&buffer));
                buffer.clear();
            }

            let next = find_str(&line, cur(cursor, 2), "$$");
            let temp_buf = line[cursor + 2..next].to_string();

            if temp_buf.is_empty() {
                contents.push(Node::text("$$$$"));
            } else {
                contents.push(render_math(temp_buf, DisplayMode::Inline));
            }

            cursor = next;
            cursor = cur(cursor, 2);
            continue;
        }

        // Italics (_)
        if &line[cursor..cur(cursor, 1)] == "_" && line[cur(cursor, 1)..].contains("_") {
            // Push existing buffer and reset buffer
            if !buffer.is_empty() {
                contents.push(Node::text(&buffer));
                buffer.clear();
            }

            let next = find_char(&line, cur(cursor, 1), '_');
            let temp_buf = line[cur(cursor, 1)..next].to_string();

            if temp_buf.is_empty() {
                contents.push(Node::text("__"));
            } else {
                contents.push(Node::element("em", None, inline_formatting(temp_buf)));
            }

            cursor = next;
            cursor = cur(cursor, 1);
            continue;
        }

        // Code (`)
        if &line[cursor..cur(cursor, 1)] == "`" && line[cur(cursor, 1)..].contains("`") {
            // Push existing buffer and reset buffer
            if !buffer.is_empty() {
                contents.push(Node::text(&buffer));
                buffer.clear();
            }

            let next = find_char(&line, cur(cursor, 1), '`');
            let temp_buf = line[cur(cursor, 1)..next].to_string();

            if temp_buf.is_empty() {
                contents.push(Node::text("``"));
            } else {
                contents.push(Node::element(
                    "code",
                    None,
                    vec![Node::text(&escape_html(temp_buf))],
                ));
            }

            cursor = next;
            cursor = cur(cursor, 1);
            continue;
        }

        // Subscript (~)
        if &line[cursor..cur(cursor, 1)] == "~" && line[cur(cursor, 1)..].contains("~") {
            // Push existing buffer and reset buffer
            if !buffer.is_empty() {
                contents.push(Node::text(&buffer));
                buffer.clear();
            }

            let next = find_char(&line, cur(cursor, 1), '~');
            let temp_buf = line[cur(cursor, 1)..next].to_string();

            if temp_buf.is_empty() {
                contents.push(Node::text("~~"));
            } else {
                contents.push(Node::element("sub", None, inline_formatting(temp_buf)));
            }

            cursor = next;
            cursor = cur(cursor, 1);
            continue;
        }

        // Superscript (^)
        if &line[cursor..cur(cursor, 1)] == "^" && line[cur(cursor, 1)..].contains("^") {
            // Push existing buffer and reset buffer
            if !buffer.is_empty() {
                contents.push(Node::text(&buffer));
                buffer.clear();
            }

            let next = find_char(&line, cur(cursor, 1), '^');
            let temp_buf = line[cur(cursor, 1)..next].to_string();

            if temp_buf.is_empty() {
                contents.push(Node::text("^^"));
            } else {
                contents.push(Node::element("sup", None, inline_formatting(temp_buf)));
            }

            cursor = next;
            cursor = cur(cursor, 1);
            continue;
        }

        // Links
        if &line[cursor..cur(cursor, 1)] == "["
            && line[cursor..].contains("](")
            && line[cursor..].contains(")")
        {
            // Push existing buffer and reset buffer
            if !buffer.is_empty() {
                contents.push(Node::text(&buffer));
                buffer.clear();
            }

            // Move cursor inside link title
            let mut link_title = String::new();
            let mut is_inside_nested = false;
            cursor = cur(cursor, 1);
            while &line[cursor..cur(cursor, 1)] != "]" || is_inside_nested {
                if &line[cursor..cur(cursor, 1)] == "[" {
                    is_inside_nested = true;
                }
                if &line[cursor..cur(cursor, 1)] == "]" {
                    is_inside_nested = false
                }
                link_title += &line[cursor..cur(cursor, 1)];
                cursor = cur(cursor, 1)
            }

            // Move cursor inside link URL
            let mut link_url = String::new();
            let mut is_inside_link = false;
            cursor = cur(cursor, 2);
            while &line[cursor..cur(cursor, 1)] != ")" || is_inside_link {
                if &line[cursor..cur(cursor, 1)] == "<" {
                    is_inside_link = true;
                    cursor = cur(cursor, 1);
                    continue;
                }
                if &line[cursor..cur(cursor, 1)] == ">" {
                    is_inside_link = false;
                    cursor = cur(cursor, 1);
                    continue;
                }
                link_url += &line[cursor..cur(cursor, 1)];
                cursor = cur(cursor, 1)
            }

            if link_title.is_empty() || link_url.is_empty() {
                contents.push(Node::text(&format!("[{link_title}]({link_url})")));
            } else {
                contents.push(Node::element(
                    "a",
                    Node::props(vec![["href", &link_url]]),
                    inline_formatting(link_title),
                ));
            }

            cursor = cur(cursor, 1);
            continue;
        }

        // Default
        buffer += &line[cursor..cur(cursor, 1)];
        cursor = cur(cursor, 1);
    }

    if !buffer.is_empty() {
        contents.push(Node::text(&buffer));
    }

    return contents;
}
