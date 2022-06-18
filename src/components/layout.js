import Chrome from "./navigation/chrome";
import Navbar from "./navigation/navbar";
import Toolbar from "./navigation/toolbar";
import VerticalSplit from "./windows/vertical-split";
import React from "react";
import ToolsWindow from "./toolswindow";


export default function Layout({ selected, children }) {
  return (
      <div className="inline-grid grid-cols-2 grid-rows-3 w-screen h-screen grid-cols-[min-content_1fr]
      grid-rows-[min-content_1fr] overflow-hidden">
        <Chrome />
        <Navbar selected={selected} />
        <MainWindow />
        <Toolbar />
      </div>
  )
}


const MainWindow = ({ children }) => {
  let isOpen = true;
  let toolsWindow = <ToolsWindow />;

  const closeToolsWindow = () => {
    isOpen = false;
    console.log("Clicked!")
  }

  const openToolsWindow = () => {
    isOpen = true;
  }


  if (isOpen) {
    return (
        <VerticalSplit
            gutterClassName="bg-primary-100 h-4 hover:bg-blue-200 flex items-center justify-between text-slate-300 px-2"
        >
          <main className="bg-red-400">
            { children }
            CIAO
          </main>
          {toolsWindow}
        </VerticalSplit>
    )
  } else {
    return (
        <main className="bg-red-400">
          { children }
        </main>
      )
  }
};