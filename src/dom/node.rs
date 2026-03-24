use markup5ever_rcdom::NodeData as RcDomNodeData;
use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

use crate::dom::attr::Attribute;

#[derive(Default, Debug)]
pub enum NodeData {
    #[default]
    Document,
    Doctype {
        name: String,
        public_id: String,
        system_id: String,
    },
    Text(String),
    Comment(String),
    Element {
        name: String,
        attrs: Vec<Attribute>,
    },
    ProcessingInstruction {
        target: String,
        contents: String,
    },
}

/// The owned node with its actual contents, those are kept private and aren't
/// exposed directly. Instead the [`Node`] provides methods to get references
/// to them.
#[derive(Default, Debug)]
pub struct InnerNode {
    data: Rc<NodeData>,
    parent: Option<Weak<RefCell<InnerNode>>>,
    children: Vec<Node>,
}

impl InnerNode {
    fn new(data: NodeData) -> Self {
        Self {
            data: Rc::new(data),
            ..Default::default()
        }
    }
}

/// The public-facing part of the node containing a reference-counter pointer
/// to the inner node, allowing it to be copied cheaply and shared.
#[derive(Default, Debug, Clone)]
pub struct Node {
    handle: Rc<RefCell<InnerNode>>,
}

/// Convert a parsed RcDom tree into our own tree representation
impl From<markup5ever_rcdom::Handle> for Node {
    fn from(value: markup5ever_rcdom::Handle) -> Self {
        let node_data = match &value.data {
            RcDomNodeData::Document => NodeData::Document,
            RcDomNodeData::Doctype {
                name,
                public_id,
                system_id,
            } => NodeData::Doctype {
                name: name.into(),
                public_id: public_id.into(),
                system_id: system_id.into(),
            },
            RcDomNodeData::Comment { contents } => NodeData::Comment(contents.into()),
            RcDomNodeData::Text { contents } => NodeData::Text(contents.borrow().to_string()),
            RcDomNodeData::Element { name, attrs, .. } => NodeData::Element {
                name: name.local.to_string(),
                attrs: attrs
                    .borrow()
                    .iter()
                    .map(|attr| Attribute {
                        name: attr.name.local.to_string(),
                        value: attr.value.to_string(),
                    })
                    .collect(),
            },
            RcDomNodeData::ProcessingInstruction { target, contents } => {
                NodeData::ProcessingInstruction {
                    target: target.into(),
                    contents: contents.into(),
                }
            }
        };

        let mut children: Vec<Node> = Vec::new();

        for child in value.children.borrow().iter() {
            let converted_child: Node = child.clone().into();
            children.push(converted_child.into());
        }

        let mut node = Node::new(node_data);
        node.set_children(children);

        node
    }
}

impl Node {
    pub fn new(data: NodeData) -> Self {
        Self {
            handle: Rc::new(RefCell::new(InnerNode::new(data))),
        }
    }

    pub fn set_children(&mut self, children: Vec<Node>) {
        for child in children {
            child.handle.borrow_mut().parent = Some(Rc::downgrade(&self.handle));
            self.handle.borrow_mut().children.push(child);
        }
    }

    pub fn get_node_data(&self) -> Rc<NodeData> {
        self.handle.borrow().data.clone()
    }

    /// Get a reference to child by its index
    pub fn get_child(&self, idx: usize) -> Node {
        self.handle.borrow().children[idx].clone()
    }

    /// Clone the list of references to node children
    pub fn get_children(&self) -> Vec<Node> {
        self.handle.borrow().children.clone()
    }
}
