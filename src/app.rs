use std::io;

use crossterm::event::{self, Event, KeyCode};
use ratatui::{
    DefaultTerminal,
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    style::Stylize,
    widgets::{Paragraph, Widget},
};

use crate::{
    context::BrowserContext,
    tabs::{ActiveTab, page::PageTab, tree::TreeTab},
};

pub struct App {
    #[allow(dead_code)]
    context: BrowserContext,
    exit: bool,

    active_tab: ActiveTab,
    page_tab: PageTab,
    tree_tab: TreeTab,
}

impl Widget for &mut App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let layout = Layout::vertical([Constraint::Length(1), Constraint::Fill(1)]).split(area);

        // Title
        Paragraph::new(self.active_tab.get_title())
            .centered()
            .black()
            .on_white()
            .render(layout[0], buf);

        // Body
        match self.active_tab {
            ActiveTab::Page => self.page_tab.render(layout[1], buf),
            ActiveTab::Tree => self.tree_tab.render(layout[1], buf),
            _ => {}
        };
    }
}

impl App {
    pub fn new(context: BrowserContext) -> Self {
        Self {
            page_tab: PageTab::new(context.dom.clone()),
            tree_tab: TreeTab::new(context.dom.clone()),
            context,
            exit: false,
            active_tab: ActiveTab::Tree,
        }
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.render(frame.area(), frame.buffer_mut()))?;
            self.handle_events()?;
        }

        Ok(())
    }

    fn handle_events(&mut self) -> io::Result<()> {
        let event = event::read()?;

        match event {
            Event::Key(key_event) if key_event.is_press() => match key_event.code {
                KeyCode::Char('q') => self.exit = true,
                KeyCode::Char('1') => self.active_tab = ActiveTab::Page,
                KeyCode::Char('2') => self.active_tab = ActiveTab::Tree,
                KeyCode::Char('3') => self.active_tab = ActiveTab::Console,
                _ => {}
            },
            _ => {}
        };

        // let the active tab handle events
        match self.active_tab {
            ActiveTab::Page => self.page_tab.handle_event(event),
            ActiveTab::Tree => self.tree_tab.handle_event(event),
            _ => {}
        }

        Ok(())
    }
}
