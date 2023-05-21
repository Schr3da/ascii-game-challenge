use core_dtos::prelude::*;

pub fn options_view() -> UiView {
    UiView {
        id: UiViewIds::Options,
        state: UiViewState {
            selected_id: ViewComponentIds::Options(OptionMenu::Back),
        },
        children: vec![
            UiViewChild::Label(UiLabel {
                id: ViewComponentIds::Options(OptionMenu::Title),
                text: "Options".to_string(),
            }),
            UiViewChild::List(UiList {
                id: ViewComponentIds::Options(OptionMenu::OptionList),
                children: vec![],
            }),
        ],
    }
}
