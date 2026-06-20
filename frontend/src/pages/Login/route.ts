import { createRoute } from '@tanstack/react-router'
import { rootRoute } from '@/routeConfig'
import { Login } from './Login'

export const loginRoute = createRoute({
  getParentRoute: () => rootRoute,
  path: '/login',
  loader: async () => {
    const { getStatus } = await import('@/api/queries')
    return getStatus()
  },
  component: Login,
})
