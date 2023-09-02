use core_dtos::prelude::*;
use core_state::prelude::*;

pub struct KeyboardInputs;

impl KeyboardInputs {
    async fn handle_game_canvas_navigation(app_state: &mut AppState, next: Navigation) {
        if !app_state.has_game_started() {
            return;
        }

        let event = SendEvents::Renderer(RenderEvents::OnUpdateSelectedCell(next));
        app_state.send(event).await;
    }

    async fn handle_right_arrow_key(app_state: &mut AppState) {
        Self::handle_game_canvas_navigation(app_state, Navigation::Right).await;
    }

    async fn handle_left_arrow_key(app_state: &mut AppState) {
        Self::handle_game_canvas_navigation(app_state, Navigation::Left).await;
    }

    async fn handle_up_arrow_key(app_state: &mut AppState) {
        let event = match app_state.ecs_current_game_status {
            GameStatus::GameDidStart => {
                SendEvents::Renderer(RenderEvents::OnUpdateSelectedCell(Navigation::Up))
            }
            GameStatus::GameDidNotStart | GameStatus::GameDidPaused | GameStatus::GameWillEnded => {
                SendEvents::Ui(UiEvents::OnSelect(SelectionDirections::Previous))
            }
        };

        app_state.send(event).await;
    }

    async fn handle_down_arrow_key(app_state: &mut AppState) {
        let event = match app_state.ecs_current_game_status {
            GameStatus::GameDidStart => {
                SendEvents::Renderer(RenderEvents::OnUpdateSelectedCell(Navigation::Down))
            }
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
            return Self::handle_show_popup_menu(app_state).await;
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

    async fn handle_show_popup_menu(app_state: &mut AppState) {
        if !app_state.has_game_started() {
            return;
        }

        let event = SendEvents::Ui(UiEvents::OnOpenPopup(UiPopupViewIds::Actions));
        app_state.send(event).await;
    }

    async fn handle_run_tick(app_state: &mut AppState) {
        if app_state.ecs_current_game_status != GameStatus::GameDidStart {
            return Self::handle_key_pressed('n'.to_string(), app_state).await;
        }

        let event = SendEvents::Tick;
        app_state.send(event).await;
    }

    async fn handle_camera_navigation(app_state: &mut AppState, next: Navigation) {
        if app_state.ecs_current_game_status != GameStatus::GameDidStart {
            return;
        }

        if app_state.is_popup_visible() {
            return;
        }

        let event = SendEvents::Renderer(RenderEvents::OnUpdateCamera(next));
        app_state.send(event).await
    }

    async fn handle_key_pressed(shortcut: String, app_state: &mut AppState) {
        let event = SendEvents::Ui(UiEvents::OnShortcut(shortcut));
        app_state.send(event).await;
    }

    async fn handle_popup_event(key: Keys, app_state: &mut AppState) -> bool {
        match key {
            Keys::Esc => {
                let event = SendEvents::Ui(UiEvents::OnClosePopup);
                app_state.send(event).await;
            }
            Keys::Enter => {
                let event = SendEvents::Ui(UiEvents::OnOpenPopup(UiPopupViewIds::Actions));
                app_state.send(event).await;
            }
            Keys::DownArrow | Keys::Tab => {
                let event = SendEvents::Ui(UiEvents::OnSelect(SelectionDirections::Next));
                app_state.send(event).await;
            }
            Keys::UpArrow | Keys::BackTab => {
                let event = SendEvents::Ui(UiEvents::OnSelect(SelectionDirections::Previous));
                app_state.send(event).await;
            }
            Keys::Char(s) => {
                let event = SendEvents::Ui(UiEvents::OnShortcut(s.to_string()));
                app_state.send(event).await;
            }
            _ => return false,
        };

        true
    }

    async fn handle_view_event(key: Keys, app_state: &mut AppState) -> bool {
        match key {
            Keys::Esc => Self::handle_close_view(app_state).await,
            Keys::DownArrow | Keys::Tab => Self::handle_down_arrow_key(app_state).await,
            Keys::UpArrow | Keys::BackTab => Self::handle_up_arrow_key(app_state).await,
            Keys::LeftArrow => Self::handle_left_arrow_key(app_state).await,
            Keys::RightArrow => Self::handle_right_arrow_key(app_state).await,
            Keys::Enter => Self::handle_enter_key(app_state).await,
            Keys::Char('q') => Self::handle_quit_application(app_state).await,
            Keys::Char('n') => Self::handle_run_tick(app_state).await,
            Keys::Char(' ') => Self::handle_show_popup_menu(app_state).await,
            Keys::Char('j') => Self::handle_camera_navigation(app_state, Navigation::Left).await,
            Keys::Char('l') => Self::handle_camera_navigation(app_state, Navigation::Right).await,
            Keys::Char('i') => Self::handle_camera_navigation(app_state, Navigation::Up).await,
            Keys::Char('k') => Self::handle_camera_navigation(app_state, Navigation::Down).await,
            Keys::Char(s) => Self::handle_key_pressed(s.to_string(), app_state).await,
            _ => return false,
        };

        true
    }

    pub async fn handle_event(key: Keys, app_state: &mut AppState) -> bool {
        match &app_state.ecs_current_popup_state.clone() {
            Some(_) => Self::handle_popup_event(key, app_state).await,
            None => Self::handle_view_event(key, app_state).await,
        }
    }
}
