import { colorizeAnsi } from '@/utilities/ansi'

export type LineKind = 'input' | 'stdout' | 'stderr'

export type Line = {
  text: string
  kind: LineKind
}

export function promptText(currentDir: string): string {
  const dir = currentDir === '/root' ? '~' : currentDir
  return `root@deploydodo:${dir}#`
}

export function renderOutput(line: Line, currentDir: string): React.ReactNode {
  if (line.kind === 'input') {
    return (
      <>
        <span className="text-[#5faf5f] select-none">
          {promptText(currentDir)}
        </span>{' '}
        <span className="text-[#e0e0e0]">{line.text}</span>
      </>
    )
  }
  if (line.kind === 'stderr') {
    return (
      <span className="text-[#d75f5f] whitespace-pre-wrap">
        {line.text}
      </span>
    )
  }
  return <span className="whitespace-pre-wrap">{colorizeAnsi(line.text)}</span>
}

type LineViewProps = {
  index: number
  line: Line
  currentDir: string
}

export function LineView({ index, line, currentDir }: LineViewProps) {
  return (
    <div className="flex items-start leading-6 py-0.5 min-h-[28px]">
      <span className="w-8 text-right pr-3 text-white/15 select-none font-mono text-sm shrink-0">
        {index + 1}
      </span>
      <div className="flex-1 font-mono text-sm">
        {renderOutput(line, currentDir)}
      </div>
    </div>
  )
}
