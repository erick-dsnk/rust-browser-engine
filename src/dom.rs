use std::collections::{
    HashMap,
    HashSet,
};
use std::fmt;

#[derive(PartialEq, Eq, Clone)]
pub struct Node {
    pub node_type: NodeType,
    pub children: Vec<Node>,
}

#[derive(PartialEq, Eq, Clone)]
pub enum NodeType {
    Text(String),
    Element(ElementData),
    Comment(String),
}

#[derive(PartialEq, Eq, Clone)]
pub struct ElementData {
    pub tag_name: String,
    attrs: AttrMap,
}

impl ElementData {
    pub fn new(tag_name: String, attrs: AttrMap) -> Self {
        return ElementData {
            tag_name,
            attrs,
        }
    }

    pub fn get_id(&self) -> Option<&String> {
        return self.attrs.get("id")
    }

    pub fn get_classes(&self) -> HashSet<&str> {
        return match self.attrs.get("class") {
            Some(s) => s.split(' ').collect(),
            None => HashSet::new(),
        }
    }
}

pub type AttrMap = HashMap<String, String>;

impl Node {
    pub fn new(node_type: NodeType, children: Vec<Node>) -> Self {      
        return Node {
            node_type,
            children,
        }
    }
}

impl fmt::Debug for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.node_type)
    }
}

impl fmt::Debug for NodeType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return match *self {
            NodeType::Text(ref t) | NodeType::Comment(ref t) => write!(f, "{}", t),
            NodeType::Element(ref e) => write!(f, "{:?}", e),
        }
    }
}

impl fmt::Debug for ElementData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut attr_string = String::new();

        for (attr, value) in self.attrs.iter() {
            attr_string.push_str(&format!(" {}=\"{}\"", attr, value));
        }

        write!(f, "<{},{}>", self.tag_name, attr_string)
    }
}

pub fn pretty_print(n: &Node, indent_size: usize) {
    let indent = (0 .. indent_size).map(|_| " ").collect::<String>();

    match n.node_type {
        NodeType::Element(ref e) => println!("{}{:?}", indent, e),
        NodeType::Text(ref t) => println!("{}{}", indent, t),
        NodeType::Comment(ref c) => println!("{}<!--{}-->", indent, c),
    };

    for child in n.children.iter() {
        pretty_print(&child, indent_size + 4);
    }

    match n.node_type {
        NodeType::Element(ref e) => println!("{}<{}/>", indent, e.tag_name),
        _ => {}, 
    }
}