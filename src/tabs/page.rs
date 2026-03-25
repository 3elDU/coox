use std::{cell::RefCell, rc::Rc};

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
    widgets::{Scrollable, ScrollableState},
};

pub struct PageTab {
    tree: Rc<RefCell<Tree>>,
    state: ScrollableState,
}

impl Widget for &mut PageTab {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let mut rendered_tree = Vec::new();
        walk_tree(
            &mut rendered_tree,
            self.tree.borrow().get_root_node(),
            &self.tree.borrow(),
        );

        Scrollable::new(rendered_tree).render(area, buf, &mut self.state);
    }
}

impl PageTab {
    pub fn new(tree: Rc<RefCell<Tree>>) -> Self {
        Self {
            tree,
            state: Default::default(),
        }
    }

    pub fn handle_event(&mut self, event: Event) {
        self.state.handle_event(event);
    }
}

fn walk_tree(output: &mut Vec<String>, node: &Node, tree: &Tree) {
    if let NodeData::Text(contents) = &node.data {
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

    for child in node.get_children(tree) {
        walk_tree(output, child, tree);
    }
}
