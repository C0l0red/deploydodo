import { useEffect, useRef } from 'react'
import { type Line } from './TerminalOutput'

export function useAutoScroll(lines: Line[]) {
  const containerRef = useRef<HTMLDivElement>(null)

  useEffect(() => {
    const c = containerRef.current
    if (c) c.scrollTop = c.scrollHeight
  }, [lines])

  return containerRef
}
