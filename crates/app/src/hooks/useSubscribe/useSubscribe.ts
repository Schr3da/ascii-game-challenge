import { invoke } from "@tauri-apps/api";
import { listen } from "@tauri-apps/api/event";
import { useCallback, useEffect } from "react";
import { SubscriptionCallback } from "./useSubscribe.types";

export const useSubscribe = (cb: SubscriptionCallback) => {
  const handleDidMount = useCallback(() => {
    invoke("did_webview_mount", { isMounted: true });
  }, []);

  const handleSubscribe = useCallback(async () => {
    await listen("OnWorldDidUpdate", cb);
  }, [cb]);

  useEffect(() => {
    handleSubscribe();
    handleDidMount();
  }, []);
}