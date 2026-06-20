import { createRoute } from '@tanstack/react-router'
import { requireAuth, rootRoute } from '@/routeConfig'
import { DashboardLayout } from '@/layouts/DashboardLayout'
import { DashboardIndex } from './DashboardIndex'
import { Projects } from './Projects'
import { Servers } from './Servers'
import { ServerDetail } from './Servers/Detail'
import { Sources } from './Sources'
import { Destinations } from './Destinations'
import { SharedVariables } from './SharedVariables'
import { Keys } from './Keys'
import { Terminal } from './Terminal'
import { Settings } from './Settings'
import { Pending } from '@/pages/Pending/Pending'

const dashboardParentRoute = createRoute({
  getParentRoute: () => rootRoute,
  path: '/dashboard',
  beforeLoad: requireAuth,
  pendingComponent: Pending,
  component: DashboardLayout,
})

const dashboardIndexRoute = createRoute({
  getParentRoute: () => dashboardParentRoute,
  path: '/',
  component: DashboardIndex,
})

const projectsRoute = createRoute({
  getParentRoute: () => dashboardParentRoute,
  path: '/projects',
  component: Projects,
})

const serversRoute = createRoute({
  getParentRoute: () => dashboardParentRoute,
  path: '/servers',
  component: Servers,
})

const serverDetailRoute = createRoute({
  getParentRoute: () => dashboardParentRoute,
  path: '/servers/$serverId',
  component: ServerDetail,
})

const sourcesRoute = createRoute({
  getParentRoute: () => dashboardParentRoute,
  path: '/sources',
  component: Sources,
})

const destinationsRoute = createRoute({
  getParentRoute: () => dashboardParentRoute,
  path: '/destinations',
  component: Destinations,
})

const sharedVariablesRoute = createRoute({
  getParentRoute: () => dashboardParentRoute,
  path: '/shared-variables',
  component: SharedVariables,
})

const keysRoute = createRoute({
  getParentRoute: () => dashboardParentRoute,
  path: '/keys',
  component: Keys,
})

const terminalRoute = createRoute({
  getParentRoute: () => dashboardParentRoute,
  path: '/terminal',
  component: Terminal,
})

const settingsRoute = createRoute({
  getParentRoute: () => dashboardParentRoute,
  path: '/settings',
  component: Settings,
})

export const dashboardRoute = dashboardParentRoute.addChildren([
  dashboardIndexRoute,
  projectsRoute,
  serversRoute,
  serverDetailRoute,
  sourcesRoute,
  destinationsRoute,
  sharedVariablesRoute,
  keysRoute,
  terminalRoute,
  settingsRoute,
])
