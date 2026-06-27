import { useState } from 'react'
import { Link, useParams } from '@tanstack/react-router'
import { motion } from 'framer-motion'
import { cn } from '@/utilities/cn'
import { OutlineButton } from './Components'
import { TabTransition } from '@/components/Animated'
import { ConfigurationTab } from './Components/DetailTabs/ConfigurationTab'
import { ProxyTab } from './Components/DetailTabs/ProxyTab'
import { ResourcesTab } from './Components/DetailTabs/ResourcesTab'
import { TerminalTab } from './Components/DetailTabs/TerminalTab'
import { SecurityTab } from './Components/DetailTabs/SecurityTab'

// ─── Types ────────────────────────────────────────────────────────────────────
type Tab = 'Configuration' | 'Proxy' | 'Resources' | 'Terminal' | 'Security'

// ─── Active tab indicator animation ──────────────────────────────────────────
const tabIndicatorVariants = {
  initial: { scaleX: 0 },
  animate: {
    scaleX: 1,
    transition: { duration: 0.2, ease: [0.25, 0.46, 0.45, 0.94] as const },
  },
} as const

// ═══════════════════════════════════════════════════════════════════════════════
export function ServerDetail() {
  const [activeTab, setActiveTab] = useState<Tab>('Configuration')
  const { serverId } = useParams({ from: '/dashboard/servers/$serverId' })

  // ─── Tab definitions ─────────────────────────────────────────────────────────
  const tabs: Tab[] = ['Configuration', 'Proxy', 'Resources', 'Terminal', 'Security']

  // ─── Tab content renderer ────────────────────────────────────────────────────
  function renderTabContent() {
    switch (activeTab) {
      case 'Configuration':
        return <ConfigurationTab />
      case 'Proxy':
        return <ProxyTab />
      case 'Resources':
        return <ResourcesTab />
      case 'Security':
        return <SecurityTab />
      default:
        return null
    }
  }

  const isTerminal = activeTab === 'Terminal'

  // ─── Render ──────────────────────────────────────────────────────────────────
  return (
    <div className="flex flex-col">
      {/* Back link */}
      <Link
        to="/dashboard/servers"
        className="inline-flex items-center gap-2 font-sans font-normal text-sm leading-6 text-text-secondary hover:text-high-contrast transition-colors duration-150 w-fit mb-5"
      >
        <svg className="size-4 shrink-0" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2">
          <line x1="19" y1="12" x2="5" y2="12" />
          <polyline points="12 19 5 12 12 5" />
        </svg>
        Back to Servers
      </Link>

      {/* Header */}
      <div className="flex items-center gap-3 mb-5">
        <h1 className="font-sans font-semibold text-[40px] leading-none tracking-[-0.5px] text-high-contrast m-0">
          Localhost
        </h1>
        <span className="font-manrope font-semibold text-xs leading-4 px-2 py-1 rounded-md bg-[#eaf6ec] text-[#2e7d32]">
          Currently used
        </span>
      </div>

      {/* Tabs + action buttons */}
      <div className="sticky top-0 z-10 bg-white pt-4 flex items-end justify-between border-b border-neutral-100">
        <div className="flex gap-6">
          {tabs.map((tab) => (
            <button
              key={tab}
              onClick={() => setActiveTab(tab)}
              className={cn(
                'relative font-manrope text-sm leading-6 pb-2 transition-colors duration-150 outline-none',
                activeTab === tab
                  ? 'font-bold text-high-contrast'
                  : 'font-normal text-text-secondary hover:text-high-contrast'
              )}
            >
              {tab}
              {activeTab === tab && (
                <motion.div
                  layoutId="activeTabIndicator"
                  className="absolute bottom-0 left-0 right-0 h-0.5 bg-high-contrast"
                  variants={tabIndicatorVariants}
                  initial="initial"
                  animate="animate"
                  transition={{
                    type: 'spring',
                    stiffness: 380,
                    damping: 30,
                  }}
                />
              )}
            </button>
          ))}
        </div>
        {activeTab === 'Proxy' && (
          <div className="flex gap-2 pb-2">
            <OutlineButton onClick={() => { }}>
              <svg className="size-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round">
                <rect x="6" y="4" width="4" height="16" /><rect x="14" y="4" width="4" height="16" />
              </svg>
              Stop Proxy
            </OutlineButton>
            <OutlineButton onClick={() => { }}>
              <svg className="size-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round">
                <path d="M23 4v6h-6" /><path d="M1 20v-6h6" />
                <path d="M3.51 9a9 9 0 0 1 14.85-3.36L23 10M1 14l4.64 4.36A9 9 0 0 0 20.49 15" />
              </svg>
              Restart Proxy
            </OutlineButton>
          </div>
        )}
      </div>

      {/* ─── TAB CONTENT ──────────────────────────────────────────────────── */}
      <div className="mt-4">
        {isTerminal ? null : (
          <TabTransition tabKey={activeTab}>
            {renderTabContent()}
          </TabTransition>
        )}
        <div className={isTerminal ? '' : 'hidden'}>
          <TerminalTab serverId={Number(serverId)} />
        </div>
      </div>
    </div>
  )
}
