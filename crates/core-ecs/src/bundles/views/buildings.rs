use core_dtos::prelude::*;

pub fn buildings_view() -> UiView {
    UiView {
        id: UiViewIds::Popup(UiPopupViewIds::Buildings),
        layout: UiLayout {
            margin: 0,
            alignment: LayoutAlignments::Vertical,
            constraints: vec![LayoutConstraints::Percentage(100)],
        },
        state: UiViewState {
            selected_id: ViewComponentIds::CommandPopup(CommandIds::Build(Some(BuildingIds::Lumbarjack))),
            selectable_ids: BuildingIds::get_selectable_items(),
            view_data: ViewDataTypes::Popup(PopupState::default()),
            ..Default::default()
        },
        children: BuildingIds::get_ui_items(),
    }
}
