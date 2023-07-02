use core_dtos::prelude::*;

pub fn command_popup_view() -> UiView {
    UiView {
        id: UiViewIds::Popup(UiPopupViewIds::Command),
        layout: UiLayout {
            margin: 0,
            alignment: LayoutAlignments::Vertical,
            constraints: vec![LayoutConstraints::Percentage(100)],
        },
        state: UiViewState {
            selected_id: ViewComponentIds::CommandPopup(CommandIds::Move),
            selectable_ids: CommandIds::get_selectable_items(),
            view_data: ViewDataTypes::CommandPopup(CommandPopupState::default()),
            ..Default::default()
        },
        children: vec![UiViewChild::List(UiList {
            id: ViewComponentIds::Main(MainMenuIds::MenuList),
            label: "Available Actions".to_string(),
            children: vec![
                UiLabel {
                    id: ViewComponentIds::CommandPopup(CommandIds::Move),
                    alignment: TextAlignment::Left,
                    text: "[m] Move".to_string(),
                },
                UiLabel {
                    id: ViewComponentIds::CommandPopup(CommandIds::Build),
                    alignment: TextAlignment::Left,
                    text: "[b] Build".to_string(),
                },
                UiLabel {
                    id: ViewComponentIds::CommandPopup(CommandIds::Inspect),
                    alignment: TextAlignment::Left,
                    text: "[i] Inspect".to_string(),
                },
            ],
        })],
    }
}
