use tui::backend::Backend;
use tui::layout::{Constraint, Direction, Layout};
use tui::style::{Color, Style};
use tui::widgets::Block;
use tui::Frame;

use core_dtos::prelude::*;

use super::colors_utils::*;
use super::popup_utils::*;
use super::render_utils::*;

pub struct DrawUtils;

impl DrawUtils {
    fn menu_view<B: Backend>(context: &mut Frame<B>, view: &UiView) {
        let size = context.size();

        let block = Block::default().style(Style::default().bg(Color::White).fg(Color::Black));
        context.render_widget(block, size);

        let root_layout = Layout::default()
            .margin(0)
            .direction(Direction::Horizontal)
            .constraints(
                [
                    Constraint::Percentage(20),
                    Constraint::Percentage(60),
                    Constraint::Percentage(20),
                ]
                .as_ref(),
            )
            .split(size);

        RenderUtils::render_view(context, root_layout[1], view);
    }

    fn options_view<B: Backend>(context: &mut Frame<B>, view: &UiView) {
        let size = context.size();

        let block = Block::default().style(Style::default().bg(Color::White).fg(Color::White));
        context.render_widget(block, size);

        let root_layout = Layout::default()
            .margin(0)
            .direction(Direction::Horizontal)
            .constraints(
                [
                    Constraint::Percentage(20),
                    Constraint::Percentage(60),
                    Constraint::Percentage(20),
                ]
                .as_ref(),
            )
            .split(size);

        RenderUtils::render_view(context, root_layout[1], view);
    }

    fn game_view<B: Backend>(context: &mut Frame<B>, view: &UiView) {
        let size = context.size();

        let block = Block::default().style(Style::default().bg(Color::White).fg(Color::Black));

        context.render_widget(block, size);

        let root_layout = Layout::default()
            .horizontal_margin(2)
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(100)].as_ref())
            .split(size);

        RenderUtils::render_view(context, root_layout[0], view);
    }

    pub fn draw_cursor<B: Backend>(context: &mut Frame<B>, cell: &Option<SelectedCell>) {
        let next = match cell {
            Some(d) => d,
            None => return,
        };

        let cell = &next.cell;
        let frame = &next.frame;

        let size = context.size();

        let tile_frame = tui::layout::Rect {
            x: frame.x as u16,
            y: frame.y as u16 + next.top,
            width: frame.width as u16,
            height: frame.height as u16,
        };

        if !size.intersects(tile_frame) {
            return;
        }

        let background = ColorUtils::to_terminal_color(&cell.background);
        let foreground = ColorUtils::to_terminal_color(&cell.foreground);

        let block = Block::default()
            .title(cell.symbol.to_string())
            .style(Style::default().bg(background).fg(foreground));

        context.render_widget(block, tile_frame);
    }

    pub fn draw_view<B: Backend>(context: &mut Frame<B>, view: &Option<UiView>) {
        let next = match view {
            Some(v) => v,
            None => return,
        };

        match &next.id {
            UiViewIds::Main => _ = Self::menu_view(context, next),
            UiViewIds::Options => _ = Self::options_view(context, next),
            UiViewIds::Game => _ = Self::game_view(context, next),
            UiViewIds::Popup(p) => match p {
                UiPopupViewIds::Buildings | UiPopupViewIds::Actions | UiPopupViewIds::Logs => {
                    _ = PopupUtils::render(context, next)
                }
            },
            UiViewIds::Quit => return,
        };
    }
}
