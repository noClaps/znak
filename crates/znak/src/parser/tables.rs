use crate::parser::{
    inline_formatting::inline_formatting,
    types::{Node, element},
};

pub(crate) fn tables(input: &str) -> Node {
    let lines = input.trim().lines().collect::<Vec<&str>>();
    let head_row = lines[0];
    let align_row = lines[1];
    let tbody = lines[2..].to_vec();

    let thead = head_row[1..head_row.len() - 1]
        .split("|")
        .map(|item| item.trim());

    let alignments = align_row[1..align_row.len() - 1]
        .split("|")
        .map(|col| {
            let col = col.trim();
            match (col.starts_with(":"), col.ends_with(":")) {
                (true, true) => "center",
                (false, true) => "right",
                (true, false) => "left",
                _ => "",
            }
        })
        .collect::<Vec<&str>>();

    let mut thead_nodes = vec![];
    for (i, th) in thead.enumerate() {
        let children = inline_formatting(th.to_string());
        thead_nodes.push(element!("th", [align = alignments[i]], children));
    }

    let mut tbody_nodes = vec![];
    for line in tbody {
        let mut tr_nodes = vec![];
        for (i, col) in line[1..line.len() - 1].split("|").enumerate() {
            let children = inline_formatting(col.trim().to_string());
            tr_nodes.push(element!("td", [align = alignments[i]], children));
        }
        tbody_nodes.push(element!("tr", tr_nodes));
    }

    element!(
        "table",
        vec![
            element!("thead", vec![element!("tr", thead_nodes)]),
            element!("tbody", tbody_nodes),
        ]
    )
}
