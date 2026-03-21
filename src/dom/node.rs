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

#[derive(Default, Debug)]
pub struct Node {
    pub data: NodeData,
    pub parent: Option<Weak<RefCell<Node>>>,
    pub children: Vec<Handle>,
}

pub type Handle = Rc<RefCell<Node>>;

impl From<Node> for Handle {
    fn from(value: Node) -> Self {
        Rc::new(RefCell::new(value))
    }
}

impl Node {
    pub fn new(data: NodeData) -> Handle {
        Self {
            data,
            ..Default::default()
        }
        .into()
    }

    pub fn new_with_children(children: Vec<Handle>, data: NodeData) -> Handle {
        let node = Self::new(data);

        // Set parent on children
        for child in &children {
            child.borrow_mut().parent = Some(Rc::downgrade(&node));
        }

        node.borrow_mut().children = children;
        node
    }

    pub fn from_rcdom(value: markup5ever_rcdom::Handle) -> Handle {
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

        let mut children: Vec<Handle> = Vec::new();

        for child in value.children.borrow().iter() {
            let converted_child = Node::from_rcdom(child.clone());
            children.push(converted_child.into());
        }

        Node::new_with_children(children, node_data)
    }
}
