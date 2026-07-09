import { useRef, useState } from 'react'
import { motion } from 'framer-motion'
import { StaggerContainer, StaggerItem, staggerItemVariants } from '@/components/Animated'
import { useServersQuery } from '@/api/queries'
import { useTerminalSocket } from '@/hooks/useTerminalSocket'

type Status = 'connecting' | 'open' | 'closed'

const statusLabel: Record<Status, string> = {
  connecting: 'Connecting…',
  open: 'Connected',
  closed: 'Disconnected',
}

const statusColor: Record<Status, string> = {
  connecting: 'bg-amber-400',
  open: 'bg-green-500',
  closed: 'bg-error',
}

function TerminalSession({ serverId }: { serverId: number }) {
  const containerRef = useRef<HTMLDivElement>(null)
  const [status, setStatus] = useState<Status>('connecting')

  useTerminalSocket(containerRef, serverId, setStatus)

  return (
    <div className="flex flex-col gap-2 rounded-lg overflow-hidden border border-neutral-100">
      <div className="flex items-center gap-2 px-4 py-2 bg-secondary">
        <span className={`size-2 rounded-full ${statusColor[status]}`} />
        <span className="font-manrope text-sm text-pure-white">{statusLabel[status]}</span>
      </div>
      {/* xterm mounts here; height drives the fit-addon cols/rows measurement */}
      <div ref={containerRef} className="h-130 w-full bg-high-contrast px-2" />
    </div>
  )
}

export function Terminal() {
  const { data: servers, isLoading } = useServersQuery()
  const [selectedId, setSelectedId] = useState<number | null>(null)

  const activeId = selectedId ?? servers?.[0]?.id ?? null

  return (
    <StaggerContainer className="flex flex-col gap-6">
      <div className="flex items-end justify-between">
        <StaggerItem className="flex flex-col gap-2">
          <h1 className="font-sans font-semibold text-[40px] leading-12 tracking-[-0.5px] text-high-contrast m-0">
            Terminal
          </h1>
          <p className="font-sans font-normal text-base leading-6 text-text-secondary m-0">
            Access your server's terminal session
          </p>
        </StaggerItem>

        {servers && servers.length > 0 && (
          <motion.select
            variants={staggerItemVariants}
            value={activeId ?? undefined}
            onChange={(e) => setSelectedId(Number(e.target.value))}
            className="border border-text-secondary rounded-lg px-3 py-2 font-manrope text-sm text-high-contrast bg-background"
          >
            {servers.map((s) => (
              <option key={s.id} value={s.id}>
                {s.name}
              </option>
            ))}
          </motion.select>
        )}
      </div>

      <StaggerItem>
        {isLoading ? (
          <p className="font-manrope text-sm text-text-secondary">Loading servers…</p>
        ) : activeId == null ? (
          <p className="font-manrope text-sm text-text-secondary">
            No servers available. Add a server to open a terminal.
          </p>
        ) : (
          // key forces a fresh terminal + socket when switching servers
          <TerminalSession key={activeId} serverId={activeId} />
        )}
      </StaggerItem>
    </StaggerContainer>
  )
}
