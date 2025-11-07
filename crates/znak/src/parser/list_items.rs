use std::mem;

use highlight::Highlight;

use crate::parser::{
    parse,
    types::{Node, element},
};

pub(crate) enum ListType {
    Ordered,
    Unordered,
}

pub(crate) fn list_items(input: String, hl: &Highlight, list_type: ListType) -> Vec<Node> {
    let (split_input, seg_match) = match list_type {
        ListType::Ordered => {
            let input = input.lines();
            let mut lines = vec![];
            let mut val = String::new();
            for line in input {
                let mut chars = line.chars().skip_while(|c| c.is_digit(10));
                if chars.next().is_some_and(|c| c == '.') && chars.next().is_some_and(|c| c == ' ')
                {
                    lines.push(mem::take(&mut val));
                    val += chars.collect::<String>().as_str();
                    val += "\n";
                } else {
                    val += line;
                    val += "\n";
                };
            }
            lines.push(mem::take(&mut val));
            (lines[1..].to_vec(), "   ")
        }
        ListType::Unordered => {
            let input = input.lines();
            let mut lines = vec![];
            let mut val = String::new();
            for line in input {
                if line.starts_with("- ") {
                    lines.push(mem::take(&mut val));
                    val += &line[2..];
                    val += "\n";
                } else {
                    val += line;
                    val += "\n";
                }
            }
            lines.push(mem::take(&mut val));
            (lines, "  ")
        }
    };

    let mut lines = vec![];
    for line in split_input.iter().map(|l| l.as_str()) {
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
