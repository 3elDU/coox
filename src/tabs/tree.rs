use std::{cell::RefCell, rc::Rc, usize};

use crossterm::event::Event;
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    widgets::{StatefulWidget, Widget},
};

use crate::{
    dom::{
        node::{Node, NodeData},
        tree::Tree,
    },
    format::tree::FormattedNode,
    widgets::{Scrollable, ScrollableState},
};

/// Shows the HTML tree as a scrollable list
pub struct TreeTab {
    tree: Rc<RefCell<Tree>>,
    state: ScrollableState,
}

impl Widget for &mut TreeTab {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let mut lines = Vec::new();

        let tree = self.tree.borrow();
        self.build_list(&mut lines, tree.get_root_node(), &tree, 0);

        Scrollable::new(lines).render(area, buf, &mut self.state);
    }
}

impl TreeTab {
    pub fn new(tree: Rc<RefCell<Tree>>) -> Self {
        Self {
            tree,
            state: Default::default(),
        }
    }

    pub fn handle_event(&mut self, event: Event) {
        self.state.handle_event(event);
    }

    fn build_list(&self, lines: &mut Vec<String>, node: &Node, tree: &Tree, depth: usize) -> usize {
        let formatted: Result<FormattedNode, _> = node.try_into();

        // If try_into gives an err, it means the node doesn't need to be shown
        if let Err(_) = formatted {
            return 0;
        }

        let formatted = formatted.unwrap();

        let children: Vec<&Node> = node
            .get_children(tree)
            .into_iter()
            .filter(|child| {
                // Filter out empty text nodes
                if let NodeData::Text(contents) = &child.data
                    && contents.trim().is_empty()
                {
                    return false;
                }

                true
            })
            .collect();

        let indent = " ".repeat(depth * 4);

        if let Some(text) = formatted.opening_string {
            lines.push(indent.clone() + &text);
        }

        if let Some(text) = formatted.text_content {
            lines.push(indent.clone() + &text);
        }

        for child_node in &children {
            self.build_list(lines, child_node, tree, depth + 1);
        }

        if let Some(text) = formatted.closing_string {
            lines.push(indent.clone() + &text);
        }

        return children.len();
    }
}
