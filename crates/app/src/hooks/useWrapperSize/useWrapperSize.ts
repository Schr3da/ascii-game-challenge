import { useEffect, useState } from "react";
import { useApplicationContext } from "../../providers";

export const useWrapperSize = <T extends HTMLElement | null>(
  ref: React.MutableRefObject<T>
) => {
  const { windowWidth, windowHeight } = useApplicationContext();
  const [width, setWidth] = useState(0);

  const [height, setHeight] = useState(0);

  useEffect(() => {
    if (ref.current == null) {
      return;
    }
    setWidth(ref.current.clientWidth);
    setHeight(ref.current.clientHeight);
  }, [windowWidth, windowHeight]);

    console.log(height);

  return { width, height };
};
