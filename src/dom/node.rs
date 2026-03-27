use crate::dom::{
    attr::Attribute,
    tree::{NodeId, Tree},
};

#[derive(Default, Clone, Debug)]
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

#[derive(Default, Clone, Debug)]
pub struct Node {
    /// Id of this node in the parent tree
    ///
    /// None if this node does not belong to any tree.
    pub(super) id: Option<NodeId>,

    /// Id of parent of this node in the tree
    ///
    /// None if this node does not have a parent
    pub(super) parent: Option<NodeId>,

    pub data: NodeData,
    pub style: taffy::Style,
    pub unrounded_layout: taffy::Layout,
    pub final_layout: taffy::Layout,

    pub(super) children: Vec<NodeId>,
}

impl Node {
    pub fn new(data: NodeData) -> Self {
        Self {
            data,
            ..Default::default()
        }
    }

    pub(super) fn adopt_tree(&mut self, id: NodeId) {
        self.id = Some(id);
    }

    pub fn get_child<'a>(&self, tree: &'a Tree, idx: usize) -> &'a Node {
        tree.get_node(self.children[idx])
    }

    pub fn get_children<'a>(&self, tree: &'a Tree) -> Vec<&'a Node> {
        self.children.iter().map(|id| tree.get_node(*id)).collect()
    }
}
