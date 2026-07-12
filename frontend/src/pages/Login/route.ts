import { createRoute, redirect } from '@tanstack/react-router'
import { rootRoute } from '@/routeConfig'
import { Login } from './Login'

export const loginRoute = createRoute({
  getParentRoute: () => rootRoute,
  path: '/login',
  beforeLoad: async () => {
    const { queryClient } = await import('@/api/client')
    const { statusQueryOptions } = await import('@/api/queries')

    const status = await queryClient.ensureQueryData(statusQueryOptions)
    if (!status.isAdminOnboarded) {
      throw redirect({ to: '/onboarding' })
    }
  },
  loader: async () => {
    const { queryClient } = await import('@/api/client')
    const { statusQueryOptions } = await import('@/api/queries')
    return queryClient.ensureQueryData(statusQueryOptions)
  },
  component: Login,
})
