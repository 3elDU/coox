use std::usize;

use crossterm::event::{Event, KeyCode};
use markup5ever_rcdom::{Handle, NodeData};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Style,
    widgets::{List, ListState, StatefulWidget, Widget},
};

use crate::format::tree::FormattedNode;

/// Shows the HTML tree as a scrollable list
pub struct TreeTab {
    tree: Handle,
    state: ListState,
}

impl Widget for &mut TreeTab {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let mut lines = Vec::new();
        self.build_list(&mut lines, self.tree.clone(), 0);

        let list: List<'_> = List::new(lines).highlight_style(Style::new().black().on_white());
        StatefulWidget::render(list, area, buf, &mut self.state);
    }
}

impl TreeTab {
    pub fn new(tree: Handle) -> Self {
        Self {
            tree,
            state: ListState::default().with_selected(Some(0)),
        }
    }

    pub fn handle_event(&mut self, event: Event) {
        match event {
            Event::Key(key_event) if key_event.is_press() => match key_event.code {
                KeyCode::Down => self.state.select_next(),
                KeyCode::Up => self.state.select_previous(),
                KeyCode::PageDown => self.state.scroll_down_by(10),
                KeyCode::PageUp => self.state.scroll_up_by(10),
                _ => {}
            },
            _ => {}
        }
    }

    fn build_list(&self, lines: &mut Vec<String>, node: Handle, depth: usize) -> usize {
        let formatted: Result<FormattedNode, _> = node.clone().try_into();

        // If try_into gives an err, it means the node doesn't need to be shown
        if let Err(_) = formatted {
            return 0;
        }

        let formatted = formatted.unwrap();

        let children: Vec<Handle> = node
            .children
            .borrow()
            .iter()
            .filter(|child| {
                // Filter out empty text nodes
                if let NodeData::Text { contents } = &child.data
                    && contents.borrow().trim().is_empty()
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
