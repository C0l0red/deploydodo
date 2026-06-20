import { createRoute, redirect } from '@tanstack/react-router'
import { requireAuth, rootRoute } from '@/routeConfig'
import { Welcome } from '@/pages/Welcome/Welcome'
import { Pending } from '@/pages/Pending/Pending'

export const welcomeRoute = createRoute({
  getParentRoute: () => rootRoute,
  path: '/welcome',
  beforeLoad: async () => {
    const { status } = await requireAuth()
    if (status.isServerSetup) throw redirect({ to: '/dashboard' })
  },
  loader: async () => {
    const { getStatus } = await import('@/api/queries')
    return getStatus()
  },
  pendingComponent: Pending,
  component: Welcome,
})
