import { SubscriptionEvents } from "../../shared";

export type SubscriptionCallback = (event: SubscriptionEvents) => void; 

export type Subscriptions = {[id: string]: SubscriptionCallback | null};