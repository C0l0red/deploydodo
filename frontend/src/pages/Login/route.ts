import { createRoute, redirect } from '@tanstack/react-router'
import { rootRoute } from '@/routeConfig'
import { statusQuery } from '@/api/queries'

export const loginRoute = createRoute({
  getParentRoute: () => rootRoute,
  path: '/login',
  beforeLoad: async () => {
    const { statusQuery } = await import('@/api/queries')

    const status = await statusQuery()
    if (!status.isAdminOnboarded) {
      throw redirect({ to: '/onboarding' })
    }
  },
  loader: statusQuery,
}).lazy(() => import('./Login').then((page) => page.LoginRoute))
