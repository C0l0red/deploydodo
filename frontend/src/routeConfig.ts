import { createRootRoute, Outlet, redirect } from '@tanstack/react-router'

export const rootRoute = createRootRoute({ component: Outlet })

export async function requireAuth() {
  const { queryClient } = await import('@/api/client')
  const { validateSessionOptions, statusQueryOptions } = await import('@/api/queries')

  const validateResponse = await queryClient.ensureQueryData(validateSessionOptions)
  if (!validateResponse.valid) throw redirect({ to: '/login' })

  const status = await queryClient.ensureQueryData(statusQueryOptions)
  if (!status.isAdminOnboarded) throw redirect({ to: '/onboarding' })

  return { status }
}
