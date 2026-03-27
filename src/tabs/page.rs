use std::{cell::RefCell, rc::Rc};

use crossterm::event::Event;
use ratatui::{
    buffer::Buffer,
    layout::{Offset, Rect},
    widgets::{StatefulWidget, Widget},
};
use taffy::{AvailableSpace, Size};

use crate::{
    config::FONT_METRICS,
    dom::{
        node::{Node, NodeData},
        tree::Tree,
    },
    text::render::render_text,
};

pub struct PageTab {
    tree: Rc<RefCell<Tree>>,
}

impl Widget for &mut PageTab {
    fn render(self, area: Rect, buf: &mut Buffer) {
        self.tree.borrow_mut().layout(Size {
            width: AvailableSpace::Definite(area.width as f32 * FONT_METRICS.width),
            height: AvailableSpace::Definite(area.height as f32 * FONT_METRICS.height),
        });

        let tree = self.tree.borrow();
        walk_tree(tree.get_root_node(), &tree, area, buf);
    }
}

impl PageTab {
    pub fn new(tree: Rc<RefCell<Tree>>) -> Self {
        Self { tree }
    }

    #[allow(unused)]
    pub fn handle_event(&mut self, event: Event) {}
}

fn walk_tree(node: &Node, tree: &Tree, area: Rect, buf: &mut Buffer) {
    if let NodeData::Element { name, .. } = &node.data
        && matches!(
            name.as_str(),
            "area"
                | "base"
                | "basefont"
                | "datalist"
                | "head"
                | "link"
                | "meta"
                | "noembed"
                | "noframes"
                | "param"
                | "rp"
                | "script"
                | "style"
                | "template"
                | "title"
        )
    {
        // Skip rendering hidden elements
        return;
    }

    if let NodeData::Text(text) = &node.data {
        render_text(text, node.final_layout, area, buf);
    }

    for child in node.get_children(tree) {
        walk_tree(child, tree, area, buf);
    }
}
