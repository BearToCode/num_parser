import Chrome from "./navigation/chrome";
import Navbar from "./navigation/navbar";
import Toolbar from "./navigation/toolbar";


export default function Layout({ children }) {
  return (
      <div className="inline-grid grid-cols-2 grid-rows-3 w-screen h-screen grid-cols-[min-content_1fr]
      grid-rows-[min-content_1fr]">
        <Chrome />
        <Navbar />
        <main className="bg-blue-500 border-blue-600 border-2">
          { children }
        </main>
        <Toolbar />
      </div>
      // <div>
      //   <Navbar />
      //   <Toolbar />
      //   <div>
      //     { children }
      //   </div>
      // </div>
  )
}