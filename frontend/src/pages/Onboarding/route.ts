import { createRoute, redirect } from '@tanstack/react-router'
import { rootRoute } from '@/routeConfig'
import { Onboarding } from './Onboarding'
import { Pending } from '@/pages/Pending/Pending'

export const onboardingRoute = createRoute({
  getParentRoute: () => rootRoute,
  path: '/onboarding',
  beforeLoad: async () => {
    const { queryClient } = await import('@/api/client')
    const { statusQueryOptions, validateSessionOptions } = await import('@/api/queries')

    const status = await queryClient.ensureQueryData(statusQueryOptions)
    if (status.isAdminOnboarded) {
      const validateSession = await queryClient.ensureQueryData(validateSessionOptions)
      throw redirect({ to: validateSession.valid ? '/welcome' : '/login' })
    }
  },
  pendingComponent: Pending,
  component: Onboarding,
})
