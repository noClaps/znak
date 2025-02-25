use std::collections::HashMap;

use highlight::Theme;

use crate::types::Node;

use super::parser::parse;

pub fn containers(input: String, code_theme: Theme) -> Node {
    let (head, body) = input.split_once("\n").unwrap_or_default();
    let body = body.trim();

    let head_sections = head.splitn(3, " ").collect::<Vec<&str>>();
    let container_type = head_sections[1];
    let meta = if head_sections.len() > 2 {
        head_sections[2]
    } else {
        ""
    };

    let next_attr_index = meta.find("{");
    let attr = match next_attr_index {
        None => "",
        Some(index) => &meta[index + 1..meta.len() - 1],
    };

    let title = if meta.replace(&format!("{{{}}}", attr), "").is_empty() {
        container_type.to_uppercase()
    } else {
        meta.replace(&format!("{{{}}}", attr), "")
    };
    let title = title.trim();

    let mut attr_object = HashMap::new();
    if !attr.is_empty() {
        for a in attr.split(" ") {
            let (key, val) = a.split_once("=").unwrap_or_default();
            attr_object.insert(key.to_string(), val[1..val.len() - 1].to_string());
        }
    }

    attr_object.insert(
        "class".to_string(),
        format!(
            "znak-container {} {}",
            container_type,
            attr_object.get("class").unwrap_or(&"".to_string())
        )
        .trim()
        .to_string(),
    );

    let href = attr_object.remove("href");

    let lines = body.lines().collect::<Vec<&str>>();
    let content = lines.join("\n");

    Node::element(
        "div",
        Some(attr_object),
        vec![Node::element(
            "p",
            Node::props(vec![["class", &format!("{}-heading", container_type)]]),
            vec![Node::element(
                "b",
                None,
                vec![match href {
                    None => Node::text(&title),
                    Some(href) => Node::element(
                        "a",
                        Node::props(vec![
                            ["href", &href],
                            ["target", "_blank"],
                            ["rel", "noopener noreferrer"],
                        ]),
                        vec![Node::text(&title)],
                    ),
                }],
            )],
        )]
        .into_iter()
        .chain(parse(content.to_string(), code_theme))
        .collect(),
    )
}
