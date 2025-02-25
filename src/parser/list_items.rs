use highlight::Theme;
use regex::Regex;

use crate::types::Node;

use super::parser::parse;

pub enum ListType {
    Ordered,
    Unordered,
}

pub fn list_items(input: String, code_theme: Theme, list_type: ListType) -> Vec<Node> {
    let (re, seg_re) = match list_type {
        ListType::Ordered => (r"(?m)^\d+\. ", r"^(   |\t)"),
        ListType::Unordered => (r"(?m)^- ", r"^(  |\t)"),
    };

    let re = match Regex::new(re) {
        Ok(re) => re,
        Err(_) => unreachable!(),
    };

    let lines = re.split(&input).filter(|l| !l.is_empty()).map(|l| l.trim());
    let tokens = lines
        .map(|line| {
            let segments = line.lines();
            if segments.clone().count() == 1 {
                return Node::element("li", None, parse(line.to_string(), code_theme.clone()));
            }

            let segments = segments.collect::<Vec<&str>>();
            return Node::element(
                "li",
                None,
                parse(
                    format!(
                        "{}\n\n{}",
                        segments[0],
                        segments[1..]
                            .into_iter()
                            .map(|l| {
                                let re = match Regex::new(seg_re) {
                                    Ok(re) => re,
                                    Err(_) => unreachable!(),
                                };
                                re.replace(l, "").to_string()
                            })
                            .collect::<Vec<String>>()
                            .join("\n")
                    ),
                    code_theme.clone(),
                ),
            );
        })
        .collect();
    tokens
}
