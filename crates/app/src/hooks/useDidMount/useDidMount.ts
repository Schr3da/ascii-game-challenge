import { invoke } from "@tauri-apps/api";
import { useEffect } from "react";

export const useDidMount = () => {
  useEffect(() => {
    invoke("did_webview_mount", { isMounted: true });
  }, []);
}