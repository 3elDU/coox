use crossterm::event::Event;
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    widgets::{StatefulWidget, Widget},
};

use crate::{
    dom::node::{Handle, NodeData},
    widgets::{Scrollable, ScrollableState},
};

pub struct PageTab {
    tree: Handle,
    state: ScrollableState,
}

impl Widget for &mut PageTab {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let mut rendered_tree = Vec::new();
        walk_tree(&mut rendered_tree, self.tree.clone());

        Scrollable::new(rendered_tree).render(area, buf, &mut self.state);
    }
}

impl PageTab {
    pub fn new(tree: Handle) -> Self {
        Self {
            tree,
            state: Default::default(),
        }
    }

    pub fn handle_event(&mut self, event: Event) {
        self.state.handle_event(event);
    }
}

fn walk_tree(output: &mut Vec<String>, handle: Handle) {
    if let NodeData::Text(ref contents) = handle.borrow().data {
        // Collapse whitespace: if the current line is empty, and the last line
        // in the list is empty too, do not add the current line
        if contents.trim().is_empty() {
            match output.last() {
                Some(line) if !line.trim().is_empty() => output.push(contents.trim().to_string()),
                None => output.push(contents.trim().to_string()),
                _ => {}
            }
        } else {
            output.push(contents.trim().to_string());
        }
    }

    for child in &handle.borrow().children {
        walk_tree(output, child.clone());
    }
}
