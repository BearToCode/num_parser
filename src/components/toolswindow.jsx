import React from "react";
import { MdOutlineClose } from 'react-icons/md';
// import { KalkCalculator, ConsoleLine } from "@paddim8/kalk-component";


const ToolsWindow = ({ children, closeFn }) => {
  return (
      <div className="flex flex-col px-2">
        <div className="flex text-lg text-neutral-500 h-6 items-center">
          <div className="flex-grow" />
          <button className="hover:bg-neutral-700 rounded" onClick={() => {
            closeFn();
          }}>
            <MdOutlineClose />
          </button>
        </div>
        <div>
          { children }
        </div>
      </div>
  )
}


export default ToolsWindow;