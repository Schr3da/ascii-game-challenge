use core_dtos::prelude::*;

pub fn command_popup_view() -> UiView {
    UiView {
        id: UiViewIds::Popup(UiPopupViewIds::Actions),
        layout: UiLayout {
            margin: 0,
            alignment: LayoutAlignments::Vertical,
            constraints: vec![LayoutConstraints::Percentage(100)],
        },
        state: UiViewState {
            selected_id: ViewComponentIds::Popup(PopupIds::Build(None)),
            selectable_ids: PopupIds::get_selectable_items(),
            view_data: ViewDataTypes::Popup(PopupState::default()),
            ..Default::default()
        },
        children: PopupIds::get_ui_items(),
    }
}
