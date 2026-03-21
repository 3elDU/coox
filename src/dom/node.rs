use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

use crate::dom::attr::Attribute;

#[derive(Default)]
pub enum NodeData {
    #[default]
    Document,
    Text(String),
    Comment(String),
    Element {
        name: String,
        attrs: Vec<Attribute>,
    },
}

#[derive(Default)]
pub struct Node {
    pub parent: Option<Weak<Handle>>,
    pub children: Vec<Handle>,
    pub data: NodeData,
}

pub type Handle = Rc<RefCell<Node>>;

impl From<Node> for Handle {
    fn from(value: Node) -> Self {
        Rc::new(RefCell::new(value))
    }
}

impl Node {
    pub fn new(data: NodeData) -> Self {
        Self {
            data,
            ..Default::default()
        }
    }

    pub fn new_with_children(children: Vec<Node>, data: NodeData) -> Self {
        Self {
            children: children.into_iter().map(|child| child.into()).collect(),
            data,
            ..Default::default()
        }
    }
}
