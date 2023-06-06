import { invoke, os } from "@tauri-apps/api";
import { appWindow } from '@tauri-apps/api/window'
import { UnlistenFn, listen, Event } from "@tauri-apps/api/event";
import { SendEvents } from "../../../shared";
import { SubscriptionCallback, SubscriptionEventTypes } from "../../SubscribeService";
import { Platforms } from "../ApiService.types";


export class TauriApi {

  private static didSubscribe = false;

  private static didMount = false;

  private static ecsListener: UnlistenFn = () => { };

  public static getPlatform = async (): Promise<Platforms> => {
    const next = await os.platform();
    switch (next) {
      case 'linux':
      case 'dragonfly':
      case 'freebsd':
      case 'netbsd':
      case 'solaris':
      case 'openbsd':
        return Platforms.Linux;
      case 'win32':
        return Platforms.Windows;
      case 'android':
        return Platforms.Android;
      case 'ios':
        return Platforms.iOS;
      case 'darwin':
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