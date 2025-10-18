use std::collections::HashMap;

pub(crate) enum Node {
    Element(Element),
    Text(String),
}

impl Node {
    pub(crate) fn element<S: Into<String>>(
        tag_name: S,
        properties: Vec<(S, S)>,
        children: Vec<Node>,
    ) -> Self {
        let mut props = HashMap::new();
        for (k, v) in properties {
            props.insert(k.into(), v.into());
        }
        Node::Element(Element {
            tag_name: tag_name.into(),
            properties: props,
            children: children,
        })
    }

    pub(crate) fn element_map<S: Into<String>>(
        tag_name: S,
        properties: HashMap<String, String>,
        children: Vec<Node>,
    ) -> Node {
        Node::Element(Element {
            tag_name: tag_name.into(),
            properties: properties,
            children: children,
        })
    }

    pub(crate) fn text<S: Into<String>>(text: S) -> Self {
        Node::Text(text.into())
    }
}

pub(crate) struct Element {
    pub(crate) tag_name: String,
    pub(crate) properties: HashMap<String, String>,
    pub(crate) children: Vec<Node>,
}
