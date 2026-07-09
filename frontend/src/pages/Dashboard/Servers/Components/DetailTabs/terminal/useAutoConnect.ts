import { useEffect, useRef } from 'react'

export function useAutoConnect(
  containers: { id: string }[] | undefined,
  onConnect: (containerId: string) => void,
) {
  const autoConnected = useRef(false)

  useEffect(() => {
    if (autoConnected.current || !containers || containers.length === 0) return
    autoConnected.current = true
    onConnect(containers[0].id)
  }, [containers, onConnect])
}
