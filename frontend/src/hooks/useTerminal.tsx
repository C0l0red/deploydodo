import {
  createContext,
  useContext,
  useRef,
  useState,
  useCallback,
  type ReactNode,
} from 'react'

export type TerminalMessage =
  | { type: 'stdout'; data: string }
  | { type: 'stderr'; data: string }
  | { type: 'error'; message: string }
  | { type: 'cd'; dir: string }

type Listener = (msg: TerminalMessage) => void

type TerminalContextValue = {
  connected: boolean
  connect: (serverId: number, token: string) => void
  runCommand: (container: string, cmd: string) => Promise<TerminalMessage[]>
  disconnect: () => void
  addListener: (fn: Listener) => void
  removeListener: (fn: Listener) => void
}

const TerminalContext = createContext<TerminalContextValue | null>(null)

export function useTerminalContext() {
  const ctx = useContext(TerminalContext)
  if (!ctx) throw new Error('Missing TerminalProvider')
  return ctx
}

export function TerminalProvider({ children }: { children: ReactNode }) {
  const wsRef = useRef<WebSocket | null>(null)
  const [connected, setConnected] = useState(false)
  const pendingRef = useRef<{
    resolve: (msgs: TerminalMessage[]) => void
    messages: TerminalMessage[]
  } | null>(null)
  const listenersRef = useRef<Set<Listener>>(new Set())

  const addListener = useCallback((fn: Listener) => {
    listenersRef.current.add(fn)
  }, [])

  const removeListener = useCallback((fn: Listener) => {
    listenersRef.current.delete(fn)
  }, [])

  const notify = useCallback((msg: TerminalMessage) => {
    listenersRef.current.forEach((fn) => fn(msg))
  }, [])

  const connect = useCallback(
    (serverId: number, token: string) => {
      // Close existing connection if any
      wsRef.current?.close()
      pendingRef.current = null

      const protocol = window.location.protocol === 'https:' ? 'wss' : 'ws'
      const host = window.location.host
      const url = `${protocol}://${host}/api/servers/${serverId}/terminal?token=${encodeURIComponent(token)}`

      const ws = new WebSocket(url)
      wsRef.current = ws

      ws.onopen = () => setConnected(true)

      ws.onmessage = (e) => {
        try {
          const msg = JSON.parse(e.data) as TerminalMessage | { type: 'done' }
          if (msg.type === 'stdout' || msg.type === 'stderr') {
            const pending = pendingRef.current
            if (pending) {
              pending.messages.push(msg)
            } else {
              notify(msg)
            }
          } else if (msg.type === 'error') {
            const pending = pendingRef.current
            if (pending) {
              pending.messages.push(msg)
              pending.resolve([...pending.messages])
              pendingRef.current = null
            } else {
              notify(msg)
            }
          } else if (msg.type === 'done') {
            const pending = pendingRef.current
            if (pending) {
              pending.resolve([...pending.messages])
              pendingRef.current = null
            }
          } else if (msg.type === 'cd') {
            const pending = pendingRef.current
            if (pending) {
              pending.messages.push(msg)
              pending.resolve([...pending.messages])
              pendingRef.current = null
            }
          }
        } catch {
          // ignore parse errors
        }
      }

      ws.onclose = () => {
        setConnected(false)
        pendingRef.current = null
      }
      ws.onerror = () => setConnected(false)
    },
    [notify],
  )

  const runCommand = useCallback(
    (container: string, cmd: string): Promise<TerminalMessage[]> => {
      const ws = wsRef.current
      if (!ws || ws.readyState !== WebSocket.OPEN) {
        return Promise.resolve([
          { type: 'error', message: 'Not connected' } as TerminalMessage,
        ])
      }

      return new Promise((resolve) => {
        pendingRef.current = { resolve, messages: [] }
        ws.send(JSON.stringify({ type: 'run', container, cmd }))
      })
    },
    [],
  )

  const disconnect = useCallback(() => {
    wsRef.current?.close()
    wsRef.current = null
    setConnected(false)
  }, [])

  return (
    <TerminalContext.Provider
      value={{
        connected,
        connect,
        runCommand,
        disconnect,
        addListener,
        removeListener,
      }}
    >
      {children}
    </TerminalContext.Provider>
  )
}
