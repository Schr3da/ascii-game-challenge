import { TauriApi } from "./tauri";

import { SendEvents } from "../../shared";
import { SubscriptionCallback } from "../SubscribeService";
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

  public static ecsSubscriptionListener = async (cb: SubscriptionCallback) => {
    if (!this.isTauriSuppored) {
      return Promise.resolve();
    }

    await TauriApi.ecsSubscriptionListener(cb);
  };

  public static sendEcsEvent = async (event: SendEvents) => {
    if (!this.isTauriSuppored) {
      return Promise.resolve();
    }

    console.log("sending event to rust: ", event);
    await TauriApi.sendEcsEvent(event);
  };

  public static disposeEcsSubscriptionListener = () => {
    if (!this.isTauriSuppored) {
      return;
    }

    TauriApi.disposeEcsSubscriptionListener();
  };
}
