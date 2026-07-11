import { useEffect, useRef } from 'react'
import { Terminal } from '@xterm/xterm'
import { FitAddon } from '@xterm/addon-fit'
import '@xterm/xterm/css/xterm.css'

type Status = 'connecting' | 'open' | 'closed'

/**
 * Mounts an xterm.js terminal into `containerRef` and bridges it to the backend
 * PTY WebSocket at `/api/servers/{serverId}/terminal`.
 *
 * The initial terminal size is measured with the fit addon and passed to the
 * backend as `cols`/`rows` query params, since that is when the remote PTY is
 * allocated (see backend `TerminalParams`).
 */
export function useTerminalSocket(
  containerRef: React.RefObject<HTMLDivElement | null>,
  serverId: number,
  onStatus?: (status: Status) => void,
) {
  // Keep the latest status callback without re-running the effect.
  const onStatusRef = useRef(onStatus)

  useEffect(() => {
    const container = containerRef.current
    if (!container) return

    const term = new Terminal({
      cursorBlink: true,
      fontFamily: 'Ubuntu, ui-monospace, SFMono-Regular, Menlo, monospace',
      fontSize: 13,
      theme: { background: '#181818', foreground: '#e2e2e2' },
    })
    const fit = new FitAddon()
    term.loadAddon(fit)
    term.open(container)

    fit.fit()

    const token = localStorage.getItem('session_token') ?? ''
    const proto = window.location.protocol === 'https:' ? 'wss' : 'ws'
    const url =
      `${proto}://${window.location.host}/api/servers/${serverId}/terminal` +
      `?cols=${term.cols}&rows=${term.rows}&token=${encodeURIComponent(token)}`

    onStatusRef.current?.('connecting')
    const ws = new WebSocket(url)
    ws.binaryType = 'arraybuffer'

    ws.onopen = () => onStatusRef.current?.('open')
    ws.onclose = () => onStatusRef.current?.('closed')

    ws.onmessage = (e) => {
      if (typeof e.data === 'string') {
        term.write(e.data)
      } else {
        term.write(new Uint8Array(e.data as ArrayBuffer))
      }
    }

    const encoder = new TextEncoder()
    const dataSub = term.onData((data) => {
      if (ws.readyState === WebSocket.OPEN) ws.send(encoder.encode(data))
    })

    let resizeTimer: ReturnType<typeof setTimeout> | undefined
    const resizeObserver = new ResizeObserver(() => {
      clearTimeout(resizeTimer)
      resizeTimer = setTimeout(() => {
        try {
          fit.fit()
        } catch {
          return
        }
        if (ws.readyState === WebSocket.OPEN) {
          ws.send(JSON.stringify({ type: 'resize', cols: term.cols, rows: term.rows }))
        }
      }, 100)
    })
    resizeObserver.observe(container)

    return () => {
      clearTimeout(resizeTimer)
      resizeObserver.disconnect()
      dataSub.dispose()
      if (ws.readyState === WebSocket.OPEN) {
        ws.close()
      }
      term.dispose()
    }
  }, [containerRef, serverId])
}
