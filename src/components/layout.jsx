import Chrome from "./navigation/chrome";
import Navbar from "./navigation/navbar";
import Toolbar from "./navigation/toolbar";
import React, {useRef} from "react";
import VerticalPanels from "./windows/vertical-panels";


export default function Layout({ selected, children }) {
  const VerticalPanelsRef = useRef();

  function toggleToolsWindow(element) {
    if (VerticalPanelsRef.current.currentWindow() == 'both') {
      VerticalPanelsRef.current.closeWindow('bottom');
    } else {
      VerticalPanelsRef.current.setWindowContent('bottom', element);
      VerticalPanelsRef.current.openWindow('bottom');
    }
  }


  return (
      <div className="inline-grid w-screen h-screen grid-cols-[min-content_1fr]
      grid-rows-[min-content_1fr] overflow-hidden">
        <Chrome />
        <Navbar selected={selected} />
        <VerticalPanels 
          ref={VerticalPanelsRef} 
          topElement={<>{children}</>}
          bottomElement={<></>}
          topElementContainerStyle={'bg-transparent'}
          bottomElementContainerStyle={'bg-primary-200'}
          initialState={'top'}
          fixedSizePanel={'bottom'}
        />
        <Toolbar
            openToolsWindow={toggleToolsWindow}
        />
      </div>
  )
}
