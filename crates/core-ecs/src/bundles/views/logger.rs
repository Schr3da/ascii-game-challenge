use core_dtos::prelude::*;

pub fn logger_popup_view() -> UiView {
    UiView {
        id: UiViewIds::Popup(UiPopupViewIds::Logger),
        layout: UiLayout {
            margin: 0,
            alignment: LayoutAlignments::Vertical,
            constraints: vec![LayoutConstraints::Percentage(100)],
        },
        state: UiViewState {
            selected_id: ViewComponentIds::Popup(PopupIds::Logger(None)),
            selectable_ids: vec![],
            view_data: ViewDataTypes::Logger(LoggerState::default()),
            ..Default::default()
        },
        children: vec![],
    }
}
