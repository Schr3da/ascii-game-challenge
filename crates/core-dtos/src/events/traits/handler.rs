use crate::prelude::*;

pub trait EcsEventHandler {
    fn handle_event(&mut self, event: SendEvents);

    fn handle_ui_event(&mut self, event: UiEvents);

    fn handle_game_event(&mut self, event: RenderEvents);

    fn handle_general_event(&mut self, event: GeneralEvents);
}
