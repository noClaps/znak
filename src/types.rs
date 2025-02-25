use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct Text {
    pub value: String,
}

#[derive(Clone, Debug)]
pub struct Element {
    pub tag_name: String,
    pub properties: Option<HashMap<String, String>>,
    pub children: Vec<Node>,
}

#[derive(Clone, Debug)]
pub enum Node {
    Text(Text),
    Element(Element),
}

impl Node {
    pub fn text(value: &str) -> Node {
        Node::Text(Text {
            value: value.to_string(),
        })
    }

    pub fn element(
        tag_name: &str,
        properties: Option<HashMap<String, String>>,
        children: Vec<Node>,
    ) -> Node {
        Node::Element(Element {
            tag_name: tag_name.to_string(),
            properties,
            children,
        })
    }

    pub fn props(props: Vec<[&str; 2]>) -> Option<HashMap<String, String>> {
        let mut map = HashMap::new();
        for [key, val] in props {
            map.insert(key.to_string(), val.to_string());
        }
        Some(map)
    }
}
