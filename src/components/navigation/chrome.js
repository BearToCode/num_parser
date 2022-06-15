import { VscChromeClose, VscChromeRestore, VscChromeMinimize, VscChromeMaximize } from 'react-icons/vsc'
import { appWindow } from "@tauri-apps/api/window";
import useIsMaximized from "../hooks/is-maximized";

function MinimizeButton() {
  return (
      <button className="h-full hover:bg-neutral-700 px-3" onClick={() => appWindow.minimize()}>
        <VscChromeMinimize />
      </button>
  )
}

function MaximizeButton() {
  const isMaximized = useIsMaximized();
  if (isMaximized === undefined) return null;
  if (isMaximized) {
    return (
        <button className="h-full hover:bg-neutral-700 px-3" onClick={() => appWindow.unmaximize()}>
          <VscChromeRestore />
        </button>
    )
  } else {
    return (
      <button className="h-full hover:bg-neutral-700 px-3" onClick={() => appWindow.maximize()}>
        <VscChromeMaximize />
      </button>
    )
  }
}

function CloseButton() {
  return (
      <button className="h-full hover:bg-red-700 px-3" onClick={() => appWindow.close()}>
        <VscChromeClose />
      </button>
  )
}


export default function Chrome() {
  return (
      <div data-tauri-drag-region className="h-8 bg-neutral-800 toolbar text-neutral-400 flex flex-row col-span-2
       items-center justify-between text-sm border-b border-neutral-600">
        <ul className="space-x-4 flex flex-row select-none w-min ml-4">
          <li>File</li>
          <li>Edit</li>
          <li>View</li>
          <li>Help</li>
        </ul>
        <ul className="text-base flex h-full">
          <MinimizeButton />
          <MaximizeButton />
          <CloseButton />
        </ul>
      </div>
  )
}