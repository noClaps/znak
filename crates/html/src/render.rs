use std::fmt::Display;

use crate::Node;

impl Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut output = String::new();
        match self {
            Node::Comment(comment) => {
                output += &format!("<!-- {comment} -->");
            }
            Node::DocType => output += "<!doctype html>",
            Node::Element {
                tag_name,
                properties,
                children,
            } => match children.len() {
                0 => {
                    output += &format!(
                        "<{tag_name}{} />",
                        properties
                            .iter()
                            .map(|(k, v)| format!(" {k}=\"{v}\""))
                            .collect::<Vec<_>>()
                            .join("")
                    );
                }
                _ => {
                    output += &format!(
                        "<{tag_name}{}>{}</{tag_name}>",
                        properties
                            .iter()
                            .map(|(k, v)| format!(" {k}=\"{v}\""))
                            .collect::<Vec<_>>()
                            .join(""),
                        children
                            .iter()
                            .map(|c| c.to_string())
                            .collect::<Vec<_>>()
                            .join("")
                    );
                }
            },
            Node::Text(text) => output += text,
            Node::Root(root) => {
                output += &root
                    .iter()
                    .map(|el| el.to_string())
                    .collect::<Vec<_>>()
                    .join("");
            }
        }

        write!(f, "{}", output)
    }
}

#[cfg(test)]
mod tests {
    use crate::types::{doctype, element, root};

    use super::*;

    #[test]
    fn render() {
        let want = "<!doctype html><html lang=\"en\"><head /><body /></html>";
        let got = root!([
            doctype!(),
            element!("html", {"lang" => "en"}, [element!("head"), element!("body")])
        ]);
        assert_eq!(want, got.to_string());

        let start = "<html><head /></html>";
        assert_eq!(start, start.parse::<Node>().unwrap().to_string())
    }
}
