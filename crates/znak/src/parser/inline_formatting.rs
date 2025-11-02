use std::mem;

use html::escape_html;
use math::{MathDisplay, render_math};

use crate::parser::types::{Node, element, text};

macro_rules! c2str {
    ($iter:expr) => {
        $iter.iter().collect::<String>()
    };
}

macro_rules! cfind {
    ($haystack:expr, $pat:expr) => {
        $haystack
            .windows($pat.len())
            .position(|w| c2str!(w) == $pat)
    };
}

macro_rules! formatting {
    ($tag:expr, c=$pat:expr, $line:expr, $cursor:expr, $contents:expr, $buffer:expr) => {
        if $line[$cursor] == $pat && $line[$cursor + 1..].contains(&$pat) {
            if !$buffer.is_empty() {
                $contents.push(text!(mem::take($buffer)));
            }
            $cursor += 1;
            // safe to unwrap as checked in if condition
            let next_index = $line[$cursor..].iter().position(|&c| c == $pat).unwrap();
            let temp_buf = c2str!($line[$cursor..$cursor + next_index]);

            if temp_buf.is_empty() {
                $contents.push(text!(c2str!(vec![$pat, $pat])));
            } else {
                let children = inline_formatting(temp_buf);
                $contents.push(element!($tag, children));
            }
            $cursor += next_index + 1;
            continue;
        }
    };
    ($tag:expr, s=$pat:expr, $line:expr, $cursor:expr, $contents:expr, $buffer:expr) => {
        if c2str!($line[$cursor..]).starts_with($pat) && c2str!($line[$cursor + 2..]).contains($pat)
        {
            if !$buffer.is_empty() {
                $contents.push(text!(mem::take($buffer)));
            }
            $cursor += 2;
            // safe to unwrap as checked in if condition
            let next_index = cfind!($line[$cursor..], $pat).unwrap();
            let temp_buf = c2str!($line[$cursor..$cursor + next_index]);

            if temp_buf.is_empty() {
                $contents.push(text!($pat.repeat(2)));
            } else {
                let children = inline_formatting(temp_buf);
                $contents.push(element!($tag, children));
            }
            $cursor += next_index + 2;
            continue;
        }
    };
}

pub(crate) fn inline_formatting(line: String) -> Vec<Node> {
    let mut contents = vec![];
    let buffer = &mut String::new();
    let line: Vec<_> = line.chars().collect();

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
        formatting!("strong", s = "**", line, cursor, contents, buffer);

        // Underline (__)
        formatting!("u", s = "__", line, cursor, contents, buffer);

        // Strikethrough (~~)
        formatting!("s", s = "~~", line, cursor, contents, buffer);

        // Highlight (==)
        formatting!("mark", s = "==", line, cursor, contents, buffer);

        // Inline math ($$)
        if c2str!(line[cursor..]).starts_with("$$") && c2str!(line[cursor + 2..]).contains("$$") {
            if !buffer.is_empty() {
                contents.push(text!(mem::take(buffer)));
            }
            cursor += 2;
            // safe to unwrap as checked in if condition
            let next_index = cfind!(line[cursor..], "$$").unwrap();
            let temp_buf = c2str!(line[cursor..cursor + next_index]);

            if temp_buf.is_empty() {
                contents.push(text!("$$$$"));
            } else {
                let math = render_math(temp_buf.to_string(), MathDisplay::Inline);
                contents.push(text!(math));
            }
            cursor += next_index + 2;
            continue;
        }

        // Italics (_)
        formatting!("em", c = '_', line, cursor, contents, buffer);

        // Code (`)
        if line[cursor] == '`' && line[cursor + 1..].contains(&'`') {
            if !buffer.is_empty() {
                contents.push(text!(mem::take(buffer)));
            }
            cursor += 1;
            // safe to unwrap as checked in if condition
            let next_index = line[cursor..].iter().position(|&c| c == '`').unwrap();
            let temp_buf = c2str!(line[cursor..cursor + next_index]);

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
        formatting!("sub", c = '~', line, cursor, contents, buffer);

        // Superscript (^)
        formatting!("sup", c = '^', line, cursor, contents, buffer);

        // Links
        if line[cursor] == '['
            && c2str!(line[cursor + 1..]).contains("](")
            && c2str!(line[cursor + 1..]).contains(")")
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
                let children = inline_formatting(link_title);
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
