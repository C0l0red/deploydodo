import { Outlet, useNavigate, useRouter, useLocation } from '@tanstack/react-router'
import { Logo } from '@/components/Logo'
import {
  SearchIcon,
  BellIcon,
  HelpIcon,
  NavDashboardIcon,
  NavProjectsIcon,
  NavServersIcon,
  NavSourcesIcon,
  NavDestinationsIcon,
  NavSharedVariablesIcon,
  NavKeysIcon,
  NavTerminalIcon,
  NavSettingsIcon,
  NavLogoutIcon,
} from '@/assets/icons'
import type { ComponentType, SVGProps } from 'react'
import { useLayoutEffect, useRef, useState } from 'react'
import { cn } from '@/utilities/cn'

type NavItemDef = {
  Icon: ComponentType<SVGProps<SVGSVGElement>>
  label: string
  to: string
}

const mainNavItems: NavItemDef[] = [
  { Icon: NavDashboardIcon, label: 'Dashboard', to: '/dashboard' },
  { Icon: NavProjectsIcon, label: 'Projects', to: '/dashboard/projects' },
  { Icon: NavServersIcon, label: 'Servers', to: '/dashboard/servers' },
  { Icon: NavSourcesIcon, label: 'Sources', to: '/dashboard/sources' },
  { Icon: NavDestinationsIcon, label: 'Destinations', to: '/dashboard/destinations' },
  { Icon: NavSharedVariablesIcon, label: 'Shared Variables', to: '/dashboard/shared-variables' },
  { Icon: NavKeysIcon, label: 'Keys & Tokens', to: '/dashboard/keys' },
  { Icon: NavTerminalIcon, label: 'Terminal', to: '/dashboard/terminal' },
  { Icon: NavSettingsIcon, label: 'Settings', to: '/dashboard/settings' },
]

function NavItem({ Icon, label, to }: NavItemDef) {
  const navigate = useNavigate()
  const router = useRouter()
  const pathname = router.state.location.pathname
  const isActive =
    to === '/dashboard'
      ? pathname === to
      : pathname === to || pathname.startsWith(to + '/')

  return (
    <button
      onClick={() => navigate({ to })}
      className={cn([
        'flex items-center gap-2 w-full pl-3 pr-2 py-2 rounded-lg text-left transition-colors duration-150',
        isActive ? 'bg-neutral-200' : 'hover:bg-neutral-200',
      ])}
    >
      <Icon className="size-4 shrink-0" />
      <span
        className={cn([
          'font-manrope text-sm leading-6',
          isActive ? 'font-bold text-secondary' : 'font-normal text-high-contrast',
        ])}
      >
        {label}
      </span>
    </button>
  )
}

export function DashboardLayout() {
  const navigate = useNavigate()
  const location = useLocation()
  const [ready, setReady] = useState(false)
  const isFirstRender = useRef(true)

  useLayoutEffect(() => {
    if (isFirstRender.current) {
      isFirstRender.current = false
      setReady(true)
      return
    }
    setReady(false)
    const id = requestAnimationFrame(() => setReady(true))
    return () => cancelAnimationFrame(id)
  }, [location.pathname])

  return (
    <div className="h-screen bg-background flex flex-col">
      <header className="h-16 flex items-center shrink-0">
        <div className="w-60 h-full flex items-center px-6 border-r border-b border-neutral-100 shrink-0">
          <Logo />
        </div>

        <div className="flex-1 flex items-center justify-between pl-3 pr-6 border-b border-neutral-100 h-full">
          <div className="w-80 flex items-center gap-2 px-2 py-2 border border-neutral-100 rounded-lg">
            <SearchIcon className="size-6 shrink-0" />
            <span className="flex-1 font-manrope font-normal text-sm leading-6 text-text-secondary">
              Search input field
            </span>
            <span className="font-manrope font-normal text-[12px] leading-4 text-text-secondary">
              ⌘+K
            </span>
          </div>

          <div className="flex items-center gap-4">
            <button className="size-9 flex items-center justify-center rounded-lg hover:bg-neutral-200 transition-colors">
              <BellIcon className="size-6" />
            </button>
            <button className="size-9 flex items-center justify-center rounded-lg hover:bg-neutral-200 transition-colors">
              <HelpIcon className="size-6" />
            </button>
            <button className="size-8 rounded-full bg-primary-darker flex items-center justify-center shrink-0">
              <span className="font-manrope font-bold text-sm leading-6 text-pure-white">JS</span>
            </button>
          </div>
        </div>
      </header>

      <div className="flex flex-1 overflow-hidden">
        <nav className="w-60 border-r border-neutral-100 flex flex-col justify-between pt-6 px-6 pb-2 shrink-0">
          <div className="flex flex-col gap-2">
            {mainNavItems.map((item) => (
              <NavItem key={item.to} {...item} />
            ))}
          </div>
          <div className="flex flex-col gap-2">
            <div className="border-t border-neutral-100" />
            <button
              onClick={() => {
                localStorage.removeItem('session_token')
                navigate({ to: '/login' })
              }}
              className="flex items-center gap-2 w-full pl-3 pr-2 py-2 rounded-lg text-left transition-colors duration-150 hover:bg-neutral-200"
            >
              <NavLogoutIcon className="size-4 shrink-0" />
              <span className="font-manrope font-normal text-sm leading-6 text-high-contrast">
                Logout
              </span>
            </button>
          </div>
        </nav>

        <main className="flex-1 p-8 overflow-y-auto">
          <div
            key={location.pathname}
            className={`transition-all duration-150 ease-out ${ready ? 'opacity-100 translate-y-0' : 'opacity-0 translate-y-1'}`}
          >
            <Outlet />
          </div>
        </main>
      </div>
    </div>
  )
}
