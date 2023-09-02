import { UiView, ViewComponentIds } from "../../shared"

export type ViewContextValue = {
  nextView: UiView | null;
  previousView: UiView | null;
  selectedViewId: String;
  isViewSelected: (next: ViewComponentIds) => boolean;
}