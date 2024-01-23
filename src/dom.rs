use std::collections::{HashMap,HashSet};

struct Node{
    child: Vec<Node>,
    node_type:NodeType,

}
#[derive(Debug)]
enum NodeType{
    Text(String),
    Element(ElementData), 
}
#[derive(Debug)]
struct ElementData{
    tag_name: String,
    attribute: AttrMap,
}

type AttrMap = HashMap<String,String>;

fn text(data: String)-> Node{
    Node{
        child: Vec::new(),
        node_type: NodeType::Text(data),
    }
}

fn elem(name:String, attribute:AttrMap, child:Vec<Node>)->Node{
    Node{
        child: child,
        node_type: NodeType::Element(ElementData{
            tag_name:name,
            attribute:attribute,
        })
    }

}