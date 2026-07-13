import { createRoute } from '@tanstack/react-router'
import { requireAuth, rootRoute } from '@/routeConfig'

export const selectServerRoute = createRoute({
  getParentRoute: () => rootRoute,
  path: '/setup/server',
  beforeLoad: requireAuth,
}).lazy(() => import('.').then((page) => page.SetupRoute))
