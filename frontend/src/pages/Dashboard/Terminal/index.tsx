import { useState } from 'react'
import { motion } from 'framer-motion'
import { StaggerContainer, StaggerItem, staggerItemVariants } from '@/components/Animated'
import { useServersQuery } from '@/api/queries'
import { TerminalSession } from './TerminalSession'
import { createLazyRoute } from '@tanstack/react-router'

export const TerminalRoute = createLazyRoute('terminal')({
  component: Terminal
})

function Terminal() {
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
