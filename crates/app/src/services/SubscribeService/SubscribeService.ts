import {
  Subscriptions,
  SubscriptionCallback,
  SubscriptionEventTypes,
} from "./SubscribeService.types";

import { ApiService } from "../ApiService";
import {
  EcsSubscriptionIds,
  GameMeta,
  UiView,
} from "../../shared.d";

class SubscribeService {
  public isInitialised = false;

  private subscriptions: Subscriptions = {
    GeneralSubscription: {},
    ViewSubscription: {},
    PopupSubscription: {},
    GameMetaSubscription: {},
  };

  public init = async () => {
    await ApiService.ecsGeneralSubscriptionListener(
      this.onReceivedGeneralEvent
    );

    await ApiService.ecsViewRenderSubscriptionListener(
      this.onReceivedViewSubscriptionEvent
    );

    await ApiService.ecsPopRenderSubscriptionListener(
      this.onReceivedPopupSubscriptionEvent
    );

    await ApiService.ecsGameStatusSubscriptionListener(
      this.onReceivedGameMetaSubscriptionEvent
    );

    await ApiService.webviewDidSubscribe();
    this.isInitialised = true;
  };

  private onReceivedGeneralEvent = (event: SubscriptionEventTypes) => {
    Object.values(this.subscriptions.GeneralSubscription).forEach((cb) => {
      cb && cb(event);
    });
  };

  private onReceivedViewSubscriptionEvent = (view: UiView | null) => {
    Object.values(this.subscriptions.ViewSubscription).forEach((cb) => {
      cb && cb(view);
    });
  };

  private onReceivedPopupSubscriptionEvent = (view: UiView | null) => {
    Object.values(this.subscriptions.PopupSubscription).forEach((cb) => {
      cb && cb(view);
    });
  };

  private onReceivedGameMetaSubscriptionEvent = (status: GameMeta) => {
    Object.values(this.subscriptions.GameMetaSubscription).forEach((cb) => {
      cb && cb(status);
    });
  };

  public subscribe = (
    topic: EcsSubscriptionIds,
    cb: SubscriptionCallback
  ): string => {
    const id = `subscriber-${topic}-id-${
      Object.keys(this.subscriptions).length
    }`;

    this.subscriptions[topic][id] = cb;
    return id;
  };

  public unsubscribe = (topic: EcsSubscriptionIds, id: string) => {
    this.subscriptions[topic][id] = null;
  };

  public dispose() {
    ApiService.disposeEcsGeneralSubscriptionListener();
    ApiService.disposeEcsViewRenderSubscriptionListener();
    ApiService.disposeEcsPopupRenderSubscriptionListener();

    (Object.keys(this.subscriptions) as Array<EcsSubscriptionIds>).forEach(
      (topic) => {
        for (let id in this.subscriptions) {
          this.unsubscribe(topic, id);
        }
      }
    );

    this.subscriptions = {
      GeneralSubscription: {},
      PopupSubscription: {},
      ViewSubscription: {},
      GameMetaSubscription: {},
    };
  }
}

const instance = new SubscribeService();

export const sharedInstance = async (): Promise<SubscribeService> => {
  if (!instance.isInitialised) {
    await instance.init();
  }
  return instance;
};
