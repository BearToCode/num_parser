import { Link, useLocation } from 'react-router-dom'
// Icons
import { AiOutlineFunction } from 'react-icons/ai'
import { IoDocumentText } from 'react-icons/io5'
import { ImSphere } from 'react-icons/im'
import { Tb3DCubeSphere } from 'react-icons/tb'
import { BsBraces } from 'react-icons/bs'


export default function Navbar() {
  return (
      <nav className="w-14 bg-primary-200 border-r border-neutral-600">
        <ul className="flex flex-col justify-center text-3xl text-neutral-500">
          <NavBarItem to="/2d">
            <AiOutlineFunction />
          </NavBarItem>
          <NavBarItem to="/3d">
            <ImSphere />
          </NavBarItem>
          <NavBarItem to="/code">
            <BsBraces />
          </NavBarItem>
          <NavBarItem to="/document">
            <IoDocumentText />
          </NavBarItem>
          <NavBarItem to="/addons">
            <Tb3DCubeSphere />
          </NavBarItem>
        </ul>
      </nav>
  )
}

const NavBarItem = ({ children, to }) => {
  let classes = `justify-center flex py-2 relative `;

  const location = useLocation();
  const selected = to === location.pathname;

  if (selected)
    classes += "text-white";
  return (
    <Link to={to} className={classes}>
      { children }
      {
        selected &&
          <div className="absolute left-0 top-0 bottom-0 w-0.5 bg-amber-400">

          </div>
      }
    </Link>
  )
}