import Chrome from "./navigation/chrome";
import Navbar from "./navigation/navbar";
import Toolbar from "./navigation/toolbar";
import VerticalSplit from "./navigation/vertical-split";
import React, {useState} from "react";
import ToolsWindow from "./toolswindow";


export default function Layout({ selected, children }) {
  const [isToolsWindowOpen, setToolsWindowOpen] = useState(false);
  const [toolsWindowChildren, setToolsWindowChildren] = useState(null);

  const closeToolsWindow = () => {
    setToolsWindowOpen(false);
    console.log("Pippo!")
  }

  const openToolsWindow = (toolsWindowChildren) => {
    setToolsWindowOpen(true);
    setToolsWindowChildren(toolsWindowChildren);
  }


  return (
      <div className="inline-grid grid-cols-2 grid-rows-3 w-screen h-screen grid-cols-[min-content_1fr]
      grid-rows-[min-content_1fr] overflow-hidden">
        <Chrome />
        <Navbar selected={selected} />
        <MainWindow
            isToolsWindowOpen={isToolsWindowOpen}
            closeToolsWindow={closeToolsWindow}
            toolsWindowChildren={toolsWindowChildren}
        />
        <Toolbar
            openToolsWindow={openToolsWindow}
        />
      </div>
  )
}


const MainWindow = ({ children, toolsWindowChildren, closeToolsWindow, isToolsWindowOpen }) => {
  let toolsWindow = (
      <ToolsWindow closeFn={closeToolsWindow}>
        { toolsWindowChildren }
      </ToolsWindow>
  )

  if (isToolsWindowOpen) {
    return (
        <VerticalSplit>
          <main className="bg-primary-200">
            { children }
          </main>
          {toolsWindow}
        </VerticalSplit>
    )
  } else {
    return (
        <main className="bg-primary-200">
          { children }
        </main>
      )
  }
};