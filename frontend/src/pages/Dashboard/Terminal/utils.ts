export type Status = 'connecting' | 'open' | 'closed'

export const statusLabel: Record<Status, string> = {
  connecting: 'Connecting…',
  open: 'Connected',
  closed: 'Disconnected',
}

export const statusColor: Record<Status, string> = {
  connecting: 'bg-amber-400',
  open: 'bg-green-500',
  closed: 'bg-error',
}
