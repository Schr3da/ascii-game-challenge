import { GeneralSubscription, RenderSubscription, UiSubscription, UiView } from "../../shared";

export type EcsContextValue = {
  previousView: UiView | null;
  nextView: UiView | null;
  previousGeneralEvent: GeneralSubscription;
  nextGeneralEvent: GeneralSubscription;
  previousUiEvent: UiSubscription;
  nextUiEvent: UiSubscription;
  previousRendererEvent: RenderSubscription;
  nextRendererEvent: RenderSubscription;
}
