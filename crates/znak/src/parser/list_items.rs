use highlight::Highlight;
use regex::Regex;

use crate::parser::{
    parse,
    types::{Node, element},
};

pub(crate) enum ListType {
    Ordered,
    Unordered,
}

pub(crate) fn list_items(input: String, hl: &Highlight, list_type: ListType) -> Vec<Node> {
    let (re, seg_match) = match list_type {
        // can unwrap as known safe regex
        ListType::Ordered => (Regex::new(r#"(?m)^\d+\. "#).unwrap(), "   "),
        ListType::Unordered => (Regex::new("(?m)^- ").unwrap(), "  "),
    };

    let re_split_input = re.split(input.as_str());
    let mut lines = vec![];
    for line in re_split_input {
        if line == "" {
            continue;
        }
        lines.push(line.trim());
    }

    let mut tokens = vec![];
    for line in lines {
        let segments = line.split("\n").collect::<Vec<&str>>();
        if segments.len() == 1 {
            let children = parse(segments[0].to_string(), hl);
            tokens.push(element!("li", children));
        } else {
            let mut remaining_lines = String::new();
            for &line in segments[1..].iter() {
                remaining_lines += format!("{}\n", line.replacen(seg_match, "", 1)).as_str();
            }
            let input = format!("{}\n\n{}", segments[0], remaining_lines);
            let children = parse(input, hl);
            tokens.push(element!("li", children));
        }
    }

    tokens
}
