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
    const { queryClient } = await import('@/api/client')
    const { statusQueryOptions } = await import('@/api/queries')

    return await queryClient.ensureQueryData(statusQueryOptions)
  },
  pendingComponent: Pending,
  component: Welcome,
})
