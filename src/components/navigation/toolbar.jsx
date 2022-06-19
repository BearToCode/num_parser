import { BiTerminal } from 'react-icons/bi'
// Tools
import Terminal from "./tools/terminal";

export default function Toolbar({ openToolsWindow }) {
  return (
      <div className="col-span-2 h-7 bg-primary-200 border-t border-neutral-600">
        <ul className="flex items-center text-neutral-400 text-xs h-full mx-4">
          <ToolbarItem
              icon={<BiTerminal />}
              text="Terminal"
              openToolsWindow={openToolsWindow}
              tool={<Terminal />}
          />
        </ul>
      </div>
  )
}

const ToolbarItem = ({ icon, text, openToolsWindow, tool }) => {
  return (
      <button className="flex items-center" onClick={() => openToolsWindow(tool)}>
        <div className="text-lg mr-1">
          {icon}
        </div>
        {text}
      </button>
  )
}