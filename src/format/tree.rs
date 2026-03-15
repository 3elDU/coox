use markup5ever_rcdom::{Handle, NodeData};

/// A [`Node`] converted for use in printing out the html tree
pub struct FormattedNode {
    pub opening_string: Option<String>,
    pub text_content: Option<String>,
    pub closing_string: Option<String>,
}

impl TryFrom<Handle> for FormattedNode {
    // The error here is symbolic, doesn't actually represent a malfunction
    type Error = ();

    fn try_from(node: Handle) -> Result<Self, Self::Error> {
        let (opening, text, closing) = match &node.data {
            NodeData::Document => (Some("<!doctype html>".to_owned()), None, None),
            NodeData::Comment { contents } => (
                Some("<!--".to_owned()),
                Some(format!("<!-- {} -->", contents)),
                Some("-->".to_owned()),
            ),
            NodeData::Element { name, attrs, .. } => {
                let attrs = attrs
                    .borrow()
                    .iter()
                    .map(|attr| format!("{}=\"{}\"", attr.name.local, attr.value))
                    .collect::<Vec<_>>()
                    .join(" ");

                let opening = if attrs.is_empty() {
                    format!("<{}>", name.local)
                } else {
                    format!("<{} {}>", name.local, attrs)
                };

                (Some(opening), None, Some(format!("</{}>", name.local)))
            }
            NodeData::Text { contents } => (None, Some(contents.borrow().replace("\n", "")), None),
            NodeData::ProcessingInstruction { target, contents } => {
                (Some(format!("<?{} {} ?>", target, contents)), None, None)
            }
            _ => return Err(()),
        };

        Ok(Self {
            opening_string: opening,
            text_content: text,
            closing_string: closing,
        })
    }
}
