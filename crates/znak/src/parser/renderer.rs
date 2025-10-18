use crate::parser::types::Node;

pub(crate) fn renderer(token: Node) -> String {
    match token {
        Node::Text(text) => text,
        Node::Element(element) => {
            let mut attributes_list = String::new();
            for (key, val) in element.properties {
                attributes_list += format!(" {key}=\"{val}\"").as_str();
            }

            let mut contents = String::new();
            let children_len = element.children.len();

            if children_len == 0 {
                return format!("<{}{} />", element.tag_name, attributes_list);
            }

            for item in element.children {
                contents += renderer(item).as_str();
            }

            format!(
                "<{}{}>{}</{}>",
                element.tag_name, attributes_list, contents, element.tag_name
            )
        }
    }
}
