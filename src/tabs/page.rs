use ratatui::{
    buffer::Buffer,
    layout::Rect,
    widgets::{Paragraph, Widget, Wrap},
};

use crate::dom::node::{Handle, NodeData};

pub struct PageTab {
    tree: Handle,
}

impl Widget for &PageTab {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let mut rendered_tree = String::new();
        walk_tree(&mut rendered_tree, self.tree.clone());

        Paragraph::new(rendered_tree)
            .wrap(Wrap { trim: true })
            .render(area, buf);
    }
}

impl PageTab {
    pub fn new(tree: Handle) -> Self {
        Self { tree }
    }
}

fn walk_tree(output: &mut String, handle: Handle) {
    if let NodeData::Text(ref contents) = handle.borrow().data {
        output.push_str(contents.as_str());
    }

    for child in &handle.borrow().children {
        walk_tree(output, child.clone());
    }
}
