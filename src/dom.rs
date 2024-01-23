use std::collections::{HashMap,HashSet};

pub struct Node{
    child: Vec<Node>,
    node_type:NodeType,

}
#[derive(Debug)]
pub enum NodeType{
    Text(String),
    Comment(String),
    Element(ElementData), 
}
#[derive(Debug)]
pub struct ElementData{
    tag_name: String,
    attribute: AttrMap,
}

pub type AttrMap = HashMap<String,String>;

pub fn text(data: String)-> Node{
    Node{
        child: Vec::new(),
        node_type: NodeType::Text(data),
    }
}

pub fn comment(data: String)->Node{
    Node { 
        child: Vec::new(), 
        node_type: NodeType::Comment(data),  
    }
}

pub fn elem(name:String, attribute:AttrMap, child:Vec<Node>)->Node{
    Node{
        child: child,
        node_type: NodeType::Element(ElementData{
            tag_name:name,
            attribute:attribute,
        })
    }

}