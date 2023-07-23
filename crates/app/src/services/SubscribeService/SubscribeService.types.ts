import {
  GameStatus,
  GeneralSubscription,
  RenderSubscription,
  SelectedCell,
  UiSubscription,
  UiView,
} from "../../shared.d";

export type SubscriptionEventTypes =
  | RenderSubscription
  | UiSubscription
  | GeneralSubscription;

export type SubscriptionCallback =
  | GeneralSubscriptionCallback
  | ViewRenderSubscriptionCallback
  | PopupRenderSubscriptionCallback
  | GameStatusSubscriptionCallback;

export type GeneralSubscriptionCallback = (
  event: SubscriptionEventTypes
) => void;

export type ViewRenderSubscriptionCallback = (view: UiView | null) => void;

export type PopupRenderSubscriptionCallback = (view: UiView | null) => void;

export type GameStatusSubscriptionCallback = (status: GameStatus) => void;

export type SelectedTileRenderSubscriptionCallback = (
  view: SelectedCell
) => void;

export type Subscriptions = {
  GeneralSubscription: { [id: string]: GeneralSubscriptionCallback | null };
  ViewSubscription: { [id: string]: ViewRenderSubscriptionCallback | null };
  PopupSubscription: { [id: string]: PopupRenderSubscriptionCallback };
  GameStatusSubscription: { [id: string]: GameStatusSubscriptionCallback };
};
