use crate::parser::types::Node;

pub(crate) fn renderer(token: Node) -> String {
    match token {
        Node::Text(text) => text,
        Node::Element {
            tag_name,
            children,
            properties,
        } => {
            let mut attributes_list = String::new();
            for (key, val) in properties {
                attributes_list += format!(" {key}=\"{val}\"").as_str();
            }

            let mut contents = String::new();
            let children_len = children.len();

            if children_len == 0 {
                return format!("<{}{} />", tag_name, attributes_list);
            }

            for item in children {
                contents += renderer(item).as_str();
            }

            format!(
                "<{}{}>{}</{}>",
                tag_name, attributes_list, contents, tag_name
            )
        }
    }
}
