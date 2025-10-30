use std::collections::HashMap;

pub(crate) enum Node {
    Element {
        tag_name: String,
        properties: HashMap<String, String>,
        children: Vec<Node>,
    },
    Text(String),
}

macro_rules! element {
    ($tag:expr) => {
        Node::Element {
            tag_name: $tag.to_string(),
            properties: std::collections::HashMap::new(),
            children: vec![],
        }
    };
    ($tag:expr, [$($key:ident=$val:expr$(,)?)+]) => {
        Node::Element {
            tag_name: $tag.to_string(),
            properties: std::collections::HashMap::from([$((stringify!($key).to_string(), $val.to_string()),)+]),
            children: vec![],
        }
    };
    ($tag:expr, $children:expr) => {
        Node::Element {
            tag_name: $tag.to_string(),
            properties: std::collections::HashMap::new(),
            children: $children,
        }
    };
    ($tag:expr, [$($key:ident=$val:expr$(,)?)+], $children:expr) => {
        Node::Element {
            tag_name: $tag.to_string(),
            properties: std::collections::HashMap::from([$((stringify!($key).to_string(), $val.to_string()),)+]),
            children: $children,
        }
    };
    ($tag:expr, {$props:expr}, $children:expr) => {
        Node::Element {
            tag_name: $tag.to_string(),
            properties: $props,
            children: $children,
        }
    };
}
macro_rules! text {
    ($text:expr) => {
        Node::Text($text.to_string())
    };
}
pub(crate) use {element, text};
