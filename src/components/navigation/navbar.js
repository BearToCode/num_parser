import { AiOutlineFunction } from 'react-icons/ai'
import { IoDocumentText } from 'react-icons/io5'

export default function Navbar() {
  return (
      <nav className="w-14 bg-neutral-800 border-r border-neutral-600">
        <ul className="flex flex-col justify-center space-y-4 my-4 text-3xl text-neutral-400">
          <button className="justify-center flex">
            <AiOutlineFunction />
          </button>
          <button className="justify-center flex">
            <IoDocumentText />
          </button>
        </ul>
      </nav>
  )
}