import { invoke } from "@tauri-apps/api";
import { UnlistenFn, listen, Event } from "@tauri-apps/api/event";
import { SendEvents } from "../../../shared";
import { SubscriptionCallback, SubscriptionEventTypes } from "../../SubsriberService";

export class TauriApi {

  private static didSubscribe = false;

  private static didMount = false;

  private static ecsListener: UnlistenFn = () => { };

  public static async webviewDidMount() {
    if (TauriApi.didMount) {
      throw new Error("webviewDidMount should only be called once");
    }

    await invoke("webview_did_mount", { isMounted: true });
    TauriApi.didMount = true;
  }

  public static async webviewDidSubscribe() {
    if (TauriApi.didSubscribe) {
      throw new Error("webviewDidSubscribe should only be called once");
    }

    await invoke("webview_did_subscribe", { hasSubscribed: true });
    TauriApi.didSubscribe = true;
  }

  public static async ecsSubscriptionListener(cb: SubscriptionCallback) {
    TauriApi.disposeEcsSubscriptionListener();
    TauriApi.ecsListener = await listen("ecs-subscription", ((event: Event<SubscriptionEventTypes | null>) => {
      const payload = event.payload;
      payload && cb(payload);
    }));
  }

  public static async sendEcsEvent(event: SendEvents) {
    await invoke("webview_ecs_event", { event });
  }

  public static disposeEcsSubscriptionListener() {
    TauriApi.ecsListener();
    TauriApi.ecsListener = () => { };
  }
}