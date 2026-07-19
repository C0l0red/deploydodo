import { queryOptions, useQuery } from '@tanstack/react-query'
import { api, handleQuery, queryClient } from '@/api/client'

const statusOptions = queryOptions({
  queryKey: ['status'],
  queryFn: handleQuery(api.status),
})

export const statusQuery = () => queryClient.ensureQueryData(statusOptions)

export const invalidateStatusQuery = () =>
  queryClient.invalidateQueries({
    queryKey: statusOptions.queryKey,
    exact: true,
    refetchType: 'all',
  })

export function useStatusQuery() {
  return useQuery(statusOptions)
}

const serversOptions = queryOptions({
  queryKey: ['servers'],
  queryFn: handleQuery(api.listServers),
})

export const invalidateServersQuery = () =>
  queryClient.invalidateQueries({
    queryKey: serversOptions.queryKey,
    exact: true,
    refetchType: 'all',
  })

export const serversQuery = () => queryClient.ensureQueryData(serversOptions)

export function useServersQuery() {
  return useQuery(serversOptions)
}

const validateSessionOptions = queryOptions({
  queryKey: ['validateSession'],
  queryFn: handleQuery(api.validateSession),
  staleTime: 5000,
})

export const validateSessionQuery = () => queryClient.ensureQueryData(validateSessionOptions)

export const invalidateValidateSessionQuery = () =>
  queryClient.invalidateQueries({
    queryKey: validateSessionOptions.queryKey,
    exact: true,
    refetchType: 'all',
  })
