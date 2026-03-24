use crate::dom::node::{Node, NodeData};

/// A [`Node`] converted for use in printing out the html tree
pub struct FormattedNode {
    pub opening_string: Option<String>,
    pub text_content: Option<String>,
    pub closing_string: Option<String>,
}

impl TryFrom<Node> for FormattedNode {
    // The error here is symbolic, doesn't actually represent a malfunction
    type Error = ();

    fn try_from(node: Node) -> Result<Self, Self::Error> {
        let (opening, text, closing) = match node.get_node_data().as_ref() {
            NodeData::Document => (Some("<!doctype html>".to_owned()), None, None),
            NodeData::Comment(contents) => (
                Some("<!--".to_owned()),
                Some(format!("<!-- {} -->", contents)),
                Some("-->".to_owned()),
            ),
            NodeData::Element { name, attrs, .. } => {
                let attrs = attrs
                    .iter()
                    .map(|attr| format!("{}=\"{}\"", attr.name, attr.value))
                    .collect::<Vec<_>>()
                    .join(" ");

                let opening = if attrs.is_empty() {
                    format!("<{}>", name)
                } else {
                    format!("<{} {}>", name, attrs)
                };

                (Some(opening), None, Some(format!("</{}>", name)))
            }
            NodeData::Text(contents) => (None, Some(contents.replace("\n", "")), None),
            _ => return Err(()),
        };

        Ok(Self {
            opening_string: opening,
            text_content: text,
            closing_string: closing,
        })
    }
}
