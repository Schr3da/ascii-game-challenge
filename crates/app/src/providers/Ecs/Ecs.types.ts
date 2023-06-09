import {
  GeneralSubscription,
  RenderSubscription,
  UiSubscription,
  UiView,
  ViewComponentIds,
} from "../../shared";

export type EcsContextValue = {
  previousView: UiView | null;
  nextView: UiView | null;
  previousPopupView: UiView | null;
  nextPopupView: UiView | null;
  previousGeneralEvent: GeneralSubscription;
  nextGeneralEvent: GeneralSubscription;
  previousUiEvent: UiSubscription;
  nextUiEvent: UiSubscription;
  previousRendererEvent: RenderSubscription;
  nextRendererEvent: RenderSubscription;
  isViewComponentSelected: (id: ViewComponentIds) => boolean;
  isPopupViewComponentSelected: (id: ViewComponentIds) => boolean;
};
