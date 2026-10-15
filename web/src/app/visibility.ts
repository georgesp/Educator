import { useEffect, useState } from "react";

const isVisible = () => typeof document === "undefined" || document.visibilityState !== "hidden";

/**
 * Whether the application is on screen: false while it is in the background (iPad: another app,
 * a call; computer: window hidden). SUB023811.
 */
export function usePageVisible(): boolean {
  const [visible, setVisible] = useState(isVisible);

  useEffect(() => {
    const update = () => setVisible(isVisible());
    document.addEventListener("visibilitychange", update);
    return () => document.removeEventListener("visibilitychange", update);
  }, []);

  return visible;
}
