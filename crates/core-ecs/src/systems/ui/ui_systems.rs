use bevy_ecs::prelude::*;
use core_dtos::prelude::*;

use crate::prelude::*;

pub struct UiSystems;

impl UiSystems {
    pub fn next_popup(
        id: UiPopupViewIds,
        mut store: ResMut<UiStoreResource>,
        logger: ResMut<LoggerResource>,
        mut views: Query<&mut UiView>,
    ) {
        let selected_tile = match &store.selected_game_tile {
            Some(t) => t,
            None => return,
        };

        let id_to_compare = UiViewIds::Popup(id.clone());
        let mut view = match views.iter_mut().find(|v| v.id == id_to_compare) {
            Some(v) => v,
            None => return,
        };

        view.state.view_data = match id_to_compare {
            UiViewIds::Popup(UiPopupViewIds::Logs) => logger.as_ref().into(),
            _ => selected_tile.into(),
        };

        store.current_popup = Some(id);
    }

    pub fn next_view(
        id: ViewComponentIds,
        store: &mut ResMut<UiStoreResource>,
        subscriber: &Res<SubscriberResource>,
    ) {
        let current_view = store.current_view.clone();

        let next_view = match id.to_view_route() {
            Some(n) => n,
            None => return,
        };

        match id {
            ViewComponentIds::Main(MainMenuIds::NewGame) => {
                store.previous_view.push(current_view);
                store.current_view = next_view;
                store.current_game_status = GameStatus::GameDidStart;
                store.selected_game_tile = Some(SelectedCell::default());
            }
            ViewComponentIds::Main(MainMenuIds::Options) => {
                store.previous_view.push(current_view);
                store.current_view = next_view;
                store.current_game_status = GameStatus::GameDidNotStart;
            }
            ViewComponentIds::Options(OptionMenuIds::Back) => {
                store.previous_view.push(current_view);
                store.current_view = next_view;
                store.current_game_status = GameStatus::GameDidNotStart;
            }
            ViewComponentIds::Main(MainMenuIds::Quit) => {
                store.current_game_status = GameStatus::GameWillEnded;
                _ = subscriber
                    .sender
                    .blocking_send(EcsEvents::Send(SendEvents::General(
                        GeneralEvents::OnApplicationWillClose,
                    )));
            }
            _ => {
                println!("Click on item not supported {:?}", next_view);
            }
        }
    }

    pub fn on_click(mut store: ResMut<UiStoreResource>, subscriber: Res<SubscriberResource>) {
        let next = match &subscriber.next_event {
            Some(SendEvents::Ui(UiEvents::OnClick(id))) => id,
            _ => return,
        };

        UiSystems::next_view(next.clone(), &mut store, &subscriber);
    }

    pub fn on_close_popup(mut store: ResMut<UiStoreResource>, subscriber: Res<SubscriberResource>) {
        match &subscriber.next_event {
            Some(SendEvents::Ui(UiEvents::OnClosePopup)) => {}
            _ => return,
        };

        store.current_popup = None;
    }

    pub fn on_close_view(mut store: ResMut<UiStoreResource>, subscriber: Res<SubscriberResource>) {
        match &subscriber.next_event {
            Some(SendEvents::Ui(UiEvents::OnCloseView)) => {}
            _ => return,
        };

        store.selected_game_tile = None;
        store.current_game_status = GameStatus::GameDidPaused;

        match store.previous_view.pop() {
            Some(v) => store.current_view = v,
            None => store.current_view = UiViewIds::Main,
        }
    }

    pub fn on_open_popup(mut store: ResMut<UiStoreResource>, subscriber: Res<SubscriberResource>) {
        let next = match &subscriber.next_event {
            Some(SendEvents::Ui(UiEvents::OnOpenPopup(e))) => e,
            _ => return,
        };

        store.current_popup = Some(next.clone());
    }

    pub fn on_select_by_id(
        store: Res<UiStoreResource>,
        subscriber: Res<SubscriberResource>,
        mut views_query: Query<&mut UiView>,
    ) {
        let id = match &subscriber.next_event {
            Some(SendEvents::Ui(UiEvents::OnSelectById(id))) => id,
            _ => return,
        };

        let mut view = match views_query.iter_mut().find(|v| v.id == store.current_view) {
            Some(v) => v,
            None => return,
        };

        view.state.selected_id = id.clone();
    }

    fn select_previous(mut view: Mut<UiView>) {
        let ids = &view.state.selectable_ids;
        let id = &view.state.selected_id;
        let current = ids.iter().position(|i| i == id).unwrap_or(0);

        if current == 0 {
            return view.state.selected_id = ids.last().cloned().unwrap_or_default();
        }

        match ids.get(current - 1) {
            Some(n) => view.state.selected_id = n.clone(),
            None => view.state.selected_id = ids.last().cloned().unwrap_or_default(),
        };
    }

    fn select_next(mut view: Mut<UiView>) {
        let ids = &view.state.selectable_ids;
        let id = &view.state.selected_id;
        let current = ids.iter().position(|i| i == id).unwrap_or(0);

        match ids.get(current + 1) {
            Some(n) => view.state.selected_id = n.clone(),
            None => view.state.selected_id = ids.first().cloned().unwrap_or_default(),
        };
    }

    pub fn on_select(
        store: Res<UiStoreResource>,
        subscriber: Res<SubscriberResource>,
        mut views_query: Query<&mut UiView>,
    ) {
        let direction = match &subscriber.next_event {
            Some(SendEvents::Ui(UiEvents::OnSelect(d))) => d,
            _ => return,
        };

        let next = match &store.current_popup {
            Some(id) => views_query
                .iter_mut()
                .find(|v| v.id == UiViewIds::Popup(id.clone())),
            None => views_query.iter_mut().find(|v| v.id == store.current_view),
        };

        let view = match next {
            Some(v) => v,
            None => return,
        };

        match direction {
            SelectionDirections::Next => Self::select_next(view),
            SelectionDirections::Previous => Self::select_previous(view),
        };
    }

    pub fn on_ui_did_update(subscription: Res<SubscriberResource>) {
        _ = subscription
            .sender
            .blocking_send(EcsEvents::Send(SendEvents::Renderer(
                RenderEvents::OnWorldWillUpdate,
            )));
    }

    fn find_matching_label(shortcut: &String, component: &UiLabel) -> Option<UiLabel> {
        let next = match &component.shortcut {
            Some(c) => c == shortcut,
            None => return None,
        };

        match next {
            true => Some(component.clone()),
            false => None,
        }
    }

    fn find_matching_label_in<'a>(shortcut: &String, labels: &Vec<UiLabel>) -> Option<UiLabel> {
        let next = labels
            .iter()
            .find(|l| Self::find_matching_label(shortcut, l).is_some());
        next.cloned()
    }

    fn find_shortcut_recursive(shortcut: &String, children: &Vec<UiViewChild>) -> Option<UiLabel> {
        let inital_value: Option<UiLabel> = None;

        children.iter().fold(inital_value, |result, c| {
            if result.is_some() {
                return result;
            }

            match &c {
                UiViewChild::List(l) => Self::find_matching_label_in(shortcut, &l.children),
                UiViewChild::Label(l) => Self::find_matching_label(shortcut, l),
                UiViewChild::Section(s) => Self::find_shortcut_recursive(shortcut, &s.children),
                _ => result,
            }
        })
    }

    fn find_shortcut_for_view(
        shortcut: &String,
        id: &UiViewIds,
        views: &Query<&mut UiView>,
    ) -> Option<UiLabel> {
        let view = match views.iter().find(|v| &v.id == id) {
            Some(v) => v,
            None => return None,
        };

        Self::find_shortcut_recursive(shortcut, &view.children)
    }

    fn find_shortcut_for_popup(
        shortcut: &String,
        id: &UiPopupViewIds,
        views: &Query<&mut UiView>,
    ) -> Option<UiLabel> {
        let id_to_compare = UiViewIds::Popup(id.clone());
        let view = match views.iter().find(|v| &v.id == &id_to_compare) {
            Some(v) => v,
            None => return None,
        };

        Self::find_shortcut_recursive(shortcut, &view.children)
    }

    pub fn on_shortcut(
        mut store: ResMut<UiStoreResource>,
        mut logger: ResMut<LoggerResource>,
        subscriber: Res<SubscriberResource>,
        views: Query<&mut UiView>,
    ) {
        let shortcut = match &subscriber.next_event {
            Some(SendEvents::Ui(UiEvents::OnShortcut(s))) => s,
            _ => return,
        };

        logger.log("next".to_string());

        let next = match &store.current_popup {
            Some(id) => Self::find_shortcut_for_popup(shortcut, id, &views),
            None => Self::find_shortcut_for_view(shortcut, &store.current_view, &views),
        };

        let label = match next {
            Some(n) => n,
            None => return,
        };

        if store.current_popup.is_some() {
            return match label.id.to_popup_route() {
                Some(p) => Self::next_popup(p, store, logger, views),
                None => return,
            };
        }

        Self::next_view(label.id, &mut store, &subscriber);
    }
}
