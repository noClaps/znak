use crate::types::Node;

use super::inline_formatting::inline_formatting;

pub fn tables(input: String) -> Node {
    let lines = input.lines().collect::<Vec<&str>>();
    let head_row = lines[0];
    let align_row = lines[1];
    let tbody = lines[2..].to_vec();

    let thead = head_row[1..head_row.len() - 1]
        .split("|")
        .map(|col| col.trim());
    let alignments = align_row[1..align_row.len() - 1]
        .split("|")
        .map(|col| {
            let trimmed_col = col.trim();
            if trimmed_col.starts_with(":") && trimmed_col.ends_with(":") {
                return "center";
            }
            if trimmed_col.ends_with(":") {
                return "right";
            }
            if trimmed_col.starts_with(":") {
                return "left";
            }
            return "";
        })
        .collect::<Vec<&str>>();

    Node::element(
        "table",
        None,
        vec![
            Node::element(
                "thead",
                None,
                vec![Node::element(
                    "tr",
                    None,
                    thead
                        .enumerate()
                        .map(|(index, th)| {
                            Node::element(
                                "th",
                                Node::props(vec![["align", alignments[index]]]),
                                inline_formatting(th.to_string()),
                            )
                        })
                        .collect(),
                )],
            ),
            Node::element(
                "tbody",
                None,
                tbody
                    .into_iter()
                    .map(|line| {
                        Node::element(
                            "tr",
                            None,
                            line[1..line.len() - 1]
                                .split("|")
                                .enumerate()
                                .map(|(index, col)| {
                                    Node::element(
                                        "td",
                                        Node::props(vec![["align", alignments[index]]]),
                                        inline_formatting(col.trim().to_string()),
                                    )
                                })
                                .collect(),
                        )
                    })
                    .collect(),
            ),
        ],
    )
}
