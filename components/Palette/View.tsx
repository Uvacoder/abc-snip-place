import { Command, CommandMenu, useCommands, useKmenu } from 'kmenu'
import { FC } from 'react'
import {
  FiCode,
  FiCopy,
  FiDownloadCloud,
  FiGithub,
  FiGitlab,
  FiMoon,
  FiPlus,
  FiShare,
  FiShare2,
  FiSun,
} from 'react-icons/fi'
import { BiPaintRoll } from 'react-icons/bi'
import { useTheme } from 'next-themes'
import { useEffect } from 'react'
import { useState } from 'react'
import type { Snip } from '@typings/snip'

const Palette: FC<{ snip: Snip }> = ({ snip }) => {
  const [input, setInput, open, setOpen] = useKmenu()

  const [mounted, setMounted] = useState(false)
  useEffect(() => setMounted(true), [])
  const { setTheme } = useTheme()

  const main: Command[] = [
    {
      category: 'Account',
      commands: [
        {
          icon: <FiGithub />,
          text: 'Continue With GitHub',
        },
        {
          icon: <FiGitlab />,
          text: 'Continue With GitLab',
        },
      ],
    },
    {
      category: 'Utility',
      commands: [
        {
          icon: <FiPlus />,
          text: 'New Snip',
          href: 'https://snip.au/',
        },
        {
          icon: <FiCopy />,
          text: 'Copy Snip',
          perform: () => navigator.clipboard.writeText(snip.code),
        },
        {
          icon: <FiShare2 />,
          text: 'Copy URL',
          perform: () =>
            navigator.clipboard.writeText(`https://snip.au/${snip.slug}`),
        },
        {
          icon: <FiDownloadCloud />,
          text: 'Download Snip',
          href: `data:application/octet-stream,${encodeURIComponent(
            snip.code
          )}`,
        },
      ],
    },
    {
      category: 'General',
      commands: [
        {
          icon: <BiPaintRoll />,
          text: 'Theme...',
        },
        {
          icon: <FiCode />,
          text: 'API',
          href: '/api',
        },
        {
          icon: <FiGithub />,
          text: 'Source',
          href: 'https://github.com/harshhhdev/snip',
          newTab: true,
        },
      ],
    },
  ]

  const themes: Command[] = [
    {
      category: 'Themes',
      commands: [
        {
          icon: <FiSun />,
          text: 'Light',
          perform: () => setTheme('light'),
        },
        {
          icon: <FiMoon />,
          text: 'Dark',
          perform: () => setTheme('dark'),
        },
      ],
    },
  ]

  const [mainCommands] = useCommands(main)
  const [themeCommands] = useCommands(themes)

  if (!mounted) return null

  return (
    <>
      <CommandMenu commands={mainCommands} index={1} main />
      <CommandMenu commands={themeCommands} index={2} placeholder='Theme...' />
    </>
  )
}

export default Palette