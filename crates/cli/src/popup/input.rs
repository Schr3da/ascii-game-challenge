use std::io::Stdout;

use core_dtos::prelude::UiPopupView;
use tui::backend::CrosstermBackend;
use tui::terminal::Terminal;

pub fn draw_popup_to_terminal_handler(
    _terminal: &mut Terminal<CrosstermBackend<Stdout>>,
    popup: &Option<UiPopupView>,
) {
  let _= match popup {
    Some(p) => p,
    None => return,
  };

  println!("Draw popup");
}
