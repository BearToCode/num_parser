import Chrome from "./navigation/chrome";
import Navbar from "./navigation/navbar";
import Toolbar from "./navigation/toolbar";


export default function Layout({ selected, children }) {
  return (
      <div className="inline-grid grid-cols-2 grid-rows-3 w-screen h-screen grid-cols-[min-content_1fr]
      grid-rows-[min-content_1fr]">
        <Chrome />
        <Navbar selected={selected} />
        <main className="bg-primary-200">
          { children }
        </main>
        <Toolbar />
      </div>
  )
}