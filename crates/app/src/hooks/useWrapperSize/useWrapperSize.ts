import { useEffect, useState } from "react";
import { useApplicationContext } from "../../providers";

export const useWrapperSize = <T extends HTMLElement | null>(
  ref: React.MutableRefObject<T>
) => {
  const { windowWidth, windowHeight } = useApplicationContext();
  const [width, setWidth] = useState(windowWidth);

  const [height, setHeight] = useState(windowHeight);

  useEffect(() => {
    if (ref.current == null) {
      return;
    }
    setWidth(ref.current.clientWidth);
    setHeight(ref.current.clientHeight);
  }, [windowWidth, windowHeight]);

  return { width, height };
};
