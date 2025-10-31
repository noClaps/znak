use std::collections::HashMap;

/// An HTML node
///
/// A node can be:
///
/// - `Root`: The node type that represents the root of an HTML document. It
///   contains a list of child nodes.
/// - `DocType`: The node type that represents `<!doctype html>`.
/// - `Comment`: The node type that represents `<!-- HTML comments -->`. It
///   contains a string with the comment contents.
/// - `Element`: The node type that represents an HTML element. It contains the
///   tag name, a map of key-value pairs for the attributes, and a list of
///   child nodes.
/// - `Text`: The node type that represents text. It contains a string with the
///   text contents.
#[derive(Debug, PartialEq, Eq)]
pub enum Node {
    Root(Vec<Node>),
    DocType,
    Comment(String),
    Element {
        tag_name: String,
        properties: HashMap<String, String>,
        children: Vec<Node>,
    },
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
