use tui::backend::Backend;
use tui::layout::{Constraint, Direction, Layout};
use tui::style::{Color, Style};
use tui::widgets::{Block, Clear};
use tui::Frame;

use core_dtos::prelude::*;

use super::render_utils::*;

pub struct PopupUtils;

impl PopupUtils {
    fn create_frame(size: &tui::layout::Rect, data: &ViewDataTypes) -> tui::layout::Rect {
        let mut width = size.width;
        if width > 30 {
            width = 30;
        }

        let pos_x = match data {
            ViewDataTypes::Popup(s)
                if size.width as i32 - s.current_selected_game_tile.frame.x - width as i32 <= 0 =>
            {
                0
            }
            _ => size.width - width,
        };

        tui::layout::Rect::new(pos_x, size.y, width, size.height)
    }

    pub fn render<B: Backend>(context: &mut Frame<B>, view: &UiView) {
        let screen_size = context.size();

        let size = Self::create_frame(&screen_size, &view.state.view_data);

        context.render_widget(Clear, size);

        let block = Block::default().style(Style::default().bg(Color::White).fg(Color::Black));
        context.render_widget(block, size);

        let root_layout = Layout::default()
            .margin(0)
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(100)].as_ref())
            .split(size);

        RenderUtils::render_popup_view(context, root_layout[0], &view);
    }
}
