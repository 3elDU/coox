use markup5ever_rcdom::{Handle as RcDomHandle, NodeData as RcDomNodeData};

use crate::dom::{
    attr::Attribute,
    node::{Node, NodeData},
};

impl TryFrom<RcDomHandle> for Node {
    type Error = ();

    fn try_from(value: markup5ever_rcdom::Handle) -> Result<Self, Self::Error> {
        let node_data = match &value.data {
            RcDomNodeData::Document => Ok(NodeData::Document),
            RcDomNodeData::Comment { contents } => Ok(NodeData::Comment(contents.into())),
            RcDomNodeData::Text { contents } => Ok(NodeData::Text(contents.borrow().to_string())),
            RcDomNodeData::Element { name, attrs, .. } => Ok(NodeData::Element {
                name: name.local.to_string(),
                attrs: attrs
                    .borrow()
                    .iter()
                    .map(|attr| Attribute {
                        name: attr.name.local.to_string(),
                        value: attr.value.to_string(),
                    })
                    .collect(),
            }),
            // Ignore all other node types
            _ => Err(()),
        }?;

        let converted_children = value
            .children
            .borrow()
            .iter()
            .map(|child| child.clone().try_into())
            .filter(|result| result.is_ok())
            .map(|result| result.unwrap())
            .collect();

        Ok(Node::new_with_children(converted_children, node_data))
    }
}
