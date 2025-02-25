use crate::types::Node;
use highlight::{highlight, Theme};

pub fn highlight_syntax(code: String, language: String, theme: Theme) -> Node {
    Node::text(&highlight(code, language, theme))
}
