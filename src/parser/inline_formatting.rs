use crate::{
    types::Node,
    utils::{escape_html::escape_html, math::render_math},
};
use pulldown_latex::config::DisplayMode;

fn parse_1_char(
    line: &Vec<(usize, char)>,
    cursor: usize,
    char: char,
    elm: &str,
) -> (Option<Node>, usize) {
    let mut node = None;
    let two_char = [char; 2].iter().collect::<String>();
    let mut cursor = cursor;

    if cursor + 1 < line.len()
        && line[cursor].1 == char
        && line[cursor + 1..]
            .iter()
            .map(|l| l.1)
            .collect::<Vec<char>>()
            .contains(&char)
    {
        let next = line[cursor + 1..]
            .iter()
            .find(|&&(_, c)| c == char)
            .unwrap()
            .0;
        let next_pos = line.iter().position(|&(i, _)| i == next).unwrap();
        let temp_buf = line[cursor + 1..next_pos]
            .iter()
            .map(|l| l.1)
            .collect::<String>();

        if temp_buf.is_empty() {
            node = Some(Node::text(&two_char));
        } else {
            node = Some(Node::element(elm, None, inline_formatting(temp_buf)));
        }

        cursor = next_pos + 1;
    }

    (node, cursor)
}

fn parse_2_chars(
    line: &Vec<(usize, char)>,
    cursor: usize,
    char: char,
    elm: &str,
) -> (Option<Node>, usize) {
    let line = line.clone();
    let mut node = None;
    let two_char = [char; 2].iter().collect::<String>();
    let four_char = [char; 4].iter().collect::<String>();
    let mut cursor = cursor;

    if cursor + 2 < line.len()
        && line[cursor..cursor + 2]
            .iter()
            .map(|i| i.1)
            .collect::<String>()
            == two_char.as_str()
        && line[cursor + 2..]
            .iter()
            .map(|l| l.1)
            .collect::<String>()
            .contains(&two_char)
    {
        let next = line[cursor + 2..]
            .iter()
            .copied()
            .find(|&(i, _)| {
                i + 2 <= line.len()
                    && line[i..i + 2].iter().map(|l| l.1).collect::<String>() == two_char.as_str()
            })
            .unwrap()
            .0;
        let next_pos = line.iter().position(|&(i, _)| i == next).unwrap();
        let temp_buf = line[cursor + 2..next_pos]
            .iter()
            .map(|l| l.1)
            .collect::<String>();

        if temp_buf.is_empty() {
            node = Some(Node::text(&four_char));
        } else {
            node = Some(Node::element(elm, None, inline_formatting(temp_buf)));
        }

        cursor = next_pos + 2;
    }

    (node, cursor)
}

pub fn inline_formatting(line: String) -> Vec<Node> {
    let mut contents: Vec<Node> = vec![];
    let mut buffer: Vec<char> = vec![];
    let mut cursor = 0;
    let line = line.char_indices().collect::<Vec<(usize, char)>>();

    while cursor < line.len() {
        // Escape characters
        if line[cursor].1 == '\\' {
            cursor += 1;
            buffer.push(line[cursor].1);
            cursor += 1;
            continue;
        }

        if !buffer.is_empty() {
            contents.push(Node::text(&buffer.iter().collect::<String>()));
            buffer.clear();
        }

        // Bold (**)
        match parse_2_chars(&line, cursor, '*', "strong") {
            (Some(node), cur) => {
                contents.push(node);
                cursor = cur;
                continue;
            }
            _ => (),
        }

        // Underline (__)
        match parse_2_chars(&line, cursor, '_', "u") {
            (Some(node), cur) => {
                contents.push(node);
                cursor = cur;
                continue;
            }
            _ => (),
        }

        // Strikethrough (~~)
        match parse_2_chars(&line, cursor, '~', "s") {
            (Some(node), cur) => {
                contents.push(node);
                cursor = cur;
                continue;
            }
            _ => (),
        }

        // Highlight (==)
        match parse_2_chars(&line, cursor, '=', "mark") {
            (Some(node), cur) => {
                contents.push(node);
                cursor = cur;
                continue;
            }
            _ => (),
        }

        // Inline math ($$)
        if cursor + 2 < line.len()
            && line[cursor..cursor + 2]
                .iter()
                .map(|l| l.1)
                .collect::<String>()
                == "$$"
            && line[cursor + 2..]
                .iter()
                .map(|l| l.1)
                .collect::<String>()
                .contains("$$")
        {
            // Push existing buffer and reset buffer
            if !buffer.is_empty() {
                contents.push(Node::text(&buffer.iter().collect::<String>()));
                buffer.clear();
            }

            let next = line[cursor + 2..]
                .iter()
                .cloned()
                .find(|&(i, _)| {
                    i + 2 <= line.len()
                        && line[i..i + 2]
                            .iter()
                            .map(|l| l.1)
                            .collect::<String>()
                            .contains("$$")
                })
                .unwrap()
                .0;
            let next_pos = line.iter().position(|&(i, _)| i == next).unwrap();
            let temp_buf = line[cursor + 2..next_pos]
                .iter()
                .map(|l| l.1)
                .collect::<String>();

            if temp_buf.is_empty() {
                contents.push(Node::text("$$$$"));
            } else {
                contents.push(render_math(temp_buf, DisplayMode::Inline));
            }

            cursor = next_pos + 2;
            continue;
        }

        // Italics (_)
        match parse_1_char(&line, cursor, '_', "em") {
            (Some(node), cur) => {
                contents.push(node);
                cursor = cur;
                continue;
            }
            _ => (),
        }

        // Code (`)
        if cursor + 1 < line.len()
            && line[cursor].1 == '`'
            && line[cursor + 1..]
                .iter()
                .map(|l| l.1)
                .collect::<Vec<char>>()
                .contains(&'`')
        {
            // Push existing buffer and reset buffer
            if !buffer.is_empty() {
                contents.push(Node::text(&buffer.iter().collect::<String>()));
                buffer.clear();
            }

            let next = line[cursor + 1..]
                .iter()
                .cloned()
                .find(|&(i, _)| line[i].1 == '`')
                .unwrap()
                .0;
            let next_pos = line.iter().position(|&(i, _)| i == next).unwrap();
            let temp_buf = line[cursor + 1..next_pos]
                .iter()
                .map(|l| l.1)
                .collect::<String>();

            if temp_buf.is_empty() {
                contents.push(Node::text("``"));
            } else {
                contents.push(Node::element(
                    "code",
                    None,
                    vec![Node::text(&escape_html(temp_buf))],
                ));
            }

            cursor = next_pos + 1;
            continue;
        }

        // Subscript (~)
        match parse_1_char(&line, cursor, '~', "sub") {
            (Some(node), cur) => {
                contents.push(node);
                cursor = cur;
                continue;
            }
            _ => (),
        }

        // Superscript (^)
        match parse_1_char(&line, cursor, '^', "sup") {
            (None, cur) => cursor = cur,
            (Some(node), cur) => {
                contents.push(node);
                cursor = cur;
                continue;
            }
        }

        // Links
        if line[cursor].1 == '['
            && line[cursor..]
                .iter()
                .map(|l| l.1)
                .collect::<String>()
                .contains("](")
            && line[cursor..]
                .iter()
                .map(|l| l.1)
                .collect::<Vec<char>>()
                .contains(&')')
        {
            // Push existing buffer and reset buffer
            if !buffer.is_empty() {
                contents.push(Node::text(&buffer.iter().collect::<String>()));
                buffer.clear();
            }

            // Move cursor inside link title
            let mut link_title = String::new();
            let mut is_inside_nested = false;
            cursor += 1;
            while line[cursor].1 != ']' || is_inside_nested {
                if line[cursor].1 == '[' {
                    is_inside_nested = true;
                }
                if line[cursor].1 == ']' {
                    is_inside_nested = false
                }
                link_title += &line[cursor].1.to_string();
                cursor += 1;
            }

            // Move cursor inside link URL
            let mut link_url = String::new();
            let mut is_inside_link = false;
            cursor += 2;
            while line[cursor].1 != ')' || is_inside_link {
                if line[cursor].1 == '<' {
                    is_inside_link = true;
                    cursor += 1;
                    continue;
                }
                if line[cursor].1 == '>' {
                    is_inside_link = false;
                    cursor += 1;
                    continue;
                }
                link_url += &line[cursor].1.to_string();
                cursor += 1;
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

            cursor += 1;
            continue;
        }

        // Default
        buffer.push(line[cursor].1);
        cursor += 1;
    }

    if !buffer.is_empty() {
        contents.push(Node::text(&buffer.iter().collect::<String>()));
    }

    return contents;
}
