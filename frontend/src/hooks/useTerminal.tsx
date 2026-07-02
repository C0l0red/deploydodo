import { useRef, useState, useCallback } from 'react'

export type TerminalMessage =
  | { type: 'stdout'; data: string }
  | { type: 'stderr'; data: string }
  | { type: 'error'; message: string }
  | { type: 'cd'; dir: string }

type Listener = (msg: TerminalMessage) => void

type PendingRequest = {
  resolve: (msgs: TerminalMessage[]) => void
  messages: TerminalMessage[]
}

const PING_INTERVAL_MS = 30_000

function routeMessage(
  msg: TerminalMessage,
  pending: PendingRequest | null,
  notify: (msg: TerminalMessage) => void,
  clearPending: () => void,
) {
  switch (msg.type) {
    case 'stdout':
    case 'stderr':
      if (pending) {
        pending.messages.push(msg)
      } else {
        notify(msg)
      }
      break
    case 'error':
      if (pending) {
        pending.messages.push(msg)
        pending.resolve([...pending.messages])
        clearPending()
      } else {
        notify(msg)
      }
      break
    case 'cd':
      if (pending) {
        pending.messages.push(msg)
        pending.resolve([...pending.messages])
        clearPending()
      }
      break
  }
}

export function useTerminalSocket() {
  const wsRef = useRef<WebSocket | null>(null)
  const pingRef = useRef<ReturnType<typeof setInterval> | null>(null)
  const pendingRef = useRef<PendingRequest | null>(null)
  const listenersRef = useRef<Set<Listener>>(new Set())
  const [connected, setConnected] = useState(false)

  const addListener = useCallback((fn: Listener) => {
    listenersRef.current.add(fn)
  }, [])

  const removeListener = useCallback((fn: Listener) => {
    listenersRef.current.delete(fn)
  }, [])

  const notify = useCallback((msg: TerminalMessage) => {
    listenersRef.current.forEach((fn) => fn(msg))
  }, [])

  const clearPending = useCallback(() => {
    pendingRef.current = null
  }, [])

  const stopPing = useCallback(() => {
    if (pingRef.current !== null) {
      clearInterval(pingRef.current)
      pingRef.current = null
    }
  }, [])

  const startPing = useCallback(() => {
    stopPing()
    pingRef.current = setInterval(() => {
      const ws = wsRef.current
      if (ws && ws.readyState === WebSocket.OPEN) {
        ws.send(JSON.stringify({ type: 'ping' }))
      }
    }, PING_INTERVAL_MS)
  }, [stopPing])

  const disconnect = useCallback(() => {
    stopPing()
    wsRef.current?.close()
    wsRef.current = null
    setConnected(false)
    pendingRef.current = null
  }, [stopPing])

  const connect = useCallback(
    (serverId: number, token: string) => {
      stopPing()
      wsRef.current?.close()
      pendingRef.current = null

      const protocol = window.location.protocol === 'https:' ? 'wss' : 'ws'
      const host = window.location.host
      const url = `${protocol}://${host}/api/servers/${serverId}/terminal?token=${encodeURIComponent(token)}`

      const ws = new WebSocket(url)
      wsRef.current = ws

      ws.onopen = () => {
        setConnected(true)
        startPing()
      }

      ws.onmessage = (e) => {
        try {
          const msg = JSON.parse(e.data) as TerminalMessage | { type: 'done' }
          if (msg.type === 'done') {
            const pending = pendingRef.current
            if (pending) {
              pending.resolve([...pending.messages])
              clearPending()
            }
            return
          }
          routeMessage(
            msg as TerminalMessage,
            pendingRef.current,
            notify,
            clearPending,
          )
        } catch {
          // ignore parse errors
        }
      }

      ws.onclose = () => {
        stopPing()
        setConnected(false)
        pendingRef.current = null
      }

      ws.onerror = () => {
        stopPing()
        setConnected(false)
      }
    },
    [notify, clearPending, startPing, stopPing],
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

  return {
    connected,
    connect,
    disconnect,
    runCommand,
    addListener,
    removeListener,
  }
}
