import { BiTerminal } from 'react-icons/bi'

export default function Toolbar() {
  return (
      <div className="col-span-2 h-7 bg-primary-200 border-t border-neutral-600">
        <ul className="flex items-center text-neutral-400 text-xs h-full mx-4">
          <ToolbarItem icon={<BiTerminal />} text="Terminal" />
        </ul>
      </div>
  )
}

const ToolbarItem = ({ icon, text }) => {
  return (
      <button className="flex items-center">
        <div className="text-lg mr-1">
          {icon}
        </div>
        {text}
      </button>
  )
}