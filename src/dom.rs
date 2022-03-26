// TODO:
// extend NodeType to include additional types like comment nodes.
use std::collections::HashMap;

pub type AttrMap = HashMap<String, String>;

#[derive(Debug)]
pub struct Node {
    pub node_type: NodeType,
    pub children: Vec<Node>,
}

#[derive(Debug)]
pub enum NodeType {
    Text(String),
    Element(ElementData),
}
#[derive(Debug)]

pub struct ElementData {
    pub tag_name: String,
    pub attributes: AttrMap,
}

pub fn text(data: String) -> Node {
    Node {
        children: Vec::new(),
        node_type: NodeType::Text(data),
    }
}

pub fn element(name: String, attrs: AttrMap, children: Vec<Node>) -> Node {
    Node {
        children: children,
        node_type: NodeType::Element(ElementData {
            tag_name: name,
            attributes: attrs,
        }),
    }
}
