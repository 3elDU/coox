use crossterm::event::{Event, KeyCode};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Style,
    text::Text,
    widgets::{List, ListState, StatefulWidget},
};

/// Presents a list of strings with optional wrapping and allows scrolling through it
pub struct Scrollable<T> {
    source: T,
}

impl<T> StatefulWidget for Scrollable<T>
where
    T: Clone + IntoIterator,
    for<'a> T::Item: Into<Text<'a>>,
{
    type State = ScrollableState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let list = List::new(self.source).highlight_style(Style::new().black().on_white());
        StatefulWidget::render(list, area, buf, &mut state.list_state);
    }
}

impl<T> Scrollable<T> {
    pub fn new(source: T) -> Self {
        Self { source }
    }
}

#[derive(Default)]
pub struct ScrollableState {
    list_state: ListState,
}

impl ScrollableState {
    pub fn handle_event(&mut self, event: Event) {
        let state = &mut self.list_state;

        match event {
            Event::Key(key_event) if key_event.is_press() => match key_event.code {
                KeyCode::Down => state.select_next(),
                KeyCode::Up => state.select_previous(),
                KeyCode::PageDown => state.scroll_down_by(10),
                KeyCode::PageUp => state.scroll_up_by(10),
                _ => {}
            },
            _ => {}
        }
    }

    pub fn get_scroll_offset(&self) -> usize {
        return self.list_state.offset();
    }
}
