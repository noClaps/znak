use crate::{
    errors::{Error, Result},
    parser::{inline::parse_inline, slugger::Slugger},
    token::Token,
};

#[allow(clippy::too_many_lines)]
pub fn parse_block(source: &str) -> Result<Vec<Token>> {
    let lines: Vec<_> = source.lines().collect();
    let mut tokens = vec![];
    let mut slugger = Slugger::new();
    let mut paragraph_lines = vec![];

    macro_rules! push_p {
        () => {
            if !paragraph_lines.is_empty() {
                tokens.push(Token::element(
                    "p",
                    &[],
                    &parse_inline(&paragraph_lines.join(" "))?,
                ));
                paragraph_lines.clear();
            }
        };
    }

    let mut i = 0;
    while i < lines.len() {
        // comment
        if lines[i].starts_with("\\\\ ") {
            push_p!();
            i += 1;
            continue;
        }
        // heading
        if lines[i].starts_with('=') {
            push_p!();
            let mut heading_level = 1;
            while &lines[i][heading_level..=heading_level] == "=" {
                heading_level += 1;
            }
            if &lines[i][heading_level..=heading_level] != " " {
                tokens.push(Token::element("p", &[], &parse_inline(lines[i])?));
                i += 1;
                continue;
            }
            let body = &lines[i][heading_level + 1..];
            let id = slugger.slug(body);
            tokens.push(Token::element(
                &format!("h{heading_level}"),
                &[("id", &id)],
                &parse_inline(body)?,
            ));
            i += 1;
            continue;
        }
        // blockquote
        if lines[i].starts_with("> ") {
            push_p!();
            let mut blockquote_lines = vec![];
            while i < lines.len() && (lines[i].starts_with("> ") || lines[i] == ">") {
                blockquote_lines.push(lines[i][1..].trim());
                i += 1;
            }
            tokens.push(Token::element(
                "blockquote",
                &[],
                &parse_block(&blockquote_lines.join("\n"))?,
            ));
            let mut attribution_lines = vec![];
            while i < lines.len() && lines[i].starts_with(">-- ") {
                if attribution_lines.is_empty() {
                    attribution_lines.push("—");
                }
                attribution_lines.push(&lines[i][4..]);
                i += 1;
            }
            if !attribution_lines.is_empty() {
                tokens.push(Token::element(
                    "p",
                    &[],
                    &parse_inline(&attribution_lines.join(" "))?,
                ));
            }
            i += 1;
            continue;
        }
        // un/ordered list
        if lines[i].starts_with("- ") || lines[i].starts_with("+ ") {
            push_p!();
            let (prefix, tag) = if lines[i].starts_with("- ") {
                ("- ", "ul")
            } else {
                ("+ ", "ol")
            };
            let mut list_items = vec![];
            while i < lines.len() && lines[i].starts_with(prefix) {
                let mut list_item = vec![&lines[i][2..]];
                i += 1;
                while i < lines.len() && (lines[i].starts_with("  ") || lines[i].is_empty()) {
                    if lines[i].is_empty() {
                        list_item.push(lines[i]);
                    } else {
                        list_item.push(&lines[i][2..]);
                    }
                    i += 1;
                }
                list_items.push(Token::element(
                    "li",
                    &[],
                    &parse_block(&list_item.join("\n"))?,
                ));
            }
            tokens.push(Token::element(tag, &[], &list_items));
            i += 1;
            continue;
        }
        // code blocks
        if lines[i].starts_with("```") {
            push_p!();
            let backtick_count = lines[i].chars().take_while(|&c| c == '`').count();
            let lang = if lines[i].len() > backtick_count {
                &lines[i][backtick_count..]
            } else {
                ""
            };
            i += 1;
            let mut code_lines = vec![];
            while i < lines.len() && lines[i] != "`".repeat(backtick_count) {
                code_lines.push(lines[i]);
                i += 1;
            }
            let attr = if lang.is_empty() {
                vec![]
            } else {
                vec![("data-lang", lang)]
            };
            tokens.push(Token::element(
                "pre",
                &attr,
                &[Token::element(
                    "code",
                    &[],
                    &[Token::text(
                        &code_lines
                            .join("\n")
                            .replace('&', "&amp;")
                            .replace('<', "&lt;"),
                    )],
                )],
            ));
            i += 1;
            continue;
        }
        // container
        if lines[i].starts_with(":::") {
            push_p!();
            let mut parts = lines[i]
                .splitn(3, char::is_whitespace)
                .filter_map(|s| (!s.is_empty()).then_some(s.trim()));
            let colons = parts.next().unwrap();
            let Some(name) = parts.next() else {
                return Err(Error::content(lines[i], "Expected container name"));
            };
            let heading = match parts.next() {
                Some(h) => h.to_string(),
                None => name.to_uppercase(),
            };
            let colon_count = colons.chars().take_while(|&c| c == ':').count();
            i += 1;
            let mut body = vec![];
            while i < lines.len() && lines[i] != ":".repeat(colon_count) {
                body.push(lines[i]);
                i += 1;
            }
            let mut children = vec![Token::element(
                "p",
                &[("class", "container-heading")],
                &parse_inline(&heading)?,
            )];
            children.append(&mut parse_block(&body.join("\n"))?);
            tokens.push(Token::element(
                "div",
                &[("class", &format!("container {name}"))],
                &children,
            ));
            i += 1;
            continue;
        }
        // block math
        if lines[i] == "$$" {
            push_p!();
            i += 1;
            let mut math_lines = vec![];
            while i < lines.len() && lines[i] != "$$" {
                math_lines.push(lines[i]);
                i += 1;
            }
            tokens.push(Token::element(
                "div",
                &[("class", "math block")],
                &[Token::text(&math_lines.join("\n"))],
            ));
            i += 1;
            continue;
        }
        // table
        if lines[i].starts_with("| ") {
            push_p!();
            let cols_count = lines[i].split('|').filter(|s| !s.trim().is_empty()).count();
            let mut rows = vec![];
            while i < lines.len() && lines[i].starts_with("| ") {
                let cells: Vec<_> = lines[i]
                    .split('|')
                    .filter_map(|s| (!s.trim().is_empty()).then_some(s.trim()))
                    .collect();
                if cells.len() != cols_count {
                    return Err(Error::content(
                        lines[i],
                        &format!(
                            "Mismatched column count in table. Expected {cols_count}, found {}",
                            cells.len()
                        ),
                    ));
                }
                rows.push(cells);
                i += 1;
            }
            let mut parts = [vec![], vec![], vec![]];
            let mut parts_idx = 0;
            for row in rows {
                if row.iter().all(|s| s.chars().all(|c| c == '-')) && parts_idx + 1 != parts.len() {
                    parts_idx = (parts_idx + 1).min(parts.len() - 1);
                    continue;
                }
                parts[parts_idx].push(row);
            }
            let mut head = parts[0].clone();
            let mut body = parts[1].clone();
            let foot = parts[2].clone();
            if !head.is_empty() && body.is_empty() && foot.is_empty() {
                body.append(&mut head);
            }

            let mut children = vec![];
            if !head.is_empty() {
                let mut head_children = vec![];
                for row in head {
                    let mut row_children = vec![];
                    for el in row {
                        row_children.push(Token::element("th", &[], &parse_block(el)?));
                    }
                    head_children.push(Token::element("tr", &[], &row_children));
                }
                children.push(Token::element("thead", &[], &head_children));
            }
            if !body.is_empty() {
                let mut body_children = vec![];
                for row in body {
                    let mut row_children = vec![];
                    for el in row {
                        row_children.push(Token::element("td", &[], &parse_block(el)?));
                    }
                    body_children.push(Token::element("tr", &[], &row_children));
                }
                children.push(Token::element("tbody", &[], &body_children));
            }
            if !foot.is_empty() {
                let mut foot_children = vec![];
                for row in foot {
                    let mut row_children = vec![];
                    for el in row {
                        row_children.push(Token::element("td", &[], &parse_block(el)?));
                    }
                    foot_children.push(Token::element("tr", &[], &row_children));
                }
                children.push(Token::element("tfoot", &[], &foot_children));
            }
            tokens.push(Token::element("table", &[], &children));
            i += 1;
            continue;
        }
        // horizontal rule
        if lines[i].chars().all(|c| c == '-') && lines[i].len() > 3 {
            push_p!();
            tokens.push(Token::element("hr", &[], &[]));
            i += 1;
            continue;
        }
        // images
        if lines[i].starts_with("![") && lines[i].contains("]<") && lines[i].ends_with('>') {
            push_p!();
            let chars: Vec<_> = lines[i].chars().skip(2).collect();
            let mut j = 0;
            let mut alt = String::new();
            let mut depth = 0;
            while j < chars.len() {
                match (chars[j], depth) {
                    (']', 0) => break,
                    ('[', _) => depth += 1,
                    (']', 1..) => depth -= 1,
                    _ => (),
                }
                alt.push(chars[j]);
                j += 1;
            }
            j += 2;
            let mut url = String::new();
            while j < chars.len() {
                match (chars[j], depth) {
                    ('>', 0) => break,
                    ('<', _) => depth += 1,
                    ('>', 1..) => depth -= 1,
                    _ => (),
                }
                url.push(chars[j]);
                j += 1;
            }

            let mut attributes = vec![("src", url.as_str())];
            if !alt.is_empty() {
                attributes.push(("alt", alt.as_str()));
            }
            tokens.push(Token::element("img", &attributes, &[]));
            i += 1;
            continue;
        }
        // raw
        if lines[i].starts_with("{{") {
            push_p!();
            i += 1;
            let mut content = vec![];
            while i < lines.len() && lines[i] != "}}" {
                content.push(lines[i]);
                i += 1;
            }
            tokens.push(Token::text(&content.join("\n")));
            i += 1;
            continue;
        }

        if lines[i].trim().is_empty() {
            push_p!();
        } else {
            paragraph_lines.push(lines[i]);
        }
        i += 1;
    }
    push_p!();

    Ok(tokens)
}
