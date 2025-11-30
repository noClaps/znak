use std::mem;

use html::escape_html;
use math::{MathDisplay, render_math};

use crate::parser::types::{Node, element, text};

// TODO: rewrite this in terms of a char iterator, and DO NOT .collect() TO A String
pub(crate) fn inline_formatting(line: &str) -> Vec<Node> {
    let mut contents = vec![];
    let buffer = &mut String::new();
    let line: Vec<_> = line.chars().collect();

    let mut cursor = 0;

    macro_rules! formatting {
        ($tag:literal, $pat:literal) => {
            if line[cursor] == $pat
                && let Some(next_index) = line[cursor + 1..].iter().position(|&c| c == $pat)
            {
                if !buffer.is_empty() {
                    contents.push(text!(mem::take(buffer)));
                }
                cursor += 1;
                let temp_buf: String = line[cursor..cursor + next_index].iter().collect();

                if temp_buf.is_empty() {
                    contents.push(text!(format!("{}{}", $pat, $pat)));
                } else {
                    let children = inline_formatting(&temp_buf);
                    contents.push(element!($tag, children));
                }
                cursor += next_index + 1;
                continue;
            }
        };
    }

    while cursor < line.len() {
        // Escape characters
        if line[cursor] == '\\' {
            cursor += 1;
            buffer.push(line[cursor]);
            cursor += 1;
            continue;
        }

        // Bold (*)
        formatting!("strong", '*');

        // Inline math ($)
        if line[cursor] == '$'
            && let Some(next_index) = line[cursor + 1..].iter().position(|&c| c == '$')
        {
            if !buffer.is_empty() {
                contents.push(text!(mem::take(buffer)));
            }
            cursor += 1;
            let temp_buf: String = line[cursor..cursor + next_index].iter().collect();

            if temp_buf.is_empty() {
                contents.push(text!("$$"));
            } else {
                let math = render_math(&temp_buf, MathDisplay::Inline);
                contents.push(text!(math));
            }
            cursor += next_index + 1;
            continue;
        }

        // Italics (_)
        formatting!("em", '_');

        // Code (`)
        if line[cursor] == '`'
            && let Some(next_index) = line[cursor + 1..].iter().position(|&c| c == '`')
        {
            if !buffer.is_empty() {
                contents.push(text!(mem::take(buffer)));
            }
            cursor += 1;
            let temp_buf: String = line[cursor..cursor + next_index].iter().collect();

            if temp_buf.is_empty() {
                contents.push(text!("``"));
            } else {
                let code = escape_html!(temp_buf);
                contents.push(element!("code", vec![text!(code)]));
            }
            cursor += next_index + 1;
            continue;
        }

        // Subscript (~)
        formatting!("sub", '~');

        // Superscript (^)
        formatting!("sup", '^');

        // Links
        if line[cursor] == '['
            && line[cursor + 1..].windows(2).any(|w| match w {
                [']', '('] => true,
                _ => false,
            })
            && line[cursor + 1..].iter().any(|&c| c == ')')
        {
            if !buffer.is_empty() {
                contents.push(text!(mem::take(buffer)));
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

            if link_title.is_empty() || link_url.is_empty() {
                contents.push(text!(format!("[{}]({})", link_title, link_url)));
            } else {
                let children = inline_formatting(&link_title);
                contents.push(element!("a", [href = link_url.as_str()], children));
            }

            cursor += 1;
            continue;
        }

        // Default
        buffer.push(line[cursor]);
        cursor += 1;
    }

    if !buffer.is_empty() {
        contents.push(text!(buffer));
    }

    contents
}
