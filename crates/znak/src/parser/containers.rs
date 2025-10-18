use std::collections::HashMap;

use highlight::Theme;

use crate::parser::{parse, types::Node};

fn concat_vec<T>(mut vec_a: Vec<T>, mut vec_b: Vec<T>) -> Vec<T> {
    vec_a.append(&mut vec_b);
    vec_a
}

pub(crate) fn containers(input: String, code_theme: Theme) -> Node {
    let (head, body) = input.split_once("\n").unwrap();
    let body = body.trim();

    let head_sections = head.splitn(3, " ").collect::<Vec<&str>>();
    let container_type = head_sections[1];
    let meta = if head_sections.len() > 2 {
        head_sections[2]
    } else {
        ""
    };

    let next_attr_index = meta.find('{');
    let attr = match next_attr_index {
        Some(idx) => &meta[idx + 1..meta.len() - 1],
        None => "",
    };
    let title = container_type.to_uppercase();
    let title = match next_attr_index {
        Some(idx) if &meta[..idx] != "" => &meta[..idx],
        None if meta != "" => meta,
        _ => title.as_str(),
    }
    .trim();

    let mut attr_map = HashMap::new();
    if attr != "" {
        for a in attr.split(" ") {
            let (key, val) = a.split_once("=").unwrap();
            attr_map.insert(key.to_string(), val[1..val.len() - 1].to_string());
        }
    }

    let class = match attr_map.get("class") {
        Some(c) => format!("znak-container {} {}", container_type, c),
        None => format!("znak-container {}", container_type),
    };
    attr_map.insert("class".to_string(), class.trim().to_string());

    let href = attr_map.remove("href");

    let content = parse(body.to_string(), code_theme);

    Node::element_map(
        "div",
        attr_map,
        concat_vec(
            vec![Node::element(
                "p",
                vec![("class", format!("{}-heading", container_type).as_str())],
                vec![Node::element(
                    "b",
                    vec![],
                    vec![match href {
                        None => Node::text(title),
                        Some(href) => Node::element(
                            "a",
                            vec![("href", href.as_str())],
                            vec![Node::text(title)],
                        ),
                    }],
                )],
            )],
            content,
        ),
    )
}
