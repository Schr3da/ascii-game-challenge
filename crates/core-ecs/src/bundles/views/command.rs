use core_dtos::prelude::*;

pub fn command_popup_view() -> UiPopupView {
    UiPopupView {
        id: UiPopupViewIds::Command,
        layout: UiLayout {
            margin: 1,
            alignment: LayoutAlignments::Horizontal,
            constraints: vec![LayoutConstraints::Percentage(100)],
        },
        state: UiPopupState {
            ..UiPopupState::default()
        },
        children: vec![UiViewChild::Label(UiLabel {
            id: ViewComponentIds::Popup(CommandPopupIds::UnknownCommandPopupId),
            alignment: TextAlignment::Center,
            text: "Command Popup".to_string(),
        })],
    }
}
