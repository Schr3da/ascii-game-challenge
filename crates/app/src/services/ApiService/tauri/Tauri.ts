import { invoke, os } from "@tauri-apps/api";
import { appWindow } from "@tauri-apps/api/window";
import { UnlistenFn, listen, Event } from "@tauri-apps/api/event";

import {
  GameMetaSubscriptionCallback,
  PopupRenderSubscriptionCallback,
  SubscriptionCallback,
  ViewRenderSubscriptionCallback,
} from "../../SubscribeService";

import { Platforms } from "../ApiService.types";
import { EcsSubscriptionIds, GameMeta, InputEvents, SendEvents, UiView } from "../../../shared.d";

export class TauriApi {
  private static didSubscribe = false;

  private static didMount = false;

  private static generalListener: UnlistenFn = () => { };

  private static popupRenderListener: UnlistenFn = () => { };

  private static viewRenderListener: UnlistenFn = () => { };

  private static gameMetaListener: UnlistenFn = () => { };

  public static getPlatform = async (): Promise<Platforms> => {
    const next = await os.platform();
    switch (next) {
      case "linux":
      case "dragonfly":
      case "freebsd":
      case "netbsd":
      case "solaris":
      case "openbsd":
        return Platforms.Linux;
      case "win32":
        return Platforms.Windows;
      case "android":
        return Platforms.Android;
      case "ios":
        return Platforms.iOS;
      case "darwin":
        return Platforms.Macos;
    }
  };

  public static closeApplication() {
    appWindow.close();
  }

  public static minimiseApplication() {
    appWindow.minimize();
  }

  public static maximiseApplication() {
    appWindow.maximize();
  }

  public static async webviewDidMount(columns: number, rows: number) {
    if (TauriApi.didMount) {
      throw new Error("webviewDidMount should only be called once");
    }

    await invoke("webview_did_mount", { isMounted: true, columns, rows});
    TauriApi.didMount = true;
  }

  public static async webviewDidSubscribe() {
    if (TauriApi.didSubscribe) {
      throw new Error("webviewDidSubscribe should only be called once");
    }

    await invoke("webview_did_subscribe", { hasSubscribed: true });
    TauriApi.didSubscribe = true;
  }

  public static async ecsGeneralSubscriptionListener(cb: SubscriptionCallback) {
    const subscriptionId: EcsSubscriptionIds = "GeneralSubscription";

    TauriApi.disposeEcsGeneralSubscriptionListener();
    TauriApi.generalListener = await listen(
      subscriptionId,
      (event: Event<| null>) => {
        const payload = event.payload;
        payload && cb(payload);
      }
    );
  }

  public static disposeEcsGeneralSubscriptionListener() {
    TauriApi.generalListener();
    TauriApi.generalListener = () => { };
  }

  public static async viewRenderSubscriptionListener(
    cb: ViewRenderSubscriptionCallback
  ) {
    const subscriptionId: EcsSubscriptionIds = "ViewSubscription";

    TauriApi.disposeViewRenderSubscriptionListener();
    TauriApi.viewRenderListener = await listen(
      subscriptionId,
      (event: Event<UiView>) => {
        event.payload && cb(event.payload);
      }
    );
  }

  public static disposeViewRenderSubscriptionListener() {
    TauriApi.viewRenderListener();
    TauriApi.viewRenderListener = () => { };
  }

  public static async popupRenderSubscriptionListener(
    cb: PopupRenderSubscriptionCallback
  ) {
    const subscriptionId: EcsSubscriptionIds = "PopupSubscription";

    TauriApi.disposePopupRenderSubscriptionListener();
    TauriApi.popupRenderListener = await listen(
      subscriptionId,
      (event: Event<UiView>) => {
        event.payload && cb(event.payload);
      }
    );
  }

  public static disposePopupRenderSubscriptionListener() {
    TauriApi.popupRenderListener();
    TauriApi.popupRenderListener = () => { };
  }

  public static async gameMetaSubscriptionListener(
    cb: GameMetaSubscriptionCallback
  ) {
    const subscriptionId: EcsSubscriptionIds = "GameMetaSubscription";

    TauriApi.disposeGameMetaSubscriptionListener();
    TauriApi.gameMetaListener = await listen(
      subscriptionId,
      (event: Event<GameMeta>) => {
        event.payload && cb(event.payload);
      }
    );
  }

  public static disposeGameMetaSubscriptionListener() {
    TauriApi.gameMetaListener();
    TauriApi.gameMetaListener = () => { };
  }

  public static async sendEcsEvent(event: SendEvents) {
    await invoke("webview_ecs_event", { event });
  }

  public static async sendInputEvent(event: InputEvents) {
    await invoke("webview_input_event", { event });
  }
}
