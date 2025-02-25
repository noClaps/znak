use crate::types::Node;

pub fn renderer(token: Node) -> String {
    match token {
        Node::Text(text) => text.value,
        Node::Element(element) => {
            let mut attributes_list = String::new();
            match element.properties {
                None => (),
                Some(props) => {
                    for (key, val) in props {
                        attributes_list += &format!(" {key}=\"{val}\"");
                    }
                }
            };

            let mut contents = String::new();
            let children_len = element.children.len();

            for item in element.children {
                contents += &renderer(item);
            }

            if children_len == 0 {
                return format!("<{}{} />", element.tag_name, attributes_list);
            }

            format!(
                "<{}{}>{}</{}>",
                element.tag_name, attributes_list, contents, element.tag_name
            )
        }
    }
}
