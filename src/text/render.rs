use ratatui::{buffer::Buffer, layout::Rect, style::Style};
use taffy::Layout;
use unicode_segmentation::UnicodeSegmentation;

use crate::config::FONT_METRICS;

pub fn render_text(text: &str, layout: Layout, area: Rect, buf: &mut Buffer) {
    let start_x = area.x + (layout.location.x / FONT_METRICS.width) as u16;
    let start_y = area.y + (layout.location.y / FONT_METRICS.height) as u16;
    let mut x = start_x;
    let mut y = start_y;

    let max_x = x + (layout.size.width / FONT_METRICS.width) as u16;
    let max_y = y + (layout.size.height / FONT_METRICS.height) as u16;

    for (i, char) in text.grapheme_indices(true) {
        if x >= buf.area.width || y >= buf.area.height {
            return;
        }

        buf.set_string(x, y, char, Style::default());

        x += 1;
        if x >= max_x {
            x = start_x;
            y += 1;
        }
    }
}
