use crossterm::event::*;

use core_dtos::prelude::*;
use core_state::prelude::*;

async fn handle_right_arrow_key(app_state: &mut AppState) {
    let event = match app_state.ecs_current_game_status {
        GameStatus::GameDidStart => SendEvents::Renderer(RenderEvents::OnUpdateSelectedCell(
            SelectedCellNavigation::Right,
        )),
        GameStatus::GameDidNotStart | GameStatus::GameDidPaused | GameStatus::GameWillEnded => {
            return;
        }
    };

    app_state.send(event).await;
}

async fn handle_left_arrow_key(app_state: &mut AppState) {
    let event = match app_state.ecs_current_game_status {
        GameStatus::GameDidStart => SendEvents::Renderer(RenderEvents::OnUpdateSelectedCell(
            SelectedCellNavigation::Left,
        )),
        GameStatus::GameDidNotStart | GameStatus::GameDidPaused | GameStatus::GameWillEnded => {
            return;
        }
    };

    app_state.send(event).await;
}

async fn handle_up_arrow_key(app_state: &mut AppState) {
    let event = match app_state.ecs_current_game_status {
        GameStatus::GameDidStart => SendEvents::Renderer(RenderEvents::OnUpdateSelectedCell(
            SelectedCellNavigation::Up,
        )),
        GameStatus::GameDidNotStart | GameStatus::GameDidPaused | GameStatus::GameWillEnded => {
            SendEvents::Ui(UiEvents::OnSelect(SelectionDirections::Previous))
        }
    };

    app_state.send(event).await;
}

async fn handle_down_arrow_key(app_state: &mut AppState) {
    let event = match app_state.ecs_current_game_status {
        GameStatus::GameDidStart => SendEvents::Renderer(RenderEvents::OnUpdateSelectedCell(
            SelectedCellNavigation::Down,
        )),
        GameStatus::GameDidNotStart | GameStatus::GameDidPaused | GameStatus::GameWillEnded => {
            SendEvents::Ui(UiEvents::OnSelect(SelectionDirections::Next))
        }
    };

    app_state.send(event).await;
}

async fn handle_close_view(app_state: &mut AppState) {
    let event = SendEvents::Ui(UiEvents::OnCloseView);
    app_state.send(event).await;
}

async fn handle_on_click(app_state: &mut AppState) {
    if let Some(s) = &app_state.ecs_current_view_state {
        let event = SendEvents::Ui(UiEvents::OnClick(s.selected_id.clone()));
        app_state.send(event).await;
    };
}

async fn handle_quit_application(app_state: &mut AppState) {
    let event = SendEvents::Ui(UiEvents::OnClick(ViewComponentIds::Main(MainMenuIds::Quit)));
    app_state.send(event).await;
}

async fn handle_show_command_popup(app_state: &mut AppState) {
    let event = SendEvents::Commands(CommandInputEvents::New);
    app_state.send(event).await;
}

pub async fn handle_view_event(event: KeyEvent, app_state: &mut AppState) -> bool {
    match event.code {
        KeyCode::Esc => handle_close_view(app_state).await,
        KeyCode::Down | KeyCode::Tab => handle_down_arrow_key(app_state).await,
        KeyCode::Up | KeyCode::BackTab => handle_up_arrow_key(app_state).await,
        KeyCode::Left => handle_left_arrow_key(app_state).await,
        KeyCode::Right => handle_right_arrow_key(app_state).await,
        KeyCode::Enter => handle_on_click(app_state).await,
        KeyCode::Char('q') => handle_quit_application(app_state).await,
        KeyCode::Char(' ') => handle_show_command_popup(app_state).await,
        _ => return false,
    };

    true
}
