use std::{collections::HashMap, str::FromStr};

use crate::types::{Node, comment, doctype, element, root, text};

#[cfg(test)]
mod tests;

/// Parses HTML into a tree of [Nodes](Node).
///
/// The parser is pretty simple, and doesn't fully support all of the strange
/// behaviors of HTML. However, any panics should be
/// [reported as a bug](https://github.com/noClaps/znak/issues/new). While the
/// goal isn't to make a full HTML5-compliant parser, it still shouldn't panic
/// if it encounters some strangely formatted HTML.
///
/// ```rust
/// use std::collections::HashMap;
/// use html::Node;
///
/// let html = "<!doctype html><html></html>";
/// let parsed: Node = html.parse().unwrap();
/// assert_eq!(parsed, Node::Root(vec![
///     Node::DocType,
///     Node::Element {
///         tag_name: "html".to_string(),
///         properties: HashMap::new(),
///         children: vec![],
///     },
/// ]));
/// ```
impl FromStr for Node {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let root = parse_impl(s, |c| c.is_whitespace());
        Ok(root!(root))
    }
}

macro_rules! clen {
    ($str:expr) => {
        $str.chars().count()
    };
}

macro_rules! c2str {
    ($iter:expr) => {
        $iter.iter().collect::<String>()
    };
}

fn parse_impl(input: &str, skip: fn(char) -> bool) -> Vec<Node> {
    let mut nodes = vec![];
    let chars: Vec<_> = input.chars().collect();

    let mut i = 0;
    while i < chars.len() {
        if skip(chars[i]) {
            i += 1;
            continue;
        }

        if c2str!(chars[i..])
            .to_lowercase()
            .starts_with("<!doctype html>")
        {
            nodes.push(doctype!());
            i += clen!("<!doctype html>");
            continue;
        }

        if c2str!(chars[i..]).starts_with("<!--") {
            i += clen!("<!--");
            let mut comment = String::new();
            while i < chars.len() && !c2str!(chars[i..]).starts_with("-->") {
                comment.push(chars[i]);
                i += 1;
            }
            nodes.push(comment!(comment.trim()));
            i += clen!("-->");
            continue;
        }

        if chars[i] == '<' {
            i += 1; // enter opening tag
            let mut opening_tag = String::new();
            let mut inside = false;
            while chars[i] != '>' || inside {
                if chars[i] == '"' {
                    inside = !inside;
                }
                opening_tag.push(chars[i]);
                i += 1;
            }

            let (tag_name, attrs) = match opening_tag.split_once(" ") {
                Some((t, a)) => (t.to_string(), a),
                None => (opening_tag.clone(), ""),
            };

            i += 1; // exit opening tag

            if opening_tag.ends_with("/") {
                let attributes = parse_attrs(&attrs[..attrs.len() - 1]);
                nodes.push(element!(tag_name, attributes, vec![]));
                continue;
            }

            let attributes = parse_attrs(attrs);

            let close_tag = format!("</{}>", tag_name);
            let open_tag = format!("<{}", tag_name);
            let mut inner_html = String::new();
            let mut depth = 0;
            let original_i = i;
            while i < chars.len() && !c2str!(chars[i..]).starts_with(&close_tag) || depth > 0 {
                if c2str!(chars[i..]).starts_with(&open_tag) {
                    depth += 1;
                }
                if c2str!(chars[i..]).starts_with(&close_tag) {
                    depth -= 1;
                }
                inner_html.push(chars[i]);
                i += 1;
                if i == chars.len() {
                    // turns out it was a self-closing element
                    inner_html.clear();
                    i = original_i;
                    break;
                }
            }

            let skip = match tag_name.as_str() {
                "pre" => |_| false,
                "p" => |c: char| c.is_whitespace() && c != ' ' && c != '\t',
                _ => skip,
            };
            let children = parse_impl(&inner_html, skip);
            nodes.push(element!(tag_name, attributes, children));

            i += clen!(close_tag);
            continue;
        }

        let text_pos = match chars[i..].iter().position(|&c| c == '<') {
            Some(pos) => pos,
            None => chars[i..].len(),
        };
        let text = c2str!(chars[i..i + text_pos]);
        nodes.push(text!(text));

        i += text_pos;
    }

    nodes
}

fn parse_attrs(attrs: &str) -> HashMap<String, String> {
    let attrs = attrs.to_string();
    if attrs.is_empty() {
        return HashMap::new();
    }

    let mut attributes = HashMap::new();
    let attrs: Vec<_> = attrs.chars().collect();
    let mut i = 0;
    while i < attrs.len() {
        if attrs[i].is_whitespace() {
            i += 1;
            continue;
        }

        // parse key
        let mut key = String::new();
        while i < attrs.len() && attrs[i] != '=' && attrs[i] != ' ' {
            key.push(attrs[i]);
            i += 1;
        }

        // parse value
        if i == attrs.len() || attrs[i] == ' ' {
            attributes.insert(key, "true".to_string());
            i += 1;
            continue;
        }
        i += 1; // skip =
        let mut val = String::new();
        if attrs[i] == '"' {
            i += 1; // skip "
            while i < attrs.len() && attrs[i] != '"' {
                val.push(attrs[i]);
                i += 1;
            }
        } else {
            while i < attrs.len() && attrs[i] != ' ' {
                val.push(attrs[i]);
                i += 1;
            }
        }
        attributes.insert(key, val);

        i += 1;
    }

    attributes
}
