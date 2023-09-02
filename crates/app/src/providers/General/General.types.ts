import { GeneralSubscription } from "../../shared";

export type GeneralContextValue = {
  nextGeneralEvent: GeneralSubscription;
  previousGeneralEvent: GeneralSubscription; 
}