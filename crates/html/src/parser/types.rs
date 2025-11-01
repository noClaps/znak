use std::collections::HashMap;

/// An HTML node.
#[derive(Debug, PartialEq, Eq)]
pub enum Node {
    /// The node type that represents the root of an HTML document. It contains
    /// a list of child nodes.
    Root(Vec<Node>),
    /// The node type that represents `<!doctype html>`. No other doctypes are
    /// accepted.
    DocType,
    /// The node type that represents `<!-- HTML comments -->`. It contains a
    /// string with the comment contents.
    Comment(String),
    /// The node type that represents an HTML element.
    Element {
        /// The tag name of the element.
        tag_name: String,
        /// A key-value map of the attributes of the element.
        properties: HashMap<String, String>,
        /// The children of the element. This is empty if the element is empty,
        /// or if it's a self-closing element.
        children: Vec<Node>,
    },
    /// The node type that represents text. It contains a string with the text
    /// contents.
    Text(String),
}

macro_rules! comment {
    ($comment:expr) => {
        Node::Comment($comment.to_string())
    };
}

macro_rules! doctype {
    () => {
        Node::DocType
    };
}

macro_rules! element {
    ($tag:expr, {$($key:ident: $val:expr$(,)?)+}, [$($child:expr$(,)?)+]) => {
        Node::Element {
            tag_name: $tag.to_string(),
            properties: std::collections::HashMap::from([$((stringify!($key).to_string(), $val.to_string()),)*]),
            children: vec![$($child,)*],
        }
    };
    ($tag:expr, {$($key:ident: $val:expr$(,)?)+}) => {
        Node::Element {
            tag_name: $tag.to_string(),
            properties: std::collections::HashMap::from([$((stringify!($key).to_string(), $val.to_string()),)*]),
            children: vec![],
        }
    };
    ($tag:expr, $props:expr, $children:expr) => {
        Node::Element {
            tag_name: $tag.to_string(),
            properties: $props,
            children: $children,
        }
    };
    ($tag:expr) => {
        Node::Element {
            tag_name: $tag.to_string(),
            properties: std::collections::HashMap::new(),
            children: vec![],
        }
    };
    ($tag:expr, [$($child:expr$(,)?)+]) => {
        Node::Element {
            tag_name: $tag.to_string(),
            properties: std::collections::HashMap::new(),
            children: vec![$($child,)*],
        }
    };
}

macro_rules! root {
    ([$($child:expr$(,)?)*]) => {
        Node::Root(vec![$($child,)*])
    };
    ($children:expr) => {
        Node::Root($children)
    };
}

macro_rules! text {
    ($text:expr) => {
        Node::Text($text.to_string())
    };
}
pub(crate) use {comment, doctype, element, root, text};
