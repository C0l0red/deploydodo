import { createRoute } from '@tanstack/react-router'
import { requireAuth, rootRoute } from '@/routeConfig'

const dashboardParentRoute = createRoute({
  getParentRoute: () => rootRoute,
  path: '/dashboard',
  beforeLoad: requireAuth,
}).lazy(() => import('./DashboardLayout').then((page) => page.DashboardLayoutRoute))

const dashboardIndexRoute = createRoute({
  getParentRoute: () => dashboardParentRoute,
  path: '/',
}).lazy(() => import('./DashboardIndex').then((page) => page.DashboardRoute))

const projectsRoute = createRoute({
  getParentRoute: () => dashboardParentRoute,
  path: '/projects',
}).lazy(() => import('./Projects').then((page) => page.ProjectsRoute))

const serversRoute = createRoute({
  getParentRoute: () => dashboardParentRoute,
  path: '/servers',
}).lazy(() => import('./Servers').then((page) => page.ServersRoute))

const serverDetailRoute = createRoute({
  getParentRoute: () => dashboardParentRoute,
  path: '/servers/$serverId',
}).lazy(() => import('./Servers/Detail').then((page) => page.ServerDetailRoute))

const sourcesRoute = createRoute({
  getParentRoute: () => dashboardParentRoute,
  path: '/sources',
}).lazy(() => import('./Sources').then((page) => page.SourcesRoute))

const destinationsRoute = createRoute({
  getParentRoute: () => dashboardParentRoute,
  path: '/destinations',
}).lazy(() => import('./Destinations').then((page) => page.DestinationsRoute))

const sharedVariablesRoute = createRoute({
  getParentRoute: () => dashboardParentRoute,
  path: '/shared-variables',
}).lazy(() => import('./SharedVariables').then((page) => page.SharedVariablesRoute))

const keysRoute = createRoute({
  getParentRoute: () => dashboardParentRoute,
  path: '/keys',
}).lazy(() => import('./Keys').then((page) => page.KeysRoute))

const terminalRoute = createRoute({
  getParentRoute: () => dashboardParentRoute,
  path: '/terminal',
}).lazy(() => import('./Terminal').then((page) => page.TerminalRoute))

const settingsRoute = createRoute({
  getParentRoute: () => dashboardParentRoute,
  path: '/settings',
}).lazy(() => import('./Settings').then((page) => page.SettingsRoute))

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
