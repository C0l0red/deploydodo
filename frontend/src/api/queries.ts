import { queryOptions, useQuery } from '@tanstack/react-query'
import { api, handleQuery, queryClient } from '@/api/client'

const statusQueryOptions = queryOptions({
  queryKey: ['status'],
  queryFn: handleQuery(api.status),
})

export const statusQuery = () => queryClient.ensureQueryData(statusQueryOptions)

export const invalidateStatusQuery = () =>
  queryClient.invalidateQueries({
    queryKey: statusQueryOptions.queryKey,
    exact: true,
    refetchType: 'all',
  })

export function useStatusQuery() {
  return useQuery(statusQueryOptions)
}

const serversQueryOptions = queryOptions({
  queryKey: ['servers'],
  queryFn: handleQuery(api.listServers),
})

export const invalidateServersQuery = () =>
  queryClient.invalidateQueries({
    queryKey: serversQueryOptions.queryKey,
    exact: true,
    refetchType: 'all',
  })

export const serversQuery = () => queryClient.ensureQueryData(serversQueryOptions)

export function useServersQuery() {
  return useQuery(serversQueryOptions)
}

const validateSessionOptions = queryOptions({
  queryKey: ['validateSession'],
  queryFn: handleQuery(api.validateSession),
})

export const validateSessionQuery = () => queryClient.ensureQueryData(validateSessionOptions)
