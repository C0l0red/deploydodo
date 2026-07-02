import { useState, useEffect, useCallback } from 'react'
import { SectionCard, SectionHeader } from '..'
import { useTerminalSocket } from '@/hooks/useTerminal'
import type { TerminalMessage } from '@/hooks/useTerminal'
import { useContainersQuery } from '@/api/queries'
import {
  type Line,
  LineView,
} from './terminal/TerminalOutput'
import { TerminalPrompt } from './terminal/TerminalPrompt'
import { useAutoConnect } from './terminal/useAutoConnect'
import { useAutoScroll } from './terminal/useAutoScroll'
import { useTerminalScroll } from './terminal/useTerminalScroll'

type Props = {
  serverId: number
}

function applyMessageToState(
  msg: TerminalMessage,
  setLines: React.Dispatch<React.SetStateAction<Line[]>>,
  setCurrentDir: React.Dispatch<React.SetStateAction<string>>,
  setError: React.Dispatch<React.SetStateAction<string | null>>,
) {
  switch (msg.type) {
    case 'stdout':
      setLines((prev) => [...prev, { text: msg.data, kind: 'stdout' }])
      break
    case 'stderr':
      setLines((prev) => [...prev, { text: msg.data, kind: 'stderr' }])
      break
    case 'cd':
      setCurrentDir(msg.dir)
      break
    case 'error':
      setError(msg.message)
      break
  }
}

export function TerminalTab({ serverId }: Props) {
  const { data: containers, isLoading } = useContainersQuery(serverId)

  const [selectedContainer, setSelectedContainer] = useState<string | null>(null)
  const [lines, setLines] = useState<Line[]>([])
  const [terminalInput, setTerminalInput] = useState('')
  const [error, setError] = useState<string | null>(null)
  const [currentDir, setCurrentDir] = useState('/')

  const containerRef = useAutoScroll(lines)
  const terminalScrollable = useTerminalScroll(containerRef)

  const token = useCallback(() => {
    return localStorage.getItem('session_token') ?? ''
  }, [])

  const { connected, connect, runCommand, disconnect, addListener, removeListener } =
    useTerminalSocket()

  const onOutput = useCallback(
    (msg: TerminalMessage) => {
      applyMessageToState(msg, setLines, setCurrentDir, setError)
    },
    [],
  )

  useEffect(() => {
    addListener(onOutput)
    return () => removeListener(onOutput)
  }, [addListener, removeListener, onOutput])

  useEffect(() => {
    const input = document.getElementById('terminal-input')
    if (input) input.focus({ preventScroll: true })
  }, [])

  useEffect(() => {
    return () => disconnect()
  }, [disconnect])

  const handleConnect = useCallback(
    (containerId: string) => {
      setError(null)
      setCurrentDir('/')
      setSelectedContainer(containerId)
      if (!connected) {
        setLines([])
        connect(serverId, token())
      }
    },
    [connected, connect, serverId, token],
  )

  useAutoConnect(containers, handleConnect)

  const handleSubmit = useCallback(
    async (e: React.FormEvent) => {
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
        applyMessageToState(msg, setLines, setCurrentDir, setError)
      }
    },
    [terminalInput, selectedContainer, runCommand],
  )

  if (isLoading) {
    return (
      <SectionCard>
        <SectionHeader title="Terminal" subtitle="Loading containers..." />
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
        <SectionHeader title="Terminal" subtitle="No containers available" />
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
          <LineView key={i} index={i} line={line} currentDir={currentDir} />
        ))}
        <TerminalPrompt
          currentDir={currentDir}
          lineCount={lines.length}
          connected={connected}
          terminalInput={terminalInput}
          onInputChange={setTerminalInput}
          onSubmit={handleSubmit}
        />
      </div>
    </SectionCard>
  )
}
