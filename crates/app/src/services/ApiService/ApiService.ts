import { TauriApi } from "./tauri";

import { SendEvents } from "../../shared.d";
import { GameStatusSubscriptionCallback, PopupRenderSubscriptionCallback, SubscriptionCallback, ViewRenderSubscriptionCallback } from "../SubscribeService";
import { Platforms } from "./ApiService.types";

export class ApiService {
  private static isTauriSuppored =
    window.__TAURI_IPC__ || window.__TAURI_METADATA__;

  public static getPlatform = async (): Promise<Platforms> => {
    if (!this.isTauriSuppored) {
      return Promise.resolve(Platforms.Web);
    }

    const next = await TauriApi.getPlatform();
    return next;
  };

  public static closeApplication = () => {
    if (!this.isTauriSuppored) {
      return;
    }

    TauriApi.closeApplication();
  };

  public static minimiseApplication = () => {
    if (!this.isTauriSuppored) {
      return;
    }

    TauriApi.minimiseApplication();
  };

  public static maximiseApplication = () => {
    if (!this.isTauriSuppored) {
      return;
    }

    TauriApi.maximiseApplication();
  };

  public static webviewDidMount = async () => {
    if (!this.isTauriSuppored) {
      return Promise.resolve();
    }

    await TauriApi.webviewDidMount();
  };

  public static webviewDidSubscribe = async () => {
    if (!this.isTauriSuppored) {
      return Promise.resolve();
    }

    await TauriApi.webviewDidSubscribe();
  };

  public static ecsGeneralSubscriptionListener = async (cb: SubscriptionCallback) => {
    if (!this.isTauriSuppored) {
      return Promise.resolve();
    }

    await TauriApi.ecsGeneralSubscriptionListener(cb);
  };

  public static ecsViewRenderSubscriptionListener = async (cb: ViewRenderSubscriptionCallback) => {
    if (!this.isTauriSuppored) {
      return Promise.resolve();
    }

    await TauriApi.viewRenderSubscriptionListener(cb);
  };

  public static ecsPopRenderSubscriptionListener = async (cb: PopupRenderSubscriptionCallback) => {
    if (!this.isTauriSuppored) {
      return Promise.resolve();
    }

    await TauriApi.popupRenderSubscriptionListener(cb);
  };

  public static ecsGameStatusSubscriptionListener = async (cb: GameStatusSubscriptionCallback) => {
    if (!this.isTauriSuppored) {
      return Promise.resolve();
    }

    await TauriApi.gameStatusSubscriptionListener(cb);
  };

  public static sendEcsEvent = async (event: SendEvents) => {
    if (!this.isTauriSuppored) {
      return Promise.resolve();
    }

    console.log("sending event to rust: ", event);
    await TauriApi.sendEcsEvent(event);
  };

  public static disposeEcsGeneralSubscriptionListener = () => {
    if (!this.isTauriSuppored) {
      return;
    }

    TauriApi.disposeEcsGeneralSubscriptionListener();
  };


  public static disposeEcsViewRenderSubscriptionListener = () => {
    if (!this.isTauriSuppored) {
      return;
    }

    TauriApi.disposeViewRenderSubscriptionListener();
  };

  public static disposeEcsPopupRenderSubscriptionListener = () => {
    if (!this.isTauriSuppored) {
      return;
    }

    TauriApi.disposePopupRenderSubscriptionListener();
  };
}
