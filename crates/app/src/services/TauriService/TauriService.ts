import { invoke } from "@tauri-apps/api";
import { UnlistenFn, listen, Event } from "@tauri-apps/api/event";
import { SendEvents, SubscriptionEvents } from "../../shared";
import { SubscriptionCallback } from "../SubsriberService";

export class TauriService {

  private static didSubscribe = false;

  private static didMount = false;

  private static ecsListener: UnlistenFn = () => { };

  public static async webviewDidMount() {
    if (TauriService.didMount) {
      throw new Error("webviewDidMount should only be called once");
    }

    await invoke("webview_did_mount", { isMounted: true });
    TauriService.didMount = true;
  }

  public static async webviewDidSubscribe() {
    if (TauriService.didSubscribe) {
      throw new Error("webviewDidSubscribe should only be called once");
    }

    await invoke("webview_did_subscribe", { hasSubscribed: true });
    TauriService.didSubscribe = true;
  }

  public static async ecsSubscriptionListener(cb: SubscriptionCallback) {
    TauriService.disposeEcsSubscriptionListener();
    TauriService.ecsListener = await listen("ecs-subscription", ((event: Event<SubscriptionEvents | null>) => {
      const payload = event.payload;
      payload && cb(payload);
    }));
  }

  public static async sendEcsEvent(event: SendEvents) {
    await invoke("webview_ecs_event", { event });
  }

  public static disposeEcsSubscriptionListener() {
    TauriService.ecsListener();
    TauriService.ecsListener = () => { };
  }
}