import { useEffect, useState, type RefObject } from 'react'

export function useTerminalScroll(containerRef: RefObject<HTMLDivElement | null>) {
  const [isScrollable, setIsScrollable] = useState(false)

  useEffect(() => {
    const terminal = containerRef.current
    if (!terminal) return
    const main = terminal.closest('main')
    if (!main) return

    const checkSticky = () => {
      const sticky = main.querySelector('.sticky') as HTMLElement | null
      if (!sticky) return
      setIsScrollable(
        sticky.getBoundingClientRect().top <=
          main.getBoundingClientRect().top + 1,
      )
    }

    main.addEventListener('scroll', checkSticky, { passive: true })
    checkSticky()
    return () => main.removeEventListener('scroll', checkSticky)
  }, [containerRef])

  return isScrollable
}
