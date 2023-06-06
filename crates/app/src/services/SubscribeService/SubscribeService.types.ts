import { GeneralSubscription, RenderSubscription, UiSubscription } from "../../shared";

export type SubscriptionEventTypes = RenderSubscription | UiSubscription | GeneralSubscription;

export type SubscriptionCallback = (event: SubscriptionEventTypes) => void;

export type Subscriptions = { [id: string]: SubscriptionCallback | null };