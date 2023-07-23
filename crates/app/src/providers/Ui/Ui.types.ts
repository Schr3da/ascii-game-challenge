import { UiSubscription} from "../../shared";

export type UiContextValue = {
  nextUiEvent: UiSubscription;
  previousUiEvent: UiSubscription; 
}