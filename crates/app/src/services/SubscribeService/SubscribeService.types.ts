import {
  GameMeta,
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
  | GameMetaSubscriptionCallback;

export type GeneralSubscriptionCallback = (
  event: SubscriptionEventTypes
) => void;

export type ViewRenderSubscriptionCallback = (view: UiView | null) => void;

export type PopupRenderSubscriptionCallback = (view: UiView | null) => void;

export type GameMetaSubscriptionCallback = (meta: GameMeta) => void;

export type SelectedTileRenderSubscriptionCallback = (
  view: SelectedCell
) => void;

export type Subscriptions = {
  GeneralSubscription: { [id: string]: GeneralSubscriptionCallback | null };
  ViewSubscription: { [id: string]: ViewRenderSubscriptionCallback | null };
  PopupSubscription: { [id: string]: PopupRenderSubscriptionCallback };
  GameMetaSubscription: { [id: string]: GameMetaSubscriptionCallback };
};
