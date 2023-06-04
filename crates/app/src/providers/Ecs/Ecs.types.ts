import { SubscriptionEvents } from "../../shared"

export type EcsContextValue = {
  previousEvent: SubscriptionEvents;
  nextEvent: SubscriptionEvents;
}