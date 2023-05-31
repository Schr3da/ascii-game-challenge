import { SubscriptionEvents } from "../../shared";
import { Subscriptions, SubscriptionCallback } from "./SubscribeService.types";
import { TauriService } from "../TauriService";

class SubscribeService {

  public isInitialised = false;

  private subscriptions: Subscriptions = {};

  public init = async () => {
    await TauriService.ecsSubscriptionListener(this.handleEvent);
    await TauriService.webviewDidSubscribe();
    this.isInitialised = true;
  }

  private handleEvent = (event: SubscriptionEvents) => {
    Object.values(this.subscriptions).forEach((cb) => {
      cb && cb(event);
    });
  }

  public subscribe = (cb: SubscriptionCallback): string => {
    const nextId = `subscriber-id-${Object.keys(this.subscriptions).length}`;
    this.subscriptions[nextId] = cb;
    return nextId;
  }

  public unsubscribe = (id: string) => {
    this.subscriptions[id] = null;
  }

  public dispose() {
    TauriService.disposeEcsSubscriptionListener();

    for (let id in this.subscriptions) {
      this.unsubscribe(id);
    }

    this.subscriptions = {};
  }
}

const instance = new SubscribeService();

export const sharedInstance = async (): Promise<SubscribeService> => {
  if (!instance.isInitialised) {
    await instance.init();
  }
  return instance;
}