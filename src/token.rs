#[derive(Clone)]
pub enum Token {
    Element {
        tag: String,
        attributes: Vec<(String, String)>,
        children: Vec<Token>,
    },
    Text(String),
}

impl Token {
    pub fn element(tag: &str, attrs: &[(&str, &str)], children: &[Token]) -> Self {
        Self::Element {
            tag: tag.to_string(),
            attributes: attrs
                .iter()
                .map(|(k, v)| (k.to_string(), v.to_string()))
                .collect(),
            children: children.to_vec(),
        }
    }

    pub fn text(text: &str) -> Self {
        Self::Text(text.to_string())
    }
}
