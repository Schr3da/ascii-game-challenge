use core_dtos::prelude::*;

pub fn quick_action_popup_view() -> UiView {
    UiView {
        id: UiViewIds::Popup(UiPopupViewIds::QuickAction),
        layout: UiLayout {
            margin: 0,
            alignment: LayoutAlignments::Vertical,
            constraints: vec![LayoutConstraints::Percentage(100)],
        },
        state: UiViewState {
            selected_id: ViewComponentIds::QuickAction,
            selectable_ids: Vec::new(),
            ..UiViewState::default()
        },
        children: vec![UiViewChild::Label(UiLabel {
            id: ViewComponentIds::CommandPopup(CommandIds::Move),
            alignment: TextAlignment::Left,
            text: ": ".to_string(),
        })],
    }
}
