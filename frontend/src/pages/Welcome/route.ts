import { createRoute, redirect } from '@tanstack/react-router'
import { requireAuth, rootRoute } from '@/routeConfig'
import { statusQuery } from '@/api/queries'

export const welcomeRoute = createRoute({
  getParentRoute: () => rootRoute,
  path: '/welcome',
  beforeLoad: async () => {
    const { status } = await requireAuth()
    if (status.isServerSetup) throw redirect({ to: '/dashboard' })
  },
  loader: statusQuery,
}).lazy(() => import('@/pages/Welcome/Welcome').then((page) => page.WelcomeRoute))
