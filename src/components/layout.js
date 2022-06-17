import Chrome from "./navigation/chrome";
import Navbar from "./navigation/navbar";
import Toolbar from "./navigation/toolbar";
import ToolWindow from './toolswindow';
import VerticalSplit from "./windows/vertical-split";



export default function Layout({ selected, children }) {
  return (
      <div className="inline-grid grid-cols-2 grid-rows-3 w-screen h-screen grid-cols-[min-content_1fr]
      grid-rows-[min-content_1fr] overflow-hidden">
        <Chrome />
        <Navbar selected={selected} />
        <div className="relative">
          <VerticalSplit
            gutterClassName="bg-primary-100"
            gutterInnerElements={
            <span className="text-gray-600">
              Terminal
            </span>
            }
          >
            <main className="">
              { children }
            </main>
            <ToolWindow />
          </VerticalSplit>
        </div>
        <Toolbar />
      </div>
  )
}