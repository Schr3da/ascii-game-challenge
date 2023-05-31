import { TauriApi } from "./tauri";

import { SendEvents } from "../../shared";
import { SubscriptionCallback } from "../SubsriberService";

export class ApiService {

  private static isTauriSuppored = window.__TAURI_IPC__ || window.__TAURI_METADATA__;

  public static async webviewDidMount() {
    if (!this.isTauriSuppored) {
      return Promise.resolve();
    }

    await TauriApi.webviewDidMount();
  }

  public static async webviewDidSubscribe() {
    if (!this.isTauriSuppored) {
      return Promise.resolve();
    }

    await TauriApi.webviewDidSubscribe();
  }

  public static async ecsSubscriptionListener(cb: SubscriptionCallback) {
    if (!this.isTauriSuppored) {
      return Promise.resolve();
    }

    await TauriApi.ecsSubscriptionListener(cb);
  }

  public static async sendEcsEvent(event: SendEvents) {
    if (!this.isTauriSuppored) {
      return Promise.resolve();
    }

    await TauriApi.sendEcsEvent(event);
  }

  public static disposeEcsSubscriptionListener() {
    if (!this.isTauriSuppored) {
      return;
    }

    TauriApi.disposeEcsSubscriptionListener();
  }
}