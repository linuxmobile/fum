use ratatui::{buffer::Buffer, layout::Rect, style::Stylize, widgets::{Block, Widget}};

use crate::{get_color, state::FumState};

use super::FumWidget;

pub fn render(widget: &FumWidget, area: Rect, buf: &mut Buffer, state: &mut FumState) {
    if let FumWidget::Empty { bg, fg, .. } = widget {
        let (bg, fg) = get_color!(bg, fg, &state.parent_bg, &state.parent_fg);

        Block::new()
            .bg(*bg)
            .fg(*fg)
            .render(area, buf);
    }
}
