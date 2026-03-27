use taffy::{
    AlignItems, FlexDirection, LayoutBlockContainer, LayoutFlexboxContainer, LayoutPartialTree,
    RoundTree, Size, Style, TraversePartialTree, TraverseTree, compute_block_layout,
    compute_leaf_layout,
};

use crate::{
    dom::{
        node::NodeData,
        tree::{NodeId, Tree},
    },
    text::{FontMetrics, measure_text},
};

impl From<taffy::NodeId> for NodeId {
    fn from(value: taffy::NodeId) -> Self {
        NodeId(value.into())
    }
}

impl From<NodeId> for taffy::NodeId {
    fn from(value: NodeId) -> Self {
        taffy::NodeId::from(value.0)
    }
}

pub struct ChildIter<'a>(std::slice::Iter<'a, NodeId>);
impl Iterator for ChildIter<'_> {
    type Item = taffy::NodeId;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().copied().map(taffy::NodeId::from)
    }
}

impl TraversePartialTree for Tree {
    type ChildIter<'a> = ChildIter<'a>;

    fn child_ids(&self, parent_node_id: taffy::NodeId) -> Self::ChildIter<'_> {
        ChildIter(self.get_node(parent_node_id).children.iter())
    }

    fn child_count(&self, parent_node_id: taffy::NodeId) -> usize {
        self.get_node(parent_node_id).children.len()
    }

    fn get_child_id(&self, parent_node_id: taffy::NodeId, child_index: usize) -> taffy::NodeId {
        self.get_node(parent_node_id).children[child_index].into()
    }
}

impl TraverseTree for Tree {}

impl LayoutPartialTree for Tree {
    type CoreContainerStyle<'a> = &'a Style;

    type CustomIdent = String;

    fn get_core_container_style(&self, node_id: taffy::NodeId) -> Self::CoreContainerStyle<'_> {
        &self.get_node(node_id).style
    }

    fn set_unrounded_layout(&mut self, node_id: taffy::NodeId, layout: &taffy::Layout) {
        self.get_node_mut(node_id).unrounded_layout = *layout;
    }

    fn compute_child_layout(
        &mut self,
        node_id: taffy::NodeId,
        inputs: taffy::LayoutInput,
    ) -> taffy::LayoutOutput {
        let node = self.get_node_mut(node_id);

        let resolve_calc_val = |_val, _basis| 0.0;
        let measure_zero_size = |_known_dimensions, _available_space| Size::zero();

        match &node.data {
            NodeData::Doctype { .. }
            | NodeData::Comment { .. }
            | NodeData::ProcessingInstruction { .. } => {
                compute_leaf_layout(inputs, &node.style, resolve_calc_val, measure_zero_size)
            }
            NodeData::Text(contents) => compute_leaf_layout(
                inputs,
                &node.style,
                resolve_calc_val,
                |known_dimensions, available_space| {
                    measure_text(
                        contents.as_str(),
                        &FontMetrics {
                            width: 12.0,
                            height: 24.0,
                        },
                        known_dimensions,
                        available_space,
                    )
                },
            ),
            NodeData::Element { name, .. } => match name.as_str() {
                // Hidden elements
                // TODO: put noscript in here once we support javascript
                "area" | "base" | "basefont" | "datalist" | "head" | "link" | "meta"
                | "noembed" | "noframes" | "param" | "rp" | "script" | "style" | "template"
                | "title" => {
                    compute_leaf_layout(inputs, &node.style, resolve_calc_val, measure_zero_size)
                }
                // Inline elements
                "cite" | "dfn" | "em" | "i" | "var" | "b" | "strong" | "code" | "kbd" | "samp"
                | "tt" | "big" | "small" | "sub" | "sup" | "ins" | "u" | "del" | "s" | "strike" => {
                    compute_block_layout(self, node_id, inputs)
                }
                _ => compute_block_layout(self, node_id, inputs),
            },
            NodeData::Document => compute_block_layout(self, node_id, inputs),
        }
    }
}

impl RoundTree for Tree {
    fn get_unrounded_layout(&self, node_id: taffy::NodeId) -> taffy::Layout {
        self.get_node(node_id).unrounded_layout
    }

    fn set_final_layout(&mut self, node_id: taffy::NodeId, layout: &taffy::Layout) {
        self.get_node_mut(node_id).final_layout = *layout;
    }
}

impl LayoutFlexboxContainer for Tree {
    type FlexboxContainerStyle<'a> = Style;
    type FlexboxItemStyle<'a> = Style;

    fn get_flexbox_container_style(
        &self,
        _node_id: taffy::NodeId,
    ) -> Self::FlexboxContainerStyle<'_> {
        Style {
            flex_direction: FlexDirection::Column,
            align_items: Some(AlignItems::Start),
            ..Default::default()
        }
    }

    fn get_flexbox_child_style(&self, child_node_id: taffy::NodeId) -> Self::FlexboxItemStyle<'_> {
        Style::default()
    }
}

impl LayoutBlockContainer for Tree {
    type BlockContainerStyle<'a> = &'a Style;
    type BlockItemStyle<'a> = &'a Style;

    fn get_block_container_style(&self, node_id: taffy::NodeId) -> Self::BlockContainerStyle<'_> {
        &self.get_node(node_id).style
    }

    fn get_block_child_style(&self, child_node_id: taffy::NodeId) -> Self::BlockItemStyle<'_> {
        &self.get_node(child_node_id).style
    }
}
