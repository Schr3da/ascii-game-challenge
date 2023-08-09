import { useCallback, useEffect, useRef } from "react";

export const useDebounce = () => {
  const timerRef = useRef<number | null>(null);

  const clearDebounce = useCallback(() => {
    if (!timerRef.current) {
      return;
    }
    clearTimeout(timerRef.current);
    timerRef.current = null;
  }, []);

  const debounce = useCallback((cb: Function, ms: number) => {
    clearDebounce();
    timerRef.current = setTimeout(cb, ms);
  }, [clearDebounce]);

    useEffect(() => clearDebounce, []);

  return {
    debounce,
    clearDebounce,
  };
};
