import { useState } from 'react'
import { Link } from '@tanstack/react-router'
import { cn } from '@/utilities/cn'
import { OutlineButton } from './Components'
import { ConfigurationTab } from './Components/DetailTabs/ConfigurationTab'
import { ProxyTab } from './Components/DetailTabs/ProxyTab'
import { ResourcesTab } from './Components/DetailTabs/ResourcesTab'
import { TerminalTab } from './Components/DetailTabs/TerminalTab'
import { SecurityTab } from './Components/DetailTabs/SecurityTab'

// ─── Types ────────────────────────────────────────────────────────────────────
type Tab = 'Configuration' | 'Proxy' | 'Resources' | 'Terminal' | 'Security'

// ═══════════════════════════════════════════════════════════════════════════════
export function ServerDetail() {
  const [activeTab, setActiveTab] = useState<Tab>('Configuration')

  // ─── Tab definitions ─────────────────────────────────────────────────────────
  const tabs: Tab[] = ['Configuration', 'Proxy', 'Resources', 'Terminal', 'Security']

  // ─── Render ──────────────────────────────────────────────────────────────────
  return (
    <div className="flex flex-col gap-5">
      {/* Back link */}
      <Link
        to="/dashboard/servers"
        className="inline-flex items-center gap-2 font-sans font-normal text-sm leading-6 text-text-secondary hover:text-high-contrast transition-colors duration-150 w-fit"
      >
        <svg className="size-4 shrink-0" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2">
          <line x1="19" y1="12" x2="5" y2="12" />
          <polyline points="12 19 5 12 12 5" />
        </svg>
        Back to Servers
      </Link>

      {/* Header */}
      <div className="flex items-center gap-3">
        <h1 className="font-sans font-semibold text-[40px] leading-none tracking-[-0.5px] text-high-contrast m-0">
          Localhost
        </h1>
        <span className="font-manrope font-semibold text-xs leading-4 px-2 py-1 rounded-md bg-[#eaf6ec] text-[#2e7d32]">
          Currently used
        </span>
      </div>

      {/* Tabs + action buttons */}
      <div className="flex items-end justify-between border-b border-neutral-100">
        <div className="flex gap-6">
          {tabs.map((tab) => (
            <button
              key={tab}
              onClick={() => setActiveTab(tab)}
              className={cn(
                'font-manrope text-sm leading-6 pb-2 transition-colors duration-150 outline-none',
                activeTab === tab
                  ? 'font-bold text-high-contrast border-b-2 border-high-contrast -mb-px'
                  : 'font-normal text-text-secondary hover:text-high-contrast'
              )}
            >
              {tab}
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

      {/* ─── TAB CONTENT ────────────────────────────────────────────────────────── */}
      <div className="mt-4">
        {activeTab === 'Configuration' && <ConfigurationTab />}
        {activeTab === 'Proxy' && <ProxyTab />}
        {activeTab === 'Resources' && <ResourcesTab />}
        {activeTab === 'Terminal' && <TerminalTab />}
        {activeTab === 'Security' && <SecurityTab />}
      </div>
    </div>
  )
}
