import React from "react";
import { KalkCalculator, ConsoleLine } from "@paddim8/kalk-component";


const ToolsWindow = () => {
  return (
      <div className="kalk">
        <kalk-calculator>
          <console-line>kalker</console-line>
          <console-line>
            <span className="hint">Type 'help' for instructions.</span>
          </console-line>
        </kalk-calculator>
      </div>
  )
}


export default ToolsWindow;