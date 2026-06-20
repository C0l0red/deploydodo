import { useState, useRef, useEffect } from 'react'
import { SectionCard, SectionHeader } from '..'

export function TerminalTab() {
  const [terminalLines, setTerminalLines] = useState<string[]>([])
  const [terminalInput, setTerminalInput] = useState('')
  const containerRef = useRef<HTMLDivElement>(null)

  useEffect(() => {
    const container = containerRef.current
    const inputForm = document.getElementById('terminal-input-form')
    if (container && inputForm) {
      const formBottom = inputForm.offsetTop + inputForm.offsetHeight
      if (formBottom > container.scrollTop + container.clientHeight) {
        container.scrollTop = formBottom - container.clientHeight + 40
      }
    }
  }, [terminalLines])

  // Focus the input on mount without scrolling the page
  useEffect(() => {
    const input = document.getElementById('terminal-input')
    if (input) {
      input.focus({ preventScroll: true })
    }
  }, [])

  const handleTerminalSubmit = (e: React.FormEvent) => {
    e.preventDefault()
    if (!terminalInput.trim()) return
    const cmd = terminalInput.trim()
    const append = (lines: string[]) =>
      setTerminalLines((prev) => [...prev, `root@DeployDodo-explorer:~# ${cmd}`, ...lines])
    if (cmd === 'help') {
      append(['Available commands: help, clear, docker ps, uname -a, ls'])
    } else if (cmd === 'clear') {
      setTerminalLines([])
    } else if (cmd === 'docker ps') {
      append([
        'CONTAINER ID   IMAGE              STATUS         PORTS',
        'a1b2c3d4e5f6   traefik:v3.0       Up 3 hours     0.0.0.0:80->80/tcp',
        'f6e5d4c3b2a1   postgres:15        Up 3 hours     0.0.0.0:5432->5432/tcp',
      ])
    } else if (cmd === 'uname -a') {
      append(['Linux DeployDodo-explorer 6.1.0-21-amd64 #1 SMP x86_64 GNU/Linux'])
    } else if (cmd === 'ls') {
      append(['deploydodo  docker  logs  ssl  tmp'])
    } else {
      append([`bash: ${cmd}: command not found`])
    }
    setTerminalInput('')
  }

  return (
    <SectionCard>
      <SectionHeader
        title="Terminal"
        subtitle="Configuration file ( /data/DeployDodo/proxy/docker-compose.yml )"
      />
      <div
        ref={containerRef}
        className="border border-neutral-100 rounded-xl py-5 px-3 min-h-[550px] max-h-[700px] overflow-y-auto font-mono text-sm flex flex-col bg-white select-text cursor-text"
        onClick={() => document.getElementById('terminal-input')?.focus({ preventScroll: true })}
      >
        {(() => {
          const totalMinLines = 29
          const displayRows: (
            | { type: 'history'; content: string; key: string }
            | { type: 'input'; key: string }
            | { type: 'empty'; key: string }
          )[] = []

          terminalLines.forEach((line, i) => {
            displayRows.push({ type: 'history', content: line, key: `hist-${i}` })
          })

          displayRows.push({ type: 'input', key: 'active-input' })

          while (displayRows.length < totalMinLines) {
            displayRows.push({ type: 'empty', key: `empty-${displayRows.length}` })
          }

          return displayRows.map((row, idx) => {
            const lineNum = idx + 1
            return (
              <div key={row.key} className="flex items-start leading-6 py-0.5 min-h-[28px]">
                <span className="w-10 text-right pr-4 text-text-secondary/40 select-none font-mono text-sm">
                  {lineNum}
                </span>
                {row.type === 'history' && (
                  <div className="flex-1 font-mono text-sm whitespace-pre-wrap select-text">
                    {row.content.startsWith('root@DeployDodo-explorer:~# ') ? (
                      <>
                        <span className="text-text-secondary select-none">root@DeployDodo-explorer:~# </span>
                        <span className="text-high-contrast font-semibold">
                          {row.content.substring('root@DeployDodo-explorer:~# '.length)}
                        </span>
                      </>
                    ) : (
                      <span className="text-text-secondary/80">{row.content}</span>
                    )}
                  </div>
                )}
                {row.type === 'input' && (
                  <form id="terminal-input-form" onSubmit={handleTerminalSubmit} className="flex-1 flex items-center gap-1.5 min-w-0">
                    <span className="text-text-secondary select-none shrink-0 font-mono text-sm leading-6">
                      root@DeployDodo-explorer:~#
                    </span>
                    <input
                      id="terminal-input"
                      type="text"
                      value={terminalInput}
                      onChange={(e) => setTerminalInput(e.target.value)}
                      className="flex-1 bg-transparent border-none outline-none text-high-contrast font-semibold font-mono text-sm p-0 m-0 leading-6 focus:ring-0 focus:outline-none focus:border-none"
                      autoComplete="off"
                      spellCheck={false}
                    />
                  </form>
                )}
                {row.type === 'empty' && <div className="flex-1" />}
              </div>
            )
          })
        })()}
        <div />
      </div>
    </SectionCard>
  )
}
