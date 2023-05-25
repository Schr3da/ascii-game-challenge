use core_dtos::prelude::*;

pub fn options_view() -> UiView {
    UiView {
        id: UiViewIds::Options,
        layout: UiLayout {
            margin: 1,
            alignment: LayoutAlignments::Vertical,
            constraints: vec![
                LayoutConstraints::Percentage(20),
                LayoutConstraints::Percentage(60),
                LayoutConstraints::Percentage(20),
            ],
        },
        state: UiViewState {
            selected_id: ViewComponentIds::Options(OptionMenuIds::Back),
            selectable_ids: Vec::new(),
        },
        children: vec![
            UiViewChild::Label(UiLabel {
                id: ViewComponentIds::Options(OptionMenuIds::Title),
                text: "Options".to_string(),
            }),
            UiViewChild::List(UiList {
                id: ViewComponentIds::Options(OptionMenuIds::OptionList),
                label: "".to_string(),
                children: vec![],
            }),
        ],
    }
}
