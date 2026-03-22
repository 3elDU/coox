use std::usize;

use crossterm::event::Event;
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    widgets::{StatefulWidget, Widget},
};

use crate::{
    dom::node::{Handle, NodeData},
    format::tree::FormattedNode,
    widgets::{Scrollable, ScrollableState},
};

/// Shows the HTML tree as a scrollable list
pub struct TreeTab {
    tree: Handle,
    state: ScrollableState,
}

impl Widget for &mut TreeTab {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let mut lines = Vec::new();
        self.build_list(&mut lines, self.tree.clone(), 0);

        Scrollable::new(lines).render(area, buf, &mut self.state);
    }
}

impl TreeTab {
    pub fn new(tree: Handle) -> Self {
        Self {
            tree,
            state: Default::default(),
        }
    }

    pub fn handle_event(&mut self, event: Event) {
        self.state.handle_event(event);
    }

    fn build_list(&self, lines: &mut Vec<String>, node: Handle, depth: usize) -> usize {
        let formatted: Result<FormattedNode, _> = node.clone().try_into();

        // If try_into gives an err, it means the node doesn't need to be shown
        if let Err(_) = formatted {
            return 0;
        }

        let formatted = formatted.unwrap();

        let children: Vec<Handle> = node
            .borrow()
            .children
            .iter()
            .filter(|child| {
                // Filter out empty text nodes
                if let NodeData::Text(contents) = &child.borrow().data
                    && contents.trim().is_empty()
                {
                    return false;
                }

                true
            })
            .map(|node| node.clone())
            .collect();

        let indent = " ".repeat(depth * 4);

        if let Some(text) = formatted.opening_string {
            lines.push(indent.clone() + &text);
        }

        if let Some(text) = formatted.text_content {
            lines.push(indent.clone() + &text);
        }

        for child_node in children.iter() {
            self.build_list(lines, child_node.clone(), depth + 1);
        }

        if let Some(text) = formatted.closing_string {
            lines.push(indent.clone() + &text);
        }

        return children.len();
    }
}
