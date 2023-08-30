use core_dtos::prelude::*;

pub struct PopupViews;

impl PopupViews {
    pub fn buildings() -> UiView {
        UiView {
            id: UiViewIds::Popup(UiPopupViewIds::Buildings),
            layout: UiLayout {
                margin: 0,
                alignment: LayoutAlignments::Vertical,
                constraints: vec![LayoutConstraints::Percentage(100)],
            },
            state: UiViewState {
                selected_id: ViewComponentIds::Popup(PopupIds::Build(Some(
                    BuildingIds::Lumbarjack,
                ))),
                selectable_ids: BuildingIds::get_selectable_items(),
                view_data: ViewDataTypes::Popup(PopupState::default()),
                ..Default::default()
            },
            children: BuildingIds::get_ui_items(),
        }
    }

    pub fn command() -> UiView {
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

    pub fn logger() -> UiView {
        UiView {
            id: UiViewIds::Popup(UiPopupViewIds::Logs),
            layout: UiLayout {
                margin: 0,
                alignment: LayoutAlignments::Vertical,
                constraints: vec![LayoutConstraints::Percentage(100)],
            },
            state: UiViewState {
                selected_id: ViewComponentIds::Popup(PopupIds::Log(Some(LoggerIds::Print))),
                selectable_ids: vec![],
                view_data: ViewDataTypes::Logger(LoggerState::default()),
                ..Default::default()
            },
            children: LoggerIds::get_ui_items(),
        }
    }
}
