use crossterm::event::*;

use core_dtos::prelude::*;
use core_state::prelude::*;

async fn handle_game_canvas_navigation(app_state: &mut AppState, next: SelectedCellNavigation) {
    if !app_state.has_game_started() {
        return;
    }

    let event = SendEvents::Renderer(RenderEvents::OnUpdateSelectedCell(next));
    app_state.send(event).await;
}

async fn handle_right_arrow_key(app_state: &mut AppState) {
    handle_game_canvas_navigation(app_state, SelectedCellNavigation::Right).await;
}

async fn handle_left_arrow_key(app_state: &mut AppState) {
    handle_game_canvas_navigation(app_state, SelectedCellNavigation::Left).await;
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

async fn handle_enter_key(app_state: &mut AppState) {
    if app_state.ecs_current_game_status == GameStatus::GameDidStart {
        return handle_show_command_popup(app_state).await;
    }

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

async fn handle_run_tick(app_state: &mut AppState) {
    if app_state.ecs_current_game_status != GameStatus::GameDidStart {
        return;
    }

    let event = SendEvents::Tick;
    app_state.send(event).await;
}

async fn handle_camera_navigation(app_state: &mut AppState, next: CameraNavigation) {
    if app_state.ecs_current_game_status != GameStatus::GameDidStart {
        return;
    }

    if app_state.is_popup_visible() {
        return;
    }

    let event = SendEvents::Renderer(RenderEvents::OnUpdateCamera(next));
    app_state.send(event).await
}

pub async fn handle_key_pressed(shortcut: String, app_state: &mut AppState) {
    let event = SendEvents::Ui(UiEvents::OnShortcut(shortcut));
    app_state.send(event).await;
}

pub async fn handle_view_event(event: KeyEvent, app_state: &mut AppState) -> bool {
    match event.code {
        KeyCode::Esc => handle_close_view(app_state).await,
        KeyCode::Down | KeyCode::Tab => handle_down_arrow_key(app_state).await,
        KeyCode::Up | KeyCode::BackTab => handle_up_arrow_key(app_state).await,
        KeyCode::Left => handle_left_arrow_key(app_state).await,
        KeyCode::Right => handle_right_arrow_key(app_state).await,
        KeyCode::Enter => handle_enter_key(app_state).await,
        KeyCode::Char('q') => handle_quit_application(app_state).await,
        KeyCode::Char('n') => handle_run_tick(app_state).await,
        KeyCode::Char(' ') => handle_show_command_popup(app_state).await,
        KeyCode::Char('j') => handle_camera_navigation(app_state, CameraNavigation::Left).await,
        KeyCode::Char('l') => handle_camera_navigation(app_state, CameraNavigation::Right).await,
        KeyCode::Char('i') => handle_camera_navigation(app_state, CameraNavigation::Up).await,
        KeyCode::Char('k') => handle_camera_navigation(app_state, CameraNavigation::Down).await,
        KeyCode::Char(s) => handle_key_pressed(s.to_string(), app_state).await,
        _ => return false,
    };

    true
}
