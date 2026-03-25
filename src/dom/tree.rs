use markup5ever_rcdom::NodeData as RcDomNodeData;

use crate::dom::{
    attr::Attribute,
    node::{Node, NodeData},
};

#[derive(Clone, Copy, Debug)]
/// An index into the parent tree. This is to avoid confusion between
/// node-local indices and global indices.
pub(super) struct NodeId(usize);

impl From<usize> for NodeId {
    fn from(value: usize) -> Self {
        Self(value)
    }
}

impl From<NodeId> for usize {
    fn from(value: NodeId) -> Self {
        value.0
    }
}

#[derive(Clone, Debug)]
pub struct Tree {
    /// A flatenned list of nodes.
    /// Each node has a list of its children ids
    pub(super) nodes: Vec<Node>,

    /// Id of the root node. This will be set when a first node is added into
    /// the tree
    root: NodeId,
}

impl Default for Tree {
    fn default() -> Self {
        Self {
            nodes: Vec::new(),
            root: usize::MAX.into(),
        }
    }
}

impl Tree {
    /// Get the root node. This will panic on a non-populated tree
    pub fn get_root_node(&self) -> &Node {
        self.get_node(self.root)
    }

    pub(super) fn get_node(&self, id: impl Into<NodeId>) -> &Node {
        &self.nodes[id.into().0]
    }

    pub(super) fn get_node_mut(&mut self, id: impl Into<NodeId>) -> &mut Node {
        &mut self.nodes[id.into().0]
    }

    pub(super) fn add_node(&mut self, node: Node) -> NodeId {
        self.nodes.push(node);
        let idx = self.nodes.len() - 1;

        if idx == 0 {
            self.root = idx.into();
        }

        self.nodes[idx].adopt_tree(idx.into());

        idx.into()
    }

    /// Append a node as a child of another node
    pub fn append_child(&mut self, parent: &mut Node, child: Node) {
        // Add node to the tree if necessary
        let child = child.id.unwrap_or_else(|| self.add_node(child));

        // FIXME: Remove node from other parent if it has one

        parent.children.push(child);
        self.get_node_mut(child).parent = Some(parent.id.unwrap());
    }

    /// Append an existing node as a child of another node
    pub(super) fn append_child_id(&mut self, parent: NodeId, child: NodeId) {
        self.nodes[parent.0].children.push(child);
        self.nodes[child.0].parent = Some(parent);
    }
}

fn convert_tree(tree: &mut Tree, rcdom_node: markup5ever_rcdom::Handle) -> NodeId {
    let data = match &rcdom_node.data {
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

    let parent = tree.add_node(Node::new(data));

    for child in rcdom_node.children.borrow().iter() {
        let child = convert_tree(tree, child.clone());
        tree.append_child_id(parent, child);
    }

    parent
}

/// Convert a parsed RcDom tree into our own tree representation
impl From<markup5ever_rcdom::Handle> for Tree {
    fn from(value: markup5ever_rcdom::Handle) -> Self {
        let mut tree = Tree::default();

        convert_tree(&mut tree, value);

        tree
    }
}
