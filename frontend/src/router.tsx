import { createRoute, createRouter, redirect } from '@tanstack/react-router'
import { rootRoute } from '@/routeConfig'
import { onboardingRoute } from '@/pages/Onboarding/route'
import { welcomeRoute } from '@/pages/Welcome/route'
import { selectServerRoute } from '@/pages/Setup/route'
import { dashboardRoute } from '@/pages/Dashboard/route'
import { loginRoute } from '@/pages/Login/route'

const indexRoute = createRoute({
  getParentRoute: () => rootRoute,
  path: '/',
  beforeLoad: () => redirect({ to: '/onboarding' }),
})

const routeTree = rootRoute.addChildren([
  indexRoute,
  onboardingRoute,
  welcomeRoute,
  selectServerRoute,
  loginRoute,
  dashboardRoute,
])

export const router = createRouter({ routeTree })

declare module '@tanstack/react-router' {
  interface Register {
    router: typeof router
  }
}
