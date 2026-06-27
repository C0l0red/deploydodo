import { useState, useRef, useEffect, useCallback } from 'react'
import { SectionCard, SectionHeader } from '..'
import { useTerminalContext } from '@/hooks/useTerminal'
import type { TerminalMessage } from '@/hooks/useTerminal'
import { useContainersQuery } from '@/api/queries'
import { colorizeAnsi } from '@/utilities/ansi'

type Props = {
  serverId: number
}

type LineKind = 'input' | 'stdout' | 'stderr'

type Line = {
  text: string
  kind: LineKind
}

export function TerminalTab({ serverId }: Props) {
  const { data: containers, isLoading } = useContainersQuery(serverId)

  const [selectedContainer, setSelectedContainer] = useState<string | null>(
    null,
  )
  const [lines, setLines] = useState<Line[]>([])
  const [terminalInput, setTerminalInput] = useState('')
  const [error, setError] = useState<string | null>(null)
  const [currentDir, setCurrentDir] = useState('/')

  const containerRef = useRef<HTMLDivElement>(null)
  const autoConnected = useRef(false)
  const [terminalScrollable, setTerminalScrollable] = useState(false)

  const token = useCallback(() => {
    return localStorage.getItem('session_token') ?? ''
  }, [])

  const { connected, connect, runCommand, addListener, removeListener } =
    useTerminalContext()

  const onOutput = useCallback(
    (msg: TerminalMessage) => {
      if (msg.type === 'stdout') {
        setLines((prev) => [...prev, { text: msg.data, kind: 'stdout' }])
      } else if (msg.type === 'stderr') {
        setLines((prev) => [...prev, { text: msg.data, kind: 'stderr' }])
      } else if (msg.type === 'error') {
        setError(msg.message)
      }
    },
    [],
  )

  useEffect(() => {
    addListener(onOutput)
    return () => removeListener(onOutput)
  }, [addListener, removeListener, onOutput])

  useEffect(() => {
    const terminal = containerRef.current
    if (!terminal) return
    const main = terminal.closest('main')
    if (!main) return

    const checkSticky = () => {
      const sticky = main.querySelector('.sticky') as HTMLElement | null
      if (!sticky) return
      setTerminalScrollable(
        sticky.getBoundingClientRect().top <=
          main.getBoundingClientRect().top + 1,
      )
    }

    main.addEventListener('scroll', checkSticky, { passive: true })
    checkSticky()
    return () => main.removeEventListener('scroll', checkSticky)
  }, [])

  useEffect(() => {
    const input = document.getElementById('terminal-input')
    if (input) input.focus({ preventScroll: true })
  }, [])

  useEffect(() => {
    const c = containerRef.current
    if (c) c.scrollTop = c.scrollHeight
  }, [lines])

  useEffect(() => {
    if (autoConnected.current || !containers || containers.length === 0) return
    autoConnected.current = true
    handleConnect(containers[0].id)
  }, [containers])

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault()
    const cmd = terminalInput.trim()
    if (!cmd || !selectedContainer) return

    setLines((prev) => [...prev, { text: cmd, kind: 'input' }])

    if (cmd === 'clear') {
      setLines([])
      setTerminalInput('')
      return
    }

    setTerminalInput('')
    const messages = await runCommand(selectedContainer, cmd)
    for (const msg of messages) {
      if (msg.type === 'cd') {
        setCurrentDir(msg.dir)
      } else if (msg.type === 'stdout') {
        setLines((prev) => [...prev, { text: msg.data, kind: 'stdout' }])
      } else if (msg.type === 'stderr') {
        setLines((prev) => [...prev, { text: msg.data, kind: 'stderr' }])
      } else if (msg.type === 'error') {
        setError(msg.message)
      }
    }
  }

  const handleConnect = (containerId: string) => {
    setError(null)
    setCurrentDir('/')
    setSelectedContainer(containerId)
    if (!connected) {
      setLines([])
      connect(serverId, token())
    }
  }

  function promptText(): string {
    const dir = currentDir === '/root' ? '~' : currentDir
    return `root@deploydodo:${dir}#`
  }

  function renderOutput(line: Line): React.ReactNode {
    if (line.kind === 'input') {
      return (
        <>
          <span className="text-[#5faf5f] select-none">
            {promptText()}
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

  if (isLoading) {
    return (
      <SectionCard>
        <SectionHeader
          title="Terminal"
          subtitle="Loading containers..."
        />
        <div className="flex items-center justify-center py-20">
          <span className="font-manrope text-sm text-text-secondary">
            Looking for running containers...
          </span>
        </div>
      </SectionCard>
    )
  }

  if (!containers || containers.length === 0) {
    return (
      <SectionCard>
        <SectionHeader
          title="Terminal"
          subtitle="No containers available"
        />
        <div className="flex items-center justify-center py-20">
          <span className="font-manrope text-sm text-text-secondary">
            No running containers found on this server.
          </span>
        </div>
      </SectionCard>
    )
  }

  if (!selectedContainer || !connected) {
    return (
      <SectionCard>
        <SectionHeader
          title="Terminal"
          subtitle={connected ? 'Connecting...' : 'Establishing connection...'}
        />
        <div className="flex items-center justify-center py-20">
          <span className="font-manrope text-sm text-text-secondary">
            {connected
              ? 'Selecting container...'
              : 'Establishing connection...'}
          </span>
        </div>
      </SectionCard>
    )
  }

  return (
    <SectionCard>
      <SectionHeader
        title="Terminal"
        subtitle={`Connected — ${selectedContainer.slice(0, 12)}...`}
      />
      {error && (
        <div className="mb-4 px-3 py-2 rounded bg-[#3a2020] border border-[#d75f5f] font-manrope text-sm text-[#d75f5f]">
          {error}
          <button
            onClick={() => {
              setError(null)
              setSelectedContainer(null)
            }}
            className="ml-3 underline hover:no-underline"
          >
            Pick another container
          </button>
        </div>
      )}
      <div
        ref={containerRef}
        className={`border border-neutral-100 rounded-xl py-5 px-3 min-h-[550px] font-mono text-sm flex flex-col bg-[#1c1c1c] text-[#c6c6c6] select-text cursor-text ${terminalScrollable ? 'max-h-[700px] overflow-y-auto' : 'overflow-y-hidden'}`}
        onClick={() =>
          document
            .getElementById('terminal-input')
            ?.focus({ preventScroll: true })
        }
      >
        {lines.map((line, i) => (
          <div
            key={i}
            className="flex items-start leading-6 py-0.5 min-h-[28px]"
          >
            <span className="w-8 text-right pr-3 text-white/15 select-none font-mono text-sm shrink-0">
              {i + 1}
            </span>
            <div className="flex-1 font-mono text-sm">
              {renderOutput(line)}
            </div>
          </div>
        ))}
        <form
          id="terminal-input-form"
          onSubmit={handleSubmit}
          className="flex items-center gap-1.5 min-w-0"
        >
          <span className="w-8 text-right pr-3 text-white/15 select-none font-mono text-sm shrink-0 leading-6">
            {lines.length + 1}
          </span>
          <span className="text-[#5faf5f] select-none shrink-0 font-mono text-sm leading-6">
            {promptText()}
          </span>
          <input
            id="terminal-input"
            type="text"
            value={terminalInput}
            onChange={(e) => setTerminalInput(e.target.value)}
            className="flex-1 bg-transparent border-none outline-none text-[#e0e0e0] font-mono text-sm p-0 m-0 leading-6 focus:ring-0 focus:outline-none focus:border-none placeholder-transparent"
            autoComplete="off"
            spellCheck={false}
            disabled={!connected}
          />
        </form>
      </div>
    </SectionCard>
  )
}
