use crate::token::Token;

const SELF_CLOSING_TAGS: [&str; 3] = ["br", "hr", "img"];

pub fn render(tokens: Vec<Token>) -> String {
    tokens
        .into_iter()
        .map(|token| match token {
            Token::Text(text) => text,
            Token::Element {
                tag,
                attributes,
                children,
            } => {
                let attributes = attributes
                    .iter()
                    .map(|(k, v)| {
                        if v.contains('"') {
                            format!("{k}='{v}'")
                        } else {
                            format!("{k}=\"{v}\"")
                        }
                    })
                    .reduce(|acc, a| acc + " " + &a);
                if SELF_CLOSING_TAGS.contains(&tag.as_str()) {
                    match attributes {
                        Some(attr) => format!("<{tag} {attr}>"),
                        None => format!("<{tag}>"),
                    }
                } else {
                    match attributes {
                        Some(attr) => format!("<{tag} {attr}>{}</{tag}>", render(children)),
                        None => format!("<{tag}>{}</{tag}>", render(children)),
                    }
                }
            }
        })
        .collect()
}
