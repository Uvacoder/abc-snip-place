import { FC, ReactNode } from 'react'
import { FiCommand } from 'react-icons/fi'
import Link from 'next/link'
import { useKmenu } from 'kmenu'

const Nav: FC = () => {
  const [input, setInput, open, setOpen] = useKmenu()

  return (
    <div className='flex items-center justify-between w-full'>
      <Link href='/'>
        <h1 className='text-black dark:text-white text-3xl font-bold'>snip</h1>
      </Link>
      <button
        className='bg-white shadow-2xl text-black dark:bg-gray-900 dark:hover:bg-gray-800 dark:text-white w-12 h-12 rounded-lg flex justify-center text-xl items-center'
        onClick={() => setOpen(1)}
      >
        <FiCommand size={25} />
      </button>
    </div>
  )
}

export default Nav
