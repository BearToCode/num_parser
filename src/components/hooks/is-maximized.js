import {useEffect, useState} from "react";
import {appWindow} from "@tauri-apps/api/window";

export default function useIsMaximized() {
  const [isMaximized, setIsMaximized] = useState(undefined);
  useEffect(() => {
    // Handler to call on window resize
    function handleResize() {
      appWindow.isMaximized().then(
          response => {
            debugger
            setIsMaximized(response)
          }
      )
    }
    // Add event listener
    window.addEventListener("resize", handleResize);
    // Call handler right away so state gets updated with initial window size
    handleResize();
    // Remove event listener on cleanup
    return () => window.removeEventListener("resize", handleResize);
  }, []); // Empty array ensures that effect is only run on mount
  return isMaximized;
}