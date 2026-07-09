import { type FormEvent } from 'react'
import { promptText } from './TerminalOutput'

type Props = {
  currentDir: string
  lineCount: number
  connected: boolean
  terminalInput: string
  onInputChange: (value: string) => void
  onSubmit: (e: FormEvent) => void
}

export function TerminalPrompt({
  currentDir,
  lineCount,
  connected,
  terminalInput,
  onInputChange,
  onSubmit,
}: Props) {
  return (
    <form
      id="terminal-input-form"
      onSubmit={onSubmit}
      className="flex items-center gap-1.5 min-w-0"
    >
      <span className="w-8 text-right pr-3 text-white/15 select-none font-mono text-sm shrink-0 leading-6">
        {lineCount + 1}
      </span>
      <span className="text-[#5faf5f] select-none shrink-0 font-mono text-sm leading-6">
        {promptText(currentDir)}
      </span>
      <input
        id="terminal-input"
        type="text"
        value={terminalInput}
        onChange={(e) => onInputChange(e.target.value)}
        className="flex-1 bg-transparent border-none outline-none text-[#e0e0e0] font-mono text-sm p-0 m-0 leading-6 focus:ring-0 focus:outline-none focus:border-none placeholder-transparent"
        autoComplete="off"
        spellCheck={false}
        disabled={!connected}
      />
    </form>
  )
}
