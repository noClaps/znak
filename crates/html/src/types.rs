use std::collections::HashMap;

/// An HTML node.
///
/// # Parsing
///
/// You can parse a string to a Node using the `.parse()` method on it. It is
/// safe to unwrap since the parser never panics, and any panics should be
/// [reported as bugs](https://github.com/noClaps/znak/issues/new). While the
/// goal isn’t to make a full HTML5-compliant parser, it still shouldn’t panic
/// if it encounters some strangely formatted HTML.
///
/// ```rust
/// use html::Node;
/// use std::collections::HashMap;
///
/// let html = "<!doctype html><html><body></body></html>";
/// let parsed: Node = html.parse().unwrap();
/// assert_eq!(parsed, Node::Root(vec![
///     Node::DocType,
///     Node::Element {
///         tag_name: "html".to_string(),
///         properties: HashMap::new(),
///         children: vec![
///             Node::Element {
///                 tag_name: "body".to_string(),
///                 properties: HashMap::new(),
///                 children: vec![]
///             }
///         ]
///     }
/// ]))
/// ```
///
/// # Rendering
///
/// When starting with a [Node], you can render it to an HTML string using the
/// `to_string()` method.
///
/// ```rust
/// use html::Node;
///
/// let root = Node::Root(vec![Node::DocType]);
/// assert_eq!(root.to_string(), "<!doctype html>");
/// ```
#[derive(Debug, PartialEq, Eq, Clone)]
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
    ($tag:expr, {$($key:expr => $val:expr$(,)?)+}, [$($child:expr$(,)?)+]) => {
        Node::Element {
            tag_name: $tag.to_string(),
            properties: std::collections::HashMap::from([$(($key.to_string(), $val.to_string()),)*]),
            children: vec![$($child,)*],
        }
    };
    ($tag:expr, {$($key:expr => $val:expr$(,)?)+}) => {
        Node::Element {
            tag_name: $tag.to_string(),
            properties: std::collections::HashMap::from([$(($key.to_string(), $val.to_string()),)*]),
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
