import { createRoute, redirect } from '@tanstack/react-router'
import { rootRoute } from '@/routeConfig'
import { statusQuery, validateSessionQuery } from '@/api/queries'

export const onboardingRoute = createRoute({
  getParentRoute: () => rootRoute,
  path: '/onboarding',
  beforeLoad: async () => {
    const status = await statusQuery()
    if (status.isAdminOnboarded) {
      const validateSession = await validateSessionQuery()
      throw redirect({ to: validateSession.valid ? '/welcome' : '/login' })
    }
  },
}).lazy(() => import('./Onboarding').then((page) => page.OnboardingRoute))
