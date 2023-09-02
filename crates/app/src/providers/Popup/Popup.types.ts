import { UiView, ViewComponentIds } from "../../shared.d";

export type PopupContextValues = {
  nextPopup: UiView | null;
  previousPopup: UiView | null;
  selectedPopupId: String;
  isPopupSelected: (next: ViewComponentIds) => void;
}