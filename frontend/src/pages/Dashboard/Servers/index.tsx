import { useState } from 'react'
import { motion } from 'framer-motion'
import { PlusIcon } from '@/assets/icons'
import { StaggerContainer, StaggerItem, staggerItemVariants } from '@/components/Animated'
import { useServersQuery } from '@/api/queries'
import { ServerCard, AddServerModal } from './Components'
import { createLazyRoute } from '@tanstack/react-router'

export const ServersRoute = createLazyRoute('servers')({
  component: Servers,
})

function Servers() {
  const [modalOpen, setModalOpen] = useState(false)
  const { data: servers, isLoading, isError, error, refetch } = useServersQuery()

  return (
    <div className="flex flex-col gap-8">
      <AddServerModal
        open={modalOpen}
        onClose={() => setModalOpen(false)}
        onSuccess={() => refetch()}
      />

      <StaggerContainer className="flex items-end justify-between">
        <StaggerItem className="flex flex-col gap-2">
          <h1 className="font-sans font-semibold text-[40px] leading-12 tracking-[-0.5px] text-high-contrast m-0">
            Servers
          </h1>
          <p className="font-sans font-normal text-base leading-6 text-text-secondary m-0">
            All your servers are here.
          </p>
        </StaggerItem>
        <motion.button
          variants={staggerItemVariants}
          onClick={() => setModalOpen(true)}
          className="flex items-center gap-2 pl-2.5 pr-4 py-2 border border-text-secondary rounded-lg hover:bg-neutral-200 transition-colors"
        >
          <PlusIcon className="size-4 shrink-0" />
          <span className="font-manrope font-bold text-sm leading-6 text-high-contrast">
            Add server
          </span>
        </motion.button>
      </StaggerContainer>

      {isError ? (
        <div className="flex flex-col items-center gap-3 py-20">
          <span className="font-manrope text-sm text-error">
            Failed to load servers: {(error as { message?: string })?.message ?? 'Unknown error'}
          </span>
          <button
            onClick={() => refetch()}
            className="font-manrope font-bold text-sm text-high-contrast border border-neutral-100 rounded-lg px-4 py-2 hover:bg-neutral-200 transition-colors"
          >
            Retry
          </button>
        </div>
      ) : isLoading ? (
        <div className="flex items-center justify-center py-20">
          <span className="font-manrope text-sm text-text-secondary">Loading servers...</span>
        </div>
      ) : servers?.length === 0 ? (
        <div className="flex items-center justify-center py-20">
          <span className="font-manrope text-sm text-text-secondary">No servers found.</span>
        </div>
      ) : (
        <div className="flex flex-wrap gap-6">
          {servers?.map((server) => (
            <ServerCard
              key={server.id}
              serverId={String(server.id)}
              name={server.name}
              description={`${server.hostname} · ${server.serverType}`}
              status="Running"
            />
          ))}
        </div>
      )}
    </div>
  )
}
